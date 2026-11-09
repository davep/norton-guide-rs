#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideMaker {
    NortonGuide,
    ExpertHelp,
    Unknown,
}

impl GuideMaker {
    pub fn from_magic(magic: u16) -> Self {
        match &magic.to_le_bytes() {
            b"NG" => Self::NortonGuide,
            b"EH" => Self::ExpertHelp,
            _ => Self::Unknown,
        }
    }

    pub fn to_magic(&self) -> u16 {
        match self {
            Self::NortonGuide => u16::from_le_bytes(*b"NG"),
            Self::ExpertHelp => u16::from_le_bytes(*b"EH"),
            Self::Unknown => 0,
        }
    }

    pub fn to_magic_string(&self) -> String {
        match self {
            Self::NortonGuide => "NG".to_string(),
            Self::ExpertHelp => "EH".to_string(),
            Self::Unknown => "??".to_string(),
        }
    }

    pub fn made_with(&self) -> &'static str {
        match self {
            Self::NortonGuide => "Norton Guide",
            Self::ExpertHelp => "Expert Help",
            Self::Unknown => "Unknown",
        }
    }

    pub fn is_valid(&self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

// magic.rs ends here
