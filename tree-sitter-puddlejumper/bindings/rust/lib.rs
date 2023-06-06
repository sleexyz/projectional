#[cfg(all(feature = "native", feature = "wasm"))]
compile_error!("feature \"native\" and feature \"wasm\" cannot be enabled at the same time");

#[cfg(feature = "native")]
use tree_sitter;

#[cfg(feature = "wasm")]
use tree_sitter_c2rust as tree_sitter;

#[cfg(feature = "wasm")]
mod scanner;

extern "C" {
    fn tree_sitter_puddlejumper() -> tree_sitter::Language;
}

/// Get the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> tree_sitter::Language {
    unsafe { tree_sitter_puddlejumper() }
}

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &'static str = include_str!("../../src/node-types.json");

// Uncomment these to include any queries that this grammar contains

// pub const HIGHLIGHTS_QUERY: &'static str = include_str!("../../queries/highlights.scm");
// pub const INJECTIONS_QUERY: &'static str = include_str!("../../queries/injections.scm");
// pub const LOCALS_QUERY: &'static str = include_str!("../../queries/locals.scm");
// pub const TAGS_QUERY: &'static str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {

    #[cfg(feature = "native")]
    use tree_sitter;

    #[cfg(feature = "wasm")]
    use tree_sitter_c2rust as tree_sitter;

    // use tree_sitter_c2rust as tree_sitter;
    // use tree_sitter;

    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language())
            .expect("Error loading puddlejumper language");
    }
}
