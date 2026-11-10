//! Provides code for working with a Norton Guide's "magic" number.

/// The known tools that can create a Norton Guide file, represented by
/// their magic numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideMaker {
    /// The Norton Guide software, which uses the magic number "NG".
    NortonGuide,
    /// The Expert Help software, which uses the magic number "EH".
    ExpertHelp,
    /// An unknown or unrecognized tool, which uses a magic number that is
    /// not recognized.
    Unknown,
}

/// Implementations for the `GuideMaker` enum.
impl GuideMaker {
    /// Creates a `GuideMaker` instance from a given magic number.
    ///
    /// # Arguments
    ///
    /// * `magic` - A 16-bit unsigned integer representing the magic number.
    ///
    /// # Returns
    ///
    /// * `GuideMaker` - Returns a `GuideMaker` variant corresponding to the
    ///   provided magic number.
    pub fn from_magic(magic: u16) -> Self {
        match &magic.to_le_bytes() {
            b"NG" => Self::NortonGuide,
            b"EH" => Self::ExpertHelp,
            _ => Self::Unknown,
        }
    }

    /// Converts a `GuideMaker` instance to its corresponding magic number.
    ///
    /// # Returns
    ///
    /// * `u16` - The 16-bit unsigned integer representing the magic number
    ///   for the `GuideMaker` variant.
    pub fn to_magic(&self) -> u16 {
        match self {
            Self::NortonGuide => u16::from_le_bytes(*b"NG"),
            Self::ExpertHelp => u16::from_le_bytes(*b"EH"),
            Self::Unknown => 0,
        }
    }

    /// Converts a `GuideMaker` instance to its corresponding magic string.
    ///
    /// # Returns
    ///
    /// * `String` - A string representation of the magic number for the
    ///   `GuideMaker` variant.
    pub fn to_magic_string(&self) -> String {
        match self {
            Self::NortonGuide => "NG".to_string(),
            Self::ExpertHelp => "EH".to_string(),
            Self::Unknown => "??".to_string(),
        }
    }

    /// Returns a human-readable string indicating the software that created
    /// the Norton Guide file.
    ///
    /// # Returns
    ///
    /// * `&'static str` - A string indicating the software that created the
    ///   Norton Guide file.
    pub fn made_with(&self) -> &'static str {
        match self {
            Self::NortonGuide => "Norton Guide",
            Self::ExpertHelp => "Expert Help",
            Self::Unknown => "Unknown",
        }
    }

    /// Checks if the `GuideMaker` instance represents a valid and
    /// recognised tool that created the Norton Guide file.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the `GuideMaker` is valid, otherwise
    ///   returns `false`.
    pub fn is_valid(&self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

// magic.rs ends here
