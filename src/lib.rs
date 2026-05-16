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
pub use font::{CTFont, FontNameKey, UIFontType};
pub use font_collection::{FontCollection, FontCollectionOptions};
pub use font_descriptor::{FontDescriptor, FontFormat, FontOrientation};
pub use font_feature::{FontFeature, FontFeatureSelector, FontFeatureSetting};
pub use font_manager::{AutoActivationSetting, FontManager, FontManagerScope};
pub use font_traits::{symbolic_traits, FontTraits};
pub use font_variation::{FontVariationAxis, FontVariationCoordinate};
pub use frame::CTFrame;
pub use framesetter::CTFramesetter;
pub use glyph::{CharacterCollection, GlyphId, GlyphInfo};
pub use line::{bounds_options, CTLine, LineTruncationType};
pub use paragraph::{
    LineBreakMode, ParagraphStyle, ParagraphStyleOptions, TextAlignment, WritingDirection,
};
pub use ruby_annotation::{RubyAlignment, RubyAnnotation, RubyOverhang, RubyPosition};
pub use run::{run_status, CTRun};
pub use text_tab::TextTab;
pub use types::{CGAffineTransform, CGPoint, CGRect, CGSize, TextRange, TypographicBounds};
pub use typesetter::{CTTypesetter, TypesetterOptions};
