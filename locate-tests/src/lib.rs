use elm_json::project::Project;
use std::path::Path;
use tree_sitter::Tree;
use walkdir::WalkDir;
use workdir::WorkDir;

#[derive(Debug)]
pub struct ElmModule(String);

pub struct TestProjectMirror<P> {
    workdir: WorkDir<P>,
}

impl<P: AsRef<Path>> TestProjectMirror<P> {
    pub fn from_project(
        project_root: impl AsRef<Path>,
        test_path: impl AsRef<Path>,
        workdir: WorkDir<P>,
    ) -> Result<TestProjectMirror<P> {
        let test_full_path = project_root.as_ref().join(test_path);
        for entry in WalkDir::new("foo") {
            println!("{}", entry?.path().display());
        }
        // print!(
        //     "{}",
        //     sexp::parse(&tree.root_node().to_sexp()).unwrap().pretty()
        // );
        // assert!(false);
        // let root_node = tree.root_node();
        TestProjectMirror { workdir }
    }
}

#[derive(Debug)]
pub struct TestDefinitions<'a> {
    exposed: Box<[&'a str]>,
    private: Box<[&'a str]>,
}

impl<'a> TestDefinitions<'a> {
    pub fn from_sources(
        elm_json: &Project,
        project: &'a TestProjectMirror<impl AsRef<Path>>,
    ) -> TestDefinitions<'a> {
        // print!(
        //     "{}",
        //     sexp::parse(&tree.root_node().to_sexp()).unwrap().pretty()
        // );
        // assert!(false);
        // let root_node = tree.root_node();
        TestDefinitions {
            exposed: Box::new([]),
            private: Box::new([]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    mod fixtures;

    #[test]
    fn test_test_definition_from_tree() {
        fixtures::TEST_TREES.with(|test_trees| {
            for test_tree in test_trees.iter() {
                assert_debug_snapshot!(TestDefinitions::from_sources(test_tree));
            }
        })
    }
}
