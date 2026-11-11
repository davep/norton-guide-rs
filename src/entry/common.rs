//! Provides the common entry structure for a Norton Guide database.

use crate::entry::parent::EntryParent;
use crate::entry::types::EntryType;
use crate::reader::Reader;

pub(crate) const MAX_LINE_LENGTH: usize = 1024;

#[derive(Debug, Clone)]
pub struct Common {
    pub offset: u64,
    pub entry_type: EntryType,
    pub size: u16,
    pub line_count: u16,
    pub has_see_also: bool,
    pub parent: Option<EntryParent>,
    pub previous: u32,
    pub next: u32,
}

impl Common {
    pub fn new(guide: &mut Reader) -> std::io::Result<Self> {
        let offset = guide.position()?;
        let entry_type: EntryType = EntryType::try_from(guide.read_encrypted_word()?).unwrap();
        let size = guide.read_encrypted_word()?;
        let line_count = guide.read_encrypted_word()?;
        let has_see_also = guide.read_encrypted_word()? > 0;
        let parent = EntryParent::new(guide)?;
        let previous = guide.read_encrypted_long()?;
        let next = guide.read_encrypted_long()?;
        Ok(Self {
            offset,
            entry_type,
            size,
            line_count,
            has_see_also,
            parent,
            previous,
            next,
        })
    }
}

// common.rs ends here
