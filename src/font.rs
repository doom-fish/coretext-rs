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
use crate::types::{CFRange, CGAffineTransform, CGRect, CGSize, TextRange};

/// Constants for `CTFontCopyName` / `CTFontCopyLocalizedName`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FontNameKey {
    Copyright = 0,
    Family = 1,
    SubFamily = 2,
    Style = 3,
    Unique = 4,
    Full = 5,
    Version = 6,
    PostScript = 7,
    Trademark = 8,
    Manufacturer = 9,
    Designer = 10,
    Description = 11,
    VendorUrl = 12,
    DesignerUrl = 13,
    License = 14,
    LicenseUrl = 15,
    SampleText = 16,
    PostScriptCid = 17,
}

/// UI font selectors for `CTFontCreateUIFontForLanguage`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum UIFontType {
    None = u32::MAX,
    User = 0,
    UserFixedPitch = 1,
    System = 2,
    EmphasizedSystem = 3,
    SmallSystem = 4,
    SmallEmphasizedSystem = 5,
    MiniSystem = 6,
    MiniEmphasizedSystem = 7,
    Views = 8,
    Application = 9,
    Label = 10,
    MenuTitle = 11,
    MenuItem = 12,
    MenuItemMark = 13,
    MenuItemCommandKey = 14,
    WindowTitle = 15,
    PushButton = 16,
    UtilityWindowTitle = 17,
    AlertHeader = 18,
    SystemDetail = 19,
    EmphasizedSystemDetail = 20,
    Toolbar = 21,
    SmallToolbar = 22,
    Message = 23,
    Palette = 24,
    ToolTip = 25,
    ControlContent = 26,
}

/// An immutable `CTFont` wrapper.
pub struct CTFont {
    raw: bridge::Handle,
}

impl_handle!(CTFont);

impl CTFont {
    pub fn new(name: &str, size: f64) -> CoreTextResult<Self> {
        let name = cstring(name)?;
        let raw = unsafe { bridge::ct_font_create_with_name(name.as_ptr(), size) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_with_name returned NULL",
        )?))
    }

    pub fn from_descriptor(descriptor: &FontDescriptor, size: f64) -> CoreTextResult<Self> {
        let raw = unsafe { bridge::ct_font_create_with_descriptor(descriptor.as_raw(), size) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_create_with_descriptor returned NULL",
        )?))
    }

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

    pub fn copy_with_family(&self, size: f64, family: &str) -> CoreTextResult<Self> {
        let family = cstring(family)?;
        let raw = unsafe { bridge::ct_font_copy_with_family(self.raw, size, family.as_ptr()) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_copy_with_family returned NULL",
        )?))
    }

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

    pub fn descriptor(&self) -> CoreTextResult<FontDescriptor> {
        let raw = unsafe { bridge::ct_font_copy_descriptor(self.raw) };
        Ok(FontDescriptor::from_raw(expect_handle(
            raw,
            "ct_font_copy_descriptor returned NULL",
        )?))
    }

    #[must_use]
    pub fn size(&self) -> f64 {
        unsafe { bridge::ct_font_get_size(self.raw) }
    }

    #[must_use]
    pub fn matrix(&self) -> CGAffineTransform {
        unsafe { bridge::ct_font_get_matrix(self.raw) }
    }

    #[must_use]
    pub fn symbolic_traits(&self) -> u32 {
        unsafe { bridge::ct_font_get_symbolic_traits(self.raw) }
    }

    pub fn traits(&self) -> CoreTextResult<FontTraits> {
        unsafe { json_from_owned(bridge::ct_font_copy_traits_json(self.raw)) }
    }

    pub fn postscript_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_postscript_name(self.raw)) }
    }

    pub fn family_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_family_name(self.raw)) }
    }

    pub fn full_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_full_name(self.raw)) }
    }

    pub fn display_name(&self) -> CoreTextResult<String> {
        unsafe { string_from_owned(bridge::ct_font_copy_display_name(self.raw)) }
    }

    #[must_use]
    pub fn name(&self, key: FontNameKey) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_copy_name(self.raw, key as u32)) }
    }

    pub fn localized_name(&self, key: FontNameKey) -> CoreTextResult<Option<String>> {
        Ok(unsafe {
            option_string_from_owned(bridge::ct_font_copy_localized_name(self.raw, key as u32))
        })
    }

    #[must_use]
    pub fn ascent(&self) -> f64 {
        unsafe { bridge::ct_font_get_ascent(self.raw) }
    }

    #[must_use]
    pub fn descent(&self) -> f64 {
        unsafe { bridge::ct_font_get_descent(self.raw) }
    }

    #[must_use]
    pub fn leading(&self) -> f64 {
        unsafe { bridge::ct_font_get_leading(self.raw) }
    }

    #[must_use]
    pub fn units_per_em(&self) -> u32 {
        unsafe { bridge::ct_font_get_units_per_em(self.raw) }
    }

    #[must_use]
    pub fn glyph_count(&self) -> isize {
        unsafe { bridge::ct_font_get_glyph_count(self.raw) }
    }

    #[must_use]
    pub fn bounding_box(&self) -> CGRect {
        unsafe { bridge::ct_font_get_bounding_box(self.raw) }
    }

    #[must_use]
    pub fn underline_position(&self) -> f64 {
        unsafe { bridge::ct_font_get_underline_position(self.raw) }
    }

    #[must_use]
    pub fn underline_thickness(&self) -> f64 {
        unsafe { bridge::ct_font_get_underline_thickness(self.raw) }
    }

    #[must_use]
    pub fn slant_angle(&self) -> f64 {
        unsafe { bridge::ct_font_get_slant_angle(self.raw) }
    }

    #[must_use]
    pub fn cap_height(&self) -> f64 {
        unsafe { bridge::ct_font_get_cap_height(self.raw) }
    }

    #[must_use]
    pub fn x_height(&self) -> f64 {
        unsafe { bridge::ct_font_get_x_height(self.raw) }
    }

    pub fn supported_languages(&self) -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_copy_supported_languages_json(self.raw)) }
    }

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

    pub fn glyph_with_name(&self, glyph_name: &str) -> CoreTextResult<u16> {
        let glyph_name = cstring(glyph_name)?;
        Ok(unsafe { bridge::ct_font_get_glyph_with_name(self.raw, glyph_name.as_ptr()) })
    }

    #[must_use]
    pub fn name_for_glyph(&self, glyph: u16) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_copy_name_for_glyph(self.raw, glyph)) }
    }

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

    pub fn variation_axes(&self) -> CoreTextResult<Vec<FontVariationAxis>> {
        unsafe { json_from_owned(bridge::ct_font_copy_variation_axes_json(self.raw)) }
    }

    pub fn variation_coordinates(&self) -> CoreTextResult<Vec<FontVariationCoordinate>> {
        unsafe { json_from_owned(bridge::ct_font_copy_variation_json(self.raw)) }
    }

    pub fn features(&self) -> CoreTextResult<Vec<FontFeature>> {
        unsafe { json_from_owned(bridge::ct_font_copy_features_json(self.raw)) }
    }

    pub fn feature_settings(&self) -> CoreTextResult<Vec<FontFeatureSetting>> {
        unsafe { json_from_owned(bridge::ct_font_copy_feature_settings_json(self.raw)) }
    }

    pub fn available_tables(&self) -> CoreTextResult<Vec<u32>> {
        unsafe { json_from_owned(bridge::ct_font_copy_available_tables_json(self.raw)) }
    }

    #[must_use]
    pub fn has_table(&self, tag: u32) -> bool {
        unsafe { bridge::ct_font_has_table(self.raw, tag) }
    }
}
