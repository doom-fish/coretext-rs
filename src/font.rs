use apple_cf::cg::CGContext;

use crate::adaptive_image::AdaptiveImageProvider;
use crate::bridge;
use crate::common::{
    cstring, expect_handle, impl_handle, json_from_owned, option_string_from_owned,
    optional_cstring, string_from_owned,
};
use crate::error::{CoreTextError, CoreTextResult};
use crate::font_descriptor::{FontDescriptor, FontOrientation};
use crate::font_feature::{FontFeature, FontFeatureSetting};
use crate::font_traits::FontTraits;
use crate::font_variation::{FontVariationAxis, FontVariationCoordinate};
use crate::types::{CFRange, CGAffineTransform, CGPoint, CGRect, CGSize, TextRange};

/// Constants for `CTFontCopyName` / `CTFontCopyLocalizedName`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FontNameKey {
    /// Selects the copyright name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Copyright = 0,
    /// Selects the family name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Family = 1,
    /// Selects the sub family name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    SubFamily = 2,
    /// Selects the style name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Style = 3,
    /// Selects the unique name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Unique = 4,
    /// Selects the full name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Full = 5,
    /// Selects the version name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Version = 6,
    /// Selects the post script name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    PostScript = 7,
    /// Selects the trademark name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Trademark = 8,
    /// Selects the manufacturer name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Manufacturer = 9,
    /// Selects the designer name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Designer = 10,
    /// Selects the description name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    Description = 11,
    /// Selects the vendor URL name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    VendorUrl = 12,
    /// Selects the designer URL name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    DesignerUrl = 13,
    /// Selects the license name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    License = 14,
    /// Selects the license URL name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    LicenseUrl = 15,
    /// Selects the sample text name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    SampleText = 16,
    /// Selects the post script CID name key used by `CTFontCopyName` and `CTFontCopyLocalizedName`.
    PostScriptCid = 17,
}

/// UI font selectors for `CTFontCreateUIFontForLanguage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum UIFontType {
    /// Selects the none UI font case used by `CTFontCreateUIFontForLanguage`.
    None = u32::MAX,
    /// Selects the user UI font case used by `CTFontCreateUIFontForLanguage`.
    User = 0,
    /// Selects the user fixed pitch UI font case used by `CTFontCreateUIFontForLanguage`.
    UserFixedPitch = 1,
    /// Selects the system UI font case used by `CTFontCreateUIFontForLanguage`.
    System = 2,
    /// Selects the emphasized system UI font case used by `CTFontCreateUIFontForLanguage`.
    EmphasizedSystem = 3,
    /// Selects the small system UI font case used by `CTFontCreateUIFontForLanguage`.
    SmallSystem = 4,
    /// Selects the small emphasized system UI font case used by `CTFontCreateUIFontForLanguage`.
    SmallEmphasizedSystem = 5,
    /// Selects the mini system UI font case used by `CTFontCreateUIFontForLanguage`.
    MiniSystem = 6,
    /// Selects the mini emphasized system UI font case used by `CTFontCreateUIFontForLanguage`.
    MiniEmphasizedSystem = 7,
    /// Selects the views UI font case used by `CTFontCreateUIFontForLanguage`.
    Views = 8,
    /// Selects the application UI font case used by `CTFontCreateUIFontForLanguage`.
    Application = 9,
    /// Selects the label UI font case used by `CTFontCreateUIFontForLanguage`.
    Label = 10,
    /// Selects the menu title UI font case used by `CTFontCreateUIFontForLanguage`.
    MenuTitle = 11,
    /// Selects the menu item UI font case used by `CTFontCreateUIFontForLanguage`.
    MenuItem = 12,
    /// Selects the menu item mark UI font case used by `CTFontCreateUIFontForLanguage`.
    MenuItemMark = 13,
    /// Selects the menu item command key UI font case used by `CTFontCreateUIFontForLanguage`.
    MenuItemCommandKey = 14,
    /// Selects the window title UI font case used by `CTFontCreateUIFontForLanguage`.
    WindowTitle = 15,
    /// Selects the push button UI font case used by `CTFontCreateUIFontForLanguage`.
    PushButton = 16,
    /// Selects the utility window title UI font case used by `CTFontCreateUIFontForLanguage`.
    UtilityWindowTitle = 17,
    /// Selects the alert header UI font case used by `CTFontCreateUIFontForLanguage`.
    AlertHeader = 18,
    /// Selects the system detail UI font case used by `CTFontCreateUIFontForLanguage`.
    SystemDetail = 19,
    /// Selects the emphasized system detail UI font case used by `CTFontCreateUIFontForLanguage`.
    EmphasizedSystemDetail = 20,
    /// Selects the toolbar UI font case used by `CTFontCreateUIFontForLanguage`.
    Toolbar = 21,
    /// Selects the small toolbar UI font case used by `CTFontCreateUIFontForLanguage`.
    SmallToolbar = 22,
    /// Selects the message UI font case used by `CTFontCreateUIFontForLanguage`.
    Message = 23,
    /// Selects the palette UI font case used by `CTFontCreateUIFontForLanguage`.
    Palette = 24,
    /// Selects the tool tip UI font case used by `CTFontCreateUIFontForLanguage`.
    ToolTip = 25,
    /// Selects the control content UI font case used by `CTFontCreateUIFontForLanguage`.
    ControlContent = 26,
}

