# CoreText coverage

This document tracks the safe-wrapper coverage of `coretext-rs` 0.8.0.

- Architecture: safe Rust wrappers over a bundled Swift bridge, plus an opt-in legacy `raw-ffi` module.
- Validation: `tests/api_coverage.rs`, per-area integration tests under `tests/`, and the numbered runtime examples under `examples/`.
- SDK audit baseline: Xcode macOS SDK `MacOSX26.2.sdk`; the audit tables have not been regenerated for newer SDKs.
- `COVERAGE_AUDIT.md` counts a symbol as verified when it is reachable from Rust at all. 240 of its 468 verified rows are only declared in the unsafe `raw-ffi` module; 228 have a safe wrapper. `COVERAGE_AUDIT_V2.md` counts safe wrappers for the 204 functions in the wrapped headers (177 wrapped).

## Requested area matrix

| Area | Rust surface | Validation | Example |
| --- | --- | --- | --- |
| Font | `CTFont`, `FontNameKey`, `UIFontType` | `tests/font.rs`, `tests/api_coverage.rs` | `examples/02_font_overview.rs` |
| Font traits | `FontTraits`, `symbolic_traits` | `tests/font.rs`, `tests/font_descriptor.rs` | `examples/02_font_overview.rs` |
| Font features | `FontFeature`, `FontFeatureSelector`, `FontFeatureSetting` | `tests/font.rs`, `tests/font_descriptor.rs` | `examples/02_font_overview.rs` |
| Font variation | `FontVariationAxis`, `FontVariationCoordinate` | `tests/font.rs`, `tests/font_descriptor.rs` | `examples/02_font_overview.rs` |
| Font descriptor | `FontDescriptor`, `FontFormat`, `FontOrientation` | `tests/font_descriptor.rs` | `examples/03_descriptor_collection_manager.rs` |
| Font collection | `FontCollection`, `FontCollectionOptions` | `tests/font_collection.rs` | `examples/03_descriptor_collection_manager.rs` |
| Font manager | `FontManager`, `FontManagerScope`, `AutoActivationSetting` | `tests/font_manager.rs` | `examples/03_descriptor_collection_manager.rs` |
| Frame | `CTFrame` | `tests/layout.rs`, `tests/api_coverage.rs` | `examples/01_layout_smoke.rs` |
| Framesetter | `CTFramesetter` | `tests/layout.rs`, `tests/api_coverage.rs` | `examples/01_layout_smoke.rs` |
| Glyph | `GlyphInfo`, `GlyphId`, `CharacterCollection` | `tests/glyph.rs` | `examples/04_glyph_info.rs` |
| Line | `CTLine`, `LineTruncationType`, `bounds_options` | `tests/layout.rs`, `tests/api_coverage.rs` | `examples/01_layout_smoke.rs` |
| Paragraph style | `ParagraphStyle`, `ParagraphStyleOptions`, `TextAlignment`, `LineBreakMode`, `WritingDirection` | `tests/layout.rs`, `tests/api_coverage.rs` | `examples/01_layout_smoke.rs` |
| Text tabs | `TextTab` | `tests/layout.rs` | `examples/01_layout_smoke.rs` |
| Run | `CTRun`, `run_status` | `tests/layout.rs`, `tests/api_coverage.rs` | `examples/01_layout_smoke.rs` |
| Typesetter | `CTTypesetter`, `TypesetterOptions` | `tests/layout.rs` | `examples/01_layout_smoke.rs` |
| Ruby annotation | `RubyAnnotation`, `RubyAlignment`, `RubyOverhang`, `RubyPosition` | `tests/ruby_annotation.rs` | `examples/05_ruby_annotation.rs` |
| Drawing | `CTLine::draw`, `CTFrame::draw`, `CTFont::draw_glyphs` | `tests/drawing.rs` | — |
| Glyph paths and carets | `CTFont::path_for_glyph`, `GlyphPath`, `PathElement`, `CTLine::caret_offsets`, `CaretOffset` | `tests/drawing.rs` | — |
| Range validation | `TextRange` (UTF-16 code units) checked by every range-taking wrapper | `tests/layout.rs` | — |
| Thread safety | `Send`/`Sync` only on font and attribute objects | `tests/thread_safety.rs` | — |

## Supporting surface

The requested areas rely on a few supporting wrappers that are also validated by the suite:

- `AttributedString`
- Geometry helpers: `CGAffineTransform`, `CGPoint`, `CGSize`, `CGRect`
- Text helpers: `TextRange`, `TypographicBounds`

## Intentionally not wrapped in this pass

The safe API targets the areas above rather than every symbol in `CoreText.framework`. The following audited APIs remain outside the safe wrapper surface:

- `CTRunDelegate` (run delegates with ascent, descent and width callbacks) and `CTRunDraw`; both are declared in `raw-ffi` only
- `CTFramesetterCreateFrame` with non-rectangular paths or frame attributes (`CTFramesetter::create_frame_in_rect` builds a rectangular path)
- Arbitrary ruby attribute dictionaries: `RubyAnnotation::with_attributes` sets only the size factor and calls `CTRubyAnnotationCreateWithAttributes` only when a single position has text
- `CTFontManagerCompareFontFamilyNames`
- Deprecated graphics-font registration APIs such as `CTFontManagerRegisterGraphicsFont` / `CTFontManagerUnregisterGraphicsFont`
- Deprecated typesetter options such as `kCTTypesetterOptionDisableBidiProcessing`
- Platform-specific constants such as `kCTFontRegistrationUserInfoAttribute` that are not part of the macOS wrapper target

## Raw FFI note

`src/ffi.rs` remains available behind the `raw-ffi` feature for low-level consumers and compatibility work. The safe Swift-backed wrappers are the primary API surface and receive the broad coverage described above.
