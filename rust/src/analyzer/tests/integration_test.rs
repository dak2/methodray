//! Integration Tests - End-to-end analyzer tests
//!
//! This module contains integration tests that verify:
//! - Class/method definition handling
//! - Instance variable type tracking across methods
//! - Type error detection for undefined methods
//! - Method chain type inference

use crate::analyzer::AstInstaller;
use crate::env::{GlobalEnv, LocalEnv};
use crate::parser::parse_ruby_source;
use crate::types::Type;

/// Helper to run analysis on Ruby source code
fn analyze(source: &str) -> (GlobalEnv, LocalEnv) {
    let parse_result = parse_ruby_source(source, "test.rb".to_string()).unwrap();

    let mut genv = GlobalEnv::new();

    // Register common methods
    genv.register_builtin_method(Type::string(), "upcase", Type::string());
    genv.register_builtin_method(Type::string(), "downcase", Type::string());

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

    (genv, lenv)
}

#[test]
fn test_class_method_error_detection() {
    let source = r#"
class User
  def test
    x = 123
    y = x.upcase
  end
end
"#;

    let (genv, _lenv) = analyze(source);

    // Type error should be detected: Integer doesn't have upcase method
    assert_eq!(genv.type_errors.len(), 1);
    assert_eq!(genv.type_errors[0].method_name, "upcase");
}

#[test]
fn test_class_with_instance_variable() {
    let source = r#"
class User
  def initialize
    @name = "John"
  end

  def greet
    @name.upcase
  end
end
"#;

    let (genv, _lenv) = analyze(source);

    // No type errors should occur - @name is String
    assert_eq!(genv.type_errors.len(), 0);
}

#[test]
fn test_instance_variable_type_error() {
    let source = r#"
class User
  def initialize
    @name = 123
  end

  def greet
    @name.upcase
  end
end
"#;

    let (genv, _lenv) = analyze(source);

    // Type error should be detected: @name is Integer, not String
    assert_eq!(genv.type_errors.len(), 1);
    assert_eq!(genv.type_errors[0].method_name, "upcase");
}

#[test]
fn test_multiple_classes() {
    let source = r#"
class User
  def name
    x = 123
    x.upcase
  end
end

class Post
  def title
    y = "hello"
    y.upcase
  end
end
"#;

    let (genv, _lenv) = analyze(source);

    // Only User#name should have error (Integer#upcase), Post#title is fine
    assert_eq!(genv.type_errors.len(), 1);
    assert_eq!(genv.type_errors[0].method_name, "upcase");
}

#[test]
fn test_method_chain() {
    let source = r#"
x = "hello"
y = x.upcase.downcase
"#;

    let (genv, lenv) = analyze(source);

    let y_vtx = lenv.get_var("y").unwrap();
    assert_eq!(genv.get_vertex(y_vtx).unwrap().show(), "String");
}
