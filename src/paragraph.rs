use std::ffi::CString;

use serde::Serialize;

use crate::bridge;
use crate::common::{expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::text_tab::TextTab;

/// Text alignment options mirroring `CTTextAlignment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TextAlignment {
    /// Selects the left case of `CTTextAlignment`.
    Left = 0,
    /// Selects the right case of `CTTextAlignment`.
    Right = 1,
    /// Selects the center case of `CTTextAlignment`.
    Center = 2,
    /// Selects the justified case of `CTTextAlignment`.
    Justified = 3,
    /// Selects the natural case of `CTTextAlignment`.
    #[default]
    Natural = 4,
}

impl TextAlignment {
    pub(crate) const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Center,
            3 => Self::Justified,
            _ => Self::Natural,
        }
    }
}

/// Line-breaking behavior for paragraph layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum LineBreakMode {
    /// Selects the word wrapping case of `CTLineBreakMode`.
    #[default]
    WordWrapping = 0,
    /// Selects the char wrapping case of `CTLineBreakMode`.
    CharWrapping = 1,
    /// Selects the clipping case of `CTLineBreakMode`.
    Clipping = 2,
    /// Selects the truncating head case of `CTLineBreakMode`.
    TruncatingHead = 3,
    /// Selects the truncating tail case of `CTLineBreakMode`.
    TruncatingTail = 4,
    /// Selects the truncating middle case of `CTLineBreakMode`.
    TruncatingMiddle = 5,
}

impl LineBreakMode {
    pub(crate) const fn from_raw(raw: u8) -> Self {
        match raw {
            1 => Self::CharWrapping,
            2 => Self::Clipping,
            3 => Self::TruncatingHead,
            4 => Self::TruncatingTail,
            5 => Self::TruncatingMiddle,
            _ => Self::WordWrapping,
        }
    }
}

/// Base writing direction for a paragraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i8)]
pub enum WritingDirection {
    /// Selects the natural case of `CTWritingDirection`.
    #[default]
    Natural = -1,
    /// Selects the left to right case of `CTWritingDirection`.
    LeftToRight = 0,
    /// Selects the right to left case of `CTWritingDirection`.
    RightToLeft = 1,
}

impl WritingDirection {
    pub(crate) const fn from_raw(raw: i8) -> Self {
        match raw {
            0 => Self::LeftToRight,
            1 => Self::RightToLeft,
            _ => Self::Natural,
        }
    }
}

/// Builder-style options for creating a paragraph style.
#[derive(Debug, Clone, Default)]
pub struct ParagraphStyleOptions {
    /// Configures the `alignment` input passed to `CTParagraphStyleCreate`.
    pub alignment: Option<TextAlignment>,
    /// Configures the `first_line_head_indent` input passed to `CTParagraphStyleCreate`.
    pub first_line_head_indent: Option<f64>,
    /// Configures the `head_indent` input passed to `CTParagraphStyleCreate`.
    pub head_indent: Option<f64>,
    /// Configures the `tail_indent` input passed to `CTParagraphStyleCreate`.
    pub tail_indent: Option<f64>,
    /// Configures the `text_tabs` input passed to `CTParagraphStyleCreate`.
    pub text_tabs: Vec<TextTab>,
    /// Configures the `default_tab_interval` input passed to `CTParagraphStyleCreate`.
    pub default_tab_interval: Option<f64>,
    /// Configures the `line_break_mode` input passed to `CTParagraphStyleCreate`.
    pub line_break_mode: Option<LineBreakMode>,
    /// Configures the `line_height_multiple` input passed to `CTParagraphStyleCreate`.
    pub line_height_multiple: Option<f64>,
    /// Configures the `maximum_line_height` input passed to `CTParagraphStyleCreate`.
    pub maximum_line_height: Option<f64>,
    /// Configures the `minimum_line_height` input passed to `CTParagraphStyleCreate`.
    pub minimum_line_height: Option<f64>,
    /// Configures the `paragraph_spacing` input passed to `CTParagraphStyleCreate`.
    pub paragraph_spacing: Option<f64>,
    /// Configures the `paragraph_spacing_before` input passed to `CTParagraphStyleCreate`.
    pub paragraph_spacing_before: Option<f64>,
    /// Configures the `base_writing_direction` input passed to `CTParagraphStyleCreate`.
    pub base_writing_direction: Option<WritingDirection>,
    /// Configures the `maximum_line_spacing` input passed to `CTParagraphStyleCreate`.
    pub maximum_line_spacing: Option<f64>,
    /// Configures the `minimum_line_spacing` input passed to `CTParagraphStyleCreate`.
    pub minimum_line_spacing: Option<f64>,
    /// Configures the `line_spacing_adjustment` input passed to `CTParagraphStyleCreate`.
    pub line_spacing_adjustment: Option<f64>,
    /// Configures the `line_bounds_options` input passed to `CTParagraphStyleCreate`.
    pub line_bounds_options: Option<u64>,
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct EncodedParagraphStyleOptions {
    alignment: Option<u8>,
    first_line_head_indent: Option<f64>,
    head_indent: Option<f64>,
    tail_indent: Option<f64>,
    default_tab_interval: Option<f64>,
    line_break_mode: Option<u8>,
    line_height_multiple: Option<f64>,
    maximum_line_height: Option<f64>,
    minimum_line_height: Option<f64>,
    paragraph_spacing: Option<f64>,
    paragraph_spacing_before: Option<f64>,
    base_writing_direction: Option<i8>,
    maximum_line_spacing: Option<f64>,
    minimum_line_spacing: Option<f64>,
    line_spacing_adjustment: Option<f64>,
    line_bounds_options: Option<u64>,
}

impl ParagraphStyleOptions {
    fn encoded(&self) -> EncodedParagraphStyleOptions {
        EncodedParagraphStyleOptions {
            alignment: self.alignment.map(|value| value as u8),
            first_line_head_indent: self.first_line_head_indent,
            head_indent: self.head_indent,
            tail_indent: self.tail_indent,
            default_tab_interval: self.default_tab_interval,
            line_break_mode: self.line_break_mode.map(|value| value as u8),
            line_height_multiple: self.line_height_multiple,
            maximum_line_height: self.maximum_line_height,
            minimum_line_height: self.minimum_line_height,
            paragraph_spacing: self.paragraph_spacing,
            paragraph_spacing_before: self.paragraph_spacing_before,
            base_writing_direction: self.base_writing_direction.map(|value| value as i8),
            maximum_line_spacing: self.maximum_line_spacing,
            minimum_line_spacing: self.minimum_line_spacing,
            line_spacing_adjustment: self.line_spacing_adjustment,
            line_bounds_options: self.line_bounds_options,
        }
    }

