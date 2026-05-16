use std::fmt;

/// Errors returned by coretext-rs wrappers.
#[derive(Debug)]
pub enum CoreTextError {
    /// A CoreFoundation or CoreText API returned NULL unexpectedly.
    Null(&'static str),
    /// Converting a `CFStringRef` to a Rust `String` failed.
    StringConversion,
    /// The provided Rust string contains an interior NUL byte.
    NulByte,
}

impl fmt::Display for CoreTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null(msg) => write!(f, "null reference: {msg}"),
            Self::StringConversion => write!(f, "CFString conversion failed"),
            Self::NulByte => write!(f, "string contains an interior NUL byte"),
        }
    }
}

impl std::error::Error for CoreTextError {}

pub type CoreTextResult<T> = Result<T, CoreTextError>;
