//! Perform tests against a guide with rich content, including short and
//! long entries, as well as menus.

use rstest::*;

mod fixtures;

use fixtures::*;

use norton_guide::NortonGuide;
use norton_guide::entry::Entry;

#[rstest]
fn test_first_entry_is_short(mut os_lib: NortonGuide) {
    assert!(matches!(os_lib.load().unwrap(), Entry::Short(_)));
}

#[rstest]
fn test_first_line_of_first_entry_is_correct(mut os_lib: NortonGuide) {
    if let Entry::Short(short_entry) = os_lib.load().unwrap() {
        assert_eq!(
            short_entry.lines.first().unwrap(),
            " OL_95AppTitle()          Set/get the Windows 95 application title."
        );
    } else {
        panic!("First entry is not a short entry");
    }
}

#[rstest]
fn test_first_entry_has_correct_line_count(mut os_lib: NortonGuide) {
    if let Entry::Short(short_entry) = os_lib.load().unwrap() {
        assert_eq!(short_entry.lines.len(), 13);
    } else {
        panic!("First entry is not a short entry");
    }
}

// test_rich_guide.rs ends here
