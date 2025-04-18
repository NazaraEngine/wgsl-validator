use std::fmt;
use naga::{WithSpan, front::wgsl::ParseError, valid::ValidationError};

#[derive(Debug)]
pub enum WgslError {
    ValidationErr {
        src: String,
        error: WithSpan<ValidationError>,
        emitted: String,
    },
    ParserErr {
        error: String,
        line: usize,
        pos: usize,
    },
}

impl WgslError {
    pub fn from_parse_err(err: ParseError, src: &str) -> Self {
        let error = err.emit_to_string(src);
        let loc = err.location(src);
        if let Some(loc) = loc {
            Self::ParserErr {
                error,
                line: loc.line_number as usize,
                pos: loc.line_position as usize,
            }
        } else {
            Self::ParserErr {
                error,
                line: 0,
                pos: 0,
            }
        }
    }
}

impl fmt::Display for WgslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WgslError::ParserErr { error, .. } => write!(f, "{}", error),
            WgslError::ValidationErr { emitted, .. } => write!(f, "{}", emitted),
        }
    }
}
