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

pub mod attributed_string;
pub mod error;
#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
pub mod ffi;
pub mod font;
pub mod font_collection;
pub mod font_descriptor;
pub mod font_feature;
pub mod font_manager;
pub mod font_traits;
pub mod font_variation;
pub mod frame;
pub mod framesetter;
pub mod glyph;
pub mod line;
pub mod paragraph;
pub mod ruby_annotation;
pub mod run;
pub mod text_tab;
pub mod types;
pub mod typesetter;

pub use attributed_string::AttributedString;
pub use error::{CoreTextError, CoreTextResult};
pub use font::{font_type_id, CTFont, FontNameKey, UIFontType};
pub use font_collection::{font_collection_type_id, FontCollection, FontCollectionOptions, MutableFontCollection};
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
    paragraph_style_type_id, LineBreakMode, ParagraphStyle, ParagraphStyleOptions,
    TextAlignment, WritingDirection,
};
pub use ruby_annotation::{ruby_annotation_type_id, RubyAlignment, RubyAnnotation, RubyOverhang, RubyPosition};
pub use run::{run_status, run_type_id, CTRun};
pub use text_tab::{text_tab_type_id, TextTab};
pub use types::{CGAffineTransform, CGPoint, CGRect, CGSize, TextRange, TypographicBounds};
pub use typesetter::{typesetter_type_id, CTTypesetter, TypesetterOptions};
