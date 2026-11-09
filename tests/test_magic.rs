use norton_guide::magic::GuideMaker;

const NORTON_GUIDE_MAGIC: u16 = u16::from_le_bytes(*b"NG");
const EXPERT_HELP_MAGIC: u16 = u16::from_le_bytes(*b"EH");

#[test]
fn test_from_magic() {
    assert_eq!(
        GuideMaker::from_magic(NORTON_GUIDE_MAGIC),
        GuideMaker::NortonGuide
    );
    assert_eq!(
        GuideMaker::from_magic(EXPERT_HELP_MAGIC),
        GuideMaker::ExpertHelp
    );
    assert_eq!(GuideMaker::from_magic(42), GuideMaker::Unknown);
}

#[test]
fn test_to_magic() {
    assert_eq!(GuideMaker::NortonGuide.to_magic(), NORTON_GUIDE_MAGIC);
    assert_eq!(GuideMaker::ExpertHelp.to_magic(), EXPERT_HELP_MAGIC);
    assert_eq!(GuideMaker::Unknown.to_magic(), 0);
}

#[test]
fn test_to_magic_string() {
    assert_eq!(
        GuideMaker::from_magic(NORTON_GUIDE_MAGIC).to_magic_string(),
        "NG"
    );
    assert_eq!(
        GuideMaker::from_magic(EXPERT_HELP_MAGIC).to_magic_string(),
        "EH"
    );
    assert_eq!(GuideMaker::from_magic(42).to_magic_string(), "??");
}

#[test]
fn test_made_with() {
    assert_eq!(
        GuideMaker::from_magic(NORTON_GUIDE_MAGIC).made_with(),
        "Norton Guide"
    );
    assert_eq!(
        GuideMaker::from_magic(EXPERT_HELP_MAGIC).made_with(),
        "Expert Help"
    );
    assert_eq!(GuideMaker::from_magic(42).made_with(), "Unknown");
}

#[test]
fn test_is_valid() {
    assert!(GuideMaker::from_magic(NORTON_GUIDE_MAGIC).is_valid());
    assert!(GuideMaker::from_magic(EXPERT_HELP_MAGIC).is_valid());
    assert!(!GuideMaker::from_magic(42).is_valid());
}

// test_magic.rs ends here
