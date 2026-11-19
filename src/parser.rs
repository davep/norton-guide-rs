//! Norton Guide parser for the text inside a guide.
//!
//! This module contains the parser for the text inside a Norton Guide. It
//! is used to parse the text and convert it into a more usable format.

use crate::dosify::from_dos_char;

/// The character that marks an upcoming attribute.
const CTRL_CHAR: char = '^';

/// The `ControlCode` enum represents the different control codes that can
/// be used in a Norton Guide. Each variant corresponds to a specific
/// control code character, which is represented by its ASCII value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlCode {
    /// The control code for a foreground/background colour attribute.
    Attr,
    /// The control code for bold text.
    Bold,
    /// The control code for a character.
    Char,
    /// The control code for a control character.
    Control,
    /// The control code for normal text (no special attributes).
    Normal,
    /// The control code for reverse text.
    Reverse,
    /// The control code for underlined text.
    Underline,
    /// An unknown/unsupported control code.
    Unsupported,
}

impl From<char> for ControlCode {
    fn from(value: char) -> Self {
        match value {
            'a' | 'A' => ControlCode::Attr,
            'b' | 'B' => ControlCode::Bold,
            'c' | 'C' => ControlCode::Char,
            'n' | 'N' => ControlCode::Normal,
            'r' | 'R' => ControlCode::Reverse,
            'u' | 'U' => ControlCode::Underline,
            CTRL_CHAR => ControlCode::Control,
            _ => ControlCode::Unsupported,
        }
    }
}

/// The `TextMode` enum represents the different text modes that can be
/// applied to text in a Norton Guide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextMode {
    /// The text is in normal mode (no special attributes).
    Normal,
    /// The text is in bold mode (bold attribute applied).
    Bold,
    /// The text is in underline mode (underline attribute applied).
    Underline,
    /// The text is in reverse mode (reverse attribute applied).
    Reverse,
    /// The text is in attribute mode (foreground/background colour incoming).
    Attribute,
}

/// The `ParseEvent` enum represents the events emitted by the [`Parser`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseEvent<'a> {
    /// A slice of plain text to display.
    Text(&'a str),
    /// A foreground/background colour attribute (`^aXX`).
    Colour(u8),
    /// Reset text mode back to normal (`^n` or toggled colour attribute).
    Normal,
    /// Enable/disable bold mode (`^b`).
    Bold(bool),
    /// Enable/disable reverse video mode (`^r`).
    Reverse(bool),
    /// Enable/disable underline mode (`^u`).
    Underline(bool),
}

/// Parser for Norton Guide text lines.
///
/// Implements [`Iterator`], yielding a stream of [`ParseEvent`] items as it
/// scans through the text and its control codes.
pub struct Parser<'a> {
    raw: &'a str,
    mode: TextMode,
    last_attribute: Option<u8>,
}

/// The `Parser` struct is responsible for parsing the text of a Norton
/// Guide and emitting events that represent the different control codes and
/// text modes.
impl<'a> Parser<'a> {
    /// Create a new parser for the given string slice.
    ///
    /// # Arguments
    ///
    /// * `raw` - A string slice containing the text to be parsed.
    ///
    /// # Returns
    ///
    /// A new instance of the `Parser` struct.
    pub fn new(raw: &'a str) -> Self {
        Self {
            raw,
            mode: TextMode::Normal,
            last_attribute: None,
        }
    }

    /// Return the [`ControlCode`] following a leading `^`, if any.
    fn ctrl_code(&self) -> ControlCode {
        self.raw
            .chars()
            .nth(1)
            .map_or(ControlCode::Unsupported, ControlCode::from)
    }

    /// Advance the unparsed slice by `count` bytes.
    fn advance(&mut self, count: usize) {
        self.raw = &self.raw[count.min(self.raw.len())..];
    }

