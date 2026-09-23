use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::bridge;
use crate::common::{cstring, json_from_owned, option_string_from_owned};
use crate::error::{CoreTextError, CoreTextResult};
use crate::font_descriptor::FontDescriptor;

/// Registration scope for CoreText font manager operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum FontManagerScope {
    /// Selects the none case of `CTFontManagerScope`.
    #[default]
    None = 0,
    /// Selects the process case of `CTFontManagerScope`.
    Process = 1,
    /// Selects the persistent case of `CTFontManagerScope`.
    Persistent = 2,
    /// Selects the session case of `CTFontManagerScope`.
    Session = 3,
}

impl FontManagerScope {
    pub(crate) const fn from_raw(raw: u32) -> Self {
        match raw {
            1 => Self::Process,
            2 => Self::Persistent,
            3 => Self::Session,
            _ => Self::None,
        }
    }
}

/// Auto-activation preference for font manager lookups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum AutoActivationSetting {
    /// Selects the default case of `CTFontManagerAutoActivationSetting`.
    #[default]
    Default = 0,
    /// Selects the disabled case of `CTFontManagerAutoActivationSetting`.
    Disabled = 1,
    /// Selects the enabled case of `CTFontManagerAutoActivationSetting`.
    Enabled = 2,
    /// Selects the prompt user case of `CTFontManagerAutoActivationSetting`.
    PromptUser = 3,
}

impl AutoActivationSetting {
    pub(crate) const fn from_raw(raw: u32) -> Self {
        match raw {
            1 => Self::Disabled,
            2 => Self::Enabled,
            3 => Self::PromptUser,
            _ => Self::Default,
        }
    }
}

const WAIT_FOREVER: u64 = u64::MAX;

static REGISTRATION_TIMEOUT_NANOSECONDS: AtomicU64 = AtomicU64::new(30_000_000_000);

/// Namespace for CoreText font manager APIs.
pub struct FontManager;

impl FontManager {
    pub fn set_registration_timeout(timeout: Option<Duration>) {
        let nanoseconds = timeout.map_or(WAIT_FOREVER, |timeout| {
            u64::try_from(timeout.as_nanos())
                .map_or(WAIT_FOREVER - 1, |value| value.min(WAIT_FOREVER - 1))
        });
        REGISTRATION_TIMEOUT_NANOSECONDS.store(nanoseconds, Ordering::Relaxed);
    }

    #[must_use]
    pub fn registration_timeout() -> Option<Duration> {
        let nanoseconds = REGISTRATION_TIMEOUT_NANOSECONDS.load(Ordering::Relaxed);
        (nanoseconds != WAIT_FOREVER).then(|| Duration::from_nanos(nanoseconds))
    }

