// Adapted from https://github.com/oxc-project/oxc/blob/0e6498f7e218420153910074d5a55bbef7c1c1d4/tasks/common/src/test_file.rs

use std::path::Path;

pub struct TestFiles {
    files: &'static [TestFile],
}

impl TestFiles {
    pub fn files(&self) -> &'static [TestFile] {
        self.files
    }

    /// One large and one medium Java source file, written to be representative
    /// of real-world code: declarations, generics, lambdas, streams, switch
    /// expressions, records, Javadoc, and comments.
    pub fn minimal() -> Self {
        Self {
            files: &[
                TestFile {
                    file_name: "InventoryLedger.java",
                    source_text: include_str!("../fixtures/InventoryLedger.java"),
                },
                TestFile {
                    file_name: "RetryPolicy.java",
                    source_text: include_str!("../fixtures/RetryPolicy.java"),
                },
            ],
        }
    }
}

pub struct TestFile {
    pub file_name: &'static str,
    pub source_text: &'static str,
}

/// The directory holding the committed test files.
pub fn fixtures_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures"))
}
