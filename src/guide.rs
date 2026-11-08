use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Header {
    magic: u16,
}

#[derive(Debug)]
pub struct NortonGuide {
    header: Header,
    path: PathBuf,
    file: File,
}

impl Header {
    pub fn new(file: &mut File) -> io::Result<Self> {
        let mut buffer = [0u8; 2];
        file.read_exact(&mut buffer)?;
        let magic = u16::from_le_bytes(buffer);
        Ok(Self { magic })
    }

    pub fn magic(&self) -> u16 {
        self.magic
    }
}

impl NortonGuide {
    /// The length of a title in the header.
    pub const TITLE_LENGTH: usize = 40;
    /// The length of a line in the credits, in the header.
    pub const CREDIT_LENGTH: usize = 66;

    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let header = Header::new(&mut file.try_clone()?)?;
        Ok(Self { header, path, file })
    }

    /// Returns the path of the Norton Guide file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Moves the file cursor to the specified position in the Norton Guide file.
    pub fn goto(&mut self, position: u64) -> io::Result<u64> {
        self.file.seek(SeekFrom::Start(position))
    }

    pub fn magic(&self) -> u16 {
        self.header.magic()
    }

    pub fn magic_string(&self) -> String {
        String::from_utf8_lossy(&self.magic().to_le_bytes()).to_string()
    }

}

// guide.rs ends here
