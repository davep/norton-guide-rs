//! Provides code for working with parent entries in a Norton Guide database.

use std::io;

use crate::entry::optionals::{optional_index, optional_offset};
use crate::reader::Reader;

/// The `EntryParent` struct represents the parent entry of a Norton Guide entry.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn new(guide: &mut Reader) -> io::Result<Self> {
        Ok(Self {
            line: optional_index(guide.read_encrypted_word()?),
            offset: optional_offset(guide.read_encrypted_long()?),
            menu: optional_index(guide.read_encrypted_word()?),
            prompt: optional_index(guide.read_encrypted_word()?),
        })
    }

    pub fn exists(&self) -> bool {
        self.offset.is_some()
    }
}

// parent.rs ends here
