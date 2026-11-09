use norton_guide::entry_type::EntryType;

#[test]
fn test_entry_type_try_from() {
    assert_eq!(EntryType::try_from(0).unwrap(), EntryType::Short);
    assert_eq!(EntryType::try_from(1).unwrap(), EntryType::Long);
    assert_eq!(EntryType::try_from(2).unwrap(), EntryType::Menu);
    assert!(EntryType::try_from(3).is_err());
}

// test_entry_type.rs ends here
