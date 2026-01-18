//! AST Installer - AST traversal and node dispatch
//!
//! This module is responsible for:
//! - Traversing the Ruby AST (Abstract Syntax Tree)
//! - Dispatching each node type to the appropriate handler
//! - Coordinating the graph construction process

use crate::env::{GlobalEnv, LocalEnv};
use crate::graph::{ChangeSet, VertexId};
use crate::source_map::SourceLocation;
use ruby_prism::Node;

use super::calls::install_method_call;
use super::definitions::{exit_scope, extract_class_name, install_class, install_method};
use super::literals::install_literal;
use super::variables::{
    install_ivar_read, install_ivar_write, install_local_var_read, install_local_var_write,
    install_self,
};

/// Build graph from AST
pub struct AstInstaller<'a> {
    genv: &'a mut GlobalEnv,
    lenv: &'a mut LocalEnv,
    changes: ChangeSet,
    source: &'a str,
}

impl<'a> AstInstaller<'a> {
    pub fn new(genv: &'a mut GlobalEnv, lenv: &'a mut LocalEnv, source: &'a str) -> Self {
        Self {
            genv,
            lenv,
            changes: ChangeSet::new(),
            source,
        }
    }

    /// Install node (returns Vertex ID)
    pub fn install_node(&mut self, node: &Node) -> Option<VertexId> {
        // Class definition
        if let Some(class_node) = node.as_class_node() {
            return self.install_class_node(&class_node);
        }

        // Method definition
        if let Some(def_node) = node.as_def_node() {
            return self.install_def_node(&def_node);
        }

        // Instance variable write: @name = value
        if let Some(ivar_write) = node.as_instance_variable_write_node() {
            let ivar_name =
                String::from_utf8_lossy(ivar_write.name().as_slice()).to_string();
            let value_vtx = self.install_node(&ivar_write.value())?;
            return Some(install_ivar_write(self.genv, ivar_name, value_vtx));
        }

        // Instance variable read: @name
        if let Some(ivar_read) = node.as_instance_variable_read_node() {
            let ivar_name = String::from_utf8_lossy(ivar_read.name().as_slice()).to_string();
            return install_ivar_read(self.genv, &ivar_name);
        }

        // self
        if node.as_self_node().is_some() {
            return Some(install_self(self.genv));
        }

        // x = "hello"
        if let Some(write_node) = node.as_local_variable_write_node() {
            let value = write_node.value();
            let val_vtx = self.install_node(&value)?;
            let var_name =
                String::from_utf8_lossy(write_node.name().as_slice()).to_string();
            return Some(install_local_var_write(
                self.genv,
                self.lenv,
                &mut self.changes,
                var_name,
                val_vtx,
            ));
        }

        // x
        if let Some(read_node) = node.as_local_variable_read_node() {
            let var_name = String::from_utf8_lossy(read_node.name().as_slice()).to_string();
            return install_local_var_read(self.lenv, &var_name);
        }

        // Literals (String, Integer, Array, Hash, nil, true, false, Symbol)
        if let Some(vtx) = install_literal(self.genv, node) {
            return Some(vtx);
        }

        // x.upcase (method call)
        if let Some(call_node) = node.as_call_node() {
            let recv_vtx = if let Some(receiver) = call_node.receiver() {
                self.install_node(&receiver)?
            } else {
                // Implicit receiver (self) - not yet supported
                return None;
            };

            let method_name =
                String::from_utf8_lossy(call_node.name().as_slice()).to_string();
            let location =
                SourceLocation::from_prism_location_with_source(&node.location(), self.source);

            return Some(install_method_call(
                self.genv,
                recv_vtx,
                method_name,
                Some(location),
            ));
        }

        // Other nodes not yet implemented
        None
    }

    /// Install class definition
    fn install_class_node(&mut self, class_node: &ruby_prism::ClassNode) -> Option<VertexId> {
        let class_name = extract_class_name(class_node);
        install_class(self.genv, class_name);

        if let Some(body) = class_node.body() {
            if let Some(statements) = body.as_statements_node() {
                self.install_statements(&statements);
            }
        }

        exit_scope(self.genv);
        None
    }

    /// Install method definition
    fn install_def_node(&mut self, def_node: &ruby_prism::DefNode) -> Option<VertexId> {
        let method_name = String::from_utf8_lossy(def_node.name().as_slice()).to_string();
        install_method(self.genv, method_name);

        // TODO: Process parameters in future implementation

        if let Some(body) = def_node.body() {
            if let Some(statements) = body.as_statements_node() {
                self.install_statements(&statements);
            }
        }

        exit_scope(self.genv);
        None
    }

    /// Process multiple statements
    fn install_statements(&mut self, statements: &ruby_prism::StatementsNode) {
        for stmt in &statements.body() {
            self.install_node(&stmt);
        }
    }

    /// Finish installation (apply changes and execute Boxes)
    pub fn finish(self) {
        self.genv.apply_changes(self.changes);
        self.genv.run_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_ruby_source;
    use crate::types::Type;

    #[test]
    fn test_install_literal() {
        let source = r#"x = "hello""#;
        let parse_result = parse_ruby_source(source, "test.rb".to_string()).unwrap();

        let mut genv = GlobalEnv::new();
        let mut lenv = LocalEnv::new();
        let mut installer = AstInstaller::new(&mut genv, &mut lenv, source);

        let root = parse_result.node();
        if let Some(program_node) = root.as_program_node() {
            let statements = program_node.statements();
            for stmt in &statements.body() {
                installer.install_node(&stmt);
            }
        }

        installer.finish();

        let x_vtx = lenv.get_var("x").unwrap();
        assert_eq!(genv.get_vertex(x_vtx).unwrap().show(), "String");
    }

    #[test]
    fn test_install_multiple_vars() {
        let source = r#"
x = "hello"
y = 42
"#;
        let parse_result = parse_ruby_source(source, "test.rb".to_string()).unwrap();

        let mut genv = GlobalEnv::new();
        let mut lenv = LocalEnv::new();
        let mut installer = AstInstaller::new(&mut genv, &mut lenv, source);

        let root = parse_result.node();
        if let Some(program_node) = root.as_program_node() {
            let statements = program_node.statements();
            for stmt in &statements.body() {
                installer.install_node(&stmt);
            }
        }

        installer.finish();

        let x_vtx = lenv.get_var("x").unwrap();
        let y_vtx = lenv.get_var("y").unwrap();

        assert_eq!(genv.get_vertex(x_vtx).unwrap().show(), "String");
        assert_eq!(genv.get_vertex(y_vtx).unwrap().show(), "Integer");
    }

    #[test]
    fn test_install_method_call() {
        let source = r#"
x = "hello"
y = x.upcase
"#;
        let parse_result = parse_ruby_source(source, "test.rb".to_string()).unwrap();

        let mut genv = GlobalEnv::new();
        genv.register_builtin_method(Type::string(), "upcase", Type::string());

        let mut lenv = LocalEnv::new();
        let mut installer = AstInstaller::new(&mut genv, &mut lenv, source);

        let root = parse_result.node();
        if let Some(program_node) = root.as_program_node() {
            let statements = program_node.statements();
            for stmt in &statements.body() {
                installer.install_node(&stmt);
            }
        }

        installer.finish();

        let x_vtx = lenv.get_var("x").unwrap();
        let y_vtx = lenv.get_var("y").unwrap();

        assert_eq!(genv.get_vertex(x_vtx).unwrap().show(), "String");
        assert_eq!(genv.get_vertex(y_vtx).unwrap().show(), "String");
    }
}