    /// Wraps `CTFontManagerCopyAvailablePostScriptNames`.
    pub fn available_postscript_names() -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_manager_copy_available_postscript_names_json()) }
    }

    /// Wraps `CTFontManagerCopyAvailableFontFamilyNames`.
    pub fn available_font_family_names() -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_manager_copy_available_font_family_names_json()) }
    }

    /// Wraps `CTFontManagerCopyAvailableFontURLs`.
    pub fn available_font_urls() -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_manager_copy_available_font_urls_json()) }
    }

    /// Wraps `CTFontManagerCreateFontDescriptorsFromURL`.
    pub fn font_descriptors_from_url(
        path: impl AsRef<Path>,
    ) -> CoreTextResult<Vec<FontDescriptor>> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        let count = unsafe { bridge::ct_font_manager_get_descriptor_count_for_url(path.as_ptr()) };
        if count <= 0 {
            return Ok(Vec::new());
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_manager_copy_descriptors_from_url(
                path.as_ptr(),
                handles.as_mut_ptr(),
                count,
            )
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        Ok(handles.into_iter().map(FontDescriptor::from_raw).collect())
    }

    /// Wraps `CTFontManagerIsSupportedFont`.
    pub fn is_supported_font(path: impl AsRef<Path>) -> CoreTextResult<bool> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        Ok(unsafe { bridge::ct_font_manager_is_supported_font(path.as_ptr()) })
    }

    /// Wraps `CTFontManagerRegisterFontsForURL`.
    pub fn register_fonts_for_url(
        path: impl AsRef<Path>,
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            bridge::ct_font_manager_register_fonts_for_url(
                path.as_ptr(),
                scope as u32,
                &raw mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(CoreTextError::Bridge(
                unsafe { option_string_from_owned(error) }
                    .unwrap_or_else(|| "font registration failed".to_string()),
            ))
        }
    }

    /// Wraps `CTFontManagerUnregisterFontsForURL`.
    pub fn unregister_fonts_for_url(
        path: impl AsRef<Path>,
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            bridge::ct_font_manager_unregister_fonts_for_url(
                path.as_ptr(),
                scope as u32,
                &raw mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(CoreTextError::Bridge(
                unsafe { option_string_from_owned(error) }
                    .unwrap_or_else(|| "font unregistration failed".to_string()),
            ))
        }
    }

    /// Wraps `CTFontManagerGetScopeForURL`.
    pub fn scope_for_url(path: impl AsRef<Path>) -> CoreTextResult<FontManagerScope> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        Ok(FontManagerScope::from_raw(unsafe {
            bridge::ct_font_manager_get_scope_for_url(path.as_ptr())
        }))
    }

    /// Wraps `CTFontManagerGetAutoActivationSetting`.
    #[must_use]
    pub fn auto_activation_setting(bundle_identifier: Option<&str>) -> AutoActivationSetting {
        let bundle_identifier = bundle_identifier.and_then(|value| cstring(value).ok());
        AutoActivationSetting::from_raw(unsafe {
            bridge::ct_font_manager_get_auto_activation_setting(
                bundle_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        })
    }

    /// Wraps `CTFontManagerSetAutoActivationSetting`.
    pub fn set_auto_activation_setting(
        bundle_identifier: Option<&str>,
        setting: AutoActivationSetting,
    ) {
        let bundle_identifier = bundle_identifier.and_then(|value| cstring(value).ok());
        unsafe {
            bridge::ct_font_manager_set_auto_activation_setting(
                bundle_identifier
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                setting as u32,
            );
        }
    }

    /// Wraps the registered-descriptor query surface in `CTFontManager`.
    #[must_use]
    pub fn registered_font_descriptors(
        scope: FontManagerScope,
        enabled: bool,
    ) -> Vec<FontDescriptor> {
        let count = unsafe {
            bridge::ct_font_manager_copy_registered_descriptor_count(scope as u32, enabled)
        };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_manager_copy_registered_descriptors(
                scope as u32,
                enabled,
                handles.as_mut_ptr(),
                count,
            )
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(FontDescriptor::from_raw).collect()
    }

    /// Wraps `CTFontManagerCreateDescriptorFromData`.
    pub fn font_descriptor_from_data(data: &[u8]) -> CoreTextResult<FontDescriptor> {
        if data.is_empty() {
            return Err(CoreTextError::Bridge("font data is empty".to_string()));
        }
        let raw = unsafe {
            bridge::ct_font_manager_create_descriptor_from_data(
                data.as_ptr(),
                isize::try_from(data.len()).unwrap_or(isize::MAX),
            )
        };
        if raw.is_null() {
            Err(CoreTextError::Bridge(
                "no font descriptor could be created from data".to_string(),
            ))
        } else {
            Ok(FontDescriptor::from_raw(raw))
        }
    }

    /// Wraps `CTFontManagerCreateFontDescriptorsFromData`.
    #[must_use]
    pub fn font_descriptors_from_data(data: &[u8]) -> Vec<FontDescriptor> {
        if data.is_empty() {
            return Vec::new();
        }
        let count = unsafe {
            bridge::ct_font_manager_create_descriptors_from_data_count(
                data.as_ptr(),
                isize::try_from(data.len()).unwrap_or(isize::MAX),
            )
        };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_manager_create_descriptors_from_data(
                data.as_ptr(),
                isize::try_from(data.len()).unwrap_or(isize::MAX),
                handles.as_mut_ptr(),
                count,
            )
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(FontDescriptor::from_raw).collect()
    }

    /// Wraps `CTFontManagerEnableFontDescriptors`.
    pub fn enable_font_descriptors(descriptors: &[FontDescriptor], enable: bool) {
        let handles: Vec<_> = descriptors.iter().map(FontDescriptor::as_raw).collect();
        unsafe {
            bridge::ct_font_manager_enable_font_descriptors(
                handles.as_ptr(),
                isize::try_from(handles.len()).unwrap_or(isize::MAX),
                enable,
            );
        }
    }

    /// Wraps `CTFontManagerRegisterFontDescriptors`.
    pub fn register_font_descriptors(
        descriptors: &[FontDescriptor],
        scope: FontManagerScope,
        enabled: bool,
    ) -> CoreTextResult<()> {
        if descriptors.is_empty() {
            return Ok(());
        }
        let handles: Vec<_> = descriptors.iter().map(FontDescriptor::as_raw).collect();
        registration_result(
            "font descriptor registration failed",
            |timeout, timed_out| unsafe {
                bridge::ct_font_manager_register_font_descriptors(
                    handles.as_ptr(),
                    isize::try_from(handles.len()).unwrap_or(isize::MAX),
                    scope as u32,
                    enabled,
                    timeout,
                    timed_out,
                )
            },
        )
    }

    /// Wraps `CTFontManagerRegisterFontURLs`.
    pub fn register_font_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
        enabled: bool,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        registration_result(
            "font URL registration failed",
            |timeout, timed_out| unsafe {
                bridge::ct_font_manager_register_font_urls(
                    paths.as_ptr(),
                    scope as u32,
                    enabled,
                    timeout,
                    timed_out,
                )
            },
        )
    }

    /// Wraps `CTFontManagerRegisterFontsForURLs`.
    pub fn register_fonts_for_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        let timeout = REGISTRATION_TIMEOUT_NANOSECONDS.load(Ordering::Relaxed);
        let mut timed_out = false;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            bridge::ct_font_manager_register_fonts_for_urls(
                paths.as_ptr(),
                scope as u32,
                timeout,
                &raw mut timed_out,
                &raw mut error,
            )
        };
        bool_registration_result(
            ok,
            timed_out,
            timeout,
            error,
            "font URL registration failed",
        )
    }

    /// Wraps `CTFontManagerRegisterFontsWithAssetNames`.
    pub fn register_fonts_with_asset_names(
        names: &[&str],
        scope: FontManagerScope,
        enabled: bool,
    ) -> CoreTextResult<()> {
        if names.is_empty() {
            return Ok(());
        }
        let names = cstring(&serde_json::to_string(names)?)?;
        let ok = unsafe {
            bridge::ct_font_manager_register_fonts_with_asset_names(
                names.as_ptr(),
                scope as u32,
                enabled,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(CoreTextError::Bridge(
                "font asset registration is unavailable or failed".to_string(),
            ))
        }
    }

    /// Wraps `CTFontManagerUnregisterFontDescriptors`.
    pub fn unregister_font_descriptors(
        descriptors: &[FontDescriptor],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if descriptors.is_empty() {
            return Ok(());
        }
        let handles: Vec<_> = descriptors.iter().map(FontDescriptor::as_raw).collect();
        registration_result(
            "font descriptor unregistration failed",
            |timeout, timed_out| unsafe {
                bridge::ct_font_manager_unregister_font_descriptors(
                    handles.as_ptr(),
                    isize::try_from(handles.len()).unwrap_or(isize::MAX),
                    scope as u32,
                    timeout,
                    timed_out,
                )
            },
        )
    }

    /// Wraps `CTFontManagerUnregisterFontURLs`.
    pub fn unregister_font_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        registration_result(
            "font URL unregistration failed",
            |timeout, timed_out| unsafe {
                bridge::ct_font_manager_unregister_font_urls(
                    paths.as_ptr(),
                    scope as u32,
                    timeout,
                    timed_out,
                )
            },
        )
    }

    /// Wraps `CTFontManagerUnregisterFontsForURLs`.
    pub fn unregister_fonts_for_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        let timeout = REGISTRATION_TIMEOUT_NANOSECONDS.load(Ordering::Relaxed);
        let mut timed_out = false;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            bridge::ct_font_manager_unregister_fonts_for_urls(
                paths.as_ptr(),
                scope as u32,
                timeout,
                &raw mut timed_out,
                &raw mut error,
            )
        };
        bool_registration_result(
            ok,
            timed_out,
            timeout,
            error,
            "font URL unregistration failed",
        )
    }
}

