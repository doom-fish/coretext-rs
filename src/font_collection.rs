use serde::Serialize;

use crate::bridge;
use crate::common::{cstring, expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::font_descriptor::FontDescriptor;

/// Options that influence how CoreText resolves a font collection query.
#[derive(Debug, Clone, Copy, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FontCollectionOptions {
    pub remove_duplicates: bool,
    pub include_disabled_fonts: bool,
    pub disallow_auto_activation: bool,
}

impl FontCollectionOptions {
    fn json(self) -> CoreTextResult<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

/// An immutable `CTFontCollection` wrapper.
pub struct FontCollection {
    raw: bridge::Handle,
}

impl_handle!(FontCollection);

impl FontCollection {
    pub fn available() -> CoreTextResult<Self> {
        Self::available_with_options(FontCollectionOptions::default())
    }

    pub fn available_with_options(options: FontCollectionOptions) -> CoreTextResult<Self> {
        let json = cstring(&options.json()?)?;
        let raw = unsafe { bridge::ct_font_collection_create_available(json.as_ptr()) };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_collection_create_available returned NULL",
        )?))
    }

    pub fn with_descriptors(
        descriptors: &[FontDescriptor],
        options: FontCollectionOptions,
    ) -> CoreTextResult<Self> {
        let descriptor_handles: Vec<_> = descriptors.iter().map(FontDescriptor::as_raw).collect();
        let json = cstring(&options.json()?)?;
        let raw = unsafe {
            bridge::ct_font_collection_create_with_descriptors(
                descriptor_handles.as_ptr(),
                isize::try_from(descriptor_handles.len()).unwrap_or(isize::MAX),
                json.as_ptr(),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_collection_create_with_descriptors returned NULL",
        )?))
    }

    pub fn copy_with_descriptors(
        &self,
        descriptors: &[FontDescriptor],
        options: FontCollectionOptions,
    ) -> CoreTextResult<Self> {
        let descriptor_handles: Vec<_> = descriptors.iter().map(FontDescriptor::as_raw).collect();
        let json = cstring(&options.json()?)?;
        let raw = unsafe {
            bridge::ct_font_collection_copy_with_descriptors(
                self.raw,
                descriptor_handles.as_ptr(),
                isize::try_from(descriptor_handles.len()).unwrap_or(isize::MAX),
                json.as_ptr(),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_font_collection_copy_with_descriptors returned NULL",
        )?))
    }

    #[must_use]
    pub fn query_descriptors(&self) -> Vec<FontDescriptor> {
        let count = unsafe { bridge::ct_font_collection_get_query_descriptor_count(self.raw) };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_collection_copy_query_descriptors(self.raw, handles.as_mut_ptr(), count)
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(FontDescriptor::from_raw).collect()
    }

    #[must_use]
    pub fn matching_descriptors(&self) -> Vec<FontDescriptor> {
        let count = unsafe { bridge::ct_font_collection_get_matching_descriptor_count(self.raw) };
        if count <= 0 {
            return Vec::new();
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_collection_copy_matching_descriptors(
                self.raw,
                handles.as_mut_ptr(),
                count,
            )
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        handles.into_iter().map(FontDescriptor::from_raw).collect()
    }

    pub fn matching_descriptors_for_family(
        &self,
        family_name: &str,
    ) -> CoreTextResult<Vec<FontDescriptor>> {
        let family_name = cstring(family_name)?;
        let count = unsafe {
            bridge::ct_font_collection_get_matching_descriptors_for_family_count(
                self.raw,
                family_name.as_ptr(),
            )
        };
        if count <= 0 {
            return Ok(Vec::new());
        }
        let mut handles = vec![std::ptr::null_mut(); usize::try_from(count).unwrap_or(0)];
        let written = unsafe {
            bridge::ct_font_collection_copy_matching_descriptors_for_family(
                self.raw,
                family_name.as_ptr(),
                handles.as_mut_ptr(),
                count,
            )
        };
        handles.truncate(usize::try_from(written).unwrap_or(0));
        Ok(handles.into_iter().map(FontDescriptor::from_raw).collect())
    }
}