    fn json(&self) -> CoreTextResult<CString> {
        let json = serde_json::to_string(&self.encoded())?;
        Ok(CString::new(json).expect("serialized paragraph style json cannot contain NUL"))
    }
}

/// An immutable `CTParagraphStyle` wrapper.
pub struct ParagraphStyle {
    raw: bridge::Handle,
}

impl_handle!(ParagraphStyle);

impl ParagraphStyle {
    /// Wraps `CTParagraphStyleCreate`.
    pub fn new(options: &ParagraphStyleOptions) -> CoreTextResult<Self> {
        let json = options.json()?;
        let tabs: Vec<_> = options.text_tabs.iter().map(TextTab::as_raw).collect();
        let raw = unsafe {
            bridge::ct_paragraph_style_create(
                json.as_ptr(),
                tabs.as_ptr(),
                isize::try_from(tabs.len()).unwrap_or(isize::MAX),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_paragraph_style_create returned NULL",
        )?))
    }

    /// Wraps `CTParagraphStyleCreate`.
    pub fn with_alignment(alignment: TextAlignment) -> CoreTextResult<Self> {
        Self::new(&ParagraphStyleOptions {
            alignment: Some(alignment),
            ..ParagraphStyleOptions::default()
        })
    }

    /// Wraps `CTParagraphStyleCreateCopy`.
    pub fn copy(&self) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_paragraph_style_copy(self.raw) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_paragraph_style_copy returned NULL",
        )?))
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn alignment(&self) -> TextAlignment {
        TextAlignment::from_raw(unsafe { bridge::ct_paragraph_style_get_alignment(self.raw) })
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn first_line_head_indent(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_first_line_head_indent(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn head_indent(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_head_indent(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn tail_indent(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_tail_indent(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn default_tab_interval(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_default_tab_interval(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn line_break_mode(&self) -> LineBreakMode {
        LineBreakMode::from_raw(unsafe { bridge::ct_paragraph_style_get_line_break_mode(self.raw) })
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn line_height_multiple(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_line_height_multiple(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn maximum_line_height(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_maximum_line_height(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn minimum_line_height(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_minimum_line_height(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn paragraph_spacing(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_paragraph_spacing(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn paragraph_spacing_before(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_paragraph_spacing_before(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn base_writing_direction(&self) -> WritingDirection {
        WritingDirection::from_raw(unsafe {
            bridge::ct_paragraph_style_get_base_writing_direction(self.raw)
        })
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn maximum_line_spacing(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_maximum_line_spacing(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn minimum_line_spacing(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_minimum_line_spacing(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn line_spacing_adjustment(&self) -> f64 {
        unsafe { bridge::ct_paragraph_style_get_line_spacing_adjustment(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn line_bounds_options(&self) -> u64 {
        unsafe { bridge::ct_paragraph_style_get_line_bounds_options(self.raw) }
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    #[must_use]
    pub fn tab_stops(&self) -> Vec<TextTab> {
        let count = unsafe { bridge::ct_paragraph_style_get_text_tab_count(self.raw) };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_paragraph_style_copy_text_tabs(self.raw, handles.as_mut_ptr(), count)
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(TextTab::from_raw).collect()
    }

    /// Wraps `CTParagraphStyleGetValueForSpecifier`.
    pub fn value_for_specifier_json(&self, specifier: u32) -> CoreTextResult<serde_json::Value> {
        unsafe {
            crate::common::json_from_owned(bridge::ct_paragraph_style_get_value_for_specifier_json(
                self.raw, specifier,
            ))
        }
    }
}

/// Wraps `CTParagraphStyleGetTypeID`.
#[must_use]
pub fn paragraph_style_type_id() -> u64 {
    unsafe { bridge::ct_paragraph_style_get_type_id() }
}
