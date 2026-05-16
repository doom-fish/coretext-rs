use core::ffi::c_void;

use crate::error::{CoreTextError, CoreTextResult};
use crate::ffi;

/// Text alignment options mirroring `CTTextAlignment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlignment {
    Left,
    Right,
    Center,
    Justified,
    #[default]
    Natural,
}

impl TextAlignment {
    pub(crate) fn to_raw(self) -> ffi::CTTextAlignment {
        match self {
            Self::Left => ffi::kCTTextAlignmentLeft,
            Self::Right => ffi::kCTTextAlignmentRight,
            Self::Center => ffi::kCTTextAlignmentCenter,
            Self::Justified => ffi::kCTTextAlignmentJustified,
            Self::Natural => ffi::kCTTextAlignmentNatural,
        }
    }

    pub(crate) fn from_raw(raw: ffi::CTTextAlignment) -> Self {
        match raw {
            ffi::kCTTextAlignmentLeft => Self::Left,
            ffi::kCTTextAlignmentRight => Self::Right,
            ffi::kCTTextAlignmentCenter => Self::Center,
            ffi::kCTTextAlignmentJustified => Self::Justified,
            _ => Self::Natural,
        }
    }
}

/// A retained `CTParagraphStyleRef` wrapper.
pub struct ParagraphStyle {
    pub(crate) raw: ffi::CTParagraphStyleRef,
}

unsafe impl Send for ParagraphStyle {}
unsafe impl Sync for ParagraphStyle {}

impl ParagraphStyle {
    /// Create a paragraph style with the given text alignment.
    pub fn with_alignment(alignment: TextAlignment) -> CoreTextResult<Self> {
        let raw_align: ffi::CTTextAlignment = alignment.to_raw();
        let setting = ffi::CTParagraphStyleSetting {
            spec: ffi::kCTParagraphStyleSpecifierAlignment,
            valueSize: core::mem::size_of::<ffi::CTTextAlignment>(),
            value: std::ptr::addr_of!(raw_align).cast::<c_void>(),
        };
        let raw = unsafe { ffi::CTParagraphStyleCreate(&setting, 1) };
        if raw.is_null() {
            Err(CoreTextError::Null("CTParagraphStyleCreate returned NULL"))
        } else {
            Ok(Self { raw })
        }
    }

    /// Read the alignment back from the style.
    pub fn alignment(&self) -> TextAlignment {
        let mut raw: ffi::CTTextAlignment = ffi::kCTTextAlignmentNatural;
        let ok = unsafe {
            ffi::CTParagraphStyleGetValueForSpecifier(
                self.raw,
                ffi::kCTParagraphStyleSpecifierAlignment,
                core::mem::size_of::<ffi::CTTextAlignment>(),
                std::ptr::addr_of_mut!(raw).cast::<c_void>(),
            )
        };
        if ok != 0 {
            TextAlignment::from_raw(raw)
        } else {
            TextAlignment::Natural
        }
    }

    /// The raw `CTParagraphStyleRef`. The caller must not release it.
    #[inline]
    pub fn as_raw(&self) -> ffi::CTParagraphStyleRef {
        self.raw
    }
}

impl Clone for ParagraphStyle {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for ParagraphStyle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
        }
    }
}
