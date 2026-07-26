//! go.mod grammar for tree-sitter, modernized bindings (LanguageFn).

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_gomod() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_gomod) };

pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");
