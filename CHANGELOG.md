# Changelog

## [0.1.0] - 2026-05-16

### Added

- Initial `coretext-rs` release — safe, pure C FFI bindings for Apple's CoreText framework on macOS.
- `CTFont` wrapper: `new`, `size`, `postscript_name`, `family_name`, `full_name`, `ascent`, `descent`, `leading`, `glyph_count`.
- `ParagraphStyle` wrapper: `with_alignment`, `alignment`; `TextAlignment` enum (Left, Right, Center, Justified, Natural).
- `AttributedString` helper: build from `&str` + `CTFont` + optional `ParagraphStyle` using `CFAttributedStringCreate`.
- `CTLine` wrapper: `create_with_attributed_string`, `glyph_count`, `string_range`, `typographic_bounds`, `bounds_with_options`, `trailing_whitespace_width`, `pen_offset_for_flush`, `runs`.
- `CTRun` wrapper: `glyph_count`, `status`, `glyphs`, `positions`, `advances`, `string_indices`, `string_range`, `typographic_bounds`.
- `CTFramesetter` wrapper: `create_with_attributed_string`, `suggest_frame_size_with_constraints`, `create_frame_in_rect`.
- `CTFrame` wrapper: `string_range`, `visible_string_range`, `lines`, `line_origins`.
- Geometry types `CGPoint`, `CGSize`, `CGRect` with `new` constructors; ergonomic `TextRange` and `TypographicBounds` structs.
- Smoke example `examples/01_layout_smoke.rs` covering the full layout pipeline.
- Header-audit test `tests/api_coverage.rs` verifying the declared symbol set against the active SDK.
