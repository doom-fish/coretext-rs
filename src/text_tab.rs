use crate::bridge;
use crate::common::{expect_handle, impl_handle, impl_thread_safe};
use crate::error::CoreTextResult;
use crate::paragraph::TextAlignment;

/// An immutable `CTTextTab` wrapper.
#[derive(Debug)]
pub struct TextTab {
    raw: bridge::Handle,
}

impl_handle!(TextTab);
impl_thread_safe!(TextTab);

impl TextTab {
    /// Wraps `CTTextTabCreate`.
    pub fn new(alignment: TextAlignment, location: f64) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_text_tab_create(alignment as u8, location) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_text_tab_create returned NULL",
        )?))
    }

    /// Wraps `CTTextTabGetAlignment`.
    #[must_use]
    pub fn alignment(&self) -> TextAlignment {
        TextAlignment::from_raw(unsafe { bridge::ct_text_tab_get_alignment(self.raw) })
    }

    /// Wraps `CTTextTabGetLocation`.
    #[must_use]
    pub fn location(&self) -> f64 {
        unsafe { bridge::ct_text_tab_get_location(self.raw) }
    }

    /// Wraps `CTTextTabGetOptions`.
    pub fn options(&self) -> crate::error::CoreTextResult<crate::AttributeValue> {
        unsafe { crate::abi::from_owned(bridge::ct_text_tab_get_options(self.raw)) }
    }
}

/// Wraps `CTTextTabGetTypeID`.
#[must_use]
pub fn text_tab_type_id() -> u64 {
    unsafe { bridge::ct_text_tab_get_type_id() }
}
