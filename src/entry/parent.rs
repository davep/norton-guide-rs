//! Provides code for working with parent entries in a Norton Guide database.

use std::io;

use crate::entry::optionals::{optional_index, optional_offset};
use crate::reader::Reader;

/// The `EntryParent` struct represents the parent entry of a Norton Guide entry.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EntryParent {
    /// The line number of the parent entry.
    pub line: Option<u16>,
    /// The offset of the parent entry in the Norton Guide database file.
    pub offset: Option<u32>,
    /// The menu index of the parent entry.
    pub menu: Option<u16>,
    /// The prompt index of the parent entry.
    pub prompt: Option<u16>,
}

/// Implementations for the `EntryParent` struct.
impl EntryParent {
    /// Creates and populates a new `EntryParent` instance.
    ///
    /// # Arguments
    ///
    /// * `guide` - A mutable reference to a `Reader` that reads from the
    ///   Norton Guide database file.
    ///
    /// # Returns
    ///
    /// * `io::Result<Self>` - Returns an `Ok(EntryParent)` if successful,
    ///   or an `Err(io::Error)` if reading fails.
    pub fn new(guide: &mut Reader) -> io::Result<Self> {
        Ok(Self {
            line: optional_index(guide.read_encrypted_word()?),
            offset: optional_offset(guide.read_encrypted_long()?),
            menu: optional_index(guide.read_encrypted_word()?),
            prompt: optional_index(guide.read_encrypted_word()?),
        })
    }

    /// Checks if the parent entry exists.
    pub fn exists(&self) -> bool {
        self.offset.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::does_not_exist(EntryParent::default(), false)]
    #[case::does_exist(EntryParent { line: Some(1), offset: Some(2), menu: Some(3), prompt: Some(4) }, true)]
    fn test_entry_parent(#[case] entry_parent: EntryParent, #[case] expected_exists: bool) {
        assert_eq!(entry_parent.exists(), expected_exists);
    }
}

// parent.rs ends here
