use crate::bridge;
use crate::common::{cstring, expect_handle, impl_handle, option_string_from_owned};
use crate::error::CoreTextResult;
use crate::font::CTFont;

pub type GlyphId = u16;

/// Adobe character collection identifiers used by `CTGlyphInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum CharacterCollection {
    #[default]
    IdentityMapping = 0,
    AdobeCNS1 = 1,
    AdobeGB1 = 2,
    AdobeJapan1 = 3,
    AdobeJapan2 = 4,
    AdobeKorea1 = 5,
}

impl CharacterCollection {
    pub(crate) const fn from_raw(raw: u16) -> Self {
        match raw {
            1 => Self::AdobeCNS1,
            2 => Self::AdobeGB1,
            3 => Self::AdobeJapan1,
            4 => Self::AdobeJapan2,
            5 => Self::AdobeKorea1,
            _ => Self::IdentityMapping,
        }
    }
}

/// An immutable `CTGlyphInfo` wrapper.
pub struct GlyphInfo {
    raw: bridge::Handle,
}

impl_handle!(GlyphInfo);

impl GlyphInfo {
    pub fn with_glyph_name(
        glyph_name: &str,
        font: &CTFont,
        base_string: &str,
    ) -> CoreTextResult<Self> {
        let glyph_name = cstring(glyph_name)?;
        let base_string = cstring(base_string)?;
        let raw = unsafe {
            bridge::ct_glyph_info_create_with_glyph_name(
                glyph_name.as_ptr(),
                font.as_raw(),
                base_string.as_ptr(),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_glyph_info_create_with_glyph_name returned NULL",
        )?))
    }

    pub fn with_glyph(glyph: GlyphId, font: &CTFont, base_string: &str) -> CoreTextResult<Self> {
        let base_string = cstring(base_string)?;
        let raw = unsafe {
            bridge::ct_glyph_info_create_with_glyph(glyph, font.as_raw(), base_string.as_ptr())
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_glyph_info_create_with_glyph returned NULL",
        )?))
    }

    pub fn with_character_identifier(
        character_identifier: u16,
        collection: CharacterCollection,
        base_string: &str,
    ) -> CoreTextResult<Self> {
        let base_string = cstring(base_string)?;
        let raw = unsafe {
            bridge::ct_glyph_info_create_with_character_identifier(
                character_identifier,
                collection as u16,
                base_string.as_ptr(),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_glyph_info_create_with_character_identifier returned NULL",
        )?))
    }

    #[must_use]
    pub fn glyph_name(&self) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_glyph_info_copy_glyph_name(self.raw)) }
    }

    #[must_use]
    pub fn glyph(&self) -> GlyphId {
        unsafe { bridge::ct_glyph_info_get_glyph(self.raw) }
    }

    #[must_use]
    pub fn character_identifier(&self) -> u16 {
        unsafe { bridge::ct_glyph_info_get_character_identifier(self.raw) }
    }

    #[must_use]
    pub fn character_collection(&self) -> CharacterCollection {
        CharacterCollection::from_raw(unsafe {
            bridge::ct_glyph_info_get_character_collection(self.raw)
        })
    }
}