    /// Reset the parser to normal text mode.
    fn normal(&mut self) -> ParseEvent<'a> {
        self.mode = TextMode::Normal;
        self.last_attribute = None;
        ParseEvent::Normal
    }

    /// Parse the next two characters as a hex byte, returning `None` if
    /// they are not valid hex digits.
    fn parse_hex_byte(&mut self) -> Option<u8> {
        u8::from_str_radix(self.raw.get(2..4)?, 16).ok()
    }

    /// Consume up to 4 characters as literal text when a 4-character control
    /// code (`^aXX` or `^cXX`) has invalid hex digits.
    fn invalid_hex_ctrl(&mut self) -> ParseEvent<'a> {
        let byte_len = self
            .raw
            .char_indices()
            .nth(4)
            .map_or(self.raw.len(), |(idx, _)| idx);
        let text = &self.raw[..byte_len];
        self.advance(byte_len);
        ParseEvent::Text(text)
    }

    /// Handle the `^aXX` control code for foreground/background colour attributes.
    fn ctrl_a(&mut self) -> ParseEvent<'a> {
        if let Some(colour) = self.parse_hex_byte() {
            self.advance(4);
            if self.mode == TextMode::Attribute && self.last_attribute == Some(colour) {
                // If a colour attribute is already active and the same
                // colour is being applied again, we will treat it as a
                // toggle and return to normal mode.
                self.normal()
            } else {
                // If a new colour attribute is being applied.
                self.last_attribute = Some(colour);
                self.mode = TextMode::Attribute;
                ParseEvent::Colour(colour)
            }
        } else {
            // The hex for the attribute is not valid, so we'll treat it as
            // a literal string instead of a control code.
            self.invalid_hex_ctrl()
        }
    }

    /// Handle the `^b` control code for toggling bold text mode.
    fn ctrl_b(&mut self) -> ParseEvent<'a> {
        self.advance(2);
        if self.mode == TextMode::Bold {
            self.mode = TextMode::Normal;
            ParseEvent::Bold(false)
        } else {
            self.mode = TextMode::Bold;
            ParseEvent::Bold(true)
        }
    }

    /// Handle the `^cXX` control code for inserting a character by its hex code.
    fn ctrl_c(&mut self) -> ParseEvent<'a> {
        if let Some(ch) = self.parse_hex_byte() {
            self.advance(4);
            ParseEvent::Text(from_dos_char(ch))
        } else {
            // The hex for the character is not valid, so we'll treat it as
            // a literal string instead of a control code.
            self.invalid_hex_ctrl()
        }
    }

    /// Handle the `^n` control code for resetting to normal text mode.
    fn ctrl_n(&mut self) -> ParseEvent<'a> {
        self.advance(2);
        self.normal()
    }

    /// Handle the `^r` control code for toggling reverse video text mode.
    fn ctrl_r(&mut self) -> ParseEvent<'a> {
        self.advance(2);
        if self.mode == TextMode::Reverse {
            self.mode = TextMode::Normal;
            ParseEvent::Reverse(false)
        } else {
            self.mode = TextMode::Reverse;
            ParseEvent::Reverse(true)
        }
    }

    /// Handle the `^u` control code for toggling underline text mode.
    fn ctrl_u(&mut self) -> ParseEvent<'a> {
        self.advance(2);
        if self.mode == TextMode::Underline {
            self.mode = TextMode::Normal;
            ParseEvent::Underline(false)
        } else {
            self.mode = TextMode::Underline;
            ParseEvent::Underline(true)
        }
    }
}

/// Implement the `Iterator` trait for the `Parser` struct, allowing it to
/// yield a stream of `ParseEvent` items as it scans through the text and
/// its control codes.
impl<'a> Iterator for Parser<'a> {
    type Item = ParseEvent<'a>;

