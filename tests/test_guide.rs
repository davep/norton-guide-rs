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
    assert_eq!(
        NortonGuide::open(DOES_NOT_EXIST_PATH).unwrap_err().kind(),
        ErrorKind::NotFound
    );
}

#[test]
fn test_open_existing_file_succeeds() {
    assert!(NortonGuide::open(EXISTS_PATH).is_ok());
}

#[test]
fn test_path_method_returns_correct_path() {
    let expected_path: &Path = Path::new(EXISTS_PATH);
    assert_eq!(
        NortonGuide::open(expected_path).unwrap().path(),
        expected_path
    );
}

#[test]
fn test_open_empty_file_fails() {
    assert_eq!(
        NortonGuide::open(EMPTY_FILE_PATH).unwrap_err().kind(),
        ErrorKind::UnexpectedEof
    );
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
    const POSITION: u64 = 10;
    assert_eq!(
        NortonGuide::open(EXISTS_PATH)
            .unwrap()
            .reader()
            .goto(POSITION)
            .unwrap(),
        POSITION
    );
}

#[test]
fn test_goto_method_past_end_of_file() {
    assert!(
        NortonGuide::open(EXISTS_PATH)
            .unwrap()
            .reader()
            .goto(1_000_000_000)
            .is_ok()
    );
}

#[test]
fn test_magic_method_returns_correct_value() {
    assert_eq!(
        NortonGuide::open(EXISTS_PATH).unwrap().magic(),
        GuideMaker::NortonGuide
    );
}

#[test]
fn test_magic_string_method_returns_correct_value() {
    assert_eq!(NortonGuide::open(EXISTS_PATH).unwrap().magic_string(), "NG");
}

#[test]
fn test_made_with() {
    assert_eq!(
        NortonGuide::open(EXISTS_PATH).unwrap().made_with(),
        "Norton Guide"
    );
}

#[test]
fn test_menu_count() {
    assert_eq!(NortonGuide::open(EXISTS_PATH).unwrap().menu_count(), 1);
}

#[test]
fn test_guide_title() {
    assert_eq!(
        NortonGuide::open(EXISTS_PATH).unwrap().title(),
        "Expert Guide"
    );
}

#[test]
fn test_guide_credits() {
    let expected_credits = [
        "Expert Guide",
        "Copyright (c) 1997-2015 David A. Pearson",
        "",
        "email: davep@davep.org",
        "  web: http://www.davep.org/",
    ];
    assert_eq!(
        NortonGuide::open(EXISTS_PATH).unwrap().credits(),
        &expected_credits
    );
}

// test_guide.rs ends here
