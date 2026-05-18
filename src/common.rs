//! Internal bridge helpers for string conversion, handle validation, and retained-handle wrappers.

use std::ffi::{CStr, CString};

use serde::de::DeserializeOwned;

use crate::bridge::{self, Handle};
use crate::error::{CoreTextError, CoreTextResult};

/// Converts Rust text into a bridge-safe `CString`.
pub fn cstring(value: &str) -> CoreTextResult<CString> {
    CString::new(value).map_err(|_| CoreTextError::NulByte)
}

/// Converts optional Rust text into an optional bridge-safe `CString`.
pub fn optional_cstring(value: Option<&str>) -> CoreTextResult<Option<CString>> {
    value.map(cstring).transpose()
}

/// Validates that a bridge call returned a non-null opaque handle.
pub fn expect_handle(raw: Handle, context: &'static str) -> CoreTextResult<Handle> {
    if raw.is_null() {
        Err(CoreTextError::Null(context))
    } else {
        Ok(raw)
    }
}

/// Converts an owned bridge string into a Rust `String` and releases the original buffer.
pub unsafe fn string_from_owned(ptr: *mut libc::c_char) -> CoreTextResult<String> {
    if ptr.is_null() {
        return Err(CoreTextError::StringConversion);
    }
    // SAFETY: ptr is non-null and points to a valid null-terminated UTF-8 string
    // allocated by the bridge. We take ownership and release it via ct_string_release.
    let value = CStr::from_ptr(ptr)
        .to_str()
        .map_err(|_| CoreTextError::StringConversion)?
        .to_owned();
    bridge::ct_string_release(ptr);
    Ok(value)
}

/// Converts an owned nullable bridge string into an optional Rust `String`.
pub unsafe fn option_string_from_owned(ptr: *mut libc::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    // SAFETY: ptr is non-null and points to a valid null-terminated UTF-8 string
    // allocated by the bridge. We take ownership and release it via ct_string_release.
    let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    bridge::ct_string_release(ptr);
    Some(value)
}

/// Deserializes owned bridge JSON into the requested Rust type.
pub unsafe fn json_from_owned<T>(ptr: *mut libc::c_char) -> CoreTextResult<T>
where
    T: DeserializeOwned,
{
    let json = string_from_owned(ptr)?;
    Ok(serde_json::from_str(&json)?)
}

macro_rules! impl_handle {
    ($name:ident) => {
        // SAFETY: All $name types wrap opaque Apple framework handles (raw pointers)
        // that are thread-safe and can be sent across threads. The underlying Objective-C
        // objects are managed via reference counting (retain/release), which is atomic.
        /// Marks this retained wrapper as safe to move across threads.
        unsafe impl Send for $name {}
        /// Marks this retained wrapper as safe to share across threads.
        unsafe impl Sync for $name {}

        impl $name {
            /// Wraps an owned bridge handle without changing its retain count.
            #[allow(dead_code)]
            #[inline]
            pub const fn from_raw(raw: crate::bridge::Handle) -> Self {
                Self { raw }
            }

            /// Returns the underlying opaque bridge handle.
            #[inline]
            pub fn as_raw(&self) -> crate::bridge::Handle {
                self.raw
            }
        }

        /// Clones the wrapper by retaining the underlying bridge handle.
        impl Clone for $name {
            fn clone(&self) -> Self {
                if self.raw.is_null() {
                    Self { raw: self.raw }
                } else {
                    Self {
                        // SAFETY: self.raw is non-null (checked above) and is a valid
                        // handle that we own. ct_retain increments the retain count and
                        // returns the same handle.
                        raw: unsafe { crate::bridge::ct_retain(self.raw) },
                    }
                }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                if !self.raw.is_null() {
                    // SAFETY: self.raw is either NULL (checked above) or a valid handle
                    // that we own and are releasing exactly once.
                    unsafe { crate::bridge::ct_release(self.raw) };
                }
            }
        }
    };
}

pub(crate) use impl_handle;
