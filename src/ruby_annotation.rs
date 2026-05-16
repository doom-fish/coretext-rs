use crate::bridge;
use crate::common::{cstring, expect_handle, impl_handle, option_string_from_owned};
use crate::error::CoreTextResult;

/// How ruby text aligns relative to base text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RubyAlignment {
    Invalid = u8::MAX,
    Auto = 0,
    Start = 1,
    Center = 2,
    End = 3,
    DistributeLetter = 4,
    DistributeSpace = 5,
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
    Invalid = u8::MAX,
    Auto = 0,
    Start = 1,
    End = 2,
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
    Before = 0,
    After = 1,
    InterCharacter = 2,
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

    pub fn copy(&self) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_ruby_annotation_copy(self.raw) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_ruby_annotation_copy returned NULL",
        )?))
    }

    #[must_use]
    pub fn alignment(&self) -> RubyAlignment {
        RubyAlignment::from_raw(unsafe { bridge::ct_ruby_annotation_get_alignment(self.raw) })
    }

    #[must_use]
    pub fn overhang(&self) -> RubyOverhang {
        RubyOverhang::from_raw(unsafe { bridge::ct_ruby_annotation_get_overhang(self.raw) })
    }

    #[must_use]
    pub fn size_factor(&self) -> f64 {
        unsafe { bridge::ct_ruby_annotation_get_size_factor(self.raw) }
    }

    #[must_use]
    pub fn text_for_position(&self, position: RubyPosition) -> Option<String> {
        unsafe {
            option_string_from_owned(bridge::ct_ruby_annotation_copy_text_for_position(
                self.raw,
                position as u8,
            ))
        }
    }
}