fn paths_json<P: AsRef<Path>>(paths: &[P]) -> CoreTextResult<std::ffi::CString> {
    let values: Vec<String> = paths
        .iter()
        .map(|path| path.as_ref().to_string_lossy().into_owned())
        .collect();
    cstring(&serde_json::to_string(&values)?)
}

fn registration_result(
    fallback: &str,
    call: impl FnOnce(u64, *mut bool) -> *mut libc::c_char,
) -> CoreTextResult<()> {
    let timeout = REGISTRATION_TIMEOUT_NANOSECONDS.load(Ordering::Relaxed);
    let mut timed_out = false;
    let messages = unsafe { json_from_owned::<Vec<String>>(call(timeout, &raw mut timed_out)) };
    if timed_out {
        return Err(CoreTextError::TimedOut(Duration::from_nanos(timeout)));
    }
    messages_to_result(&messages?, fallback)
}

fn bool_registration_result(
    ok: bool,
    timed_out: bool,
    timeout: u64,
    error: *mut libc::c_char,
    fallback: &str,
) -> CoreTextResult<()> {
    let error = unsafe { error_from_json_ptr(error, fallback) };
    if timed_out {
        Err(CoreTextError::TimedOut(Duration::from_nanos(timeout)))
    } else if ok {
        Ok(())
    } else {
        Err(error)
    }
}

