use serde::Deserialize;

/// A selector inside a CoreText feature type.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontFeatureSelector {
    pub identifier: i64,
    pub name: String,
    pub is_default: bool,
    pub is_enabled: bool,
    pub sample_text: Option<String>,
    pub tooltip_text: Option<String>,
    pub open_type_tag: Option<String>,
    pub open_type_value: Option<i64>,
}

/// A CoreText feature type and its selectors.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontFeature {
    pub type_identifier: i64,
    pub name: String,
    pub exclusive: bool,
    pub selectors: Vec<FontFeatureSelector>,
    pub open_type_tag: Option<String>,
    pub open_type_value: Option<i64>,
    pub sample_text: Option<String>,
    pub tooltip_text: Option<String>,
}

/// A non-default feature setting tuple applied to a font or descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontFeatureSetting {
    pub type_identifier: i64,
    pub selector_identifier: i64,
}
