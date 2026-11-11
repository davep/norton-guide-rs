//! Provides code for working with a Norton Guide's long entries.

use crate::NortonGuideError;
use crate::entry::common::{Common, MAX_LINE_LENGTH};
use crate::entry::types::EntryType;
use crate::reader::Reader;
use crate::rle::expand_rle;

/// The `Long` struct represents a long entry in a Norton Guide database file.
#[derive(Debug, Clone)]
pub struct Long {
    /// The common entry structure for a Norton Guide long entry.
    pub common: Common,
    /// The lines of text in the long entry.
    pub lines: Vec<String>,
    // TODO: See also
}

/// Implementations for the `Long` struct.
impl Long {
    /// Creates and populates a new `Long` instance.
    ///
    /// # Arguments
    ///
    /// * `common` - The common entry structure for the long entry.
    /// * `reader` - A mutable reference to a `Reader` that reads from the
    ///   Norton Guide database file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, NortonGuideError>` - Returns an `Ok(Long)` if
    ///   successful, or an `Err(NortonGuideError)` if reading fails.
    pub fn new(common: Common, reader: &mut Reader) -> Result<Self, NortonGuideError> {
        if common.entry_type != EntryType::Long {
            return Err(NortonGuideError::BadEntryTypeError {
                expected: EntryType::Long as u16,
                found: common.entry_type as u16,
            });
        }
        let mut lines = Vec::with_capacity(common.line_count as usize);
        for _ in 0..common.line_count {
            lines.push(reader.read_encrypted_stringz(MAX_LINE_LENGTH, expand_rle)?);
        }
        Ok(Self { common, lines })
    }
}

// long.rs ends here
