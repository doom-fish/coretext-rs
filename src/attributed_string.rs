use core::ptr::addr_of;

use crate::cf::OwnedCFString;
use crate::error::{CoreTextError, CoreTextResult};
use crate::ffi;
use crate::font::CTFont;
use crate::paragraph::ParagraphStyle;

/// An owned `CFAttributedStringRef` wrapping a `CFString` with CoreText attributes.
pub struct AttributedString {
    pub(crate) raw: ffi::CFAttributedStringRef,
}

unsafe impl Send for AttributedString {}
unsafe impl Sync for AttributedString {}

impl AttributedString {
    /// Build an attributed string from a Rust `&str` with a `CTFont` and an
    /// optional `ParagraphStyle`.
    ///
    /// Attributes applied:
    /// - `kCTFontAttributeName`
    /// - `kCTParagraphStyleAttributeName` (when `paragraph_style` is `Some`)
    pub fn new(
        text: &str,
        font: &CTFont,
        paragraph_style: Option<&ParagraphStyle>,
    ) -> CoreTextResult<Self> {
        let text_cf = OwnedCFString::from_str(text)?;

        let raw = unsafe {
            // Build keys and values arrays on the stack.
            let (dict, count) = if let Some(ps) = paragraph_style {
                let keys: [ffi::CFTypeRef; 2] = [
                    ffi::kCTFontAttributeName,
                    ffi::kCTParagraphStyleAttributeName,
                ];
                let values: [ffi::CFTypeRef; 2] = [font.as_raw(), ps.as_raw()];
                let d = ffi::CFDictionaryCreate(
                    ffi::kCFAllocatorDefault,
                    keys.as_ptr(),
                    values.as_ptr(),
                    2,
                    addr_of!(ffi::kCFTypeDictionaryKeyCallBacks),
                    addr_of!(ffi::kCFTypeDictionaryValueCallBacks),
                );
                (d, 2_isize)
            } else {
                let keys: [ffi::CFTypeRef; 1] = [ffi::kCTFontAttributeName];
                let values: [ffi::CFTypeRef; 1] = [font.as_raw()];
                let d = ffi::CFDictionaryCreate(
                    ffi::kCFAllocatorDefault,
                    keys.as_ptr(),
                    values.as_ptr(),
                    1,
                    addr_of!(ffi::kCFTypeDictionaryKeyCallBacks),
                    addr_of!(ffi::kCFTypeDictionaryValueCallBacks),
                );
                (d, 1_isize)
            };
            let _ = count; // used for documentation only
            if dict.is_null() {
                return Err(CoreTextError::Null("CFDictionaryCreate returned NULL"));
            }
            let attr =
                ffi::CFAttributedStringCreate(ffi::kCFAllocatorDefault, text_cf.as_raw(), dict);
            ffi::CFRelease(dict);
            attr
        };

        if raw.is_null() {
            Err(CoreTextError::Null(
                "CFAttributedStringCreate returned NULL",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    /// The raw `CFAttributedStringRef`. The caller must not release it.
    #[inline]
    pub fn as_raw(&self) -> ffi::CFAttributedStringRef {
        self.raw
    }
}

impl Clone for AttributedString {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for AttributedString {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
        }
    }
}
