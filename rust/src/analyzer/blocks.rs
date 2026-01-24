//! Block Handlers - Processing Ruby blocks
//!
//! This module is responsible for:
//! - Processing BlockNode (e.g., `{ |x| x.to_s }` or `do |x| x.to_s end`)
//! - Registering block parameters as local variables
//! - Managing block scope

use crate::env::{GlobalEnv, LocalEnv, ScopeKind};
use crate::graph::VertexId;

use super::parameters::install_required_parameter;

/// Enter a new block scope
///
/// Creates a new scope for the block and enters it.
/// Block scopes inherit variables from parent scopes.
pub fn enter_block_scope(genv: &mut GlobalEnv) {
    let block_scope_id = genv.scope_manager.new_scope(ScopeKind::Block);
    genv.scope_manager.enter_scope(block_scope_id);
}

/// Exit the current block scope
pub fn exit_block_scope(genv: &mut GlobalEnv) {
    genv.scope_manager.exit_scope();
}

/// Install block parameters as local variables
///
/// Block parameters are registered as Bot (untyped) type since we don't
/// know what type will be passed from the iterator method.
///
/// # Example
/// ```ruby
/// [1, 2, 3].each { |x| x.to_s }  # 'x' is a block parameter
/// ```
pub fn install_block_parameter(genv: &mut GlobalEnv, lenv: &mut LocalEnv, name: String) -> VertexId {
    // Reuse required parameter logic (Bot type)
    install_required_parameter(genv, lenv, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enter_exit_block_scope() {
        let mut genv = GlobalEnv::new();

        let initial_scope_id = genv.scope_manager.current_scope().id;

        enter_block_scope(&mut genv);
        let block_scope_id = genv.scope_manager.current_scope().id;

        // Should be in a new scope
        assert_ne!(initial_scope_id, block_scope_id);

        exit_block_scope(&mut genv);

        // Should be back to initial scope
        assert_eq!(genv.scope_manager.current_scope().id, initial_scope_id);
    }

    #[test]
    fn test_install_block_parameter() {
        let mut genv = GlobalEnv::new();
        let mut lenv = LocalEnv::new();

        enter_block_scope(&mut genv);

        let vtx = install_block_parameter(&mut genv, &mut lenv, "x".to_string());

        // Parameter should be registered in LocalEnv
        assert_eq!(lenv.get_var("x"), Some(vtx));

        exit_block_scope(&mut genv);
    }

    #[test]
    fn test_block_inherits_parent_scope_vars() {
        let mut genv = GlobalEnv::new();

        // Set variable in top-level scope
        genv.scope_manager
            .current_scope_mut()
            .set_local_var("outer".to_string(), VertexId(100));

        enter_block_scope(&mut genv);

        // Block should be able to lookup parent scope variables
        assert_eq!(genv.scope_manager.lookup_var("outer"), Some(VertexId(100)));

        exit_block_scope(&mut genv);
    }
}
