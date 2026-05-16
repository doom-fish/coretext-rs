use std::fmt;

/// Errors returned by coretext-rs wrappers.
#[derive(Debug)]
pub enum CoreTextError {
    /// A Core Text or bridge API returned a null reference unexpectedly.
    Null(&'static str),
    /// The Swift bridge reported an operational error.
    Bridge(String),
    /// Converting an owned C string into Rust failed.
    StringConversion,
    /// The provided Rust string contains an interior NUL byte.
    NulByte,
    /// JSON returned from the Swift bridge could not be parsed.
    Json(String),
}

impl fmt::Display for CoreTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null(message) => write!(f, "null reference: {message}"),
            Self::Bridge(message) => write!(f, "bridge error: {message}"),
            Self::StringConversion => write!(f, "C string conversion failed"),
            Self::NulByte => write!(f, "string contains an interior NUL byte"),
            Self::Json(message) => write!(f, "json decode failed: {message}"),
        }
    }
}

impl std::error::Error for CoreTextError {}

impl From<serde_json::Error> for CoreTextError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value.to_string())
    }
}

pub type CoreTextResult<T> = Result<T, CoreTextError>;
