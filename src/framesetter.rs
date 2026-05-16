use crate::attributed_string::AttributedString;
use crate::bridge;
use crate::common::{expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::frame::CTFrame;
use crate::types::{CGRect, CGSize, TextRange};
use crate::typesetter::CTTypesetter;

/// An immutable `CTFramesetter` wrapper.
pub struct CTFramesetter {
    raw: bridge::Handle,
}

impl_handle!(CTFramesetter);

impl CTFramesetter {
    pub fn create_with_attributed_string(
        attributed_string: &AttributedString,
    ) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_framesetter_create_with_attributed_string(attributed_string.as_raw())
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_framesetter_create_with_attributed_string returned NULL",
        )?))
    }

    pub fn create_with_typesetter(typesetter: &CTTypesetter) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_framesetter_create_with_typesetter(typesetter.as_raw()) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_framesetter_create_with_typesetter returned NULL",
        )?))
    }

    pub fn typesetter(&self) -> CoreTextResult<CTTypesetter> {
        let raw = unsafe { bridge::ct_framesetter_copy_typesetter(self.raw) };
        Ok(CTTypesetter::from_raw(expect_handle(
            raw,
            "ct_framesetter_copy_typesetter returned NULL",
        )?))
    }

    #[must_use]
    pub fn suggest_frame_size_with_constraints(&self, constraints: CGSize) -> (CGSize, TextRange) {
        self.suggest_frame_size_for_range(TextRange::new(0, 0), constraints)
    }

    #[must_use]
    pub fn suggest_frame_size_for_range(
        &self,
        range: TextRange,
        constraints: CGSize,
    ) -> (CGSize, TextRange) {
        let mut fit = range.into();
        let size = unsafe {
            bridge::ct_framesetter_suggest_frame_size(self.raw, range.into(), constraints, &mut fit)
        };
        (size, fit.into())
    }

    pub fn create_frame_in_rect(
        &self,
        rect: CGRect,
        string_range: TextRange,
    ) -> CoreTextResult<CTFrame> {
        let raw = unsafe {
            bridge::ct_framesetter_create_frame_in_rect(self.raw, string_range.into(), rect)
        };
        Ok(CTFrame::from_raw(expect_handle(
            raw,
            "ct_framesetter_create_frame_in_rect returned NULL",
        )?))
    }
}
