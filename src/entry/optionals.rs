//! Provides code for working with optional values in a guide.

/// Turn an index into an optional index.
///
/// # Arguments
///
/// * `value` - The index value to convert.
///
/// # Returns
///
/// * `Option<u16>` - Returns `Some(value)` if the value is not `0xFFFF`,
///   otherwise returns `None`.
#[inline]
pub fn optional_index(value: u16) -> Option<u16> {
    (value != 0xFFFF).then_some(value)
}

/// Turn an offset into an optional offset.
///
/// # Arguments
///
/// * `value` - The offset value to convert.
///
/// # Returns
///
/// * `Option<u32>` - Returns `Some(value)` if the value is not `0` or
///   `0xFFFFFFFF`, otherwise returns `None`.
#[inline]
pub fn optional_offset(value: u32) -> Option<u32> {
    match value {
        0 | 0xFFFFFFFF => None,
        _ => Some(value),
    }
}

// optionals.rs ends here
