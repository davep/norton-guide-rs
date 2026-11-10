use rstest::rstest;

use norton_guide::rle::{clean_rle, expand_rle};

#[rstest]
#[case::empty_string(b"", b"")]
#[case::no_marker(b"A", b"A")]
#[case::lone_marker(b"\xFF", b" ")]
#[case::one_marker(b"A\xFFA", b"A A")]
#[case::multiple_markers(b"A\xFF\xFF\xFFA", b"A   A")]
#[case::only_markers(b"\xFF\xFF\xFF", b"   ")]
fn test_clean_rle(#[case] input: &[u8], #[case] expected: &[u8]) {
    assert_eq!(clean_rle(input), expected);
}

#[rstest]
#[case::empty_string(b"", b"")]
#[case::no_marker(b"A", b"A")]
#[case::lone_marker(b"\xFF", b" ")]
#[case::repeated_marker(b"\xFF\xFF", b" ")]
#[case::repeated_marker_and_lone_marker(b"\xFF\xFF\xFF", b"  ")]
#[case::one_lone_space(b"\xFF\x01", b" ")]
#[case::multiple_lone_spaces(b"\xFF\x0A", b"          ")]
#[case::many_expansions(b"A\xFF\x02A\xFF\x02A", b"A  A  A")]
fn test_expand_rle(#[case] input: &[u8], #[case] expected: &[u8]) {
    assert_eq!(expand_rle(input), expected);
}

// test_rle.rs ends here
