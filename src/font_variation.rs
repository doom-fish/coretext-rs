use serde::Deserialize;

/// A single variation axis exposed by a variable font.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationAxis {
    pub identifier: u32,
    pub minimum_value: f64,
    pub maximum_value: f64,
    pub default_value: f64,
    pub name: String,
    pub hidden: bool,
}

/// A concrete variation coordinate applied to a variable font instance.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontVariationCoordinate {
    pub identifier: u32,
    pub value: f64,
}
