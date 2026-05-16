use crate::bridge;
use crate::common::{expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::paragraph::TextAlignment;

/// An immutable `CTTextTab` wrapper.
#[derive(Debug)]
pub struct TextTab {
    raw: bridge::Handle,
}

impl_handle!(TextTab);

impl TextTab {
    pub fn new(alignment: TextAlignment, location: f64) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_text_tab_create(alignment as u8, location) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_text_tab_create returned NULL",
        )?))
    }

    #[must_use]
    pub fn alignment(&self) -> TextAlignment {
        TextAlignment::from_raw(unsafe { bridge::ct_text_tab_get_alignment(self.raw) })
    }

    #[must_use]
    pub fn location(&self) -> f64 {
        unsafe { bridge::ct_text_tab_get_location(self.raw) }
    }
}
