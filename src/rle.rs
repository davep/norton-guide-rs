//! Provides functions for handling Norton Guide run-length encoding.

use std::iter::repeat_n;

/// The marker byte used to indicate the start of a run-length encoded
/// sequence in the Norton Guide database file.
const RLE_MARKER: u8 = 0xFF;

/// A type alias for a function that takes a byte slice and returns a vector
/// of bytes.
pub type RleFn = fn(&[u8]) -> Vec<u8>;

/// Cleans the run-length encoded (RLE) marker from the given buffer.
///
/// This function iterates through the provided buffer and replaces any
/// occurrences of the RLE marker (0xFF) with a space character (0x20).
///
/// # Arguments
///
/// * `buffer` - A slice of bytes that may contain RLE markers.
///
/// # Returns
///
/// * `Vec<u8>` - A vector of bytes with RLE markers replaced by spaces.
pub fn clean_rle(buffer: &[u8]) -> Vec<u8> {
    buffer
        .iter()
        .map(|&byte| if byte == RLE_MARKER { b' ' } else { byte })
        .collect()
}

/// Expands the run-length encoded (RLE) sequences in the given buffer.
///
/// Norton Guide database files use a very simple form of
/// run-length-encoding for spaces. Simply put, if you find a byte in a
/// string that is `0xFF`, then the next byte is the number of spaces to
/// insert into the string at this point. I've also found that `0xFF`
/// followed by `0xFF` seems to mean you should insert a literal `0xFF` (I
/// think), although I've found that using a literal space makes more sense.
///
/// # Arguments
///
/// * `buffer` - A slice of bytes that may contain RLE sequences.
///
/// # Returns
///
/// * `Vec<u8>` - A vector of bytes with RLE sequences expanded into spaces.
pub fn expand_rle(buffer: &[u8]) -> Vec<u8> {
    let mut expanded = Vec::with_capacity(buffer.len());
    let mut iter = buffer.iter().copied();

    while let Some(byte) = iter.next() {
        if byte == RLE_MARKER {
            match iter.next() {
                // Trailing 0xFF at end of slice -> single space
                None => expanded.push(b' '),
                // 0xFF followed by 0xFF -> single space
                Some(RLE_MARKER) => expanded.push(b' '),
                // 0xFF followed by count byte -> repeat space count times
                Some(count) => expanded.extend(repeat_n(b' ', usize::from(count))),
            }
        } else {
            expanded.push(byte);
        }
    }

    expanded
}
// rle.rs ends here
