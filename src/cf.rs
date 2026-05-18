/// Internal CoreFoundation helpers — owned wrappers and string conversion.
use std::ffi::{CStr, CString};

use crate::error::{CoreTextError, CoreTextResult};
use crate::ffi;

// ── OwnedCFString ──────────────────────────────────────────────────────────

/// An owned `CFStringRef` that releases itself on drop.
pub(crate) struct OwnedCFString(ffi::CFStringRef);

impl OwnedCFString {
    /// Creates an owned Core Foundation string from Rust UTF-8 text.
    pub(crate) fn from_str(s: &str) -> CoreTextResult<Self> {
        let cs = CString::new(s).map_err(|_| CoreTextError::NulByte)?;
        // SAFETY: cs.as_ptr() is valid for the duration of the call; the result
        // is a new CFStringRef that we take ownership of and will release in Drop.
        let raw = unsafe {
            ffi::CFStringCreateWithCString(
                ffi::kCFAllocatorDefault,
                cs.as_ptr(),
                ffi::kCFStringEncodingUTF8,
            )
        };
        if raw.is_null() {
            Err(CoreTextError::Null(
                "CFStringCreateWithCString returned NULL",
            ))
        } else {
            Ok(Self(raw))
        }
    }

    /// Returns the wrapped raw `CFStringRef`.
    #[inline]
    pub(crate) const fn as_raw(&self) -> ffi::CFStringRef {
        self.0
    }
}

impl Drop for OwnedCFString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: self.0 is either NULL (checked above) or a valid CFStringRef
            // that we own and are releasing exactly once.
            unsafe { ffi::CFRelease(self.0) };
        }
    }
}

// ── String conversion ──────────────────────────────────────────────────────

/// Convert an owned (+1) `CFStringRef` to a `String`, releasing it afterwards.
pub(crate) fn cfstring_into_string(raw: ffi::CFStringRef) -> CoreTextResult<String> {
    if raw.is_null() {
        return Err(CoreTextError::Null("CFStringRef is NULL"));
    }
    let result = cfstring_borrow_to_string(raw);
    // SAFETY: raw is non-null (checked above) and is a valid CFStringRef that we
    // own and are releasing exactly once.
    unsafe { ffi::CFRelease(raw) };
    result
}

fn cfstring_borrow_to_string(raw: ffi::CFStringRef) -> CoreTextResult<String> {
    // SAFETY: raw is guaranteed non-null by the caller; CFStringGetLength is a
    // trivial read that doesn't borrow the pointer.
    let len = unsafe { ffi::CFStringGetLength(raw) };
    // SAFETY: CFStringGetMaximumSizeForEncoding takes a length and returns a
    // valid buffer size (isize).
    let max = unsafe { ffi::CFStringGetMaximumSizeForEncoding(len, ffi::kCFStringEncodingUTF8) };
    let cap = usize::try_from(max).unwrap_or(0).saturating_add(1).max(1);
    let mut buf = vec![0_i8; cap];
    // SAFETY: buf.as_mut_ptr() is valid for cap bytes; CFStringGetCString fills
    // the buffer with a UTF-8 string and returns 1 on success.
    let ok = unsafe {
        ffi::CFStringGetCString(
            raw,
            buf.as_mut_ptr(),
            isize::try_from(cap).unwrap_or(isize::MAX),
            ffi::kCFStringEncodingUTF8,
        )
    };
    if ok == 0 {
        return Err(CoreTextError::StringConversion);
    }
    // SAFETY: CFStringGetCString succeeded (ok == 1), so buf contains a valid
    // null-terminated UTF-8 string.
    let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
    Ok(String::from_utf8_lossy(cstr.to_bytes()).into_owned())
}
