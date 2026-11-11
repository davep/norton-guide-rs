//! Provides code for working with a Norton Guide's short entries.

use crate::NortonGuideError;
use crate::entry::common::MAX_LINE_LENGTH;
use crate::entry::{Common, EntryType};
use crate::reader::Reader;
use crate::rle::expand_rle;

#[derive(Debug, Clone)]
pub struct Short {
    /// The common entry structure for a Norton Guide short entry.
    pub common: Common,
    /// The offsets of the lines in the short entry.
    pub offsets: Vec<u32>,
    /// The lines of text in the short entry.
    pub lines: Vec<String>,
}

/// Implementations for the `Short` struct.
impl Short {
    /// Creates and populates a new `Short` instance.
    ///
    /// # Arguments
    ///
    /// * `common` - The common entry structure for the short entry.
    /// * `reader` - A mutable reference to a `Reader` that reads from the
    ///   Norton Guide database file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, NortonGuideError>` - Returns an `Ok(Short)` if
    ///   successful, or an `Err(NortonGuideError)` if reading fails.
    pub fn new(common: Common, reader: &mut Reader) -> Result<Self, NortonGuideError> {
        if common.entry_type != EntryType::Short {
            return Err(NortonGuideError::BadEntryTypeError {
                expected: EntryType::Short as u16,
                found: common.entry_type as u16,
            });
        }
        let mut offsets = Vec::with_capacity(common.line_count as usize);
        for _ in 0..common.line_count {
            reader.skip(2)?; // Skip a word -- I don't know what this is.
            offsets.push(reader.read_encrypted_long()?);
        }
        let mut lines = Vec::with_capacity(common.line_count as usize);
        for _ in 0..common.line_count {
            lines.push(
                reader
                    .read_encrypted_stringz(MAX_LINE_LENGTH, expand_rle)
                    .unwrap(),
            );
        }
        Ok(Self {
            common,
            offsets,
            lines,
        })
    }
}

// short.rs ends here
