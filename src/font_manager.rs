use std::path::Path;

use crate::bridge;
use crate::common::{cstring, json_from_owned, option_string_from_owned};
use crate::error::{CoreTextError, CoreTextResult};
use crate::font_descriptor::FontDescriptor;

/// Registration scope for CoreText font manager operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum FontManagerScope {
    #[default]
    None = 0,
    Process = 1,
    Persistent = 2,
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
    #[default]
    Default = 0,
    Disabled = 1,
    Enabled = 2,
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

/// Namespace for CoreText font manager APIs.
pub struct FontManager;

impl FontManager {
    pub fn available_postscript_names() -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_manager_copy_available_postscript_names_json()) }
    }

    pub fn available_font_family_names() -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_manager_copy_available_font_family_names_json()) }
    }

    pub fn available_font_urls() -> CoreTextResult<Vec<String>> {
        unsafe { json_from_owned(bridge::ct_font_manager_copy_available_font_urls_json()) }
    }

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

    pub fn is_supported_font(path: impl AsRef<Path>) -> CoreTextResult<bool> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        Ok(unsafe { bridge::ct_font_manager_is_supported_font(path.as_ptr()) })
    }

    pub fn register_fonts_for_url(
        path: impl AsRef<Path>,
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            bridge::ct_font_manager_register_fonts_for_url(path.as_ptr(), scope as u32, &mut error)
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
                &mut error,
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

    pub fn scope_for_url(path: impl AsRef<Path>) -> CoreTextResult<FontManagerScope> {
        let path = cstring(&path.as_ref().to_string_lossy())?;
        Ok(FontManagerScope::from_raw(unsafe {
            bridge::ct_font_manager_get_scope_for_url(path.as_ptr())
        }))
    }

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

    #[must_use]
    pub fn registered_font_descriptors(scope: FontManagerScope, enabled: bool) -> Vec<FontDescriptor> {
        let count = unsafe { bridge::ct_font_manager_copy_registered_descriptor_count(scope as u32, enabled) };
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

    pub fn register_font_descriptors(
        descriptors: &[FontDescriptor],
        scope: FontManagerScope,
        enabled: bool,
    ) -> CoreTextResult<()> {
        if descriptors.is_empty() {
            return Ok(());
        }
        let handles: Vec<_> = descriptors.iter().map(FontDescriptor::as_raw).collect();
        let errors: Vec<String> = unsafe {
            json_from_owned(bridge::ct_font_manager_register_font_descriptors(
                handles.as_ptr(),
                isize::try_from(handles.len()).unwrap_or(isize::MAX),
                scope as u32,
                enabled,
            ))?
        };
        messages_to_result(&errors, "font descriptor registration failed")
    }

    pub fn register_font_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
        enabled: bool,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        let errors: Vec<String> = unsafe {
            json_from_owned(bridge::ct_font_manager_register_font_urls(
                paths.as_ptr(),
                scope as u32,
                enabled,
            ))?
        };
        messages_to_result(&errors, "font URL registration failed")
    }

    pub fn register_fonts_for_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            bridge::ct_font_manager_register_fonts_for_urls(paths.as_ptr(), scope as u32, &mut error)
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe { error_from_json_ptr(error, "font URL registration failed") })
        }
    }

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

    pub fn unregister_font_descriptors(
        descriptors: &[FontDescriptor],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if descriptors.is_empty() {
            return Ok(());
        }
        let handles: Vec<_> = descriptors.iter().map(FontDescriptor::as_raw).collect();
        let errors: Vec<String> = unsafe {
            json_from_owned(bridge::ct_font_manager_unregister_font_descriptors(
                handles.as_ptr(),
                isize::try_from(handles.len()).unwrap_or(isize::MAX),
                scope as u32,
            ))?
        };
        messages_to_result(&errors, "font descriptor unregistration failed")
    }

    pub fn unregister_font_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        let errors: Vec<String> = unsafe {
            json_from_owned(bridge::ct_font_manager_unregister_font_urls(
                paths.as_ptr(),
                scope as u32,
            ))?
        };
        messages_to_result(&errors, "font URL unregistration failed")
    }

    pub fn unregister_fonts_for_urls<P: AsRef<Path>>(
        paths: &[P],
        scope: FontManagerScope,
    ) -> CoreTextResult<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let paths = paths_json(paths)?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            bridge::ct_font_manager_unregister_fonts_for_urls(
                paths.as_ptr(),
                scope as u32,
                &mut error,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe { error_from_json_ptr(error, "font URL unregistration failed") })
        }
    }
}

fn paths_json<P: AsRef<Path>>(paths: &[P]) -> CoreTextResult<std::ffi::CString> {
    let values: Vec<String> = paths
        .iter()
        .map(|path| path.as_ref().to_string_lossy().into_owned())
        .collect();
    cstring(&serde_json::to_string(&values)?)
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
