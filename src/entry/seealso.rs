//! Provides code for working with a Norton Guide's "see also" entries.

use crate::NortonGuideError;
use crate::entry::menu::MAX_PROMPT_LENGTH;
use crate::reader::Reader;
use crate::rle::clean_rle;

/// The `SeeAlso` struct represents a "see also" entry in a Norton Guide
/// database file.
#[derive(Debug, Clone)]
pub struct SeeAlso {
    /// The number of "see also" entries.
    pub count: u16,
    /// The offsets of the "see also" entries in the Norton Guide database file.
    pub offsets: Vec<u32>,
    /// The prompts associated with the "see also" entries.
    pub prompts: Vec<String>,
}

/// Implementations for the `SeeAlso` struct.
impl SeeAlso {
    /// Creates and populates a new `SeeAlso` instance.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `Reader` that reads from the
    ///   Norton Guide database file.
    ///
    /// # Returns
    ///
    /// * `Result<Self, NortonGuideError>` - Returns an `Ok(SeeAlso)` if
    ///   successful, or an `Err(NortonGuideError)` if reading fails.
    pub fn new(reader: &mut Reader) -> Result<Self, NortonGuideError> {
        let count = reader.read_encrypted_word()?;
        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            offsets.push(reader.read_encrypted_word()? as u32);
        }
        let mut prompts = Vec::with_capacity(count as usize);
        for _ in 0..count {
            prompts.push(reader.read_encrypted_stringz(MAX_PROMPT_LENGTH, clean_rle)?);
        }
        Ok(Self {
            count,
            offsets,
            prompts,
        })
    }
}

// seealso.rs ends here
