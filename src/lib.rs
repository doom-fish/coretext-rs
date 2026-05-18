#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::incompatible_msrv,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines
)]

mod bridge;
mod common;

/// CoreText-ready attributed-string inputs for layout APIs like `CTLineCreateWithAttributedString`.
pub mod attributed_string;
/// Error types returned by CoreText wrapper APIs.
pub mod error;
#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
pub mod ffi;
/// Safe wrappers around the `CTFont` API family.
pub mod font;
/// Safe wrappers around the `CTFontCollection` API family.
pub mod font_collection;
/// Safe wrappers around the `CTFontDescriptor` API family.
pub mod font_descriptor;
/// Data models returned by `CTFontCopyFeatures` and related descriptor queries.
pub mod font_feature;
/// Safe wrappers around the `CTFontManager` API family.
pub mod font_manager;
/// Helpers for `CTFontSymbolicTraits` and `CTFontCopyTraits`.
pub mod font_traits;
/// Data models returned by `CTFontCopyVariation` and `CTFontCopyVariationAxes`.
pub mod font_variation;
/// Safe wrappers around the `CTFrame` API family.
pub mod frame;
/// Safe wrappers around the `CTFramesetter` API family.
pub mod framesetter;
/// Safe wrappers around the `CTGlyphInfo` API family.
pub mod glyph;
/// Safe wrappers around the `CTLine` API family.
pub mod line;
/// Safe wrappers around the `CTParagraphStyle` API family.
pub mod paragraph;
/// Safe wrappers around the `CTRubyAnnotation` API family.
pub mod ruby_annotation;
/// Safe wrappers around the `CTRun` API family.
pub mod run;
/// Safe wrappers around the `CTTextTab` API family.
pub mod text_tab;
/// Shared CoreText geometry and range types.
pub mod types;
/// Safe wrappers around the `CTTypesetter` API family.
pub mod typesetter;

pub use attributed_string::AttributedString;
pub use error::{CoreTextError, CoreTextResult};
pub use font::{font_type_id, CTFont, FontNameKey, UIFontType};
pub use font_collection::{
    font_collection_type_id, FontCollection, FontCollectionOptions, MutableFontCollection,
};
pub use font_descriptor::{font_descriptor_type_id, FontDescriptor, FontFormat, FontOrientation};
pub use font_feature::{FontFeature, FontFeatureSelector, FontFeatureSetting};
pub use font_manager::{AutoActivationSetting, FontManager, FontManagerScope};
pub use font_traits::{symbolic_traits, FontTraits};
pub use font_variation::{FontVariationAxis, FontVariationCoordinate};
pub use frame::{frame_type_id, CTFrame};
pub use framesetter::{framesetter_type_id, CTFramesetter};
pub use glyph::{glyph_info_type_id, CharacterCollection, GlyphId, GlyphInfo};
pub use line::{bounds_options, line_type_id, CTLine, LineTruncationType};
pub use paragraph::{
    paragraph_style_type_id, LineBreakMode, ParagraphStyle, ParagraphStyleOptions, TextAlignment,
    WritingDirection,
};
pub use ruby_annotation::{
    ruby_annotation_type_id, RubyAlignment, RubyAnnotation, RubyOverhang, RubyPosition,
};
pub use run::{run_status, run_type_id, CTRun};
pub use text_tab::{text_tab_type_id, TextTab};
pub use types::{CGAffineTransform, CGPoint, CGRect, CGSize, TextRange, TypographicBounds};
pub use typesetter::{typesetter_type_id, CTTypesetter, TypesetterOptions};
