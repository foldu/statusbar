use crate::parser::Rule;

#[derive(Debug)]
pub enum Error {
    KeyNotInMap(String),
    Parse(pest::error::Error<Rule>),
    InvalidKey { key: String, allowed: Vec<String> },
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Error::Parse(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::KeyNotInMap(ref key) => write!(formatter, "Key {} not in map", key),
            // FIXME: when source is stable failure will probably update to use it, so the
            // underlying error will probably be printed out instead of appending it to the parent
            // error
            Error::Parse(e) => write!(formatter, "Error while parsing format string: {}", e),
            Error::InvalidKey { key, allowed } => write!(
                formatter,
                "Invalid key: {} allowed keys: {}",
                key,
                allowed.join("|")
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(e) => Some(e),
            Error::KeyNotInMap(_) | Error::InvalidKey { .. } => None,
        }
    }
}
