use rstest::*;
use std::path::PathBuf;

#[fixture]
fn test_files() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("guides")
}

#[fixture]
pub fn valid_guide(test_files: PathBuf) -> PathBuf {
    test_files.join("eg.ng")
}

#[fixture]
pub fn missing_guide(test_files: PathBuf) -> PathBuf {
    test_files.join("does_not_exist.ng")
}

#[fixture]
pub fn empty_guide(test_files: PathBuf) -> PathBuf {
    test_files.join("empty.ng")
}

#[fixture]
pub fn not_a_guide(test_files: PathBuf) -> PathBuf {
    test_files.join("not.ng")
}

// fixtures.rs ends here
