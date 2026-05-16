use crate::ffi;
use crate::line::CTLine;
use crate::types::{CGPoint, TextRange};

/// A retained `CTFrameRef` wrapper.
pub struct CTFrame {
    pub(crate) raw: ffi::CTFrameRef,
}

unsafe impl Send for CTFrame {}
unsafe impl Sync for CTFrame {}

impl CTFrame {
    /// String range requested when creating this frame.
    pub fn string_range(&self) -> TextRange {
        TextRange::from(unsafe { ffi::CTFrameGetStringRange(self.raw) })
    }

    /// The subset of the string range that actually fits in the frame.
    pub fn visible_string_range(&self) -> TextRange {
        TextRange::from(unsafe { ffi::CTFrameGetVisibleStringRange(self.raw) })
    }

    /// Lines laid out within this frame.
    ///
    /// Each returned `CTLine` is retained for the lifetime of the `Vec`.
    pub fn lines(&self) -> Vec<CTLine> {
        unsafe {
            let array = ffi::CTFrameGetLines(self.raw);
            if array.is_null() {
                return Vec::new();
            }
            let count = ffi::CFArrayGetCount(array);
            (0..count)
                .filter_map(|i| {
                    let r = ffi::CFArrayGetValueAtIndex(array, i) as ffi::CTLineRef;
                    if r.is_null() {
                        return None;
                    }
                    ffi::CFRetain(r);
                    Some(CTLine { raw: r })
                })
                .collect()
        }
    }

    /// Origins (in frame coordinates) for each line.
    pub fn line_origins(&self) -> Vec<CGPoint> {
        unsafe {
            let array = ffi::CTFrameGetLines(self.raw);
            if array.is_null() {
                return Vec::new();
            }
            let count = ffi::CFArrayGetCount(array);
            if count <= 0 {
                return Vec::new();
            }
            let n = usize::try_from(count).unwrap_or(0);
            let mut origins = vec![ffi::CGPoint { x: 0.0, y: 0.0 }; n];
            ffi::CTFrameGetLineOrigins(self.raw, ffi::CFRange::new(0, count), origins.as_mut_ptr());
            origins
        }
    }
}

impl Clone for CTFrame {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for CTFrame {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
        }
    }
}
