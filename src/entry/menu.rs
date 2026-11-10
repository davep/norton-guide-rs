//! Provides code for working with menu entries in a Norton Guide database.

use crate::reader::Reader;

/// The `Menu` struct represents a menu entry in a Norton Guide database
/// file. It contains information about the number of prompts, their
/// offsets, the title of the menu, and the prompts themselves.
#[derive(Debug, Clone)]
pub struct Menu {
    count: u16,
    offsets: Vec<u32>,
    title: String,
    prompts: Vec<String>,
}

/// Implementations for the `Menu` struct.
impl Menu {
    /// Creates and populates a new `Menu` instance.
    ///
    /// # Arguments
    ///
    /// * `guide` - A mutable reference to a `Reader` that reads from the
    ///    Norton Guide database file.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<Self>` - Returns an `Ok(Menu)` if successful, or
    ///   an `Err(std::io::Error)` if reading fails.
    pub fn new(guide: &mut Reader) -> std::io::Result<Self> {
        // Skip the type marker and the byte size of the menu.
        guide.skip(4)?;
        // I can't for the life of me remember while I subtracted 1 from the
        // count, but it seems to be necessary for the menu to work
        // properly.
        let count = guide.read_encrypted_word()? - 1;
        // Skip 20 bytes; I'm not sure what's held in here.
        guide.skip(20)?;
        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            offsets.push(guide.read_encrypted_long()?);
        }
        // Skip a number of values I don't know the purpose of, but I've
        // never needed. It seems to be two sets of long integer arrays.
        guide.skip((count * 8) as i64)?;
        let title_length = guide.read_encrypted_word()? as usize;
        let title = guide.read_string(title_length)?;
        let mut prompts = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let prompt_length = guide.read_encrypted_word()? as usize;
            prompts.push(guide.read_encrypted_stringz(prompt_length)?);
        }
        guide.skip(1)?; // Skip the final reserved byte.
        Ok(Self {
            count,
            offsets,
            title,
            prompts,
        })
    }

    /// Returns the number of prompts in the menu.
    pub fn prompt_count(&self) -> u16 {
        self.count
    }

    /// Returns the title of the menu.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the prompt at the specified index, if it exists.
    pub fn prompt(&self, index: usize) -> Option<&str> {
        self.prompts.get(index).map(|s| s.as_str())
    }

    /// Returns the offset of the prompt at the specified index, if it exists.
    pub fn offset(&self, index: usize) -> Option<u32> {
        self.offsets.get(index).copied()
    }
}
// menu.rs ends here
