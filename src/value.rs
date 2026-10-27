use std::collections::BTreeMap;
use std::ops::Index;

use crate::{CoreTextError, CoreTextResult};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AttributeValue {
    #[default]
    Null,
    Bool(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    String(String),
    Data(Vec<u8>),
    Array(Vec<Self>),
    Dictionary(BTreeMap<String, Self>),
}

impl AttributeValue {
    pub fn dictionary<const N: usize>(entries: [(&str, Self); N]) -> Self {
        Self::Dictionary(
            entries
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value))
                .collect(),
        )
    }

    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Dictionary(_))
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            Self::Signed(value) => u64::try_from(*value).ok(),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Signed(value) => Some(*value),
            Self::Unsigned(value) => i64::try_from(*value).ok(),
            _ => None,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            Self::Signed(value) => Some(*value as f64),
            Self::Unsigned(value) => Some(*value as f64),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(values) => Some(values),
            _ => None,
        }
    }

    pub const fn as_object(&self) -> Option<&BTreeMap<String, Self>> {
        match self {
            Self::Dictionary(values) => Some(values),
            _ => None,
        }
    }

    pub(crate) fn field<T: FromValue>(&self, name: &str) -> CoreTextResult<T> {
        T::from_value(&self[name])
    }
}

impl Index<&str> for AttributeValue {
    type Output = Self;

    fn index(&self, key: &str) -> &Self::Output {
        self.as_object()
            .and_then(|values| values.get(key))
            .unwrap_or(&Self::Null)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

pub(crate) trait FromValue: Sized {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self>;
}

impl FromValue for AttributeValue {
    fn from_value(value: &Self) -> CoreTextResult<Self> {
        Ok(value.clone())
    }
}

impl FromValue for String {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        value
            .as_str()
            .map(str::to_owned)
            .ok_or(CoreTextError::InvalidBridgeValue)
    }
}

impl FromValue for bool {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        value.as_bool().ok_or(CoreTextError::InvalidBridgeValue)
    }
}

impl FromValue for i64 {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        value.as_i64().ok_or(CoreTextError::InvalidBridgeValue)
    }
}

impl FromValue for u32 {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        value
            .as_u64()
            .and_then(|value| Self::try_from(value).ok())
            .ok_or(CoreTextError::InvalidBridgeValue)
    }
}

impl FromValue for f64 {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        value.as_f64().ok_or(CoreTextError::InvalidBridgeValue)
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        value
            .as_array()
            .ok_or(CoreTextError::InvalidBridgeValue)?
            .iter()
            .map(T::from_value)
            .collect()
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: &AttributeValue) -> CoreTextResult<Self> {
        if value.is_null() {
            Ok(None)
        } else {
            T::from_value(value).map(Some)
        }
    }
}
