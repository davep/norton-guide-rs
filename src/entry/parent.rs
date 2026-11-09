//! Provides code for working with parent entries in a Norton Guide database.

use std::io;

use crate::entry::optionals::{optional_index, optional_offset};
use crate::reader::Reader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryParent {
    pub line: Option<u16>,
    pub offset: Option<u32>,
    pub menu: Option<u16>,
    pub prompt: Option<u16>,
}

impl EntryParent {
    pub fn new(guide: &mut Reader) -> io::Result<Option<Self>> {
        let line = optional_index(guide.read_encrypted_word()?);
        let offset = optional_offset(guide.read_encrypted_long()?);
        let menu = optional_index(guide.read_encrypted_word()?);
        let prompt = optional_index(guide.read_encrypted_word()?);
        Ok(Some(Self {
            line,
            offset,
            menu,
            prompt,
        }))
    }

    pub fn exists(&self) -> bool {
        self.offset.is_some()
    }
}

// parent.rs ends here
