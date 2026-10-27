use std::marker::PhantomData;

use crate::value::{AttributeValue, FromValue};
use crate::{CoreTextError, CoreTextResult};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bytes {
    pub data: *const u8,
    pub len: usize,
}

impl Bytes {
    pub(crate) const fn new(value: &[u8]) -> Self {
        Self {
            data: value.as_ptr(),
            len: value.len(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StringList {
    pub data: *const Bytes,
    pub len: usize,
}

pub struct Strings<'a> {
    values: Vec<Bytes>,
    lifetime: PhantomData<&'a str>,
}

impl<'a> Strings<'a> {
    pub(crate) fn new(values: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            values: values
                .into_iter()
                .map(|value| Bytes::new(value.as_bytes()))
                .collect(),
            lifetime: PhantomData,
        }
    }

    pub(crate) fn as_raw(&self) -> StringList {
        StringList {
            data: self.values.as_ptr(),
            len: self.values.len(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(clippy::struct_field_names)]
pub struct Value {
    pub tag: u32,
    pub signed_value: i64,
    pub unsigned_value: u64,
    pub float_value: f64,
    pub bytes: Bytes,
    pub elements: *mut Self,
    pub keys: *mut Bytes,
    pub count: usize,
}

impl Value {
    const fn empty(tag: u32) -> Self {
        Self {
            tag,
            signed_value: 0,
            unsigned_value: 0,
            float_value: 0.0,
            bytes: Bytes {
                data: std::ptr::null(),
                len: 0,
            },
            elements: std::ptr::null_mut(),
            keys: std::ptr::null_mut(),
            count: 0,
        }
    }

    unsafe fn decode(&self, depth: usize) -> CoreTextResult<AttributeValue> {
        if depth > 64 {
            return Err(CoreTextError::InvalidBridgeValue);
        }
        Ok(match self.tag {
            0 => AttributeValue::Null,
            1 => AttributeValue::Bool(self.unsigned_value != 0),
            2 => AttributeValue::Signed(self.signed_value),
            3 => AttributeValue::Unsigned(self.unsigned_value),
            4 => AttributeValue::Float(self.float_value),
            5 => AttributeValue::String(
                String::from_utf8(slice(self.bytes.data, self.bytes.len)?.to_vec())
                    .map_err(|_| CoreTextError::StringConversion)?,
            ),
            6 => AttributeValue::Data(slice(self.bytes.data, self.bytes.len)?.to_vec()),
            7 | 8 => {
                let values = slice(self.elements, self.count)?
                    .iter()
                    .map(|value| value.decode(depth + 1))
                    .collect::<CoreTextResult<Vec<_>>>()?;
                if self.tag == 7 {
                    AttributeValue::Array(values)
                } else {
                    let keys = slice(self.keys, self.count)?;
                    let entries = keys
                        .iter()
                        .zip(values)
                        .map(|(key, value)| {
                            let key = String::from_utf8(slice(key.data, key.len)?.to_vec())
                                .map_err(|_| CoreTextError::StringConversion)?;
                            Ok((key, value))
                        })
                        .collect::<CoreTextResult<_>>()?;
                    AttributeValue::Dictionary(entries)
                }
            }
            _ => return Err(CoreTextError::InvalidBridgeValue),
        })
    }
}

unsafe fn slice<'a, T>(data: *const T, len: usize) -> CoreTextResult<&'a [T]> {
    if len == 0 {
        return Ok(&[]);
    }
    if data.is_null() || len > isize::MAX as usize / std::mem::size_of::<T>() {
        return Err(CoreTextError::InvalidBridgeValue);
    }
    Ok(std::slice::from_raw_parts(data, len))
}

pub struct InputValue<'a> {
    value: Value,
    elements: Vec<Vec<Value>>,
    keys: Vec<Vec<Bytes>>,
    lifetime: PhantomData<&'a AttributeValue>,
}

impl<'a> InputValue<'a> {
    pub(crate) fn new(value: &'a AttributeValue) -> CoreTextResult<Self> {
        let mut input = Self {
            value: Value::empty(0),
            elements: Vec::new(),
            keys: Vec::new(),
            lifetime: PhantomData,
        };
        input.value = input.encode(value, 0)?;
        Ok(input)
    }

    fn encode(&mut self, value: &AttributeValue, depth: usize) -> CoreTextResult<Value> {
        if depth > 64 {
            return Err(CoreTextError::InvalidBridgeValue);
        }
        let mut raw = Value::empty(0);
        match value {
            AttributeValue::Null => {}
            AttributeValue::Bool(value) => {
                raw.tag = 1;
                raw.unsigned_value = u64::from(*value);
            }
            AttributeValue::Signed(value) => {
                raw.tag = 2;
                raw.signed_value = *value;
            }
            AttributeValue::Unsigned(value) => {
                raw.tag = 3;
                raw.unsigned_value = *value;
            }
            AttributeValue::Float(value) => {
                raw.tag = 4;
                raw.float_value = *value;
            }
            AttributeValue::String(value) => {
                raw.tag = 5;
                raw.bytes = Bytes::new(value.as_bytes());
            }
            AttributeValue::Data(value) => {
                raw.tag = 6;
                raw.bytes = Bytes::new(value);
            }
            AttributeValue::Array(values) => {
                raw.tag = 7;
                let mut elements: Vec<_> = values
                    .iter()
                    .map(|value| self.encode(value, depth + 1))
                    .collect::<CoreTextResult<_>>()?;
                raw.elements = elements.as_mut_ptr();
                raw.count = elements.len();
                self.elements.push(elements);
            }
            AttributeValue::Dictionary(values) => {
                raw.tag = 8;
                let mut elements: Vec<_> = values
                    .values()
                    .map(|value| self.encode(value, depth + 1))
                    .collect::<CoreTextResult<_>>()?;
                let mut keys: Vec<_> = values
                    .keys()
                    .map(|key| Bytes::new(key.as_bytes()))
                    .collect();
                raw.elements = elements.as_mut_ptr();
                raw.keys = keys.as_mut_ptr();
                raw.count = elements.len();
                self.elements.push(elements);
                self.keys.push(keys);
            }
        }
        Ok(raw)
    }

    pub(crate) const fn as_ptr(&self) -> *const Value {
        &raw const self.value
    }
}

pub unsafe fn from_owned<T: FromValue>(raw: *mut Value) -> CoreTextResult<T> {
    struct Owned(*mut Value);

    impl Drop for Owned {
        fn drop(&mut self) {
            unsafe { ct_value_release(self.0) };
        }
    }

    if raw.is_null() {
        return Err(CoreTextError::InvalidBridgeValue);
    }
    let owned = Owned(raw);
    T::from_value(&(*owned.0).decode(0)?)
}

#[repr(C)]
pub struct Feature {
    pub tag: Bytes,
    pub value: u32,
}

#[repr(C)]
pub struct Features {
    pub data: *const Feature,
    pub len: usize,
}

#[repr(C)]
pub struct TypesetterOptions {
    pub allow_unbounded_layout: bool,
    pub has_forced_embedding_level: bool,
    pub forced_embedding_level: i64,
}

#[repr(C)]
pub struct RubyOptions {
    pub alignment: u8,
    pub overhang: u8,
    pub size_factor: f64,
    pub texts: StringList,
}

#[repr(C)]
pub struct TextRun {
    pub text: Bytes,
    pub font: *mut core::ffi::c_void,
}

unsafe extern "C" {
    fn ct_value_release(value: *mut Value);
}
