pub mod common;
pub mod long;
pub mod menu;
pub mod optionals;
pub mod parent;
pub mod seealso;
pub mod short;
pub mod types;

pub use common::Common;
pub use long::Long;
pub use menu::Menu;
pub use seealso::SeeAlso;
pub use short::Short;
pub use types::EntryType;

use crate::NortonGuideError;

/// The `Entry` enum represents a Norton Guide entry, which can be either a short or long entry.
#[derive(Debug, Clone)]
pub enum Entry {
    /// The entry is a short entry.
    Short(Short),
    /// The entry is a long entry.
    Long(Long),
}

impl TryFrom<Entry> for Short {
    type Error = NortonGuideError;

    /// Attempts to convert an `Entry` into a `Short` entry.
    ///
    /// # Arguments
    ///
    /// * `value` - The `Entry` to convert.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Self::Error>` - Returns an `Ok(Short)` if the
    ///   conversion is successful, or an `Err(NortonGuideError)` if the
    ///   entry isnot a short entry.
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        match value {
            Entry::Short(short) => Ok(short),
            _ => Err(NortonGuideError::NotShortError),
        }
    }
}

impl TryFrom<Entry> for Long {
    type Error = NortonGuideError;

    /// Attempts to convert an `Entry` into a `Long` entry.
    ///
    /// # Arguments
    ///
    /// * `value` - The `Entry` to convert.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Self::Error>` - Returns an `Ok(Long)` if the
    ///   conversion is successful, or an `Err(NortonGuideError)` if the
    ///   entry is not a long entry.
    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        match value {
            Entry::Long(long) => Ok(long),
            _ => Err(NortonGuideError::NotLongError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entry::parent::EntryParent;

    use super::*;

    #[test]
    fn test_entry_conversion() {
        let short_entry = Entry::Short(Short {
            common: Common {
                entry_type: EntryType::Short,
                line_count: 1,
                size: 10,
                has_see_also: false,
                parent: EntryParent::default(),
                previous: 0,
                next: 0,
                offset: 0,
            },
            lines: vec!["Short entry line".to_string()],
            offsets: vec![0],
        });

        let long_entry = Entry::Long(Long {
            common: Common {
                entry_type: EntryType::Long,
                line_count: 2,
                size: 20,
                has_see_also: false,
                parent: EntryParent::default(),
                previous: 0,
                next: 0,
                offset: 0,
            },
            lines: vec!["Line 1".to_string(), "Line 2".to_string()],
            see_also: None,
        });

        assert!(Short::try_from(short_entry.clone()).is_ok());
        assert!(Long::try_from(long_entry.clone()).is_ok());
        assert!(Long::try_from(short_entry).is_err());
        assert!(Short::try_from(long_entry).is_err());
    }
}

// mod.rs ends here
