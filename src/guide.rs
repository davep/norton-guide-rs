use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct NortonGuide {

    path: PathBuf,
}

impl NortonGuide {

    /// The length of a title in the header.
    pub const TITLE_LENGTH: usize = 40;
    /// The length of a line in the credits, in the header.
    pub const CREDIT_LENGTH: usize = 66;

    pub fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            ));
        }
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

// guide.rs ends here
