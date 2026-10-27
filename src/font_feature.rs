use crate::value::{AttributeValue, FromValue};
use crate::CoreTextResult;

/// A selector inside a CoreText feature type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFeatureSelector {
    /// Wraps the `identifier` value returned by `CTFontCopyFeatures`.
    pub identifier: i64,
    /// Wraps the `name` value returned by `CTFontCopyFeatures`.
    pub name: String,
    /// Wraps the `is_default` value returned by `CTFontCopyFeatures`.
    pub is_default: bool,
    /// Wraps the `is_enabled` value returned by `CTFontCopyFeatures`.
    pub is_enabled: bool,
    /// Wraps the `sample_text` value returned by `CTFontCopyFeatures`.
    pub sample_text: Option<String>,
    /// Wraps the `tooltip_text` value returned by `CTFontCopyFeatures`.
    pub tooltip_text: Option<String>,
    /// Wraps the `open_type_tag` value returned by `CTFontCopyFeatures`.
    pub open_type_tag: Option<String>,
    /// Wraps the `open_type_value` value returned by `CTFontCopyFeatures`.
    pub open_type_value: Option<i64>,
}

/// A CoreText feature type and its selectors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFeature {
    /// Wraps the `type_identifier` value returned by `CTFontCopyFeatures`.
    pub type_identifier: i64,
    /// Wraps the `name` value returned by `CTFontCopyFeatures`.
    pub name: String,
    /// Wraps the `exclusive` value returned by `CTFontCopyFeatures`.
    pub exclusive: bool,
    /// Wraps the `selectors` value returned by `CTFontCopyFeatures`.
    pub selectors: Vec<FontFeatureSelector>,
    /// Wraps the `open_type_tag` value returned by `CTFontCopyFeatures`.
    pub open_type_tag: Option<String>,
    /// Wraps the `open_type_value` value returned by `CTFontCopyFeatures`.
    pub open_type_value: Option<i64>,
    /// Wraps the `sample_text` value returned by `CTFontCopyFeatures`.
    pub sample_text: Option<String>,
    /// Wraps the `tooltip_text` value returned by `CTFontCopyFeatures`.
    pub tooltip_text: Option<String>,
}

/// A non-default feature setting tuple applied to a font or descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontFeatureSetting {
    /// Wraps the `type_identifier` value returned by `kCTFontFeatureSettingsAttribute`.
    pub type_identifier: i64,
    /// Wraps the `selector_identifier` value returned by `kCTFontFeatureSettingsAttribute`.
    pub selector_identifier: i64,
}

impl FromValue for FontFeatureSelector {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        Ok(Self {
            identifier: value.field("identifier")?,
            name: value.field("name")?,
            is_default: value.field("isDefault")?,
            is_enabled: value.field("isEnabled")?,
            sample_text: value.field("sampleText")?,
            tooltip_text: value.field("tooltipText")?,
            open_type_tag: value.field("openTypeTag")?,
            open_type_value: value.field("openTypeValue")?,
        })
    }
}

impl FromValue for FontFeature {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        Ok(Self {
            type_identifier: value.field("typeIdentifier")?,
            name: value.field("name")?,
            exclusive: value.field("exclusive")?,
            selectors: value.field("selectors")?,
            open_type_tag: value.field("openTypeTag")?,
            open_type_value: value.field("openTypeValue")?,
            sample_text: value.field("sampleText")?,
            tooltip_text: value.field("tooltipText")?,
        })
    }
}

impl FromValue for FontFeatureSetting {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        Ok(Self {
            type_identifier: value.field("typeIdentifier")?,
            selector_identifier: value.field("selectorIdentifier")?,
        })
    }
}
