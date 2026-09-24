# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2026-09-24

### Security

- `CTFont::font_for_string` passed its range to `CTFontCreateForString` unchecked, so a range past the end of the string made CoreText read out of bounds (a segfault in testing). Every range is now checked first (see Fixed).
- Every wrapper was `Send + Sync`, including `MutableFontCollection`, whose setters mutated Core Foundation state through `&self`, so safe code could race on it. The layout objects were `Sync` too, against Apple's guidance to keep a layout on one thread. See Changed for the new markers.
- The public `from_raw` constructors were safe and adopted any pointer, which was then released on drop. They are now crate-private.

### Fixed

- Negative, overflowing or out-of-range `TextRange`s and break indices no longer reach CoreText, where they raised uncatchable `NSException`s (`CTTypesetterCreateLine`, `CTFramesetterCreateFrame`, `CTFramesetterSuggestFrameSizeWithConstraints`) or returned garbage. They are checked with checked arithmetic against the string's length in UTF-16 code units and rejected with `CoreTextError::RangeOutOfBounds` or `IndexOutOfBounds`. `TextRange` is documented as counting UTF-16 code units.
- Font registration (`register_font_urls`, `register_font_descriptors`, `register_fonts_for_urls` and the `unregister_*` functions) appended CoreText's handler messages to an unsynchronized Swift array and reported `Ok(())` when its fixed 5-second wait ran out while registration was still pending. The handler state is now lock-protected, a timeout returns `CoreTextError::TimedOut` and makes CoreText stop at its next callback, and the timeout is configurable.
- `register_fonts_for_urls` and `unregister_fonts_for_urls` leaked the bridge's message string on success.
- `CTFrame::line_origins` no longer asks CoreText for more origins than the frame has lines.
- Tautological and assertion-free tests now assert real values, and the font-enable test that changes the user's font registry only runs with `--ignored`.
- `COVERAGE_AUDIT.md` counted symbols that are only declared in the unsafe `raw-ffi` module as verified (240 of 468 rows) and now says so; the `COVERAGE_AUDIT_V2.md` totals now match its tables. The README states the macOS 10.15 and Rust 1.82 minimums.
- `build.rs` no longer adds the toolchain's Swift 5.5 back-deployment directory (`usr/lib/swift-5.5/macosx`) to the link search path or rpath. Its old `libswift_Concurrency.dylib` could shadow the SDK's `libswift_Concurrency.tbd` for the whole binary and break linking next to Swift bridges that use newer concurrency APIs.

### Changed

- **BREAKING:** only the font and attribute objects (`CTFont`, `FontDescriptor`, `FontCollection`, `MutableFontCollection`, `GlyphInfo`, `AttributedString`, `ParagraphStyle`, `TextTab`, `RubyAnnotation`) are `Send + Sync`. `CTTypesetter`, `CTFramesetter`, `CTFrame`, `CTLine` and `CTRun` are neither, because their clones and children share the same CoreText objects.
- **BREAKING:** `MutableFontCollection::set_query_descriptors` and `set_exclusion_descriptors` take `&mut self`, `clone` makes an independent mutable copy, and `as_font_collection` returns an immutable snapshot instead of an alias.
- **BREAKING:** `CTTypesetter::suggest_line_break`, `suggest_line_break_with_offset`, `suggest_cluster_break`, `suggest_cluster_break_with_offset` and `CTFramesetter::suggest_frame_size_for_range` return `CoreTextResult`.
- **BREAKING:** `from_raw` is no longer public on the wrapper types.
- **BREAKING:** `CoreTextError` is `#[non_exhaustive]` and has new variants.
- **BREAKING:** requires `apple-cf` `>=0.11, <0.12` and Rust 1.82.

### Added

- `CTLine::draw`, `CTFrame::draw` and `CTFont::draw_glyphs` draw into an `apple-cf` `CGContext`.
- `CTFont::path_for_glyph` returns a `GlyphPath` (bounding boxes and `PathElement`s) from `CTFontCreatePathForGlyph`.
- `CTLine::caret_offsets` returns `CaretOffset`s from `CTLineEnumerateCaretOffsets`.
- `AttributedString::utf16_len`, `MutableFontCollection::into_font_collection`, and `FontManager::set_registration_timeout` / `registration_timeout` (30 s by default).
- `CoreTextError::RangeOutOfBounds`, `IndexOutOfBounds`, `LengthMismatch` and `TimedOut`.

## [0.7.2] - 2026-06-06

- Clamped `CTRun` glyph, position, advance and string-index reads to the run's glyph count, and contained panics in the adaptive-image callback and release trampoline.

## [0.7.1] - 2026-05-20

- Added in-`src/` unit tests across error, font_traits, line, paragraph, and types (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.7.0] - 2026-05-19

### Added

- Added a safe `CTAdaptiveImageProviding` bridge via `AdaptiveImageProviding` / `AdaptiveImageProvider`, plus `CTFont` helpers for `CTFontGetTypographicBoundsForAdaptiveImageProvider` and `CTFontDrawImageFromAdaptiveImageProviderAtPoint`.
- Re-exported `CGContext` and `CGImage` alongside the new adaptive-image APIs so callers can implement providers without reaching into internal bridge details.

## [0.6.3] - 2026-05-18

### Changed

- Completed the remaining doc pass across the non-generated CoreText wrappers and helper modules, including retained-handle boilerplate and Core Foundation helpers, bringing nightly `rustdoc --show-coverage` item coverage to 100.0%.

## [0.6.2] - 2026-05-18

### Changed

- Added one-line Rustdoc coverage across the safe CoreText wrapper surface, bringing nightly `rustdoc --show-coverage` output to 94.4% documented items.

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
