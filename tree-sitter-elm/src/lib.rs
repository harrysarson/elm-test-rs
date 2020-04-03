extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

pub fn language() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::{Parser};
    use std::fs;
    use std::path::PathBuf;
    use insta::assert_debug_snapshot;
    use itertools::Itertools;

    #[test]
    fn test_language() {
        let mut parser = Parser::new();
        parser.set_language(language()).unwrap();

        let source_file : PathBuf = ["c-src", "examples", "basic.elm"].iter().collect();
        let source_code = fs::read_to_string(source_file).unwrap();

        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        assert_debug_snapshot!(root_node.to_sexp());
    }
}
