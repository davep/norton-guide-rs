//! Norton Guide parser for the text inside a guide.
//!
//! This module contains the parser for the text inside a Norton Guide. It
//! is used to parse the text and convert it into a more usable format.

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

impl TryFrom<char> for ControlCode {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'A' => Ok(ControlCode::Attr),
            'b' | 'B' => Ok(ControlCode::Bold),
            'c' | 'C' => Ok(ControlCode::Char),
            'n' | 'N' => Ok(ControlCode::Normal),
            'r' | 'R' => Ok(ControlCode::Reverse),
            'u' | 'U' => Ok(ControlCode::Underline),
            '^' => Ok(ControlCode::Control),
            _ => Ok(ControlCode::Unsupported),
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

struct ParseState {
    raw: String,
    ctrl: Option<usize>,
    mode: TextMode,
    last_attribute: Option<u8>,
}

impl ParseState {
    fn new(string: impl Into<String>) -> Self {
        let raw = string.into();
        let ctrl = raw.find(CTRL_CHAR);
        Self {
            raw,
            ctrl,
            mode: TextMode::Normal,
            last_attribute: None,
        }
    }

    fn work_left(&self) -> bool {
        self.ctrl.is_some() && self.ctrl.unwrap() < self.raw.len()
    }

    fn ctrl_id(&self) -> Option<ControlCode> {
        ControlCode::try_from(self.raw.chars().nth(self.ctrl? + 1)?).ok()
    }

    fn move_ctrl(&mut self, count: usize) {
        if let Some(ctrl) = self.ctrl {
            self.ctrl = Some(ctrl + count);
        } else {
            self.ctrl = Some(count);
        }
    }

    fn find_next_ctrl(&mut self) {
        self.raw = self.raw[self.ctrl.unwrap_or(0)..].to_string();
        self.ctrl = self.raw.find(CTRL_CHAR);
    }
}

pub trait ParserEvents {
    fn on_text(&mut self, _text: &str) {}
    fn on_colour(&mut self, _colour: u8) {}
    fn on_char(&mut self, _character: char) {}
    fn on_normal(&mut self) {}
    fn on_bold(&mut self) {}
    fn on_unbold(&mut self) {}
    fn on_reverse(&mut self) {}
    fn on_unreverse(&mut self) {}
    fn on_underline(&mut self) {}
    fn on_ununderline(&mut self) {}
}

pub struct Parser<'a, E: ParserEvents> {
    state: ParseState,
    events: &'a mut E,
}

impl<'a, E: ParserEvents> Parser<'a, E> {
    pub fn new(string: impl Into<String>, events: &'a mut E) -> Self {
        Self {
            state: ParseState::new(string),
            events,
        }
    }

    fn normal(&mut self) {
        self.events.on_normal();
        self.state.mode = TextMode::Normal;
        self.state.last_attribute = None;
    }

    fn ctrl_a(&mut self) {
        let ctrl_index = self
            .state
            .ctrl
            .expect("unexpected lack of control character to handle");
        if let Some(colour) = self
            .state
            .raw
            .get(ctrl_index + 2..ctrl_index + 4)
            .and_then(|hex| u8::from_str_radix(hex, 16).ok())
        {
            if self.state.mode == TextMode::Attribute && self.state.last_attribute == Some(colour) {
                // If a colour attribute is already active and the same
                // colour is being applied again, we will treat it as a
                // toggle and return to normal mode.
                self.normal();
            } else {
                // If a new colour attribute is being applied.
                self.events.on_colour(colour);
                self.state.last_attribute = Some(colour);
                self.state.mode = TextMode::Attribute;
            }
            self.state.move_ctrl(4);
        } else {
            // The hex for the attribute is not valid, so we'll treat it as
            // a literal string instead of a control code.
            self.events.on_text(&self.state.raw[..4]);
            self.state.move_ctrl(2);
        }
    }

    fn ctrl_b(&mut self) {
        if self.state.mode == TextMode::Bold {
            self.events.on_unbold();
            self.state.mode = TextMode::Normal;
        } else {
            self.events.on_bold();
            self.state.mode = TextMode::Bold;
        }
        self.state.move_ctrl(2);
    }

    fn ctrl_c(&mut self) {
        // Handle character control code
    }

    fn ctrl_n(&mut self) {
        self.normal();
        self.state.move_ctrl(2);
    }

    fn ctrl_r(&mut self) {
        if self.state.mode == TextMode::Reverse {
            self.events.on_unreverse();
            self.state.mode = TextMode::Normal;
        } else {
            self.events.on_reverse();
            self.state.mode = TextMode::Reverse;
        }
        self.state.move_ctrl(2);
    }

    fn ctrl_u(&mut self) {
        if self.state.mode == TextMode::Underline {
            self.events.on_ununderline();
            self.state.mode = TextMode::Normal;
        } else {
            self.events.on_underline();
            self.state.mode = TextMode::Underline;
        }
        self.state.move_ctrl(2);
    }

