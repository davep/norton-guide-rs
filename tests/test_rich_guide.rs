//! Perform tests against a guide with rich content, including short and
//! long entries, as well as menus.

use rstest::*;

mod fixtures;

use fixtures::*;

use norton_guide::NortonGuide;
use norton_guide::entry::{Entry, Short, Long};

#[rstest]
fn test_first_entry_is_short(mut os_lib: NortonGuide) {
    assert!(matches!(os_lib.load().unwrap(), Entry::Short(_)));
}

#[rstest]
fn test_first_line_of_first_entry_is_correct(mut os_lib: NortonGuide) {
    assert_eq!(
        Short::try_from(os_lib.load().unwrap())
            .unwrap()
            .lines
            .first()
            .unwrap(),
        " OL_95AppTitle()          Set/get the Windows 95 application title."
    );
}

#[rstest]
fn test_first_entry_has_correct_line_count(mut os_lib: NortonGuide) {
    assert_eq!(
        Short::try_from(os_lib.load().unwrap()).unwrap().lines.len(),
        13
    );
}

#[rstest]
fn test_second_entry_is_long(mut os_lib: NortonGuide) {
    let _ = os_lib.load();
    assert!(matches!(os_lib.load().unwrap(), Entry::Long(_)));
}

#[rstest]
fn test_second_entry_has_correct_line_count(mut os_lib: NortonGuide) {
    let _ = os_lib.load();
    assert_eq!(
        Long::try_from(os_lib.load().unwrap())
            .unwrap()
            .lines
            .len(),
        38
    );
}

#[rstest]
fn test_second_entry_first_line_is_correct(mut os_lib: NortonGuide) {
    let _ = os_lib.load();
    assert_eq!(
        Long::try_from(os_lib.load().unwrap())
            .unwrap()
            .lines
            .first()
            .unwrap(),
        " ^bOL_95AppTitle()"
    );
}

// test_rich_guide.rs ends here