fn messages_to_result(messages: &[String], fallback: &str) -> CoreTextResult<()> {
    if messages.is_empty() {
        Ok(())
    } else {
        Err(CoreTextError::Bridge(
            messages.join("; ").if_empty_then(fallback),
        ))
    }
}

unsafe fn error_from_json_ptr(ptr: *mut libc::c_char, fallback: &str) -> CoreTextError {
    if ptr.is_null() {
        return CoreTextError::Bridge(fallback.to_string());
    }
    match json_from_owned::<Vec<String>>(ptr) {
        Ok(messages) if messages.is_empty() => CoreTextError::Bridge(fallback.to_string()),
        Ok(messages) => CoreTextError::Bridge(messages.join("; ")),
        Err(error) => error,
    }
}

trait EmptyFallback {
    fn if_empty_then(self, fallback: &str) -> String;
}

impl EmptyFallback for String {
    fn if_empty_then(self, fallback: &str) -> String {
        if self.is_empty() {
            fallback.to_string()
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{registration_result, FontManager};
    use crate::common::json_from_owned;
    use crate::error::CoreTextError;

    unsafe extern "C" {
        fn ct_font_manager_test_registration_wait(
            handler_delay_nanoseconds: u64,
            timeout_nanoseconds: u64,
            timed_out: *mut bool,
        ) -> *mut libc::c_char;
    }

    fn nanoseconds(duration: Duration) -> u64 {
        u64::try_from(duration.as_nanos()).expect("test durations fit in u64")
    }

    fn wait_for_test_handler(delay: Duration, timeout: Duration) -> (bool, Vec<String>) {
        let mut timed_out = false;
        let json = unsafe {
            ct_font_manager_test_registration_wait(
                nanoseconds(delay),
                nanoseconds(timeout),
                &raw mut timed_out,
            )
        };
        let messages = unsafe { json_from_owned::<Vec<String>>(json) }.expect("messages json");
        (timed_out, messages)
    }

    #[test]
    fn registration_wait_collects_messages_from_a_late_handler() {
        let (timed_out, messages) =
            wait_for_test_handler(Duration::from_millis(20), Duration::from_secs(10));
        assert!(!timed_out);
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn registration_wait_times_out_instead_of_reporting_success() {
        let (timed_out, messages) =
            wait_for_test_handler(Duration::from_millis(300), Duration::from_millis(10));
        assert!(timed_out);
        assert!(messages.is_empty());
        std::thread::sleep(Duration::from_millis(600));
    }

    #[test]
    fn timed_out_registration_is_an_error() {
        let result = registration_result("fallback", |_, timed_out| {
            unsafe { timed_out.write(true) };
            unsafe { libc::strdup(c"[]".as_ptr()) }
        });
        assert!(matches!(result, Err(CoreTextError::TimedOut(_))));
    }

    #[test]
    fn registration_messages_become_errors() {
        let failed = registration_result("fallback", |_, _| unsafe {
            libc::strdup(c"[\"first\",\"second\"]".as_ptr())
        });
        assert!(
            matches!(failed, Err(CoreTextError::Bridge(message)) if message == "first; second")
        );
        let succeeded =
            registration_result("fallback", |_, _| unsafe { libc::strdup(c"[]".as_ptr()) });
        assert!(succeeded.is_ok());
    }

    #[test]
    fn registration_timeout_is_configurable() {
        assert_eq!(
            FontManager::registration_timeout(),
            Some(Duration::from_secs(30))
        );
        FontManager::set_registration_timeout(None);
        assert_eq!(FontManager::registration_timeout(), None);
        FontManager::set_registration_timeout(Some(Duration::from_millis(1500)));
        assert_eq!(
            FontManager::registration_timeout(),
            Some(Duration::from_millis(1500))
        );
        FontManager::set_registration_timeout(Some(Duration::MAX));
        assert!(FontManager::registration_timeout().is_some());
        FontManager::set_registration_timeout(Some(Duration::from_secs(30)));
    }
}
