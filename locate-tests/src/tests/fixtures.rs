use lazy_static::lazy_static;
use std::fs;
use std::path::PathBuf;
use tree_sitter::Parser;
use tree_sitter::Tree;

lazy_static! {
    pub static ref TEST_PATHS: Box<[PathBuf]> = {
        let tests_dir: PathBuf = ["fixtures", "elm-css", "tests"].iter().collect();
        assert!(tests_dir.is_dir(), "expected directory of tests");

        fs::read_dir(tests_dir)
            .expect("cannot read test fixture dir")
            .map(|entry| {
                let entry = entry.expect("cannot read test fixture dir entry");
                let path = entry.path();
                if path.is_dir() {
                    unimplemented!("Support for nested test fixtures");
                } else {
                    entry.path()
                }
            })
            .collect()
    };
    pub static ref TEST_SOURCES: Box<[String]> = {
        TEST_PATHS
            .iter()
            .map(|path| fs::read_to_string(path).expect("Able to read test fixture file"))
            .collect()
    };
}
thread_local! {
    pub static TEST_TREES: Box<[Tree]> = {
        TEST_SOURCES
            .iter()
            .map(|source_code| {
                let mut parser = Parser::new();
                parser.set_language(tree_sitter_elm::language()).unwrap();

                parser.parse(source_code, None).unwrap()
            })
            .collect()

    };
}
