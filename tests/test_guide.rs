mod fixtures;

use fixtures::*;

use norton_guide::magic::GuideMaker;
use norton_guide::{NortonGuide, NortonGuideError};
use std::io::ErrorKind;
use std::path::PathBuf;

use rstest::*;

#[rstest]
fn test_open_non_existent_file_fails(missing_guide: PathBuf) {
    assert!(matches!(
        NortonGuide::open(missing_guide).unwrap_err(),
        NortonGuideError::Io(e) if e.kind() == ErrorKind::NotFound
    ));
}

#[rstest]
fn test_open_existing_file_succeeds(valid_guide: PathBuf) {
    assert!(NortonGuide::open(valid_guide).is_ok());
}

#[rstest]
fn test_path_method_returns_correct_path(valid_guide: PathBuf) {
    assert_eq!(NortonGuide::open(&valid_guide).unwrap().path(), valid_guide,);
}

#[rstest]
fn test_open_empty_file_fails(empty_guide: PathBuf) {
    assert!(matches!(
        NortonGuide::open(empty_guide).unwrap_err(),
        NortonGuideError::Io(e) if e.kind() == ErrorKind::UnexpectedEof
    ));
}

#[rstest]
fn test_is_valid(valid_guide: PathBuf) {
    assert!(NortonGuide::open(valid_guide).unwrap().is_valid());
}

#[rstest]
fn test_is_not_valid(not_a_guide: PathBuf) {
    assert!(matches!(
        NortonGuide::open(not_a_guide).unwrap_err(),
        NortonGuideError::BadMagicError { .. }
    ));
}

#[rstest]
fn test_goto_method_moves_cursor(valid_guide: PathBuf) {
    const POSITION: u64 = 10;
    assert_eq!(
        NortonGuide::open(valid_guide)
            .unwrap()
            .reader()
            .goto(POSITION)
            .unwrap(),
        POSITION
    );
}

#[rstest]
fn test_goto_method_past_end_of_file(valid_guide: PathBuf) {
    assert!(
        NortonGuide::open(valid_guide)
            .unwrap()
            .reader()
            .goto(1_000_000_000)
            .is_ok()
    );
}

#[rstest]
fn test_magic_method_returns_correct_value(valid_guide: PathBuf) {
    assert_eq!(
        NortonGuide::open(valid_guide).unwrap().magic(),
        GuideMaker::NortonGuide
    );
}

#[rstest]
fn test_magic_string_method_returns_correct_value(valid_guide: PathBuf) {
    assert_eq!(NortonGuide::open(valid_guide).unwrap().magic_string(), "NG");
}

#[rstest]
fn test_made_with(valid_guide: PathBuf) {
    assert_eq!(
        NortonGuide::open(valid_guide).unwrap().made_with(),
        "Norton Guide"
    );
}

#[rstest]
fn test_guide_title(valid_guide: PathBuf) {
    assert_eq!(
        NortonGuide::open(valid_guide).unwrap().title(),
        "Expert Guide"
    );
}

#[rstest]
fn test_menu_count(valid_guide: PathBuf) {
    assert_eq!(NortonGuide::open(valid_guide).unwrap().menu_count(), 1);
}

#[rstest]
fn test_menu_title(valid_guide: PathBuf) {
    assert_eq!(
        NortonGuide::open(valid_guide)
            .unwrap()
            .menu(0)
            .unwrap()
            .title(),
        "Expert Guide"
    );
}

#[rstest]
fn test_guide_credits(valid_guide: PathBuf) {
    let expected_credits = [
        "Expert Guide",
        "Copyright (c) 1997-2015 David A. Pearson",
        "",
        "email: davep@davep.org",
        "  web: http://www.davep.org/",
    ];
    assert_eq!(
        NortonGuide::open(valid_guide).unwrap().credits(),
        &expected_credits
    );
}

// test_guide.rs ends here
