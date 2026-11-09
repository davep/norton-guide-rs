use std::io;
use std::path::{Path, PathBuf};

use crate::header::Header;
use crate::magic::GuideMaker;
use crate::reader::Reader;

/// The `NortonGuide` struct represents a Norton Guide database file.
#[derive(Debug)]
pub struct NortonGuide {
    /// The header of the Norton Guide database file.
    header: Header,
    /// The path to the Norton Guide database file.
    path: PathBuf,
    /// The file reader for the Norton Guide database file.
    reader: Reader,
}

/// Implementations for the `NortonGuide` struct.
impl NortonGuide {
    /// Opens a Norton Guide database file at the specified path and reads
    /// its header.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the Norton Guide database file.
    ///
    /// # Returns
    /// * `io::Result<Self>` - Returns an `Ok(NortonGuide)` if successful,
    ///   or an `Err(io::Error)` if opening the file or reading the header
    ///   fails.
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut reader = Reader::new(&path)?;
        let header = Header::new(&mut reader)?;
        Ok(Self {
            header,
            path,
            reader,
        })
    }

    /// Returns the path of the Norton Guide file.
    ///
    /// # Returns
    /// * `&Path` - A reference to the path of the Norton Guide file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn magic(&self) -> GuideMaker {
        GuideMaker::from_magic(self.header.magic())
    }

    pub fn is_valid(&self) -> bool {
        self.magic().is_valid()
    }

    /// Returns the magic number as a string.
    ///
    /// # Returns
    /// * `String` - The magic number converted to a string.
    pub fn magic_string(&self) -> String {
        self.magic().to_magic_string()
    }

    /// Returns the name of the software that created the Norton Guide file.
    ///
    /// # Returns
    /// * `&'static str` - The name of the software that created the Norton Guide file.
    pub fn made_with(&self) -> &'static str {
        self.magic().made_with()
    }

    /// Returns the number of topics in the Norton Guide database file.
    ///
    /// # Returns
    /// * `u16` - The number of topics read from the file.
    pub fn menu_count(&self) -> u16 {
        self.header.menu_count()
    }

    /// Returns the title of the Norton Guide database file.
    ///
    /// # Returns
    /// * `&str` - A reference to the title string read from the file.
    pub fn title(&self) -> &str {
        self.header.title()
    }

    /// Returns the credits associated with the Norton Guide database file.
    ///
    /// # Returns
    /// * `&[String; 5]` - A reference to the array of credit strings read from the file.
    pub fn credits(&self) -> &[String; 5] {
        self.header.credits()
    }

    /// Returns a mutable reference to the `Reader` instance for the Norton
    /// Guide database file.
    ///
    /// # Returns
    /// * `&mut Reader` - A mutable reference to the `Reader` instance.
    pub fn reader(&mut self) -> &mut Reader {
        &mut self.reader
    }
}

// guide.rs ends here
