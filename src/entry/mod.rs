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

    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        match value {
            Entry::Short(short) => Ok(short),
            _ => Err(NortonGuideError::NotShortError),
        }
    }
}

impl TryFrom<Entry> for Long {
    type Error = NortonGuideError;

    fn try_from(value: Entry) -> Result<Self, Self::Error> {
        match value {
            Entry::Long(long) => Ok(long),
            _ => Err(NortonGuideError::NotLongError),
        }
    }
}

// mod.rs ends here
