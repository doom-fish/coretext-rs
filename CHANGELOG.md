# Changelog

## [0.2.0] - 2026-05-16

### Added

- Swift bridge build pipeline that statically links a bundled `CoreTextBridge` library into the Rust crate.
- Safe wrappers for `FontDescriptor`, `FontCollection`, `FontManager`, `CTTypesetter`, `TextTab`, `GlyphInfo`, and `RubyAnnotation`.
- Font metadata helpers for traits, features, feature settings, variation axes, variation coordinates, and available table tags.
- New integration tests for fonts, descriptors, collections, font manager, layout/typesetter/frame flows, glyph info, and ruby annotations.
- New numbered examples covering layout, font metadata, descriptor/collection/manager workflows, glyph info, and ruby annotations.
- `COVERAGE.md` documenting the requested CoreText coverage pass and the audited exclusions.

### Changed

- Migrated the safe API surface from direct C FFI wrappers to Swift-backed bridge functions while keeping the legacy raw declarations behind the opt-in `raw-ffi` feature.
- Expanded paragraph style coverage to include text tabs, tab intervals, line break modes, and writing direction.
- Expanded line, run, frame, and framesetter helpers around truncation, justification, bounds, offsets, image bounds, JSON attributes, and typesetter reuse.
- Updated crate documentation for the v0.2.0 surface and examples.

### Fixed

- Added Swift runtime search paths required for tests and examples to link reliably from Cargo.
- Added Swift 6 CoreText compatibility aliases in the bridge layer.
- Corrected CFArray-backed CoreText handling for available font tables, paragraph tab stops, and ruby annotation text.

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
