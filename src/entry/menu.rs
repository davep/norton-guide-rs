//! Provides code for working with menu entries in a Norton Guide database.

use crate::reader::Reader;

#[derive(Debug, Clone)]
pub struct Menu {
    count: u16,
    offsets: Vec<u32>,
    title: String,
    prompts: Vec<String>,
}

impl Menu {
    pub fn new(guide: &mut Reader) -> std::io::Result<Self> {
        guide.skip(4)?; // Skip the type marker and the byte size of the menu.
        let count = guide.read_encrypted_word()? - 1;
        guide.skip(20)?; // Skip the reserved bytes.
        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            offsets.push(guide.read_encrypted_long()?);
        }
        guide.skip((count * 8) as i64)?; // Skip a set of 8 reserved bytes for each offset.
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

    pub fn prompt_count(&self) -> u16 {
        self.count
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn prompt(&self, index: usize) -> Option<&str> {
        self.prompts.get(index).map(|s| s.as_str())
    }

    pub fn offset(&self, index: usize) -> Option<u32> {
        self.offsets.get(index).copied()
    }
}
// menu.rs ends here
