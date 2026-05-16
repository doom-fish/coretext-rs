#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's [CoreText](https://developer.apple.com/documentation/coretext)
//! framework on macOS.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::incompatible_msrv,
    clippy::len_without_is_empty,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::ptr_as_ptr,
    clippy::ref_as_ptr,
    clippy::redundant_pub_crate,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::option_if_let_else
)]

pub mod attributed_string;
pub mod error;
pub mod ffi;
pub mod font;
pub mod frame;
pub mod framesetter;
pub mod line;
pub mod paragraph;
pub mod run;
pub mod types;

mod cf;

pub use attributed_string::AttributedString;
pub use error::{CoreTextError, CoreTextResult};
pub use font::CTFont;
pub use frame::CTFrame;
pub use framesetter::CTFramesetter;
pub use line::{bounds_options, CTLine};
pub use paragraph::{ParagraphStyle, TextAlignment};
pub use run::{run_status, CTRun};
pub use types::{CGPoint, CGRect, CGSize, TextRange, TypographicBounds};
