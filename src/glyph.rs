use crate::bridge;
use crate::common::{cstring, expect_handle, impl_handle, option_string_from_owned};
use crate::error::CoreTextResult;
use crate::font::CTFont;

/// Glyph identifier type used by `CTGlyphInfo` and `CTFont` glyph APIs.
pub type GlyphId = u16;

/// Adobe character collection identifiers used by `CTGlyphInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u16)]
pub enum CharacterCollection {
    /// Selects the identity mapping case of `CTCharacterCollection`.
    #[default]
    IdentityMapping = 0,
    /// Selects the adobe cns1 case of `CTCharacterCollection`.
    AdobeCNS1 = 1,
    /// Selects the adobe gb1 case of `CTCharacterCollection`.
    AdobeGB1 = 2,
    /// Selects the adobe japan1 case of `CTCharacterCollection`.
    AdobeJapan1 = 3,
    /// Selects the adobe japan2 case of `CTCharacterCollection`.
    AdobeJapan2 = 4,
    /// Selects the adobe korea1 case of `CTCharacterCollection`.
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
    /// Wraps `CTGlyphInfoCreateWithGlyphName`.
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

    /// Wraps `CTGlyphInfoCreateWithGlyph`.
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

    /// Wraps `CTGlyphInfoCreateWithCharacterIdentifier`.
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

    /// Wraps `CTGlyphInfoGetGlyphName`.
    #[must_use]
    pub fn glyph_name(&self) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_glyph_info_copy_glyph_name(self.raw)) }
    }

    /// Wraps `CTGlyphInfoGetGlyph`.
    #[must_use]
    pub fn glyph(&self) -> GlyphId {
        unsafe { bridge::ct_glyph_info_get_glyph(self.raw) }
    }

    /// Wraps `CTGlyphInfoGetCharacterIdentifier`.
    #[must_use]
    pub fn character_identifier(&self) -> u16 {
        unsafe { bridge::ct_glyph_info_get_character_identifier(self.raw) }
    }

    /// Wraps `CTGlyphInfoGetCharacterCollection`.
    #[must_use]
    pub fn character_collection(&self) -> CharacterCollection {
        CharacterCollection::from_raw(unsafe {
            bridge::ct_glyph_info_get_character_collection(self.raw)
        })
    }
}

/// Wraps `CTGlyphInfoGetTypeID`.
#[must_use]
pub fn glyph_info_type_id() -> u64 {
    unsafe { bridge::ct_glyph_info_get_type_id() }
}