/// An immutable `CTFont` wrapper.
pub struct CTFont {
    raw: bridge::Handle,
}

impl_handle!(CTFont);

impl CTFont {
    /// Wraps `CTFontCreateWithName`.
    pub fn new(name: &str, size: f64) -> CoreTextResult<Self> {
        let name = cstring(name)?;
        let raw = unsafe { bridge::ct_font_create_with_name(name.as_ptr(), size) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_with_name returned NULL",
        )?))
    }

    /// Wraps `CTFontCreateWithFontDescriptor`.
    pub fn from_descriptor(descriptor: &FontDescriptor, size: f64) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_font_create_with_descriptor(descriptor.as_raw(), size) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_with_descriptor returned NULL",
        )?))
    }

    /// Wraps `CTFontCreateUIFontForLanguage`.
    pub fn ui_font(ui_type: UIFontType, size: f64, language: Option<&str>) -> CoreTextResult<Self> {
        let language = optional_cstring(language)?;
        let raw = unsafe {
            bridge::ct_font_create_ui_font(
                ui_type as u32,
                size,
                language
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_ui_font returned NULL",
        )?))
    }

    /// Wraps `CTFontCopyWithAttributes`.
    pub fn copy_with_attributes(
        &self,
        size: f64,
        descriptor: Option<&FontDescriptor>,
    ) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_font_copy_with_attributes(
                self.raw,
                size,
                descriptor.map_or(std::ptr::null_mut(), FontDescriptor::as_raw),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_copy_with_attributes returned NULL",
        )?))
    }

    /// Wraps `CTFontCopyWithFamily`.
    pub fn copy_with_family(&self, size: f64, family: &str) -> CoreTextResult<Self> {
        let family = cstring(family)?;
        let raw = unsafe { bridge::ct_font_copy_with_family(self.raw, size, family.as_ptr()) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_copy_with_family returned NULL",
        )?))
    }

    /// Wraps `CTFontCopyWithSymbolicTraits`.
    pub fn copy_with_symbolic_traits(
        &self,
        size: f64,
        trait_value: u32,
        trait_mask: u32,
    ) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_font_copy_with_symbolic_traits(self.raw, size, trait_value, trait_mask)
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_copy_with_symbolic_traits returned NULL",
        )?))
    }

    /// Wraps `CTFontCreateForString`.
    pub fn font_for_string(
        &self,
        string: &str,
        range: TextRange,
        language: Option<&str>,
    ) -> CoreTextResult<Self> {
        let string = cstring(string)?;
        let language = optional_cstring(language)?;
        let raw = unsafe {
            bridge::ct_font_create_for_string(
                self.raw,
                string.as_ptr(),
                CFRange::from(range),
                language
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_for_string returned NULL",
        )?))
    }

    /// Wraps `CTFontCopyFontDescriptor`.
    pub fn descriptor(&self) -> CoreTextResult<FontDescriptor> {
        let raw = unsafe { bridge::ct_font_copy_descriptor(self.raw) };
        Ok(FontDescriptor::from_raw(expect_handle(
            raw,
            "ct_font_copy_descriptor returned NULL",
        )?))
    }

    /// Wraps `CTFontGetSize`.
    #[must_use]
    pub fn size(&self) -> f64 {
        unsafe { bridge::ct_font_get_size(self.raw) }
    }

    /// Wraps `CTFontGetMatrix`.
    #[must_use]
    pub fn matrix(&self) -> CGAffineTransform {
        unsafe { bridge::ct_font_get_matrix(self.raw) }
    }

    /// Wraps `CTFontGetSymbolicTraits`.
    #[must_use]
    pub fn symbolic_traits(&self) -> u32 {
        unsafe { bridge::ct_font_get_symbolic_traits(self.raw) }
    }

    /// Wraps `CTFontCopyTraits`.
    pub fn traits(&self) -> CoreTextResult<FontTraits> {
        unsafe { json_from_owned(bridge::ct_font_copy_traits_json(self.raw)) }
    }

    /// Wraps `CTFontCopyPostScriptName`.
    pub fn postscript_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_postscript_name(self.raw)) }
    }

    /// Wraps `CTFontCopyFamilyName`.
    pub fn family_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_family_name(self.raw)) }
    }

    /// Wraps `CTFontCopyFullName`.
    pub fn full_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_full_name(self.raw)) }
    }

    /// Wraps `CTFontCopyDisplayName`.
    pub fn display_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_display_name(self.raw)) }
    }

    /// Wraps `CTFontCopyName`.
    #[must_use]
    pub fn name(&self, key: FontNameKey) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_copy_name(self.raw, key as u32)) }
    }

    /// Wraps `CTFontCopyLocalizedName`.
    pub fn localized_name(&self, key: FontNameKey) -> CoreTextResult<Option<String>> {
        Ok(unsafe {
            option_string_from_owned(bridge::ct_font_copy_localized_name(self.raw, key as u32))
        })
    }

    /// Wraps `CTFontGetAscent`.
    #[must_use]
    pub fn ascent(&self) -> f64 {
        unsafe { bridge::ct_font_get_ascent(self.raw) }
    }

    /// Wraps `CTFontGetDescent`.
    #[must_use]
    pub fn descent(&self) -> f64 {
        unsafe { bridge::ct_font_get_descent(self.raw) }
    }

    /// Wraps `CTFontGetLeading`.
    #[must_use]
    pub fn leading(&self) -> f64 {
        unsafe { bridge::ct_font_get_leading(self.raw) }
    }

    /// Wraps `CTFontGetUnitsPerEm`.
    #[must_use]
    pub fn units_per_em(&self) -> u32 {
        unsafe { bridge::ct_font_get_units_per_em(self.raw) }
    }

    /// Wraps `CTFontGetGlyphCount`.
    #[must_use]
    pub fn glyph_count(&self) -> isize {
        unsafe { bridge::ct_font_get_glyph_count(self.raw) }
    }

    /// Wraps `CTFontGetBoundingBox`.
    #[must_use]
    pub fn bounding_box(&self) -> CGRect {
        unsafe { bridge::ct_font_get_bounding_box(self.raw) }
    }

    /// Wraps `CTFontGetUnderlinePosition`.
    #[must_use]
    pub fn underline_position(&self) -> f64 {
        unsafe { bridge::ct_font_get_underline_position(self.raw) }
    }

    /// Wraps `CTFontGetUnderlineThickness`.
    #[must_use]
    pub fn underline_thickness(&self) -> f64 {
        unsafe { bridge::ct_font_get_underline_thickness(self.raw) }
    }

    /// Wraps `CTFontGetSlantAngle`.
    #[must_use]
    pub fn slant_angle(&self) -> f64 {
        unsafe { bridge::ct_font_get_slant_angle(self.raw) }
    }

    /// Wraps `CTFontGetCapHeight`.
    #[must_use]
    pub fn cap_height(&self) -> f64 {
        unsafe { bridge::ct_font_get_cap_height(self.raw) }
    }

    /// Wraps `CTFontGetXHeight`.
    #[must_use]
    pub fn x_height(&self) -> f64 {
        unsafe { bridge::ct_font_get_x_height(self.raw) }
    }

    /// Wraps `CTFontCopySupportedLanguages`.
    pub fn supported_languages(&self) -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_copy_supported_languages_json(self.raw)) }
    }

    /// Wraps `CTFontGetGlyphsForCharacters`.
    pub fn glyphs_for_string(&self, string: &str) -> CoreTextResult<Vec<u16>> {
        let utf16: Vec<u16> = string.encode_utf16().collect();
        if utf16.is_empty() {
            return Ok(Vec::new());
        }
        let mut glyphs = vec![0_u16; utf16.len()];
        let ok = unsafe {
            bridge::ct_font_get_glyphs_for_characters(
                self.raw,
                utf16.as_ptr(),
                glyphs.as_mut_ptr(),
                isize::try_from(utf16.len()).unwrap_or(isize::MAX),
            )
        };
        if ok {
            Ok(glyphs)
        } else {
            Err(CoreTextError::Bridge(
                "one or more characters could not be mapped to glyphs".to_string(),
            ))
        }
    }

    /// Wraps `CTFontGetGlyphWithName`.
    pub fn glyph_with_name(&self, glyph_name: &str) -> CoreTextResult<u16> {
        let glyph_name = cstring(glyph_name)?;
        Ok(unsafe { bridge::ct_font_get_glyph_with_name(self.raw, glyph_name.as_ptr()) })
    }

    /// Wraps `CTFontCopyNameForGlyph`.
    #[must_use]
    pub fn name_for_glyph(&self, glyph: u16) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_copy_name_for_glyph(self.raw, glyph)) }
    }

    /// Wraps `CTFontGetBoundingRectsForGlyphs`.
    #[must_use]
    pub fn bounding_rects_for_glyphs(
        &self,
        orientation: FontOrientation,
        glyphs: &[u16],
    ) -> (CGRect, Vec<CGRect>) {
        let mut rects = vec![CGRect::default(); glyphs.len()];
        let overall = unsafe {
            bridge::ct_font_get_bounding_rects_for_glyphs(
                self.raw,
                orientation as u32,
                glyphs.as_ptr(),
                rects.as_mut_ptr(),
                isize::try_from(glyphs.len()).unwrap_or(isize::MAX),
            )
        };
        (overall, rects)
    }

    /// Wraps `CTFontGetOpticalBoundsForGlyphs`.
    #[must_use]
    pub fn optical_bounds_for_glyphs(&self, glyphs: &[u16]) -> (CGRect, Vec<CGRect>) {
        let mut rects = vec![CGRect::default(); glyphs.len()];
        let overall = unsafe {
            bridge::ct_font_get_optical_bounds_for_glyphs(
                self.raw,
                glyphs.as_ptr(),
                rects.as_mut_ptr(),
                isize::try_from(glyphs.len()).unwrap_or(isize::MAX),
            )
        };
        (overall, rects)
    }

    /// Wraps `CTFontGetAdvancesForGlyphs`.
    #[must_use]
    pub fn advances_for_glyphs(
        &self,
        orientation: FontOrientation,
        glyphs: &[u16],
    ) -> (f64, Vec<CGSize>) {
        let mut advances = vec![CGSize::default(); glyphs.len()];
        let total = unsafe {
            bridge::ct_font_get_advances_for_glyphs(
                self.raw,
                orientation as u32,
                glyphs.as_ptr(),
                advances.as_mut_ptr(),
                isize::try_from(glyphs.len()).unwrap_or(isize::MAX),
            )
        };
        (total, advances)
    }

    /// Wraps `CTFontGetVerticalTranslationsForGlyphs`.
    #[must_use]
    pub fn vertical_translations_for_glyphs(&self, glyphs: &[u16]) -> Vec<CGSize> {
        let mut translations = vec![CGSize::default(); glyphs.len()];
        unsafe {
            bridge::ct_font_get_vertical_translations_for_glyphs(
                self.raw,
                glyphs.as_ptr(),
                translations.as_mut_ptr(),
                isize::try_from(glyphs.len()).unwrap_or(isize::MAX),
            );
        }
        translations
    }

    /// Wraps `CTFontCopyVariationAxes`.
    pub fn variation_axes(&self) -> CoreTextResult<Vec<FontVariationAxis>> {
        unsafe { json_from_owned(bridge::ct_font_copy_variation_axes_json(self.raw)) }
    }

    /// Wraps `CTFontCopyVariation`.
    pub fn variation_coordinates(&self) -> CoreTextResult<Vec<FontVariationCoordinate>> {
        unsafe { json_from_owned(bridge::ct_font_copy_variation_json(self.raw)) }
    }

    /// Wraps `CTFontCopyFeatures`.
    pub fn features(&self) -> CoreTextResult<Vec<FontFeature>> {
        unsafe { json_from_owned(bridge::ct_font_copy_features_json(self.raw)) }
    }

    /// Wraps `CTFontCopyFeatureSettings`.
    pub fn feature_settings(&self) -> CoreTextResult<Vec<FontFeatureSetting>> {
        unsafe { json_from_owned(bridge::ct_font_copy_feature_settings_json(self.raw)) }
    }

    /// Wraps `CTFontCopyAvailableTables`.
    pub fn available_tables(&self) -> CoreTextResult<Vec<u32>> {
        unsafe { json_from_owned(bridge::ct_font_copy_available_tables_json(self.raw)) }
    }

    /// Checks whether `CTFontCopyAvailableTables` includes a table tag.
    #[must_use]
    pub fn has_table(&self, tag: u32) -> bool {
        unsafe { bridge::ct_font_has_table(self.raw, tag) }
    }

    /// Wraps `CTFontCopyAttribute`.
    pub fn attribute_json(&self, attr: &str) -> CoreTextResult<serde_json::Value> {
        let attr = cstring(attr)?;
        unsafe {
            crate::common::json_from_owned(bridge::ct_font_copy_attribute_json(
                self.raw,
                attr.as_ptr(),
            ))
        }
    }

    /// Wraps `CTFontCopyDefaultCascadeListForLanguages`.
    pub fn default_cascade_list(&self, languages: &[&str]) -> CoreTextResult<Vec<FontDescriptor>> {
        let json = cstring(&serde_json::to_string(languages)?)?;
        let count =
            unsafe { bridge::ct_font_copy_default_cascade_list_count(self.raw, json.as_ptr()) };
        if count <= 0 {
            return Ok(Vec::new());
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_copy_default_cascade_list(
                self.raw,
                json.as_ptr(),
                handles.as_mut_ptr(),
                count,
            )
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        Ok(handles.into_iter().map(FontDescriptor::from_raw).collect())
    }

    /// Wraps `CTFontCopyTable`.
    pub fn table_data(&self, tag: u32) -> CoreTextResult<Vec<u8>> {
        if !self.has_table(tag) {
            return Err(CoreTextError::Bridge("font table not present".to_string()));
        }
        let mut len = 0_isize;
        let bytes = unsafe { bridge::ct_font_copy_table_bytes(self.raw, tag, &mut len) };
        if bytes.is_null() {
            return if len == 0 {
                Ok(Vec::new())
            } else {
                Err(CoreTextError::Bridge(
                    "ct_font_copy_table_bytes returned NULL".to_string(),
                ))
            };
        }
        let len = usize::try_from(len).unwrap_or(0);
        let data = unsafe { std::slice::from_raw_parts(bytes, len) }.to_vec();
        unsafe { libc::free(bytes.cast()) };
        Ok(data)
    }

    /// Wraps `CTFontCreateWithFontDescriptorAndOptions`.
    pub fn from_descriptor_with_options(
        descriptor: &FontDescriptor,
        size: f64,
        options: u32,
    ) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_font_create_with_descriptor_and_options(descriptor.as_raw(), size, options)
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_with_descriptor_and_options returned NULL",
        )?))
    }

    /// Wraps `CTFontCreateWithNameAndOptions`.
    pub fn with_name_and_options(name: &str, size: f64, options: u32) -> CoreTextResult<Self> {
        let name = cstring(name)?;
        let raw =
            unsafe { bridge::ct_font_create_with_name_and_options(name.as_ptr(), size, options) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_with_name_and_options returned NULL",
        )?))
    }

    /// Wraps `CTFontGetLigatureCaretPositions`.
    #[must_use]
    pub fn ligature_caret_positions(&self, glyph: u16) -> Vec<f64> {
        let count = unsafe {
            bridge::ct_font_get_ligature_caret_positions(self.raw, glyph, std::ptr::null_mut(), 0)
        };
        if count <= 0 {
            return Vec::new();
        }
        let mut positions = vec![0.0_f64; usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_get_ligature_caret_positions(
                self.raw,
                glyph,
                positions.as_mut_ptr(),
                count,
            )
        };
        positions.truncate(usize::try_from(written).unwrap_or(0));
        positions
    }

    /// Wraps `CTFontGetStringEncoding`.
    #[must_use]
    pub fn string_encoding(&self) -> u32 {
        unsafe { bridge::ct_font_get_string_encoding(self.raw) }
    }

    /// Wraps `CTFontGetTypographicBoundsForAdaptiveImageProvider`.
    #[must_use]
    pub fn typographic_bounds_for_adaptive_image_provider(
        &self,
        provider: Option<&AdaptiveImageProvider>,
    ) -> CGRect {
        unsafe {
            bridge::ct_font_get_typographic_bounds_for_adaptive_image_provider(
                self.raw,
                provider.map_or(std::ptr::null_mut(), AdaptiveImageProvider::as_raw),
            )
        }
    }

    /// Wraps `CTFontDrawImageFromAdaptiveImageProviderAtPoint`.
    pub fn draw_image_from_adaptive_image_provider_at_point(
        &self,
        provider: &AdaptiveImageProvider,
        point: CGPoint,
        context: &CGContext,
    ) {
        unsafe {
            bridge::ct_font_draw_image_from_adaptive_image_provider_at_point(
                self.raw,
                provider.as_raw(),
                point,
                context.as_ptr(),
            );
        }
    }
}

/// Wraps `CTFontGetTypeID`.
#[must_use]
pub fn font_type_id() -> u64 {
    unsafe { bridge::ct_font_get_type_id() }
}
