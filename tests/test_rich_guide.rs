//! Perform tests against a guide with rich content, including short and
//! long entries, as well as menus.

use rstest::*;

mod fixtures;

use fixtures::*;

use norton_guide::NortonGuide;
use norton_guide::entry::{Entry, Long, Short};
use norton_guide::magic::GuideMaker;

#[rstest]
fn test_os_lib_magic(os_lib: NortonGuide) {
    assert_eq!(os_lib.magic(), GuideMaker::NortonGuide);
}

#[rstest]
fn test_os_lib_is_valid(os_lib: NortonGuide) {
    assert!(os_lib.is_valid());
}

#[rstest]
fn test_os_lib_title(os_lib: NortonGuide) {
    assert_eq!(os_lib.title(), "OSLIB v1.06");
}

#[rstest]
fn test_os_lib_credits(os_lib: NortonGuide) {
    assert_eq!(
        os_lib.credits(),
        // &[
        //     "│ OSLIB v1.06",
        //     "│ OSLIB Is Free Software with NO WARRANTY!",
        //     "│",
        //     "│ This library was compiled by Dave Pearson.",
        //     "│ davep@hagbard.demon.co.uk",
        // ]
        // TODO: Need to adding undosify support.
        &[
            "� OSLIB v1.06",
            "� OSLIB Is Free Software with NO WARRANTY!",
            "�",
            "� This library was compiled by Dave Pearson.",
            "� davep@hagbard.demon.co.uk",
        ]
    );
}

#[rstest]
fn test_magic_string(os_lib: NortonGuide) {
    assert_eq!(os_lib.magic_string(), "NG");
}

#[rstest]
fn test_made_with(os_lib: NortonGuide) {
    assert_eq!(os_lib.made_with(), "Norton Guide");
}

#[rstest]
fn test_menu_count(os_lib: NortonGuide) {
    assert_eq!(os_lib.menu_count(), 1);
}

#[rstest]
fn test_menu_title(os_lib: NortonGuide) {
    assert_eq!(os_lib.menu(0).unwrap().title(), "OSLIB");
}

#[rstest]
fn test_menu_prompt_count(os_lib: NortonGuide) {
    assert_eq!(os_lib.menu(0).unwrap().prompt_count(), 5);
}

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
fn test_repeated_load_loads_same_entry(mut os_lib: NortonGuide) {
    assert_eq!(
        Short::try_from(os_lib.load().unwrap())
            .unwrap()
            .common
            .offset,
        Short::try_from(os_lib.load().unwrap())
            .unwrap()
            .common
            .offset
    );
}

#[rstest]
fn test_second_entry_is_long(mut os_lib: NortonGuide) {
    assert!(matches!(
        os_lib.skip().unwrap().load().unwrap(),
        Entry::Long(_)
    ));
}

#[rstest]
fn test_second_entry_has_correct_line_count(mut os_lib: NortonGuide) {
    assert_eq!(
        Long::try_from(os_lib.skip().unwrap().load().unwrap())
            .unwrap()
            .lines
            .len(),
        38
    );
}

#[rstest]
fn test_second_entry_first_line_is_correct(mut os_lib: NortonGuide) {
    assert_eq!(
        Long::try_from(os_lib.skip().unwrap().load().unwrap())
            .unwrap()
            .lines
            .first()
            .unwrap(),
        " ^bOL_95AppTitle()"
    );
}

#[rstest]
fn test_count_entries(mut os_lib: NortonGuide) {
    let mut count = 0;
    while os_lib.skip().is_ok() {
        count += 1;
    }
    assert_eq!(count, 28);
}

// test_rich_guide.rs ends here
