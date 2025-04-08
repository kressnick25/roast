use std::env;

/// Character to use for newline
///
/// # System Defaults
///
/// * `unix` - LF '\n'
/// * `windows` - CRLF '\r\n'
///
/// Includes Mac OS CR, but SystemDefault will use LF on Mac OS by default
#[derive(Clone, Debug)]
pub enum LineEnding {
    SystemDefault,
    Cr,
    Lf,
    CrLf,
}

// rustc flags LineEnding::from_str as unused,
// even though it is used by clap to parse line_ending arg
#[allow(dead_code)]
impl LineEnding {
    pub fn from_str(s: &str) -> Result<LineEnding, std::string::ParseError> {
        let result = match s.to_lowercase().as_str() {
            "cr" => LineEnding::Cr,
            "lf" => LineEnding::Lf,
            "crlf" => LineEnding::CrLf,
            _ => LineEnding::SystemDefault,
        };

        Ok(result)
    }

    pub fn as_str(&self) -> &str {
        match self {
            LineEnding::Cr => "\r",
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
            LineEnding::SystemDefault => self.get_default_str(),
        }
    }

    pub fn parse_str(s: &str) -> LineEnding {
        if s.contains(LineEnding::CrLf.as_str()) {
            LineEnding::CrLf
        } else if s.contains(LineEnding::Lf.as_str()) {
            LineEnding::Lf
        } else if s.contains(LineEnding::Cr.as_str()) {
            LineEnding::Cr
        } else {
            LineEnding::SystemDefault
        }
    }

    fn get_default_str(&self) -> &str {
        match env::consts::FAMILY {
            "linux" => LineEnding::Lf.as_str(),
            "windows" => LineEnding::CrLf.as_str(),
            _ => LineEnding::Lf.as_str(),
        }
    }
}
