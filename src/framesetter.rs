use crate::attributed_string::AttributedString;
use crate::bridge;
use crate::common::{expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::frame::CTFrame;
use crate::types::{CFRange, CGRect, CGSize, TextRange};
use crate::typesetter::CTTypesetter;

/// An immutable `CTFramesetter` wrapper.
pub struct CTFramesetter {
    raw: bridge::Handle,
    string_length: usize,
}

impl_handle!(CTFramesetter { string_length });

impl CTFramesetter {
    /// Wraps `CTFramesetterCreateWithAttributedString`.
    pub fn create_with_attributed_string(
        attributed_string: &AttributedString,
    ) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_framesetter_create_with_attributed_string(attributed_string.as_raw())
        };
        Ok(Self {
            raw: expect_handle(
                raw,
                "ct_framesetter_create_with_attributed_string returned NULL",
            )?,
            string_length: attributed_string.utf16_len(),
        })
    }

    /// Wraps `CTFramesetterCreateWithTypesetter`.
    pub fn create_with_typesetter(typesetter: &CTTypesetter) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_framesetter_create_with_typesetter(typesetter.as_raw()) };
        Ok(Self {
            raw: expect_handle(raw, "ct_framesetter_create_with_typesetter returned NULL")?,
            string_length: typesetter.string_length(),
        })
    }

    /// Wraps `CTFramesetterGetTypesetter`.
    pub fn typesetter(&self) -> CoreTextResult<CTTypesetter> {
        let raw = unsafe { bridge::ct_framesetter_copy_typesetter(self.raw) };
        Ok(CTTypesetter::from_raw_with_length(
            expect_handle(raw, "ct_framesetter_copy_typesetter returned NULL")?,
            self.string_length,
        ))
    }

    /// Wraps `CTFramesetterSuggestFrameSizeWithConstraints`.
    #[must_use]
    pub fn suggest_frame_size_with_constraints(&self, constraints: CGSize) -> (CGSize, TextRange) {
        self.suggest_frame_size(CFRange::from(TextRange::new(0, 0)), constraints)
    }

    /// Wraps `CTFramesetterSuggestFrameSizeWithConstraints`.
    pub fn suggest_frame_size_for_range(
        &self,
        range: TextRange,
        constraints: CGSize,
    ) -> CoreTextResult<(CGSize, TextRange)> {
        let range = range.checked_cf_range(self.string_length)?;
        Ok(self.suggest_frame_size(range, constraints))
    }

    fn suggest_frame_size(&self, range: CFRange, constraints: CGSize) -> (CGSize, TextRange) {
        let mut fit = range;
        let size = unsafe {
            bridge::ct_framesetter_suggest_frame_size(self.raw, range, constraints, &raw mut fit)
        };
        (size, fit.into())
    }

    /// Wraps `CTFramesetterCreateFrame`.
    pub fn create_frame_in_rect(
        &self,
        rect: CGRect,
        string_range: TextRange,
    ) -> CoreTextResult<CTFrame> {
        let string_range = string_range.checked_cf_range(self.string_length)?;
        let raw =
            unsafe { bridge::ct_framesetter_create_frame_in_rect(self.raw, string_range, rect) };
        Ok(CTFrame::from_raw(expect_handle(
            raw,
            "ct_framesetter_create_frame_in_rect returned NULL",
        )?))
    }
}

/// Wraps `CTFramesetterGetTypeID`.
#[must_use]
pub fn framesetter_type_id() -> u64 {
    unsafe { bridge::ct_framesetter_get_type_id() }
}
