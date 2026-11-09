//! Provide the entry type IDs for guide entries.
//!
//! This module defines the `EntryType` enum, which represents the different
//! types of entries in a guide.

/// The `EntryType` enum represents the different types of entries in a guide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EntryType {
    /// The entry is a short entry.
    Short = 0,
    /// The entry is a long entry.
    Long = 1,
    /// The entry is a menu entry.
    Menu = 2,
}

/// Implement the `TryFrom<u16>` trait for `EntryType` to allow conversion
/// from a `u16` value to an `EntryType`.
impl TryFrom<u16> for EntryType {
    /// The associated error type for the conversion.
    type Error = ();

    /// Attempts to convert a `u16` value to an `EntryType`.
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == EntryType::Short as u16 => Ok(EntryType::Short),
            x if x == EntryType::Long as u16 => Ok(EntryType::Long),
            x if x == EntryType::Menu as u16 => Ok(EntryType::Menu),
            _ => Err(()),
        }
    }
}

// entry_type.rs ends here
