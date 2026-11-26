//! Code to support the DOS heritage of most Norton Guide databases.

/// Lookup table mapping each MS-DOS (CP437) byte to its static UTF-8 string slice.
const CP437_STR: [&str; 256] = [
    " ", "\u{263a}", "\u{263b}", "\u{2665}", "\u{2666}", "\u{2663}", "\u{2660}", "\u{2022}",
    "\u{25db}", "\u{25cb}", "\u{25d9}", "\u{2642}", "\u{2640}", "\u{266a}", "\u{266b}", "\u{263c}",
    "\u{25ba}", "\u{25c4}", "\u{2195}", "\u{203c}", "\u{00b6}", "\u{00a7}", "\u{25ac}", "\u{21a8}",
    "\u{2191}", "\u{2193}", "\u{2192}", "\u{2190}", "\u{221f}", "\u{2194}", "\u{25b2}", "\u{25bc}",
    " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@", "A", "B", "C", "D", "E",
    "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k",
    "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~",
    "\u{2302}", "\u{00c7}", "\u{00fc}", "\u{00e9}", "\u{00e2}", "\u{00e4}", "\u{00e0}", "\u{00e5}",
    "\u{00e7}", "\u{00ea}", "\u{00eb}", "\u{00e8}", "\u{00ef}", "\u{00ee}", "\u{00ec}", "\u{00c4}",
    "\u{00c5}", "\u{00c9}", "\u{00e6}", "\u{00c6}", "\u{00f4}", "\u{00f6}", "\u{00f2}", "\u{00fb}",
    "\u{00f9}", "\u{00ff}", "\u{00d6}", "\u{00dc}", "\u{00a2}", "\u{00a3}", "\u{00a5}", "\u{20a7}",
    "\u{0192}", "\u{00e1}", "\u{00ed}", "\u{00f3}", "\u{00fa}", "\u{00f1}", "\u{00d1}", "\u{00aa}",
    "\u{00ba}", "\u{00bf}", "\u{2319}", "\u{00ac}", "\u{00bd}", "\u{00bc}", "\u{00a1}", "\u{00ab}",
    "\u{00bb}", "\u{2591}", "\u{2592}", "\u{2593}", "\u{2502}", "\u{2524}", "\u{2561}", "\u{2562}",
    "\u{2556}", "\u{2555}", "\u{2563}", "\u{2551}", "\u{2557}", "\u{255d}", "\u{255c}", "\u{255b}",
    "\u{2510}", "\u{2514}", "\u{2534}", "\u{252c}", "\u{251c}", "\u{2500}", "\u{253c}", "\u{255e}",
    "\u{255f}", "\u{255a}", "\u{2554}", "\u{2569}", "\u{2566}", "\u{2560}", "\u{2550}", "\u{256c}",
    "\u{2567}", "\u{2568}", "\u{2564}", "\u{2565}", "\u{2559}", "\u{2558}", "\u{2552}", "\u{2553}",
    "\u{256b}", "\u{256a}", "\u{251b}", "\u{250c}", "\u{2588}", "\u{2584}", "\u{258c}", "\u{2590}",
    "\u{2580}", "\u{03b1}", "\u{00df}", "\u{0393}", "\u{03c0}", "\u{03a3}", "\u{03c3}", "\u{00b5}",
    "\u{03c4}", "\u{03a6}", "\u{039b}", "\u{03a9}", "\u{03b4}", "\u{221e}", "\u{03c6}", "\u{03b5}",
    "\u{2229}", "\u{2261}", "\u{00b1}", "\u{2265}", "\u{2264}", "\u{2320}", "\u{2321}", "\u{00f7}",
    "\u{2248}", "\u{00b0}", "\u{2219}", "\u{00b7}", "\u{221a}", "\u{207f}", "\u{00b2}", "\u{25a0}",
    "\u{00a0}",
];

/// Convert a single MS-DOS (CP437) byte into its Unicode `&'static str` equivalent.
#[inline]
pub const fn from_dos_char(byte: u8) -> &'static str {
    CP437_STR[byte as usize]
}

/// Try and make a slice of bytes from a guide capable of retaining
/// DOS-likeness.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to convert.
///
/// # Returns
///
/// * `String` - A string representation of the DOS-like characters.
pub fn make_dos_like(bytes: &[u8]) -> String {
    bytes.iter().copied().map(from_dos_char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty_slice(&[], "")]
    #[case::ascii_bytes(b"Hello, World!", "Hello, World!")]
    #[case::higher_text(&[0xC4], "─")]
    fn test_make_dos_like(#[case] bytes: &[u8], #[case] expected: &str) {
        assert_eq!(make_dos_like(bytes), expected);
    }
}

// dosify.rs ends here
