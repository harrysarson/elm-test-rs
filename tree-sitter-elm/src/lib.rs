extern "C" {
    fn tree_sitter_elm() -> tree_sitter::Language;
}

pub fn language() -> tree_sitter::Language {
    unsafe { tree_sitter_elm() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_display_snapshot;
    use std::fs;
    use std::path::PathBuf;
    use tree_sitter::Parser;

    #[test]
    fn test_language() {
        let mut parser = Parser::new();
        parser.set_language(language()).unwrap();

        let source_file: PathBuf = ["c-src", "examples", "basic.elm"].iter().collect();
        let source_code = fs::read_to_string(source_file).unwrap();

        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        let sexp = sexp::parse(&root_node.to_sexp()).expect("valid sexp");

        assert_display_snapshot!(sexp.pretty());
    }
}
