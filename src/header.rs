use crate::reader::Reader;
use std::io;

/// The header data of a Norton Guide database file.
#[derive(Debug)]
pub struct Header {
    /// The magic number that identifies the file as a Norton Guide database.
    magic: u16,
}

/// Implementations for the `Header` struct.
impl Header {
    /// Creates a new guide `Header` instance.
    ///
    /// # Arguments
    ///
    /// * `file` - A mutable reference to a type that implements the `Read`
    ///   trait.
    ///
    /// # Returns
    /// * `io::Result<Self>` - Returns an `Ok(Header)` if successful, or
    ///   an `Err(io::Error)` if reading fails.
    pub fn new(reader: &mut Reader) -> io::Result<Self> {
        Ok(Self {
            magic: reader.read_word()?,
        })
    }

    /// Returns the magic number of the Norton Guide database file.
    /// # Returns
    /// * `u16` - The magic number read from the file.
    pub fn magic(&self) -> u16 {
        self.magic
    }
}

// header.rs ends here
