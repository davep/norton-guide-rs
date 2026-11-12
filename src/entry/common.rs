//! Provides the common entry structure for a Norton Guide database.

use crate::entry::parent::EntryParent;
use crate::entry::types::EntryType;
use crate::reader::Reader;

/// The maximum length of a line in a Norton Guide entry.
pub(crate) const MAX_LINE_LENGTH: usize = 1024;

/// The `Common` struct represents the common entry structure for a Norton Guide database.
#[derive(Debug, Clone)]
pub struct Common {
    /// The offset of the entry in the Norton Guide database file.
    pub offset: u64,
    /// The type of the entry (short or long).
    pub entry_type: EntryType,
    /// The size of the entry in bytes.
    pub size: u16,
    /// The number of lines in the entry.
    pub line_count: u16,
    /// Indicates whether the entry has a "see also" section.
    pub has_see_also: bool,
    /// The parent entry of the current entry.
    pub parent: EntryParent,
    /// The offset of the previous entry in the Norton Guide database file.
    pub previous: u32,
    /// The offset of the next entry in the Norton Guide database file.
    pub next: u32,
}

/// Implementations for the `Common` struct.
impl Common {
    /// Creates and populates a new `Common` instance.
    ///
    /// # Arguments
    ///
    /// * `guide` - A mutable reference to a `Reader` that reads from the
    ///   Norton Guide database file.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<Self>` - Returns an `Ok(Common)` if successful,
    ///   or an `Err(std::io::Error)` if reading fails.
    pub fn new(guide: &mut Reader) -> std::io::Result<Self> {
        Ok(Self {
            offset: guide.position()?,
            entry_type: EntryType::try_from(guide.read_encrypted_word()?).unwrap(),
            size: guide.read_encrypted_word()?,
            line_count: guide.read_encrypted_word()?,
            has_see_also: guide.read_encrypted_word()? > 0,
            parent: EntryParent::new(guide)?,
            previous: guide.read_encrypted_long()?,
            next: guide.read_encrypted_long()?,
        })
    }
}

// common.rs ends here
