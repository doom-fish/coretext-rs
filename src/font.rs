use core::ptr::null;

use crate::cf::{self, OwnedCFString};
use crate::error::{CoreTextError, CoreTextResult};
use crate::ffi;

/// A retained `CTFontRef` wrapper.
pub struct CTFont {
    pub(crate) raw: ffi::CTFontRef,
}

unsafe impl Send for CTFont {}
unsafe impl Sync for CTFont {}

impl CTFont {
    /// Create a font by PostScript or family name and point size.
    ///
    /// Uses the identity transform (no matrix).
    pub fn new(name: &str, size: f64) -> CoreTextResult<Self> {
        let name_cf = OwnedCFString::from_str(name)?;
        let raw = unsafe { ffi::CTFontCreateWithName(name_cf.as_raw(), size, null()) };
        if raw.is_null() {
            Err(CoreTextError::Null("CTFontCreateWithName returned NULL"))
        } else {
            Ok(Self { raw })
        }
    }

    /// The raw `CTFontRef`. The caller must not release it.
    #[inline]
    pub fn as_raw(&self) -> ffi::CTFontRef {
        self.raw
    }

    /// Point size of the font.
    pub fn size(&self) -> f64 {
        unsafe { ffi::CTFontGetSize(self.raw) }
    }

    /// PostScript name (e.g. `"Helvetica-Bold"`).
    pub fn postscript_name(&self) -> CoreTextResult<String> {
        cf::cfstring_into_string(unsafe { ffi::CTFontCopyPostScriptName(self.raw) })
    }

    /// Family name (e.g. `"Helvetica"`).
    pub fn family_name(&self) -> CoreTextResult<String> {
        cf::cfstring_into_string(unsafe { ffi::CTFontCopyFamilyName(self.raw) })
    }

    /// Full display name (e.g. `"Helvetica"`).
    pub fn full_name(&self) -> CoreTextResult<String> {
        cf::cfstring_into_string(unsafe { ffi::CTFontCopyFullName(self.raw) })
    }

    /// Maximum distance above the baseline for ascenders.
    pub fn ascent(&self) -> f64 {
        unsafe { ffi::CTFontGetAscent(self.raw) }
    }

    /// Maximum distance below the baseline for descenders (negative value).
    pub fn descent(&self) -> f64 {
        unsafe { ffi::CTFontGetDescent(self.raw) }
    }

    /// Leading — space between lines of text.
    pub fn leading(&self) -> f64 {
        unsafe { ffi::CTFontGetLeading(self.raw) }
    }

    /// Total number of glyphs in the font.
    pub fn glyph_count(&self) -> isize {
        unsafe { ffi::CTFontGetGlyphCount(self.raw) }
    }
}

impl Clone for CTFont {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for CTFont {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
        }
    }
}
