use serde_json::Value;

use crate::bridge;
use crate::common::{
    cstring, expect_handle, impl_handle, json_from_owned, option_string_from_owned,
};
use crate::error::CoreTextResult;
use crate::font_feature::{FontFeature, FontFeatureSetting};
use crate::font_traits::FontTraits;
use crate::font_variation::{FontVariationAxis, FontVariationCoordinate};

/// Intended rendering orientation of a font when accessing glyph metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum FontOrientation {
    #[default]
    Default = 0,
    Horizontal = 1,
    Vertical = 2,
}

impl FontOrientation {
    pub(crate) const fn from_raw(raw: u32) -> Self {
        match raw {
            1 => Self::Horizontal,
            2 => Self::Vertical,
            _ => Self::Default,
        }
    }
}

/// Recognized format of a CoreText font descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum FontFormat {
    #[default]
    Unrecognized = 0,
    OpenTypePostScript = 1,
    OpenTypeTrueType = 2,
    TrueType = 3,
    PostScript = 4,
    Bitmap = 5,
}

impl FontFormat {
    pub(crate) const fn from_raw(raw: u32) -> Self {
        match raw {
            1 => Self::OpenTypePostScript,
            2 => Self::OpenTypeTrueType,
            3 => Self::TrueType,
            4 => Self::PostScript,
            5 => Self::Bitmap,
            _ => Self::Unrecognized,
        }
    }
}

/// An immutable `CTFontDescriptor` wrapper.
pub struct FontDescriptor {
    raw: bridge::Handle,
}

impl_handle!(FontDescriptor);

impl FontDescriptor {
    pub fn new(name: &str, size: f64) -> CoreTextResult<Self> {
        let name = cstring(name)?;
        let raw = unsafe { bridge::ct_font_descriptor_create(name.as_ptr(), size) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_descriptor_create returned NULL",
        )?))
    }

    pub fn with_family(&self, family: &str) -> CoreTextResult<Self> {
        let family = cstring(family)?;
        let raw = unsafe { bridge::ct_font_descriptor_copy_with_family(self.raw, family.as_ptr()) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_descriptor_copy_with_family returned NULL",
        )?))
    }

    pub fn with_symbolic_traits(&self, trait_value: u32, trait_mask: u32) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_font_descriptor_copy_with_symbolic_traits(self.raw, trait_value, trait_mask)
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_descriptor_copy_with_symbolic_traits returned NULL",
        )?))
    }

    pub fn with_variation(
        &self,
        variation_identifier: u32,
        variation_value: f64,
    ) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_font_descriptor_copy_with_variation(
                self.raw,
                variation_identifier,
                variation_value,
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_descriptor_copy_with_variation returned NULL",
        )?))
    }

    pub fn with_feature(
        &self,
        feature_type_identifier: i64,
        feature_selector_identifier: i64,
    ) -> CoreTextResult<Self> {
        let raw = unsafe {
            bridge::ct_font_descriptor_copy_with_feature(
                self.raw,
                feature_type_identifier,
                feature_selector_identifier,
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_descriptor_copy_with_feature returned NULL",
        )?))
    }

    pub fn matching_descriptor(&self) -> Option<Self> {
        let raw = unsafe { bridge::ct_font_descriptor_create_matching_descriptor(self.raw) };
        if raw.is_null() {
            None
        } else {
            Some(Self::from_raw(raw))
        }
    }

    #[must_use]
    pub fn matching_descriptors(&self) -> Vec<Self> {
        let count = unsafe { bridge::ct_font_descriptor_get_matching_descriptor_count(self.raw) };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_descriptor_copy_matching_descriptors(
                self.raw,
                handles.as_mut_ptr(),
                count,
            )
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(Self::from_raw).collect()
    }

    #[must_use]
    pub fn postscript_name(&self) -> Option<String> {
        unsafe {
            option_string_from_owned(bridge::ct_font_descriptor_copy_postscript_name(self.raw))
        }
    }

    #[must_use]
    pub fn display_name(&self) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_descriptor_copy_display_name(self.raw)) }
    }

    #[must_use]
    pub fn family_name(&self) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_descriptor_copy_family_name(self.raw)) }
    }

    #[must_use]
    pub fn style_name(&self) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_descriptor_copy_style_name(self.raw)) }
    }

    #[must_use]
    pub fn url_path(&self) -> Option<String> {
        unsafe { option_string_from_owned(bridge::ct_font_descriptor_copy_url_path(self.raw)) }
    }

    #[must_use]
    pub fn size(&self) -> f64 {
        unsafe { bridge::ct_font_descriptor_get_size(self.raw) }
    }

    #[must_use]
    pub fn orientation(&self) -> FontOrientation {
        FontOrientation::from_raw(unsafe { bridge::ct_font_descriptor_get_orientation(self.raw) })
    }

    #[must_use]
    pub fn format(&self) -> FontFormat {
        FontFormat::from_raw(unsafe { bridge::ct_font_descriptor_get_format(self.raw) })
    }

    #[must_use]
    pub fn is_enabled(&self) -> bool {
        unsafe { bridge::ct_font_descriptor_is_enabled(self.raw) }
    }

    #[must_use]
    pub fn is_downloadable(&self) -> bool {
        unsafe { bridge::ct_font_descriptor_is_downloadable(self.raw) }
    }

    pub fn traits(&self) -> CoreTextResult<FontTraits> {
        unsafe { json_from_owned(bridge::ct_font_descriptor_copy_traits_json(self.raw)) }
    }

    pub fn variation_axes(&self) -> CoreTextResult<Vec<FontVariationAxis>> {
        unsafe {
            json_from_owned(bridge::ct_font_descriptor_copy_variation_axes_json(
                self.raw,
            ))
        }
    }

    pub fn variation_coordinates(&self) -> CoreTextResult<Vec<FontVariationCoordinate>> {
        unsafe { json_from_owned(bridge::ct_font_descriptor_copy_variation_json(self.raw)) }
    }

    pub fn features(&self) -> CoreTextResult<Vec<FontFeature>> {
        unsafe { json_from_owned(bridge::ct_font_descriptor_copy_features_json(self.raw)) }
    }

    pub fn feature_settings(&self) -> CoreTextResult<Vec<FontFeatureSetting>> {
        unsafe {
            json_from_owned(bridge::ct_font_descriptor_copy_feature_settings_json(
                self.raw,
            ))
        }
    }

    pub fn attributes_json(&self) -> CoreTextResult<Value> {
        unsafe { json_from_owned(bridge::ct_font_descriptor_copy_attributes_json(self.raw)) }
    }
}
