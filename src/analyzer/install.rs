use crate::env::{GlobalEnv, LocalEnv};
use crate::graph::{BoxId, ChangeSet, MethodCallBox, VertexId};
use crate::types::Type;
use ruby_prism::Node;

/// Build graph from AST
pub struct AstInstaller<'a> {
    genv: &'a mut GlobalEnv,
    lenv: &'a mut LocalEnv,
    changes: ChangeSet,
}

impl<'a> AstInstaller<'a> {
    pub fn new(genv: &'a mut GlobalEnv, lenv: &'a mut LocalEnv) -> Self {
        Self {
            genv,
            lenv,
            changes: ChangeSet::new(),
        }
    }

    /// Install node (returns Vertex ID)
    pub fn install_node(&mut self, node: &Node) -> Option<VertexId> {
        // x = "hello"
        if let Some(write_node) = node.as_local_variable_write_node() {
            let value = write_node.value();
            let val_vtx = self.install_node(&value)?;

            // Convert ConstantId to string (using as_slice())
            let var_name = String::from_utf8_lossy(write_node.name().as_slice()).to_string();
            let var_vtx = self.genv.new_vertex();
            self.lenv.new_var(var_name, var_vtx);

            self.changes.add_edge(val_vtx, var_vtx);
            return Some(var_vtx);
        }

        // x
        if let Some(read_node) = node.as_local_variable_read_node() {
            let var_name = String::from_utf8_lossy(read_node.name().as_slice()).to_string();
            return self.lenv.get_var(&var_name);
        }

        // "hello"
        if node.as_string_node().is_some() {
            return Some(self.genv.new_source(Type::string()));
        }

        // 42
        if node.as_integer_node().is_some() {
            return Some(self.genv.new_source(Type::integer()));
        }

        // [1, 2, 3]
        if node.as_array_node().is_some() {
            return Some(self.genv.new_source(Type::array()));
        }

        // {a: 1}
        if node.as_hash_node().is_some() {
            return Some(self.genv.new_source(Type::hash()));
        }

        // nil
        if node.as_nil_node().is_some() {
            return Some(self.genv.new_source(Type::Nil));
        }

        // true
        if node.as_true_node().is_some() {
            return Some(self.genv.new_source(Type::Instance {
                class_name: "TrueClass".to_string(),
            }));
        }

        // false
        if node.as_false_node().is_some() {
            return Some(self.genv.new_source(Type::Instance {
                class_name: "FalseClass".to_string(),
            }));
        }

        // :symbol
        if node.as_symbol_node().is_some() {
            return Some(self.genv.new_source(Type::Instance {
                class_name: "Symbol".to_string(),
            }));
        }

        // x.upcase (method call)
        if let Some(call_node) = node.as_call_node() {
            // Process receiver
            let recv_vtx = if let Some(receiver) = call_node.receiver() {
                self.install_node(&receiver)?
            } else {
                // If no receiver, assume self (implicit receiver)
                // Not yet supported in current implementation
                return None;
            };

            // Get method name
            let method_name = String::from_utf8_lossy(call_node.name().as_slice()).to_string();

            // Create Vertex for return value
            let ret_vtx = self.genv.new_vertex();

            // Create MethodCallBox
            let box_id = BoxId(self.genv.next_box_id);
            self.genv.next_box_id += 1;

            let call_box = MethodCallBox::new(box_id, recv_vtx, method_name, ret_vtx);
            self.genv.boxes.insert(box_id, Box::new(call_box));
            self.genv.add_run(box_id);

            return Some(ret_vtx);
        }

        // Other nodes not yet implemented
        None
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

    #[test]
    fn test_install_literal() {
        let source = r#"x = "hello""#;

        let parse_result = parse_ruby_source(source, "test.rb".to_string()).unwrap();

        let mut genv = GlobalEnv::new();
        let mut lenv = LocalEnv::new();
        let mut installer = AstInstaller::new(&mut genv, &mut lenv);

        // ruby-prism correct API: get top-level node with node()
        let root = parse_result.node();

        // Get statements from ProgramNode
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
        let mut installer = AstInstaller::new(&mut genv, &mut lenv);

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

        // Register String#upcase
        genv.register_builtin_method(Type::string(), "upcase", Type::string());

        let mut lenv = LocalEnv::new();
        let mut installer = AstInstaller::new(&mut genv, &mut lenv);

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

    #[test]
    fn test_install_method_chain() {
        let source = r#"
x = "hello"
y = x.upcase.downcase
"#;

        let parse_result = parse_ruby_source(source, "test.rb".to_string()).unwrap();

        let mut genv = GlobalEnv::new();

        // Register String methods
        genv.register_builtin_method(Type::string(), "upcase", Type::string());
        genv.register_builtin_method(Type::string(), "downcase", Type::string());

        let mut lenv = LocalEnv::new();
        let mut installer = AstInstaller::new(&mut genv, &mut lenv);

        let root = parse_result.node();

        if let Some(program_node) = root.as_program_node() {
            let statements = program_node.statements();
            for stmt in &statements.body() {
                installer.install_node(&stmt);
            }
        }

        installer.finish();

        let y_vtx = lenv.get_var("y").unwrap();
        assert_eq!(genv.get_vertex(y_vtx).unwrap().show(), "String");
    }
}
