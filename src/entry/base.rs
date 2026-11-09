//! Provides the base entry structure for a Norton Guide database.

// use crate::entry::types::EntryType;
// use crate::entry::parent::EntryParent;
// use crate::reader::Reader;

// #[derive(Debug, Clone)]
// struct Base {
//     offset: u64,
//     entry_type: EntryType,
//     size: u16,
//     line_count: u16,
//     has_see_also: bool,
//     parent: Option<EntryParent>,
//     previous: u32,
//     next: u32,
//     lines: Vec<String>,
// }

// impl Base  {
//     pub fn new(guide: &mut Reader) -> std::io::Result<Self> {
//         let offset = guide.position()?;
//         let entry_type: EntryType = EntryType::try_from(guide.read_encrypted_word()?).unwrap();
//         let size = guide.read_encrypted_word()?;
//         let line_count = guide.read_encrypted_word()?;
//         let has_see_also = guide.read_encrypted_word()? > 0;
//         let parent = EntryParent::new(guide)?;
//         let previous = guide.read_encrypted_long()?;
//         let next = guide.read_encrypted_long()?;
//         let mut lines = Vec::with_capacity(line_count as usize);
//         for _ in 0..line_count {
//             let line_length = guide.read_encrypted_word()? as usize;
//             lines.push(guide.read_string(line_length)?);
//         }
//         Ok(Self {
//             offset,
//             entry_type,
//             size,
//             line_count,
//             has_see_also,
//             parent,
//             previous,
//             next,
//             lines,
//         })
//     }
// }

// base.rs ends here
