# coretext-rs

Safe Rust bindings for Apple's [CoreText](https://developer.apple.com/documentation/coretext) framework on macOS, backed by a bundled Swift bridge. The Cargo package is `coretext-rs`; the Rust library target is `coretext`.

```toml
[dependencies]
coretext-rs = "0.8"
```

## Highlights

- Safe wrappers for `CTFont`, `CTFontDescriptor`, `CTFontCollection`, and `CTFontManager`
- Layout pipeline coverage for `AttributedString`, `ParagraphStyle`, `TextTab`, `CTTypesetter`, `CTLine`, `CTRun`, `CTFramesetter`, and `CTFrame`
- Font metadata helpers for traits, features, feature settings, variation axes, variation coordinates, and table tags
- Glyph and annotation helpers via `GlyphInfo` and `RubyAnnotation`
- Drawing into an `apple-cf` `CGContext` with `CTLine::draw`, `CTFrame::draw` and `CTFont::draw_glyphs`, glyph outlines from `CTFont::path_for_glyph`, and caret positions from `CTLine::caret_offsets`
- Opt-in `raw-ffi` feature for the legacy low-level C declarations

## Platform and build requirements

- macOS 10.15+ (the adaptive-image APIs need macOS 15 and return empty results on older systems)
- Xcode or Command Line Tools with `xcrun` and a Swift toolchain available
- Rust 1.82+

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

## Text ranges are UTF-16

`TextRange` locations and lengths, string indices and break positions count UTF-16 code units, the way `NSString` and `CFString` index text; they are not byte offsets or `char` counts (`"é"` is one unit, `"😀"` is two). `AttributedString::utf16_len` returns the length to measure against. Every wrapper that takes a range or index checks it before calling CoreText and returns `CoreTextError::RangeOutOfBounds` or `CoreTextError::IndexOutOfBounds` instead of letting CoreText read out of bounds or raise an exception.

## Thread safety

Font and attribute objects (`CTFont`, `FontDescriptor`, `FontCollection`, `MutableFontCollection`, `GlyphInfo`, `GlyphPath`, `AttributedString`, `ParagraphStyle`, `TextTab`, `RubyAnnotation`) are `Send + Sync`. The layout objects (`CTTypesetter`, `CTFramesetter`, `CTFrame`, `CTLine`, `CTRun`) are neither: Apple asks for each layout to stay within one operation, work queue or thread, and clones and children (a frame's lines, a line's runs, a framesetter's typesetter) share the same CoreText objects. Build and draw a layout on one thread.

`MutableFontCollection` changes only through `&mut self`. Its clones are independent copies, `as_font_collection` returns an immutable snapshot, and `into_font_collection` freezes it without copying.

## Font registration

`FontManager::register_font_urls`, `register_font_descriptors`, `register_fonts_for_urls` and their `unregister_*` counterparts wait for CoreText's completion handler for at most `FontManager::registration_timeout()`: 30 seconds unless changed with `FontManager::set_registration_timeout` (`None` waits until CoreText finishes). A call that runs out of time returns `CoreTextError::TimedOut` and tells CoreText to stop. `register_fonts_with_asset_names` and `registered_font_descriptors` wrap iOS-only APIs: on macOS the first always returns an error and the second always returns an empty list.

## Covered surface

This release includes safe wrappers for the requested CoreText areas:

- `CTFont`, `FontTraits`, `FontFeature`, `FontVariation`
- `FontDescriptor`, `FontCollection`, `FontManager`
- `ParagraphStyle`, `TextTab`, `CTTypesetter`, `CTLine`, `CTRun`, `CTFramesetter`, `CTFrame`
- `GlyphInfo`, `RubyAnnotation`, `GlyphPath`, `CaretOffset`
- Supporting geometry and range types: `CGAffineTransform`, `CGPoint`, `CGSize`, `CGRect`, `TextRange`, `TypographicBounds`

Not wrapped safely: run delegates (`CTRunDelegate`), `CTRunDraw`, frames with non-rectangular paths or frame attributes, and the remaining items listed in [COVERAGE.md](COVERAGE.md). Most of these are declared in the unsafe `raw-ffi` module.

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
coretext-rs = { version = "0.8", features = ["raw-ffi"] }
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
