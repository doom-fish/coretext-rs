# coretext-rs

Safe Rust bindings for Apple's [CoreText](https://developer.apple.com/documentation/coretext) framework on macOS, backed by a bundled Swift bridge. The Cargo package is `coretext-rs`; the Rust library target is `coretext`.

> **Status:** v0.2.0 expands the crate beyond the original line/frame surface to cover descriptors, collections, font management, glyph info, ruby annotations, text tabs, typesetters, and font metadata helpers.

## Highlights

- Safe wrappers for `CTFont`, `CTFontDescriptor`, `CTFontCollection`, and `CTFontManager`
- Layout pipeline coverage for `AttributedString`, `ParagraphStyle`, `TextTab`, `CTTypesetter`, `CTLine`, `CTRun`, `CTFramesetter`, and `CTFrame`
- Font metadata helpers for traits, features, feature settings, variation axes, variation coordinates, and table tags
- Glyph and annotation helpers via `GlyphInfo` and `RubyAnnotation`
- Opt-in `raw-ffi` feature for the legacy low-level C declarations

## Platform and build requirements

- macOS
- Xcode or Command Line Tools with `xcrun` and a Swift toolchain available
- Rust 1.76+

`build.rs` compiles the bundled Swift package and links it statically against CoreText, CoreFoundation, CoreGraphics, and Foundation.

## Quick start

```rust,no_run
use coretext::{
    AttributedString, CGRect, CGSize, CTFont, CTFramesetter, ParagraphStyle,
    ParagraphStyleOptions, TextAlignment,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = CTFont::new("Helvetica", 18.0)?;
    let style = ParagraphStyle::new(&ParagraphStyleOptions {
        alignment: Some(TextAlignment::Center),
        ..ParagraphStyleOptions::default()
    })?;
    let attributed = AttributedString::new("Hello, CoreText!", &font, Some(&style))?;

    let framesetter = CTFramesetter::create_with_attributed_string(&attributed)?;
    let (size, fit_range) =
        framesetter.suggest_frame_size_with_constraints(CGSize::new(240.0, f64::INFINITY));
    let frame = framesetter.create_frame_in_rect(
        CGRect::new(0.0, 0.0, size.width + 24.0, 120.0),
        fit_range,
    )?;

    println!("font={} lines={}", font.full_name()?, frame.lines().len());
    Ok(())
}
```

## Covered surface

This release includes safe wrappers for the requested CoreText areas:

- `CTFont`, `FontTraits`, `FontFeature`, `FontVariation`
- `FontDescriptor`, `FontCollection`, `FontManager`
- `ParagraphStyle`, `TextTab`, `CTTypesetter`, `CTLine`, `CTRun`, `CTFramesetter`, `CTFrame`
- `GlyphInfo`, `RubyAnnotation`
- Supporting geometry and range types: `CGAffineTransform`, `CGPoint`, `CGSize`, `CGRect`, `TextRange`, `TypographicBounds`

See [COVERAGE.md](COVERAGE.md) for the per-area validation matrix and notes from the SDK audit.

## Examples

```bash
cargo run --example 01_layout_smoke
cargo run --example 02_font_overview
cargo run --example 03_descriptor_collection_manager
cargo run --example 04_glyph_info
cargo run --example 05_ruby_annotation
```

## Raw FFI

The safe Swift-backed wrappers are enabled by default. If you also need the legacy low-level declarations, enable the `raw-ffi` feature:

```toml
[dependencies]
coretext-rs = { path = "../coretext-rs", features = ["raw-ffi"] }
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
