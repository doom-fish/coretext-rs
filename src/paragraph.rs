use crate::bridge;
use crate::common::{expect_handle, impl_handle, impl_thread_safe};
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

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub(crate) struct EncodedParagraphStyleOptions {
    present: u64,
    alignment: u8,
    first_line_head_indent: f64,
    head_indent: f64,
    tail_indent: f64,
    default_tab_interval: f64,
    line_break_mode: u8,
    line_height_multiple: f64,
    maximum_line_height: f64,
    minimum_line_height: f64,
    paragraph_spacing: f64,
    paragraph_spacing_before: f64,
    base_writing_direction: i8,
    maximum_line_spacing: f64,
    minimum_line_spacing: f64,
    line_spacing_adjustment: f64,
    line_bounds_options: u64,
}

impl ParagraphStyleOptions {
    fn encoded(&self) -> EncodedParagraphStyleOptions {
        EncodedParagraphStyleOptions {
            present: [
                self.alignment.is_some(),
                self.first_line_head_indent.is_some(),
                self.head_indent.is_some(),
                self.tail_indent.is_some(),
                self.default_tab_interval.is_some(),
                self.line_break_mode.is_some(),
                self.line_height_multiple.is_some(),
                self.maximum_line_height.is_some(),
                self.minimum_line_height.is_some(),
                self.paragraph_spacing.is_some(),
                self.paragraph_spacing_before.is_some(),
                self.base_writing_direction.is_some(),
                self.maximum_line_spacing.is_some(),
                self.minimum_line_spacing.is_some(),
                self.line_spacing_adjustment.is_some(),
                self.line_bounds_options.is_some(),
            ]
            .into_iter()
            .enumerate()
            .fold(0, |mask, (index, present)| {
                mask | (u64::from(present) << index)
            }),
            alignment: self.alignment.unwrap_or_default() as u8,
            first_line_head_indent: self.first_line_head_indent.unwrap_or_default(),
            head_indent: self.head_indent.unwrap_or_default(),
            tail_indent: self.tail_indent.unwrap_or_default(),
            default_tab_interval: self.default_tab_interval.unwrap_or_default(),
            line_break_mode: self.line_break_mode.unwrap_or_default() as u8,
            line_height_multiple: self.line_height_multiple.unwrap_or_default(),
            maximum_line_height: self.maximum_line_height.unwrap_or_default(),
            minimum_line_height: self.minimum_line_height.unwrap_or_default(),
            paragraph_spacing: self.paragraph_spacing.unwrap_or_default(),
            paragraph_spacing_before: self.paragraph_spacing_before.unwrap_or_default(),
            base_writing_direction: self.base_writing_direction.unwrap_or_default() as i8,
            maximum_line_spacing: self.maximum_line_spacing.unwrap_or_default(),
            minimum_line_spacing: self.minimum_line_spacing.unwrap_or_default(),
            line_spacing_adjustment: self.line_spacing_adjustment.unwrap_or_default(),
            line_bounds_options: self.line_bounds_options.unwrap_or_default(),
        }
    }
}

/// An immutable `CTParagraphStyle` wrapper.
pub struct ParagraphStyle {
    raw: bridge::Handle,
}

impl_handle!(ParagraphStyle);
impl_thread_safe!(ParagraphStyle);

impl ParagraphStyle {
    /// Wraps `CTParagraphStyleCreate`.
    pub fn new(options: &ParagraphStyleOptions) -> CoreTextResult<Self> {
        let encoded = options.encoded();
        let tabs: Vec<_> = options.text_tabs.iter().map(TextTab::as_raw).collect();
        let raw = unsafe {
            bridge::ct_paragraph_style_create(
                &raw const encoded,
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
    pub fn value_for_specifier(&self, specifier: u32) -> CoreTextResult<crate::AttributeValue> {
        unsafe {
            crate::abi::from_owned(bridge::ct_paragraph_style_get_value_for_specifier(
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

#[cfg(test)]
mod tests {
    use super::{LineBreakMode, ParagraphStyleOptions, TextAlignment, WritingDirection};

    fn assert_close(left: f64, right: f64) {
        assert!(
            (left - right).abs() < f64::EPSILON,
            "expected {left} to match {right}"
        );
    }

    #[test]
    fn text_alignment_from_raw_and_default_are_stable() {
        assert_eq!(TextAlignment::default(), TextAlignment::Natural);
        assert_eq!(TextAlignment::from_raw(0), TextAlignment::Left);
        assert_eq!(TextAlignment::from_raw(1), TextAlignment::Right);
        assert_eq!(TextAlignment::from_raw(2), TextAlignment::Center);
        assert_eq!(TextAlignment::from_raw(3), TextAlignment::Justified);
        assert_eq!(TextAlignment::from_raw(255), TextAlignment::Natural);
    }

    #[test]
    fn line_break_mode_from_raw_and_default_are_stable() {
        assert_eq!(LineBreakMode::default(), LineBreakMode::WordWrapping);
        assert_eq!(LineBreakMode::from_raw(1), LineBreakMode::CharWrapping);
        assert_eq!(LineBreakMode::from_raw(2), LineBreakMode::Clipping);
        assert_eq!(LineBreakMode::from_raw(3), LineBreakMode::TruncatingHead);
        assert_eq!(LineBreakMode::from_raw(4), LineBreakMode::TruncatingTail);
        assert_eq!(LineBreakMode::from_raw(5), LineBreakMode::TruncatingMiddle);
        assert_eq!(LineBreakMode::from_raw(255), LineBreakMode::WordWrapping);
    }

    #[test]
    fn writing_direction_from_raw_and_default_are_stable() {
        assert_eq!(WritingDirection::default(), WritingDirection::Natural);
        assert_eq!(WritingDirection::from_raw(0), WritingDirection::LeftToRight);
        assert_eq!(WritingDirection::from_raw(1), WritingDirection::RightToLeft);
        assert_eq!(WritingDirection::from_raw(-1), WritingDirection::Natural);
    }

    #[test]
    fn encoded_options_preserve_selected_values() {
        let options = ParagraphStyleOptions {
            alignment: Some(TextAlignment::Center),
            first_line_head_indent: Some(12.0),
            line_break_mode: Some(LineBreakMode::TruncatingMiddle),
            base_writing_direction: Some(WritingDirection::RightToLeft),
            line_bounds_options: Some(0b101),
            ..ParagraphStyleOptions::default()
        };
        let encoded = options.encoded();

        assert_eq!(encoded.alignment, 2);
        assert_close(encoded.first_line_head_indent, 12.0);
        assert_eq!(encoded.line_break_mode, 5);
        assert_eq!(encoded.base_writing_direction, 1);
        assert_eq!(encoded.line_bounds_options, 0b101);
        assert_eq!(
            encoded.present,
            1 | (1 << 1) | (1 << 5) | (1 << 11) | (1 << 15)
        );
    }

    #[test]
    fn paragraph_style_binary_marks_present_fields() {
        let options = ParagraphStyleOptions {
            first_line_head_indent: Some(12.0),
            maximum_line_spacing: Some(18.0),
            ..ParagraphStyleOptions::default()
        };
        let encoded = options.encoded();
        assert_eq!(encoded.present, (1 << 1) | (1 << 12));
        assert_close(encoded.first_line_head_indent, 12.0);
        assert_close(encoded.maximum_line_spacing, 18.0);
    }
}
