use crate::attributed_string::AttributedString;
use crate::error::{CoreTextError, CoreTextResult};
use crate::ffi;
use crate::run::CTRun;
use crate::types::{CGRect, TextRange, TypographicBounds};

/// Option flags for `CTLine::bounds_with_options`.
pub mod bounds_options {
    use crate::ffi;
    pub const EXCLUDE_TYPOGRAPHIC_LEADING: u64 = ffi::kCTLineBoundsExcludeTypographicLeading;
    pub const EXCLUDE_TYPOGRAPHIC_SHIFTS: u64 = ffi::kCTLineBoundsExcludeTypographicShifts;
    pub const USE_HANGING_PUNCTUATION: u64 = ffi::kCTLineBoundsUseHangingPunctuation;
    pub const USE_GLYPH_PATH_BOUNDS: u64 = ffi::kCTLineBoundsUseGlyphPathBounds;
    pub const USE_OPTICAL_BOUNDS: u64 = ffi::kCTLineBoundsUseOpticalBounds;
    pub const INCLUDE_LANGUAGE_EXTENTS: u64 = ffi::kCTLineBoundsIncludeLanguageExtents;
}

/// A retained `CTLineRef` wrapper.
pub struct CTLine {
    pub(crate) raw: ffi::CTLineRef,
}

unsafe impl Send for CTLine {}
unsafe impl Sync for CTLine {}

impl CTLine {
    /// Create a `CTLine` from an attributed string.
    pub fn create_with_attributed_string(attr_str: &AttributedString) -> CoreTextResult<Self> {
        let raw = unsafe { ffi::CTLineCreateWithAttributedString(attr_str.as_raw()) };
        if raw.is_null() {
            Err(CoreTextError::Null(
                "CTLineCreateWithAttributedString returned NULL",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    /// Total number of glyphs in the line.
    pub fn glyph_count(&self) -> isize {
        unsafe { ffi::CTLineGetGlyphCount(self.raw) }
    }

    /// String range covered by this line.
    pub fn string_range(&self) -> TextRange {
        TextRange::from(unsafe { ffi::CTLineGetStringRange(self.raw) })
    }

    /// Typographic bounds of the line.
    pub fn typographic_bounds(&self) -> TypographicBounds {
        let mut ascent: f64 = 0.0;
        let mut descent: f64 = 0.0;
        let mut leading: f64 = 0.0;
        let width = unsafe {
            ffi::CTLineGetTypographicBounds(self.raw, &mut ascent, &mut descent, &mut leading)
        };
        TypographicBounds {
            width,
            ascent,
            descent,
            leading,
        }
    }

    /// Bounding rectangle for the line with the given option flags.
    ///
    /// Pass `0` for the default (no options), or combine constants from
    /// [`bounds_options`].
    pub fn bounds_with_options(&self, options: u64) -> CGRect {
        let r = unsafe { ffi::CTLineGetBoundsWithOptions(self.raw, options) };
        CGRect::new(r.origin.x, r.origin.y, r.size.width, r.size.height)
    }

    /// Width of trailing whitespace characters in the line.
    pub fn trailing_whitespace_width(&self) -> f64 {
        unsafe { ffi::CTLineGetTrailingWhitespaceWidth(self.raw) }
    }

    /// Pen offset for flushing the line.
    ///
    /// `flush_factor`: 0.0 = flush left, 0.5 = centered, 1.0 = flush right.
    /// `flush_width`: width of the line area.
    pub fn pen_offset_for_flush(&self, flush_factor: f64, flush_width: f64) -> f64 {
        unsafe { ffi::CTLineGetPenOffsetForFlush(self.raw, flush_factor, flush_width) }
    }

    /// Glyph runs composing this line.
    ///
    /// Each returned `CTRun` is retained for the lifetime of the `Vec`.
    pub fn runs(&self) -> Vec<CTRun> {
        unsafe {
            let array = ffi::CTLineGetGlyphRuns(self.raw);
            if array.is_null() {
                return Vec::new();
            }
            let count = ffi::CFArrayGetCount(array);
            (0..count)
                .filter_map(|i| {
                    let r = ffi::CFArrayGetValueAtIndex(array, i).cast::<core::ffi::c_void>()
                        as ffi::CTRunRef;
                    if r.is_null() {
                        return None;
                    }
                    ffi::CFRetain(r);
                    Some(CTRun { raw: r })
                })
                .collect()
        }
    }
}

impl Clone for CTLine {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for CTLine {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
        }
    }
}
