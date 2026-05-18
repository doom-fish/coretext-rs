use serde::Serialize;

use crate::attributed_string::AttributedString;
use crate::bridge;
use crate::common::{cstring, expect_handle, impl_handle};
use crate::error::CoreTextResult;
use crate::line::CTLine;
use crate::types::TextRange;

/// Options for `CTTypesetterCreateWithAttributedStringAndOptions`.
#[derive(Debug, Clone, Copy, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypesetterOptions {
    /// Configures the `allow_unbounded_layout` option passed to `CTTypesetterCreateWithAttributedStringAndOptions`.
    pub allow_unbounded_layout: bool,
    /// Configures the `forced_embedding_level` option passed to `CTTypesetterCreateWithAttributedStringAndOptions`.
    pub forced_embedding_level: Option<i64>,
}

impl TypesetterOptions {
    fn json(self) -> CoreTextResult<String> {
        Ok(serde_json::to_string(&self)?)
    }
}

/// An immutable `CTTypesetter` wrapper.
pub struct CTTypesetter {
    raw: bridge::Handle,
}

impl_handle!(CTTypesetter);

impl CTTypesetter {
    /// Wraps `CTTypesetterCreateWithAttributedString`.
    pub fn create_with_attributed_string(
        attributed_string: &AttributedString,
    ) -> CoreTextResult<Self> {
        Self::create_with_options(attributed_string, TypesetterOptions::default())
    }

    /// Wraps `CTTypesetterCreateWithAttributedStringAndOptions`.
    pub fn create_with_options(
        attributed_string: &AttributedString,
        options: TypesetterOptions,
    ) -> CoreTextResult<Self> {
        let json = cstring(&options.json()?)?;
        let raw = unsafe {
            bridge::ct_typesetter_create_with_attributed_string(
                attributed_string.as_raw(),
                json.as_ptr(),
            )
        };
        Ok(Self::from_raw(expect_handle(
            raw,
            "ct_typesetter_create_with_attributed_string returned NULL",
        )?))
    }

    /// Wraps `CTTypesetterCreateLine`.
    pub fn create_line(&self, string_range: TextRange) -> CoreTextResult<CTLine> {
        let raw = unsafe { bridge::ct_typesetter_create_line(self.raw, string_range.into()) };
        Ok(CTLine::from_raw(expect_handle(
            raw,
            "ct_typesetter_create_line returned NULL",
        )?))
    }

    /// Wraps `CTTypesetterCreateLineWithOffset`.
    pub fn create_line_with_offset(
        &self,
        string_range: TextRange,
        offset: f64,
    ) -> CoreTextResult<CTLine> {
        let raw = unsafe {
            bridge::ct_typesetter_create_line_with_offset(self.raw, string_range.into(), offset)
        };
        Ok(CTLine::from_raw(expect_handle(
            raw,
            "ct_typesetter_create_line_with_offset returned NULL",
        )?))
    }

    /// Wraps `CTTypesetterSuggestLineBreak`.
    #[must_use]
    pub fn suggest_line_break(&self, start_index: isize, width: f64) -> isize {
        unsafe { bridge::ct_typesetter_suggest_line_break(self.raw, start_index, width) }
    }

    /// Wraps `CTTypesetterSuggestLineBreakWithOffset`.
    #[must_use]
    pub fn suggest_line_break_with_offset(
        &self,
        start_index: isize,
        width: f64,
        offset: f64,
    ) -> isize {
        unsafe {
            bridge::ct_typesetter_suggest_line_break_with_offset(
                self.raw,
                start_index,
                width,
                offset,
            )
        }
    }

    /// Wraps `CTTypesetterSuggestClusterBreak`.
    #[must_use]
    pub fn suggest_cluster_break(&self, start_index: isize, width: f64) -> isize {
        unsafe { bridge::ct_typesetter_suggest_cluster_break(self.raw, start_index, width) }
    }

    /// Wraps `CTTypesetterSuggestClusterBreakWithOffset`.
    #[must_use]
    pub fn suggest_cluster_break_with_offset(
        &self,
        start_index: isize,
        width: f64,
        offset: f64,
    ) -> isize {
        unsafe {
            bridge::ct_typesetter_suggest_cluster_break_with_offset(
                self.raw,
                start_index,
                width,
                offset,
            )
        }
    }
}

/// Wraps `CTTypesetterGetTypeID`.
#[must_use]
pub fn typesetter_type_id() -> u64 {
    unsafe { bridge::ct_typesetter_get_type_id() }
}
