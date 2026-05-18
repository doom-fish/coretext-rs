/// Core Graphics geometry types re-exported from `apple-cf`.
pub use apple_cf::cg::{CGAffineTransform, CGPoint, CGRect, CGSize};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CFRange {
    pub location: isize,
    pub length: isize,
}

impl CFRange {
    #[inline]
    pub const fn new(location: isize, length: isize) -> Self {
        Self { location, length }
    }
}

/// A character range — location and length within a string.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TextRange {
    pub location: isize,
    pub length: isize,
}

impl TextRange {
    #[inline]
    pub const fn new(location: isize, length: isize) -> Self {
        Self { location, length }
    }
}

impl From<CFRange> for TextRange {
    fn from(value: CFRange) -> Self {
        Self::new(value.location, value.length)
    }
}

impl From<TextRange> for CFRange {
    fn from(value: TextRange) -> Self {
        Self::new(value.location, value.length)
    }
}

/// Width, ascent, descent, and leading for a line or run.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TypographicBounds {
    pub width: f64,
    pub ascent: f64,
    pub descent: f64,
    pub leading: f64,
}
