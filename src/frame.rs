use crate::bridge;
use crate::common::impl_handle;
use crate::line::CTLine;
use crate::types::{CGPoint, CGRect, TextRange};

/// An immutable `CTFrame` wrapper.
pub struct CTFrame {
    raw: bridge::Handle,
}

impl_handle!(CTFrame);

impl CTFrame {
    #[must_use]
    pub fn string_range(&self) -> TextRange {
        unsafe { bridge::ct_frame_get_string_range(self.raw) }.into()
    }

    #[must_use]
    pub fn visible_string_range(&self) -> TextRange {
        unsafe { bridge::ct_frame_get_visible_string_range(self.raw) }.into()
    }

    #[must_use]
    pub fn path_bounding_box(&self) -> CGRect {
        unsafe { bridge::ct_frame_copy_path_bounding_box(self.raw) }
    }

    #[must_use]
    pub fn has_frame_attributes(&self) -> bool {
        unsafe { bridge::ct_frame_has_frame_attributes(self.raw) }
    }

    #[must_use]
    pub fn lines(&self) -> Vec<CTLine> {
        let count = unsafe { bridge::ct_frame_get_line_count(self.raw) };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe { bridge::ct_frame_copy_lines(self.raw, handles.as_mut_ptr(), count) };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(CTLine::from_raw).collect()
    }

    #[must_use]
    pub fn line_origins(&self) -> Vec<CGPoint> {
        let count = unsafe { bridge::ct_frame_get_line_count(self.raw) };
        if count <= 0 {
            return Vec::new();
        }
        let mut origins = vec![CGPoint::default(); usize::try_from(count).unwrap_or(0)];
        let written =
            unsafe { bridge::ct_frame_copy_line_origins(self.raw, origins.as_mut_ptr(), count) };
        origins.truncate(usize::try_from(written).unwrap_or(0));
        origins
    }

    pub fn frame_attributes_json(&self) -> crate::error::CoreTextResult<serde_json::Value> {
        unsafe { crate::common::json_from_owned(bridge::ct_frame_copy_frame_attributes_json(self.raw)) }
    }
}

#[must_use]
pub fn frame_type_id() -> u64 {
    unsafe { bridge::ct_frame_get_type_id() }
}
