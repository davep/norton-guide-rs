use norton_guide::NortonGuide;
use norton_guide::magic::GuideMaker;
use std::io::ErrorKind;
use std::path::Path;

const EXISTS_PATH: &str = "tests/guides/eg.ng";
const DOES_NOT_EXIST_PATH: &str = "tests/guides/does_not_exist.ng";
const EMPTY_FILE_PATH: &str = "tests/guides/empty.ng";
const NOT_GUIDE_FILE_PATH: &str = "tests/guides/not.ng";

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

#[test]
fn test_open_empty_file_fails() {
    let result = NortonGuide::open(EMPTY_FILE_PATH);
    assert_eq!(result.unwrap_err().kind(), ErrorKind::UnexpectedEof);
}

#[test]
fn test_is_valid() {
    assert!(NortonGuide::open(EXISTS_PATH).unwrap().is_valid());
}

#[test]
fn test_is_not_valid() {
    assert!(!NortonGuide::open(NOT_GUIDE_FILE_PATH).unwrap().is_valid());
}

#[test]
fn test_goto_method_moves_cursor() {
    let mut guide = NortonGuide::open(EXISTS_PATH).unwrap();

    let position = 10;
    let result = guide.reader().goto(position).unwrap();

    assert_eq!(result, position);
}

#[test]
fn test_goto_method_past_end_of_file() {
    let mut guide = NortonGuide::open(EXISTS_PATH).unwrap();
    assert!(guide.reader().goto(1_000_000_000).is_ok());
}

#[test]
fn test_magic_method_returns_correct_value() {
    let guide = NortonGuide::open(EXISTS_PATH).unwrap();
    assert_eq!(guide.magic(), GuideMaker::NortonGuide);
}

#[test]
fn test_magic_string_method_returns_correct_value() {
    let guide = NortonGuide::open(EXISTS_PATH).unwrap();
    assert_eq!(guide.magic_string(), "NG");
}

#[test]
fn test_menu_count() {
    let guide = NortonGuide::open(EXISTS_PATH).unwrap();
    assert_eq!(guide.menu_count(), 1);
}

#[test]
fn test_guide_title() {
    let guide = NortonGuide::open(EXISTS_PATH).unwrap();
    assert_eq!(guide.title(), "Expert Guide");
}

#[test]
fn test_guide_credits() {
    let guide = NortonGuide::open(EXISTS_PATH).unwrap();
    let expected_credits = [
        "Expert Guide",
        "Copyright (c) 1997-2015 David A. Pearson",
        "",
        "email: davep@davep.org",
        "  web: http://www.davep.org/",
    ];
    assert_eq!(guide.credits(), &expected_credits);
}

// test_guide.rs ends here
