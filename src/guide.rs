use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

/// The header data of a Norton Guide database file.
#[derive(Debug)]
pub struct Header {
    /// The magic number that identifies the file as a Norton Guide database.
    magic: u16,
}

/// The `NortonGuide` struct represents a Norton Guide database file.
#[derive(Debug)]
pub struct NortonGuide {
    /// The header of the Norton Guide database file.
    header: Header,
    /// The path to the Norton Guide database file.
    path: PathBuf,
    /// The file handle for the Norton Guide database file.
    file: File,
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
    pub fn new(file: &mut impl Read) -> io::Result<Self> {
        let mut buffer = [0u8; 2];
        file.read_exact(&mut buffer)?;
        let magic = u16::from_le_bytes(buffer);
        Ok(Self { magic })
    }

    /// Returns the magic number of the Norton Guide database file.
    /// # Returns
    /// * `u16` - The magic number read from the file.
    pub fn magic(&self) -> u16 {
        self.magic
    }
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
        let mut file = File::open(&path)?;
        let header = Header::new(&mut file)?;
        Ok(Self { header, path, file })
    }

    /// Returns the path of the Norton Guide file.
    ///
    /// # Returns
    /// * `&Path` - A reference to the path of the Norton Guide file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Moves the file cursor to the specified position in the Norton Guide
    /// file.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to move the file cursor to.
    ///
    /// # Returns
    /// * `io::Result<u64>` - Returns an `Ok(u64)` with the new position if
    /// successful, or an `Err(io::Error)` if seeking fails.
    pub fn goto(&mut self, position: u64) -> io::Result<u64> {
        self.file.seek(SeekFrom::Start(position))
    }

    /// Returns the magic number of the Norton Guide database file.
    ///
    /// # Returns
    /// * `u16` - The magic number read from the file.
    pub fn magic(&self) -> u16 {
        self.header.magic()
    }

    /// Returns the magic number as a string.
    ///
    /// # Returns
    /// * `String` - The magic number converted to a string.
    pub fn magic_string(&self) -> String {
        String::from_utf8_lossy(&self.magic().to_le_bytes()).to_string()
    }
}

// guide.rs ends here
