# coretext-rs

Safe Rust bindings for Apple's [CoreText](https://developer.apple.com/documentation/coretext) framework — fonts, lines, runs, frames, and attributed strings on macOS. Pure C FFI; no Swift bridge. The published Cargo package is `coretext-rs`; the Rust library target is `coretext`.

> **Status:** v0.1.0 ships the practical CoreText surface for font metrics, paragraph styling, line/run measurement, and frame-based layout.

## Highlights

- `CTFont` — create by name/size, read metrics (size, ascent, descent, leading, glyph count, names)
- `ParagraphStyle` — alignment creation and readback via `CTParagraphStyleCreate`
- `AttributedString` — build from `&str` + `CTFont` + optional `ParagraphStyle`
- `CTLine` — create, glyph count, string range, typographic bounds, `bounds_with_options`, trailing whitespace, pen offset, runs
- `CTRun` — glyph IDs, positions, advances, string indices, string range, typographic bounds, status flags
- `CTFramesetter` — `suggest_frame_size_with_constraints`, `create_frame_in_rect`
- `CTFrame` — string range, visible range, lines, line origins
- Geometry types `CGPoint`, `CGSize`, `CGRect` with `new` constructors
- Ergonomic `TextRange` and `TypographicBounds` structs

## Quick start

```rust,no_run
use coretext::{
    AttributedString, CGRect, CGSize, CTFont, CTFramesetter, CTLine,
    ParagraphStyle, TextAlignment,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = CTFont::new("Helvetica", 24.0)?;
    println!("ascent={} descent={}", font.ascent(), font.descent());

    let para = ParagraphStyle::with_alignment(TextAlignment::Center)?;
    let attr = AttributedString::new("Hello, CoreText!", &font, Some(&para))?;

    let line = CTLine::create_with_attributed_string(&attr)?;
    let tb = line.typographic_bounds();
    println!("line width={:.2} ascent={:.2}", tb.width, tb.ascent);

    let setter = CTFramesetter::create_with_attributed_string(&attr)?;
    let (size, fit) = setter.suggest_frame_size_with_constraints(CGSize::new(
        f64::INFINITY, f64::INFINITY,
    ));
    let frame = setter.create_frame_in_rect(
        CGRect::new(0.0, 0.0, size.width + 40.0, 200.0),
        fit,
    )?;
    println!("frame lines={}", frame.lines().len());
    Ok(())
}
```

## Surface overview

### CTFont

- `CTFont::new(name, size)` — create by PostScript or family name
- `size()`, `ascent()`, `descent()`, `leading()`, `glyph_count()`
- `postscript_name()`, `family_name()`, `full_name()`

### ParagraphStyle

- `ParagraphStyle::with_alignment(alignment)`
- `alignment()` — read back the stored alignment
- `TextAlignment` enum: `Left`, `Right`, `Center`, `Justified`, `Natural`

### AttributedString

- `AttributedString::new(text, font, paragraph_style)`

### CTLine

- `CTLine::create_with_attributed_string(attr_str)`
- `glyph_count()`, `string_range()`, `typographic_bounds()`
- `bounds_with_options(options)` — options from `coretext::bounds_options`
- `trailing_whitespace_width()`, `pen_offset_for_flush(factor, width)`
- `runs()` — returns `Vec<CTRun>`

### CTRun

- `glyph_count()`, `status()`, `string_range()`
- `glyphs()`, `positions()`, `advances()`, `string_indices()`
- `typographic_bounds()`
- Status constants in `coretext::run_status`

### CTFramesetter

- `CTFramesetter::create_with_attributed_string(attr_str)`
- `suggest_frame_size_with_constraints(constraints)` → `(CGSize, TextRange)`
- `create_frame_in_rect(rect, string_range)` → `CTFrame`

### CTFrame

- `string_range()`, `visible_string_range()`
- `lines()` → `Vec<CTLine>`
- `line_origins()` → `Vec<CGPoint>`

## Smoke example

```bash
cargo run --example 01_layout_smoke
```

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
