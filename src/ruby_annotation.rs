use crate::bridge;
use crate::common::{cstring, expect_handle, impl_handle, option_string_from_owned};
use crate::error::CoreTextResult;

/// How ruby text aligns relative to base text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RubyAlignment {
    /// Selects the invalid case of `CTRubyAlignment`.
    Invalid = u8::MAX,
    /// Selects the auto case of `CTRubyAlignment`.
    Auto = 0,
    /// Selects the start case of `CTRubyAlignment`.
    Start = 1,
    /// Selects the center case of `CTRubyAlignment`.
    Center = 2,
    /// Selects the end case of `CTRubyAlignment`.
    End = 3,
    /// Selects the distribute letter case of `CTRubyAlignment`.
    DistributeLetter = 4,
    /// Selects the distribute space case of `CTRubyAlignment`.
    DistributeSpace = 5,
    /// Selects the line edge case of `CTRubyAlignment`.
    LineEdge = 6,
}

impl RubyAlignment {
    pub(crate) const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Auto,
            1 => Self::Start,
            2 => Self::Center,
            3 => Self::End,
            4 => Self::DistributeLetter,
            5 => Self::DistributeSpace,
            6 => Self::LineEdge,
            _ => Self::Invalid,
        }
    }
}

/// How ruby text may overhang adjacent characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RubyOverhang {
    /// Selects the invalid case of `CTRubyOverhang`.
    Invalid = u8::MAX,
    /// Selects the auto case of `CTRubyOverhang`.
    Auto = 0,
    /// Selects the start case of `CTRubyOverhang`.
    Start = 1,
    /// Selects the end case of `CTRubyOverhang`.
    End = 2,
    /// Selects the none case of `CTRubyOverhang`.
    None = 3,
}

impl RubyOverhang {
    pub(crate) const fn from_raw(raw: u8) -> Self {
        match raw {
            0 => Self::Auto,
            1 => Self::Start,
            2 => Self::End,
            3 => Self::None,
            _ => Self::Invalid,
        }
    }
}

/// Position of ruby text relative to base text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RubyPosition {
    /// Selects the before case of `CTRubyPosition`.
    Before = 0,
    /// Selects the after case of `CTRubyPosition`.
    After = 1,
    /// Selects the inter character case of `CTRubyPosition`.
    InterCharacter = 2,
    /// Selects the inline case of `CTRubyPosition`.
    Inline = 3,
}

impl RubyPosition {
    #[allow(dead_code)]
    pub(crate) const fn from_raw(raw: u8) -> Self {
        match raw {
            1 => Self::After,
            2 => Self::InterCharacter,
            3 => Self::Inline,
            _ => Self::Before,
        }
    }
}

/// An immutable `CTRubyAnnotation` wrapper.
pub struct RubyAnnotation {
    raw: bridge::Handle,
}

impl_handle!(RubyAnnotation);

impl RubyAnnotation {
    /// Wraps `CTRubyAnnotationCreate`.
    pub fn new(
        alignment: RubyAlignment,
        overhang: RubyOverhang,
        size_factor: f64,
        texts: [Option<&str>; 4],
    ) -> CoreTextResult<Self> {
        let before = texts[0].map(cstring).transpose()?;
        let after = texts[1].map(cstring).transpose()?;
        let inter_character = texts[2].map(cstring).transpose()?;
        let inline = texts[3].map(cstring).transpose()?;
        let raw = unsafe {
            bridge::ct_ruby_annotation_create(
                alignment as u8,
                overhang as u8,
                size_factor,
                before
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                after
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                inter_character
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                inline
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_ruby_annotation_create returned NULL",
        )?))
    }

    /// Wraps `CTRubyAnnotationCreateCopy`.
    pub fn copy(&self) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_ruby_annotation_copy(self.raw) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_ruby_annotation_copy returned NULL",
        )?))
    }

    /// Wraps `CTRubyAnnotationGetAlignment`.
    #[must_use]
    pub fn alignment(&self) -> RubyAlignment {
        RubyAlignment::from_raw(unsafe { bridge::ct_ruby_annotation_get_alignment(self.raw) })
    }

    /// Wraps `CTRubyAnnotationGetOverhang`.
    #[must_use]
    pub fn overhang(&self) -> RubyOverhang {
        RubyOverhang::from_raw(unsafe { bridge::ct_ruby_annotation_get_overhang(self.raw) })
    }

    /// Wraps `CTRubyAnnotationGetSizeFactor`.
    #[must_use]
    pub fn size_factor(&self) -> f64 {
        unsafe { bridge::ct_ruby_annotation_get_size_factor(self.raw) }
    }

    /// Wraps `CTRubyAnnotationGetTextForPosition`.
    #[must_use]
    pub fn text_for_position(&self, position: RubyPosition) -> Option<String> {
        unsafe {
            option_string_from_owned(bridge::ct_ruby_annotation_copy_text_for_position(
                self.raw,
                position as u8,
            ))
        }
    }

    /// Wraps `CTRubyAnnotationCreateWithAttributes`.
    pub fn with_attributes(
        alignment: RubyAlignment,
        overhang: RubyOverhang,
        size_factor: f64,
        texts: [Option<&str>; 4],
    ) -> CoreTextResult<Self> {
        let payload = serde_json::json!({
            "alignment": alignment as u8,
            "overhang": overhang as u8,
            "sizeFactor": size_factor,
            "texts": texts,
        });
        let json = cstring(&payload.to_string())?;
        let raw = unsafe { bridge::ct_ruby_annotation_create_with_attributes_json(json.as_ptr()) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_ruby_annotation_create_with_attributes_json returned NULL",
        )?))
    }
}

/// Wraps `CTRubyAnnotationGetTypeID`.
#[must_use]
pub fn ruby_annotation_type_id() -> u64 {
    unsafe { bridge::ct_ruby_annotation_get_type_id() }
}
