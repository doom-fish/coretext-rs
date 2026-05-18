/// Core Graphics geometry types re-exported from `apple-cf`.
pub use apple_cf::cg::{CGAffineTransform, CGPoint, CGRect, CGSize};
/// Re-exports `CFRange` for CoreText range parameters.
pub use apple_cf::raw::CFRange;

/// A character range — location and length within a string.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TextRange {
    /// Matches the `location` field of `CFRange`.
    pub location: isize,
    /// Matches the `length` field of `CFRange`.
    pub length: isize,
}

impl TextRange {
    /// Creates a `CFRange`-style span for CoreText string APIs.
    #[inline]
    pub const fn new(location: isize, length: isize) -> Self {
        Self { location, length }
    }
}

impl From<CFRange> for TextRange {
    fn from(value: CFRange) -> Self {
        Self::new(
            isize::try_from(value.location)
                .expect("CFRange::location must fit in isize on supported targets"),
            isize::try_from(value.length)
                .expect("CFRange::length must fit in isize on supported targets"),
        )
    }
}

impl From<TextRange> for CFRange {
    fn from(value: TextRange) -> Self {
        Self {
            location: i64::try_from(value.location)
                .expect("TextRange::location must fit in CFIndex on supported targets"),
            length: i64::try_from(value.length)
                .expect("TextRange::length must fit in CFIndex on supported targets"),
        }
    }
}

/// Width, ascent, descent, and leading for a line or run.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TypographicBounds {
    /// Wraps the `width` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub width: f64,
    /// Wraps the `ascent` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub ascent: f64,
    /// Wraps the `descent` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub descent: f64,
    /// Wraps the `leading` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub leading: f64,
}
