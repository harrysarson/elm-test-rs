use tree_sitter::Tree;

#[derive(Debug)]
pub struct TestDefinitions<'a> {
    exposed: Box<[&'a str]>,
    private: Box<[&'a str]>,
}

impl<'a> TestDefinitions<'a> {
    pub fn from_tree(_tree: &'a Tree) -> TestDefinitions<'a> {
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
                assert_debug_snapshot!(TestDefinitions::from_tree(test_tree));
            }
        })
    }
}
