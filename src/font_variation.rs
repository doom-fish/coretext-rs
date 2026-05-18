use serde::Deserialize;

/// A single variation axis exposed by a variable font.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationAxis {
    /// Wraps the `identifier` value returned by `CTFontCopyVariationAxes`.
    pub identifier: u32,
    /// Wraps the `minimum_value` value returned by `CTFontCopyVariationAxes`.
    pub minimum_value: f64,
    /// Wraps the `maximum_value` value returned by `CTFontCopyVariationAxes`.
    pub maximum_value: f64,
    /// Wraps the `default_value` value returned by `CTFontCopyVariationAxes`.
    pub default_value: f64,
    /// Wraps the `name` value returned by `CTFontCopyVariationAxes`.
    pub name: String,
    /// Wraps the `hidden` value returned by `CTFontCopyVariationAxes`.
    pub hidden: bool,
}

/// A concrete variation coordinate applied to a variable font instance.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationCoordinate {
    /// Wraps the `identifier` value returned by `CTFontCopyVariation`.
    pub identifier: u32,
    /// Wraps the `value` value returned by `CTFontCopyVariation`.
    pub value: f64,
}
