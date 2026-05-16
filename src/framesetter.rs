use core::ptr::null;

use crate::attributed_string::AttributedString;
use crate::error::{CoreTextError, CoreTextResult};
use crate::ffi;
use crate::frame::CTFrame;
use crate::types::{CGRect, CGSize, TextRange};

/// A retained `CTFramesetterRef` wrapper.
pub struct CTFramesetter {
    raw: ffi::CTFramesetterRef,
}

unsafe impl Send for CTFramesetter {}
unsafe impl Sync for CTFramesetter {}

impl CTFramesetter {
    /// Create a framesetter from an attributed string.
    pub fn create_with_attributed_string(attr_str: &AttributedString) -> CoreTextResult<Self> {
        let raw = unsafe { ffi::CTFramesetterCreateWithAttributedString(attr_str.as_raw()) };
        if raw.is_null() {
            Err(CoreTextError::Null(
                "CTFramesetterCreateWithAttributedString returned NULL",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    /// Suggest a frame size that fits `constraints`, returning the size and the
    /// range of the attributed string that fits within it.
    pub fn suggest_frame_size_with_constraints(&self, constraints: CGSize) -> (CGSize, TextRange) {
        let mut fit = ffi::CFRange::new(0, 0);
        let size = unsafe {
            ffi::CTFramesetterSuggestFrameSizeWithConstraints(
                self.raw,
                ffi::CFRange::new(0, 0), // whole string
                null(),                  // no extra frame attributes
                constraints,
                &mut fit,
            )
        };
        (CGSize::new(size.width, size.height), TextRange::from(fit))
    }

    /// Create a `CTFrame` that lays out the attributed string inside `rect`.
    ///
    /// Uses `CGPathCreateWithRect` internally; the identity transform is applied.
    pub fn create_frame_in_rect(
        &self,
        rect: CGRect,
        string_range: TextRange,
    ) -> CoreTextResult<CTFrame> {
        let c_rect = ffi::CGRect {
            origin: ffi::CGPoint {
                x: rect.origin.x,
                y: rect.origin.y,
            },
            size: ffi::CGSize {
                width: rect.size.width,
                height: rect.size.height,
            },
        };
        let path = unsafe { ffi::CGPathCreateWithRect(c_rect, null()) };
        if path.is_null() {
            return Err(CoreTextError::Null("CGPathCreateWithRect returned NULL"));
        }
        let raw = unsafe {
            ffi::CTFramesetterCreateFrame(self.raw, ffi::CFRange::from(string_range), path, null())
        };
        unsafe { ffi::CGPathRelease(path) };
        if raw.is_null() {
            Err(CoreTextError::Null(
                "CTFramesetterCreateFrame returned NULL",
            ))
        } else {
            Ok(CTFrame { raw })
        }
    }
}

impl Clone for CTFramesetter {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for CTFramesetter {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
        }
    }
}
