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
}
