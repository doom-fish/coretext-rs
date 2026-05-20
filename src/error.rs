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

/// Result type returned by CoreText wrapper calls.
pub type CoreTextResult<T> = Result<T, CoreTextError>;

#[cfg(test)]
mod tests {
    use super::CoreTextError;

    #[test]
    fn display_formats_each_error_variant() {
        assert_eq!(CoreTextError::Null("font").to_string(), "null reference: font");
        assert_eq!(CoreTextError::Bridge("failed".to_string()).to_string(), "bridge error: failed");
        assert_eq!(CoreTextError::StringConversion.to_string(), "C string conversion failed");
        assert_eq!(CoreTextError::NulByte.to_string(), "string contains an interior NUL byte");
        assert_eq!(CoreTextError::Json("oops".to_string()).to_string(), "json decode failed: oops");
    }

    #[test]
    fn serde_json_errors_convert_into_json_variants() {
        let parse_error = serde_json::from_str::<serde_json::Value>("{")
            .expect_err("invalid json should fail");
        let converted: CoreTextError = parse_error.into();
        let as_std_error: &dyn std::error::Error = &converted;

        match &converted {
            CoreTextError::Json(message) => assert!(!message.is_empty()),
            other => panic!("expected Json error, got {other:?}"),
        }
        assert!(as_std_error.source().is_none());
    }
}
