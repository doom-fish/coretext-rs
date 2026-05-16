use crate::ffi;
use crate::types::{CGPoint, CGSize, TextRange, TypographicBounds};

/// Status flags returned by `CTRun::status()`.
pub mod run_status {
    use crate::ffi;
    pub const NO_STATUS: u32 = ffi::kCTRunStatusNoStatus;
    pub const RIGHT_TO_LEFT: u32 = ffi::kCTRunStatusRightToLeft;
    pub const NON_MONOTONIC: u32 = ffi::kCTRunStatusNonMonotonic;
    pub const HAS_NON_IDENTITY_MATRIX: u32 = ffi::kCTRunStatusHasNonIdentityMatrix;
}

/// A retained `CTRunRef` wrapper.
pub struct CTRun {
    pub(crate) raw: ffi::CTRunRef,
}

unsafe impl Send for CTRun {}
unsafe impl Sync for CTRun {}

impl CTRun {
    /// Number of glyphs in this run.
    pub fn glyph_count(&self) -> isize {
        unsafe { ffi::CTRunGetGlyphCount(self.raw) }
    }

    /// Status flags for this run (see [`run_status`] constants).
    pub fn status(&self) -> u32 {
        unsafe { ffi::CTRunGetStatus(self.raw) }
    }

    /// Glyph IDs for every glyph in the run.
    pub fn glyphs(&self) -> Vec<u16> {
        let count = usize::try_from(unsafe { ffi::CTRunGetGlyphCount(self.raw) }).unwrap_or(0);
        if count == 0 {
            return Vec::new();
        }
        let ptr = unsafe { ffi::CTRunGetGlyphsPtr(self.raw) };
        if !ptr.is_null() {
            return unsafe { core::slice::from_raw_parts(ptr, count).to_vec() };
        }
        let mut buf = vec![0_u16; count];
        unsafe {
            ffi::CTRunGetGlyphs(
                self.raw,
                ffi::CFRange::new(0, count as isize),
                buf.as_mut_ptr(),
            );
        }
        buf
    }

    /// Glyph origin positions (in text space).
    pub fn positions(&self) -> Vec<CGPoint> {
        let count = usize::try_from(unsafe { ffi::CTRunGetGlyphCount(self.raw) }).unwrap_or(0);
        if count == 0 {
            return Vec::new();
        }
        let ptr = unsafe { ffi::CTRunGetPositionsPtr(self.raw) };
        if !ptr.is_null() {
            return unsafe { core::slice::from_raw_parts(ptr, count).to_vec() };
        }
        let mut buf = vec![ffi::CGPoint { x: 0.0, y: 0.0 }; count];
        unsafe {
            ffi::CTRunGetPositions(
                self.raw,
                ffi::CFRange::new(0, count as isize),
                buf.as_mut_ptr(),
            );
        }
        buf
    }

    /// Advance widths for each glyph.
    pub fn advances(&self) -> Vec<CGSize> {
        let count = usize::try_from(unsafe { ffi::CTRunGetGlyphCount(self.raw) }).unwrap_or(0);
        if count == 0 {
            return Vec::new();
        }
        let ptr = unsafe { ffi::CTRunGetAdvancesPtr(self.raw) };
        if !ptr.is_null() {
            return unsafe { core::slice::from_raw_parts(ptr, count).to_vec() };
        }
        let mut buf = vec![
            ffi::CGSize {
                width: 0.0,
                height: 0.0
            };
            count
        ];
        unsafe {
            ffi::CTRunGetAdvances(
                self.raw,
                ffi::CFRange::new(0, count as isize),
                buf.as_mut_ptr(),
            );
        }
        buf
    }

    /// String character indices corresponding to each glyph.
    pub fn string_indices(&self) -> Vec<isize> {
        let count = usize::try_from(unsafe { ffi::CTRunGetGlyphCount(self.raw) }).unwrap_or(0);
        if count == 0 {
            return Vec::new();
        }
        let ptr = unsafe { ffi::CTRunGetStringIndicesPtr(self.raw) };
        if !ptr.is_null() {
            return unsafe { core::slice::from_raw_parts(ptr, count).to_vec() };
        }
        let mut buf = vec![0_isize; count];
        unsafe {
            ffi::CTRunGetStringIndices(
                self.raw,
                ffi::CFRange::new(0, count as isize),
                buf.as_mut_ptr(),
            );
        }
        buf
    }

    /// String range covered by this run.
    pub fn string_range(&self) -> TextRange {
        TextRange::from(unsafe { ffi::CTRunGetStringRange(self.raw) })
    }

    /// Typographic bounds of the entire run.
    pub fn typographic_bounds(&self) -> TypographicBounds {
        let mut ascent: f64 = 0.0;
        let mut descent: f64 = 0.0;
        let mut leading: f64 = 0.0;
        let count = unsafe { ffi::CTRunGetGlyphCount(self.raw) };
        let width = unsafe {
            ffi::CTRunGetTypographicBounds(
                self.raw,
                ffi::CFRange::new(0, count),
                &mut ascent,
                &mut descent,
                &mut leading,
            )
        };
        TypographicBounds {
            width,
            ascent,
            descent,
            leading,
        }
    }
}

impl Clone for CTRun {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for CTRun {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
        }
    }
}
