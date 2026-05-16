use crate::bridge;
use crate::common::{cstring, expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::font::CTFont;
use crate::paragraph::ParagraphStyle;

/// An immutable attributed string used as input for CoreText layout objects.
pub struct AttributedString {
    raw: bridge::Handle,
}

impl_handle!(AttributedString);

impl AttributedString {
    pub fn new(
        text: &str,
        font: &CTFont,
        paragraph_style: Option<&ParagraphStyle>,
    ) -> CoreTextResult<Self> {
        let text = cstring(text)?;
        let raw = unsafe {
            bridge::ct_attributed_string_create(
                text.as_ptr(),
                font.as_raw(),
                paragraph_style.map_or(std::ptr::null_mut(), ParagraphStyle::as_raw),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_attributed_string_create returned NULL",
        )?))
    }
}
