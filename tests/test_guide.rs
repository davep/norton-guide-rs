use std::path::PathBuf;
use norton_guide::NortonGuide;

#[test]
fn test_open_non_existent_file_fails() {
    let result = NortonGuide::open(PathBuf::from("does_not_exist.ng"));

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
}
