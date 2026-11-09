use crate::reader::Reader;
use std::io;

/// The header data of a Norton Guide database file.
#[derive(Debug)]
pub struct Header {
    /// The magic number that identifies the file as a Norton Guide database.
    magic: u16,
    /// The number of topics in the Norton Guide database file.
    menu_count: u16,
    /// The title of the Norton Guide database file.
    title: String,
    /// The credits associated with the Norton Guide database file.
    credits: [String; 5],
}

/// The length of the title field in the header, in bytes.
const TITLE_LENGTH: usize = 40;
/// The length of the credit field in the header, in bytes.
const CREDIT_LENGTH: usize = 66;
/// The number of credit fields in the header.
const CREDITS_COUNT: usize = 5;

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
        let magic = reader.read_word()?;
        reader.read_long()?; // Skip the next 4 bytes (reserved)
        let menu_count = reader.read_word()?;
        let title = reader.read_string(TITLE_LENGTH)?;
        let mut credits = Vec::with_capacity(CREDITS_COUNT);
        for _ in 0..CREDITS_COUNT {
            credits.push(reader.read_string(CREDIT_LENGTH)?);
        }
        let credits = credits.try_into().unwrap();
        Ok(Self {
            magic,
            menu_count,
            title,
            credits,
        })
    }

    /// Returns the magic number of the Norton Guide database file.
    /// # Returns
    /// * `u16` - The magic number read from the file.
    pub fn magic(&self) -> u16 {
        self.magic
    }

    /// Returns the number of topics in the Norton Guide database file.
    /// # Returns
    /// * `u16` - The number of topics read from the file.
    pub fn menu_count(&self) -> u16 {
        self.menu_count
    }

    /// Returns the title of the Norton Guide database file.
    /// # Returns
    /// * `&str` - A reference to the title string read from the file.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the credits associated with the Norton Guide database file.
    /// # Returns
    /// * `&[String; 5]` - A reference to the array of credit strings read from the file.
    pub fn credits(&self) -> &[String; 5] {
        &self.credits
    }
}

// header.rs ends here
