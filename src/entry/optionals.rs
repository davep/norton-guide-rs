//! Provides code for working with optional values in a guide.

#[inline]
pub fn optional_index(value: u16) -> Option<u16> {
    if value == 0xFFFF { None } else { Some(value) }
}

#[inline]
pub fn optional_offset(value: u32) -> Option<u32> {
    if value == 0 || value == 0xFFFFFFFF {
        None
    } else {
        Some(value)
    }
}

// optionals.rs ends here
