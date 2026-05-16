/// Geometry types mirroring CoreGraphics structs.
///
/// All types are `#[repr(C)]` and ABI-compatible with the corresponding
/// `CGPoint`, `CGSize`, and `CGRect` C types.
pub use crate::ffi::{CGPoint, CGRect, CGSize};

impl CGPoint {
    /// Create a point at `(x, y)`.
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl CGSize {
    /// Create a size with the given `width` and `height`.
    #[inline]
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl CGRect {
    /// Create a rectangle with origin `(x, y)` and the given dimensions.
    #[inline]
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            origin: CGPoint { x, y },
            size: CGSize { width, height },
        }
    }
}

// ── TextRange ──────────────────────────────────────────────────────────────

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

impl From<crate::ffi::CFRange> for TextRange {
    fn from(r: crate::ffi::CFRange) -> Self {
        Self {
            location: r.location,
            length: r.length,
        }
    }
}

impl From<TextRange> for crate::ffi::CFRange {
    fn from(r: TextRange) -> Self {
        Self {
            location: r.location,
            length: r.length,
        }
    }
}

// ── TypographicBounds ──────────────────────────────────────────────────────

/// Width, ascent, descent, and leading for a line or run.
#[derive(Debug, Clone, Copy, Default)]
pub struct TypographicBounds {
    pub width: f64,
    pub ascent: f64,
    pub descent: f64,
    pub leading: f64,
}
