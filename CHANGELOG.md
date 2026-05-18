# Changelog

## [0.6.1] - 2026-05-18

### Changed

- chore: re-export OS primitives (Boolean, FourCharCode) from apple-cf

## [0.6.0] - 2026-05-18

### Changed

- Re-exported `CFIndex`, `CFTypeID`, `CFOptionFlags`, `CFRange`, `CFComparisonResult`, `CFStringEncoding`, `CGFloat`, and `CGContextRef` from `apple-cf` 0.9 instead of maintaining duplicate local raw type definitions.
- Bumped `coretext-rs` to `0.6.0` for the nominal raw CoreFoundation/CoreGraphics type re-export change.

## [0.5.0] - 2026-05-18

### Changed

- Re-exported the raw CoreFoundation `CF*Ref` typedefs from `apple-cf` 0.8 instead of duplicating 13 local aliases across `ffi.rs` and `ffi_gap.rs`.
- Bumped `coretext-rs` to `0.5.0` for the nominal raw CoreFoundation reference type change.

## [0.4.0] - 2026-05-18

### Changed

- Re-exported `CGPoint`, `CGSize`, `CGRect`, and `CGAffineTransform` from `apple-cf` 0.8 now that its Core Graphics layout matches the canonical nested `CGRect` form.
- Bumped `coretext-rs` to `0.4.0` for the nominal CG geometry type change.

## [0.3.1] - 2026-05-18

### Fixed

- Added comprehensive `SAFETY:` comments to all unsafe blocks in core FFI modules (`cf.rs`, `common.rs`, `attributed_string.rs`). These comments document the preconditions and invariants that ensure memory safety for CoreFoundation and bridge-layer operations.

## [0.3.0] - 2026-05-17

### Added

- Added the remaining safe CoreText wrappers requested by `COVERAGE_AUDIT_V2.md`, including font-collection exclusion/query helpers, font attribute/cascade/table/options/caret/string-encoding APIs, descriptor JSON attribute builders, batch font-manager registration/data helpers, and type-id/text-tab/ruby/layout accessors.

### Changed

- Reclassified the remaining callback-only, drawing-only, borrowed-pointer, deprecated, and macOS-unavailable CoreText entry points as explicit `COVERAGE_AUDIT_V2.md` exemptions, bringing the v2 audit to zero unresolved gaps.
- Refreshed the safe-surface audit metadata and coverage notes for the new `0.3.0` API surface.

## [0.2.1] - 2026-05-16

### Added

- Expanded the optional `raw-ffi` surface to cover the remaining CoreText audit gaps, including advanced `CTFont` graphics/table APIs, generic `CTFontDescriptor` matching and attribute access, batch `CTFontManager` entry points, CT string/frame/text-tab constants, and the SFNT layout/table structs from `SFNTTypes.h` and `SFNTLayoutTypes.h`.
- Added `tests/raw_ffi.rs` and `examples/06_raw_ffi_smoke.rs` to smoke-test the newly exposed raw CoreText declarations.

### Changed

- Updated the packaging include list to exclude `swift-bridge/.build` contents while still shipping the Swift sources needed to build the bridge.
- Refreshed `COVERAGE_AUDIT.md` after closing the remaining audited gaps.

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
