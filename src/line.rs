use crate::attributed_string::AttributedString;
use crate::bridge;
use crate::common::{expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::run::CTRun;
use crate::types::{CGPoint, CGRect, TextRange, TypographicBounds};

/// Option flags for `CTLine::bounds_with_options`.
pub mod bounds_options {
    pub const EXCLUDE_TYPOGRAPHIC_LEADING: u64 = 1 << 0;
    pub const EXCLUDE_TYPOGRAPHIC_SHIFTS: u64 = 1 << 1;
    pub const USE_HANGING_PUNCTUATION: u64 = 1 << 2;
    pub const USE_GLYPH_PATH_BOUNDS: u64 = 1 << 3;
    pub const USE_OPTICAL_BOUNDS: u64 = 1 << 4;
    pub const INCLUDE_LANGUAGE_EXTENTS: u64 = 1 << 5;
}

/// Truncation strategy for `CTLineCreateTruncatedLine`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum LineTruncationType {
    #[default]
    Start = 0,
    End = 1,
    Middle = 2,
}

/// An immutable `CTLine` wrapper.
pub struct CTLine {
    raw: bridge::Handle,
}

impl_handle!(CTLine);

impl CTLine {
    pub fn create_with_attributed_string(
        attributed_string: &AttributedString,
    ) -> CoreTextResult<Self> {
        let raw =
            unsafe { bridge::ct_line_create_with_attributed_string(attributed_string.as_raw()) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_line_create_with_attributed_string returned NULL",
        )?))
    }

    #[must_use]
    pub fn truncated(
        &self,
        width: f64,
        truncation_type: LineTruncationType,
        truncation_token: Option<&Self>,
    ) -> Option<Self> {
        let raw = unsafe {
            bridge::ct_line_create_truncated_line(
                self.raw,
                width,
                truncation_type as u32,
                truncation_token.map_or(std::ptr::null_mut(), Self::as_raw),
            )
        };
        if raw.is_null() {
            None
        } else {
            Some(Self::from_raw(raw))
        }
    }

    #[must_use]
    pub fn justified(&self, justification_factor: f64, justification_width: f64) -> Option<Self> {
        let raw = unsafe {
            bridge::ct_line_create_justified_line(
                self.raw,
                justification_factor,
                justification_width,
            )
        };
        if raw.is_null() {
            None
        } else {
            Some(Self::from_raw(raw))
        }
    }

    #[must_use]
    pub fn glyph_count(&self) -> isize {
        unsafe { bridge::ct_line_get_glyph_count(self.raw) }
    }

    #[must_use]
    pub fn string_range(&self) -> TextRange {
        unsafe { bridge::ct_line_get_string_range(self.raw) }.into()
    }

    #[must_use]
    pub fn pen_offset_for_flush(&self, flush_factor: f64, flush_width: f64) -> f64 {
        unsafe { bridge::ct_line_get_pen_offset_for_flush(self.raw, flush_factor, flush_width) }
    }

    #[must_use]
    pub fn typographic_bounds(&self) -> TypographicBounds {
        let mut ascent = 0.0;
        let mut descent = 0.0;
        let mut leading = 0.0;
        let width = unsafe {
            bridge::ct_line_get_typographic_bounds(
                self.raw,
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

    #[must_use]
    pub fn bounds_with_options(&self, options: u64) -> CGRect {
        unsafe { bridge::ct_line_get_bounds_with_options(self.raw, options) }
    }

    #[must_use]
    pub fn trailing_whitespace_width(&self) -> f64 {
        unsafe { bridge::ct_line_get_trailing_whitespace_width(self.raw) }
    }

    #[must_use]
    pub fn image_bounds(&self) -> CGRect {
        unsafe { bridge::ct_line_get_image_bounds(self.raw) }
    }

    #[must_use]
    pub fn string_index_for_position(&self, position: CGPoint) -> isize {
        unsafe { bridge::ct_line_get_string_index_for_position(self.raw, position) }
    }

    #[must_use]
    pub fn offset_for_string_index(&self, char_index: isize) -> (f64, f64) {
        let mut secondary = 0.0;
        let primary = unsafe {
            bridge::ct_line_get_offset_for_string_index(self.raw, char_index, &mut secondary)
        };
        (primary, secondary)
    }

    #[must_use]
    pub fn runs(&self) -> Vec<CTRun> {
        let count = unsafe { bridge::ct_line_get_run_count(self.raw) };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe { bridge::ct_line_copy_runs(self.raw, handles.as_mut_ptr(), count) };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(CTRun::from_raw).collect()
    }
}

#[must_use]
pub fn line_type_id() -> u64 {
    unsafe { bridge::ct_line_get_type_id() }
}