    /// Return the next `ParseEvent` from the parser, or `None` if there are
    /// no more events to emit.
    fn next(&mut self) -> Option<Self::Item> {
        while !self.raw.is_empty() {
            match self.raw.find(CTRL_CHAR) {
                // No more control codes: emit the remaining text and finish.
                None => {
                    let text = self.raw;
                    self.raw = "";
                    return Some(ParseEvent::Text(text));
                }
                // Plain text before the next control code: emit the text up to
                // the '^'. Next call to `next()` will start right at the '^'.
                Some(ctrl_index) if ctrl_index > 0 => {
                    let text = &self.raw[..ctrl_index];
                    self.advance(ctrl_index);
                    return Some(ParseEvent::Text(text));
                }
                // `self.raw` starts with '^': handle the control code.
                Some(_) => match self.ctrl_code() {
                    ControlCode::Attr => return Some(self.ctrl_a()),
                    ControlCode::Bold => return Some(self.ctrl_b()),
                    ControlCode::Char => return Some(self.ctrl_c()),
                    ControlCode::Normal => return Some(self.ctrl_n()),
                    ControlCode::Reverse => return Some(self.ctrl_r()),
                    ControlCode::Underline => return Some(self.ctrl_u()),
                    ControlCode::Control => {
                        let text = &self.raw[..CTRL_CHAR.len_utf8()];
                        self.advance(2);
                        return Some(ParseEvent::Text(text));
                    }
                    ControlCode::Unsupported => {
                        // So the character after the ^ is not a valid control
                        // code. The choices here are to either raise an error,
                        // or just ignore it. Given that nobody is going to be
                        // fixing broken guides these days, we'll just ignore it.
                        self.advance(1);
                    }
                },
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParseEvent::*;
    use rstest::rstest;

    #[rstest]
    #[case::attr(&['a', 'A'], ControlCode::Attr)]
    #[case::bold(&['b', 'B'], ControlCode::Bold)]
    #[case::char(&['c', 'C'], ControlCode::Char)]
    #[case::normal(&['n', 'N'], ControlCode::Normal)]
    #[case::reverse(&['r', 'R'], ControlCode::Reverse)]
    #[case::underline(&['u', 'U'], ControlCode::Underline)]
    #[case::control(&[CTRL_CHAR], ControlCode::Control)]
    #[case::unsupported(&['x', 'y', 'z'], ControlCode::Unsupported)]
    fn test_control_code_conversion(#[case] input: &[char], #[case] expected: ControlCode) {
        assert!(
            input
                .iter()
                .all(|&code| ControlCode::from(code) == expected)
        );
    }

    #[rstest]
    #[case::single_ctrl_a("^aABTest", vec![Colour(0xAB), Text("Test")])]
    #[case::multiple_ctrl_a(
        "^aABTest^aBCMoreText^aCD",
        vec![Colour(0xAB), Text("Test"), Colour(0xBC), Text("MoreText"), Colour(0xCD)]
    )]
    #[case::invalid_hex_ctrl_a("^aZZTest", vec![Text("^aZZ"), Text("Test")])]
    #[case::toggle_ctrl_a(
        "^aABTest^aABMoreText",
        vec![Colour(0xAB), Text("Test"), Normal, Text("MoreText")]
    )]
    #[case::ctrl_b_toggle(
        "^bBoldText^bNormalText",
        vec![Bold(true), Text("BoldText"), Bold(false), Text("NormalText")]
    )]
    #[case::single_ctrl_c("^c41Test", vec![Text("A"), Text("Test")])]
    #[case::multiple_ctrl_c(
        "^c41Test^c42MoreText^c43",
        vec![Text("A"), Text("Test"), Text("B"), Text("MoreText"), Text("C")]
    )]
    #[case::invalid_hex_ctrl_c("^cZZTest", vec![Text("^cZZ"), Text("Test")])]
    #[case::ctrl_r_toggle(
        "^rReverseText^rNormalText",
        vec![Reverse(true), Text("ReverseText"), Reverse(false), Text("NormalText")]
    )]
    #[case::ctrl_u_toggle(
        "^uUnderlineText^uNormalText",
        vec![Underline(true), Text("UnderlineText"), Underline(false), Text("NormalText")]
    )]
    #[case::many_ctrls(
        "^bBold^rReverse^uUnderline^nNormal",
        vec![Bold(true), Text("Bold"), Reverse(true), Text("Reverse"), Underline(true), Text("Underline"), Normal, Text("Normal")]
    )]
    #[case::unsupported_ctrl(
        "^xUnsupported^yUnsupported",
        vec![Text("xUnsupported"), Text("yUnsupported")]
    )]
    #[case::literal_ctrl("^^ literal ^^", vec![Text("^"), Text(" literal "), Text("^")])]
    #[case::lone_dangling_ctrl("^", vec![])]
    #[case::dangling_ctrl("Test^", vec![Text("Test")])]
    fn test_control_characters(#[case] input: &str, #[case] expected: Vec<ParseEvent>) {
        assert!(Parser::new(input).eq(expected));
    }
}

// parser.rs ends here
