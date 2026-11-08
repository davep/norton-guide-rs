use norton_guide::NortonGuide;
use std::io::ErrorKind;
use std::path::Path;

const EXISTS_PATH: &str = "tests/guides/eg.ng";
const DOES_NOT_EXIST_PATH: &str = "tests/guides/does_not_exist.ng";

#[test]
fn test_open_non_existent_file_fails() {
    let result = NortonGuide::open(DOES_NOT_EXIST_PATH);

    assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
}

#[test]
fn test_open_existing_file_succeeds() {
    let result = NortonGuide::open(EXISTS_PATH);

    assert!(result.is_ok());
}

#[test]
fn test_path_method_returns_correct_path() {
    let expected_path: &Path = Path::new(EXISTS_PATH);
    let result = NortonGuide::open(expected_path).unwrap();

    assert_eq!(result.path(), expected_path);
}

// test_guide.rs ends here
