extern crate cc;

use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["c-src", "src"].iter().collect();

    cc::Build::new()
        .cpp(true)
        .include(&dir)
        .file(dir.join("scanner.cc"))
        .compile("tree-sitter-elm-scanner");

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-elm-parser");
}