    pub fn parse(&mut self) {
        while self.state.work_left() {
            let ctrl_index = self
                .state
                .ctrl
                .expect("unexpected lack of control character to handle");
            if self.state.raw.chars().take(ctrl_index).count() > 0 {
                self.events.on_text(&self.state.raw[..ctrl_index]);
            }

            match self.state.ctrl_id() {
                Some(ControlCode::Attr) => {
                    self.ctrl_a();
                }
                Some(ControlCode::Bold) => {
                    self.ctrl_b();
                }
                Some(ControlCode::Char) => {
                    self.ctrl_c();
                }
                Some(ControlCode::Normal) => {
                    self.ctrl_n();
                }
                Some(ControlCode::Reverse) => {
                    self.ctrl_r();
                }
                Some(ControlCode::Underline) => {
                    self.ctrl_u();
                }
                Some(ControlCode::Control) => {
                    self.events.on_text("^");
                    self.state.move_ctrl(2);
                }
                Some(ControlCode::Unsupported) | None => {
                    // So the character after the ^ is not a valid control
                    // code. The choices here are to either raise an error,
                    // or just ignore it. Given that nobody is going to be
                    // fixing broken guides these days, we'll just ignore it.
                    self.state.move_ctrl(1);
                }
            }

            self.state.find_next_ctrl();
        }

        // Sweep up any remaining text after the last control code.
        if self.state.raw.chars().count() > 0 {
            self.events.on_text(&self.state.raw);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::attr(&['a', 'A'], ControlCode::Attr)]
    #[case::bold(&['b', 'B'], ControlCode::Bold)]
    #[case::char(&['c', 'C'], ControlCode::Char)]
    #[case::normal(&['n', 'N'], ControlCode::Normal)]
    #[case::reverse(&['r', 'R'], ControlCode::Reverse)]
    #[case::underline(&['u', 'U'], ControlCode::Underline)]
    #[case::control(&['^'], ControlCode::Control)]
    #[case::unsupported(&['x', 'y', 'z'], ControlCode::Unsupported)]
    fn test_control_code_conversion(#[case] input: &[char], #[case] expected: ControlCode) {
        assert!(
            input
                .iter()
                .all(|&code| ControlCode::try_from(code) == Ok(expected))
        );
    }

    #[derive(Default)]
    struct ParserEventsMock {
        events: Vec<String>,
        text: Vec<String>,
    }

    impl ParserEventsMock {
        fn new() -> Self {
            Self::default()
        }
    }

    impl ParserEvents for ParserEventsMock {

        fn on_text(&mut self, text: &str) {
            self.text.push(text.to_string());
        }

        fn on_bold(&mut self) {
            self.events.push("bold".to_string());
        }

        fn on_unbold(&mut self) {
            self.events.push("unbold".to_string());
        }

        fn on_reverse(&mut self) {
            self.events.push("reverse".to_string());
        }

        fn on_unreverse(&mut self) {
            self.events.push("unreverse".to_string());
        }

        fn on_underline(&mut self) {
            self.events.push("underline".to_string());
        }

        fn on_ununderline(&mut self) {
            self.events.push("ununderline".to_string());
        }

        fn on_normal(&mut self) {
            self.events.push("normal".to_string());
        }

        fn on_colour(&mut self, colour: u8) {
            self.events.push(format!("colour(0x{colour:X})"));
        }
    }

    #[rstest]
    fn test_initial_parser_state_no_ctrl() {
        const TEST_STRING: &str = "This is a test string without control characters.";
        let mut events = ParserEventsMock::new();
        let parser = Parser::new(TEST_STRING, &mut events);
        assert_eq!(parser.state.raw, TEST_STRING);
        assert_eq!(parser.state.ctrl, None);
        assert!(matches!(parser.state.mode, TextMode::Normal));
        assert_eq!(parser.state.last_attribute, None);
    }

    #[rstest]
    fn test_initial_parser_state_with_ctrl() {
        const TEST_STRING: &str = "This is a ^btest^b string with a control character.";
        let mut events = ParserEventsMock::new();
        let parser = Parser::new(TEST_STRING, &mut events);
        assert_eq!(parser.state.raw, TEST_STRING);
        assert_eq!(parser.state.ctrl, Some(10)); // Index of the first control character '^'
        assert!(matches!(parser.state.mode, TextMode::Normal));
        assert_eq!(parser.state.last_attribute, None);
    }

    #[rstest]
    #[case::single_ctrl_a("^aABTest", vec!["colour(0xAB)"], vec!["Test"])]
    #[case::multiple_ctrl_a("^aABTest^aBCMoreText^aCD", vec!["colour(0xAB)", "colour(0xBC)", "colour(0xCD)"], vec!["Test", "MoreText"])]
    #[case::toggle_ctrl_a("^aABTest^aABMoreText", vec!["colour(0xAB)", "normal"], vec!["Test", "MoreText"])]
    #[case::ctrl_b_toggle("^bBoldText^bNormalText", vec!["bold", "unbold"], vec!["BoldText", "NormalText"])]
    #[case::ctrl_r_toggle("^rReverseText^rNormalText", vec!["reverse", "unreverse"], vec!["ReverseText", "NormalText"])]
    #[case::ctrl_u_toggle("^uUnderlineText^uNormalText", vec!["underline", "ununderline"], vec!["UnderlineText", "NormalText"])]
    fn test_control_characters(#[case] input: &str, #[case] expected_events: Vec<&str>, #[case] expected_text: Vec<&str>) {
        let mut events = ParserEventsMock::new();
        let mut parser = Parser::new(input, &mut events);
        parser.parse();
        assert_eq!(events.events, expected_events);
        assert_eq!(events.text, expected_text);
    }

    // TODO: Test for invalid hex after ^a.

}

// parser.rs ends here
