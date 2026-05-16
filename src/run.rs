use serde_json::Value;

use crate::bridge;
use crate::common::{impl_handle, json_from_owned};
use crate::types::{CGAffineTransform, CGPoint, CGRect, CGSize, TextRange, TypographicBounds};

/// Status flags returned by `CTRun::status()`.
pub mod run_status {
    pub const NO_STATUS: u32 = 0;
    pub const RIGHT_TO_LEFT: u32 = 1 << 0;
    pub const NON_MONOTONIC: u32 = 1 << 1;
    pub const HAS_NON_IDENTITY_MATRIX: u32 = 1 << 2;
}

/// An immutable `CTRun` wrapper.
pub struct CTRun {
    raw: bridge::Handle,
}

impl_handle!(CTRun);

impl CTRun {
    #[must_use]
    pub fn glyph_count(&self) -> isize {
        unsafe { bridge::ct_run_get_glyph_count(self.raw) }
    }

    pub fn attributes_json(&self) -> CoreTextResult<Value> {
        unsafe { json_from_owned(bridge::ct_run_copy_attributes_json(self.raw)) }
    }

    #[must_use]
    pub fn status(&self) -> u32 {
        unsafe { bridge::ct_run_get_status(self.raw) }
    }

    #[must_use]
    pub fn glyphs(&self) -> Vec<u16> {
        let count = self.glyph_count();
        if count <= 0 {
            return Vec::new();
        }
        let mut glyphs = vec![0_u16; usize::try_from(count).unwrap_or(0)];
        let written = unsafe { bridge::ct_run_copy_glyphs(self.raw, glyphs.as_mut_ptr(), count) };
        glyphs.truncate(usize::try_from(written).unwrap_or(0));
        glyphs
    }

    #[must_use]
    pub fn positions(&self) -> Vec<CGPoint> {
        let count = self.glyph_count();
        if count <= 0 {
            return Vec::new();
        }
        let mut positions = vec![CGPoint::default(); usize::try_from(count).unwrap_or(0)];
        let written =
            unsafe { bridge::ct_run_copy_positions(self.raw, positions.as_mut_ptr(), count) };
        positions.truncate(usize::try_from(written).unwrap_or(0));
        positions
    }

    #[must_use]
    pub fn advances(&self) -> Vec<CGSize> {
        let count = self.glyph_count();
        if count <= 0 {
            return Vec::new();
        }
        let mut advances = vec![CGSize::default(); usize::try_from(count).unwrap_or(0)];
        let written =
            unsafe { bridge::ct_run_copy_advances(self.raw, advances.as_mut_ptr(), count) };
        advances.truncate(usize::try_from(written).unwrap_or(0));
        advances
    }

    #[must_use]
    pub fn string_indices(&self) -> Vec<isize> {
        let count = self.glyph_count();
        if count <= 0 {
            return Vec::new();
        }
        let mut indices = vec![0_isize; usize::try_from(count).unwrap_or(0)];
        let written =
            unsafe { bridge::ct_run_copy_string_indices(self.raw, indices.as_mut_ptr(), count) };
        indices.truncate(usize::try_from(written).unwrap_or(0));
        indices
    }

    #[must_use]
    pub fn string_range(&self) -> TextRange {
        unsafe { bridge::ct_run_get_string_range(self.raw) }.into()
    }

    #[must_use]
    pub fn typographic_bounds(&self) -> TypographicBounds {
        let mut ascent = 0.0;
        let mut descent = 0.0;
        let mut leading = 0.0;
        let width = unsafe {
            bridge::ct_run_get_typographic_bounds(self.raw, &mut ascent, &mut descent, &mut leading)
        };
        TypographicBounds {
            width,
            ascent,
            descent,
            leading,
        }
    }

    #[must_use]
    pub fn image_bounds(&self) -> CGRect {
        unsafe { bridge::ct_run_get_image_bounds(self.raw) }
    }

    #[must_use]
    pub fn text_matrix(&self) -> CGAffineTransform {
        unsafe { bridge::ct_run_get_text_matrix(self.raw) }
    }

    #[must_use]
    pub fn base_advances_and_origins(&self) -> (Vec<CGSize>, Vec<CGPoint>) {
        let count = self.glyph_count();
        if count <= 0 {
            return (Vec::new(), Vec::new());
        }
        let len = usize::try_from(count).unwrap_or(0);
        let mut advances = vec![CGSize::default(); len];
        let mut origins = vec![CGPoint::default(); len];
        let written = unsafe {
            bridge::ct_run_copy_base_advances_and_origins(
                self.raw,
                advances.as_mut_ptr(),
                origins.as_mut_ptr(),
                count,
            )
        };
        let written = usize::try_from(written).unwrap_or(0);
        advances.truncate(written);
        origins.truncate(written);
        (advances, origins)
    }
}

use crate::error::CoreTextResult;
