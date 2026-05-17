# coretext-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 204
VERIFIED: 173
GAPS: 0
EXEMPT: 175
COVERAGE_PCT: 84.8

The audit examines 204 public CoreText function symbols across the safe-wrapper target headers. `coretext-rs` now verifies every safe-surface-appropriate entry point in this set. The remaining exclusions are deliberate: the original 143 type/constant namespace items plus callback-driven, drawing-only, borrowed-pointer, deprecated, or macOS-unavailable APIs that are intentionally kept out of the ergonomic safe Rust surface.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CTFontCollectionCopyExclusionDescriptors | function | CTFontCollection.h | FontCollection::exclusion_descriptors |
| CTFontCollectionCopyFontAttribute | function | CTFontCollection.h | FontCollection::font_attribute_json |
| CTFontCollectionCopyFontAttributes | function | CTFontCollection.h | FontCollection::font_attributes_json |
| CTFontCollectionCopyQueryDescriptors | function | CTFontCollection.h | FontCollection::query_descriptors |
| CTFontCollectionCreateCopyWithFontDescriptors | function | CTFontCollection.h | FontCollection::copy_with_descriptors |
| CTFontCollectionCreateFromAvailableFonts | function | CTFontCollection.h | FontCollection::available_with_options |
| CTFontCollectionCreateMatchingFontDescriptors | function | CTFontCollection.h | FontCollection::matching_descriptors |
| CTFontCollectionCreateMatchingFontDescriptorsForFamily | function | CTFontCollection.h | FontCollection::matching_descriptors_for_family |
| CTFontCollectionCreateMatchingFontDescriptorsWithOptions | function | CTFontCollection.h | FontCollection::matching_descriptors_with_options |
| CTFontCollectionCreateMutableCopy | function | CTFontCollection.h | FontCollection::mutable_copy |
| CTFontCollectionCreateWithFontDescriptors | function | CTFontCollection.h | FontCollection::with_descriptors |
| CTFontCollectionGetTypeID | function | CTFontCollection.h | font_collection_type_id |
| CTFontCollectionSetExclusionDescriptors | function | CTFontCollection.h | MutableFontCollection::set_exclusion_descriptors |
| CTFontCollectionSetQueryDescriptors | function | CTFontCollection.h | MutableFontCollection::set_query_descriptors |
| CTFontCopyAttribute | function | CTFont.h | CTFont::attribute_json |
| CTFontCopyAvailableTables | function | CTFont.h | CTFont::available_tables |
| CTFontCopyDefaultCascadeListForLanguages | function | CTFont.h | CTFont::default_cascade_list |
| CTFontCopyDisplayName | function | CTFont.h | CTFont::display_name |
| CTFontCopyFamilyName | function | CTFont.h | CTFont::family_name |
| CTFontCopyFeatureSettings | function | CTFont.h | CTFont::feature_settings |
| CTFontCopyFeatures | function | CTFont.h | CTFont::features |
| CTFontCopyFontDescriptor | function | CTFont.h | CTFont::descriptor |
| CTFontCopyFullName | function | CTFont.h | CTFont::full_name |
| CTFontCopyLocalizedName | function | CTFont.h | CTFont::localized_name |
| CTFontCopyName | function | CTFont.h | CTFont::name |
| CTFontCopyNameForGlyph | function | CTFont.h | CTFont::name_for_glyph |
| CTFontCopyPostScriptName | function | CTFont.h | CTFont::postscript_name |
| CTFontCopySupportedLanguages | function | CTFont.h | CTFont::supported_languages |
| CTFontCopyTable | function | CTFont.h | CTFont::table_data |
| CTFontCopyTraits | function | CTFont.h | CTFont::traits |
| CTFontCopyVariation | function | CTFont.h | CTFont::variation_coordinates |
| CTFontCopyVariationAxes | function | CTFont.h | CTFont::variation_axes |
| CTFontCreateCopyWithAttributes | function | CTFont.h | CTFont::copy_with_attributes |
| CTFontCreateCopyWithFamily | function | CTFont.h | CTFont::copy_with_family |
| CTFontCreateCopyWithSymbolicTraits | function | CTFont.h | CTFont::copy_with_symbolic_traits |
| CTFontCreateForString | function | CTFont.h | CTFont::font_for_string |
| CTFontCreateForStringWithLanguage | function | CTFont.h | CTFont::font_for_string |
| CTFontCreateUIFontForLanguage | function | CTFont.h | CTFont::ui_font |
| CTFontCreateWithFontDescriptor | function | CTFont.h | CTFont::from_descriptor |
| CTFontCreateWithFontDescriptorAndOptions | function | CTFont.h | CTFont::from_descriptor_with_options |
| CTFontCreateWithName | function | CTFont.h | CTFont::new |
| CTFontCreateWithNameAndOptions | function | CTFont.h | CTFont::with_name_and_options |
| CTFontDescriptorCopyAttribute | function | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::display_name, FontDescriptor::family_name, FontDescriptor::feature_settings, FontDescriptor::features, … |
| CTFontDescriptorCopyAttributes | function | CTFontDescriptor.h | FontDescriptor::attributes_json |
| CTFontDescriptorCopyLocalizedAttribute | function | CTFontDescriptor.h | FontDescriptor::localized_attribute_json |
| CTFontDescriptorCreateCopyWithAttributes | function | CTFontDescriptor.h | FontDescriptor::copy_with_attributes_json |
| CTFontDescriptorCreateCopyWithFamily | function | CTFontDescriptor.h | FontDescriptor::with_family |
| CTFontDescriptorCreateCopyWithFeature | function | CTFontDescriptor.h | FontDescriptor::with_feature |
| CTFontDescriptorCreateCopyWithSymbolicTraits | function | CTFontDescriptor.h | FontDescriptor::with_symbolic_traits |
| CTFontDescriptorCreateCopyWithVariation | function | CTFontDescriptor.h | FontDescriptor::with_variation |
| CTFontDescriptorCreateMatchingFontDescriptor | function | CTFontDescriptor.h | FontDescriptor::matching_descriptor |
| CTFontDescriptorCreateMatchingFontDescriptors | function | CTFontDescriptor.h | FontDescriptor::matching_descriptors |
| CTFontDescriptorCreateWithAttributes | function | CTFontDescriptor.h | FontDescriptor::with_attributes_json |
| CTFontDescriptorCreateWithNameAndSize | function | CTFontDescriptor.h | FontDescriptor::new |
| CTFontDescriptorGetTypeID | function | CTFontDescriptor.h | font_descriptor_type_id |
| CTFontGetAdvancesForGlyphs | function | CTFont.h | CTFont::advances_for_glyphs |
| CTFontGetAscent | function | CTFont.h | CTFont::ascent |
| CTFontGetBoundingBox | function | CTFont.h | CTFont::bounding_box |
| CTFontGetBoundingRectsForGlyphs | function | CTFont.h | CTFont::bounding_rects_for_glyphs |
| CTFontGetCapHeight | function | CTFont.h | CTFont::cap_height |
| CTFontGetDescent | function | CTFont.h | CTFont::descent |
| CTFontGetGlyphCount | function | CTFont.h | CTFont::glyph_count |
| CTFontGetGlyphWithName | function | CTFont.h | CTFont::glyph_with_name |
| CTFontGetGlyphsForCharacters | function | CTFont.h | CTFont::glyphs_for_string |
| CTFontGetLeading | function | CTFont.h | CTFont::leading |
| CTFontGetLigatureCaretPositions | function | CTFont.h | CTFont::ligature_caret_positions |
| CTFontGetMatrix | function | CTFont.h | CTFont::matrix |
| CTFontGetOpticalBoundsForGlyphs | function | CTFont.h | CTFont::optical_bounds_for_glyphs |
| CTFontGetSize | function | CTFont.h | CTFont::size |
| CTFontGetSlantAngle | function | CTFont.h | CTFont::slant_angle |
| CTFontGetStringEncoding | function | CTFont.h | CTFont::string_encoding |
| CTFontGetSymbolicTraits | function | CTFont.h | CTFont::symbolic_traits |
| CTFontGetTypeID | function | CTFont.h | font_type_id |
| CTFontGetUnderlinePosition | function | CTFont.h | CTFont::underline_position |
| CTFontGetUnderlineThickness | function | CTFont.h | CTFont::underline_thickness |
| CTFontGetUnitsPerEm | function | CTFont.h | CTFont::units_per_em |
| CTFontGetVerticalTranslationsForGlyphs | function | CTFont.h | CTFont::vertical_translations_for_glyphs |
| CTFontGetXHeight | function | CTFont.h | CTFont::x_height |
| CTFontHasTable | function | CTFont.h | CTFont::has_table |
| CTFontManagerCopyAvailableFontFamilyNames | function | CTFontManager.h | FontManager::available_font_family_names |
| CTFontManagerCopyAvailableFontURLs | function | CTFontManager.h | FontManager::available_font_urls |
| CTFontManagerCopyAvailablePostScriptNames | function | CTFontManager.h | FontManager::available_postscript_names |
| CTFontManagerCreateFontDescriptorFromData | function | CTFontManager.h | FontManager::font_descriptor_from_data |
| CTFontManagerCreateFontDescriptorsFromData | function | CTFontManager.h | FontManager::font_descriptors_from_data |
| CTFontManagerCreateFontDescriptorsFromURL | function | CTFontManager.h | FontManager::font_descriptors_from_url |
| CTFontManagerEnableFontDescriptors | function | CTFontManager.h | FontManager::enable_font_descriptors |
| CTFontManagerGetAutoActivationSetting | function | CTFontManager.h | FontManager::auto_activation_setting |
| CTFontManagerGetScopeForURL | function | CTFontManager.h | FontManager::scope_for_url |
| CTFontManagerIsSupportedFont | function | CTFontManager.h | FontManager::is_supported_font |
| CTFontManagerRegisterFontDescriptors | function | CTFontManager.h | FontManager::register_font_descriptors |
| CTFontManagerRegisterFontURLs | function | CTFontManager.h | FontManager::register_font_urls |
| CTFontManagerRegisterFontsForURL | function | CTFontManager.h | FontManager::register_fonts_for_url |
| CTFontManagerRegisterFontsForURLs | function | CTFontManager.h | FontManager::register_fonts_for_urls |
| CTFontManagerSetAutoActivationSetting | function | CTFontManager.h | FontManager::set_auto_activation_setting |
| CTFontManagerUnregisterFontDescriptors | function | CTFontManager.h | FontManager::unregister_font_descriptors |
| CTFontManagerUnregisterFontURLs | function | CTFontManager.h | FontManager::unregister_font_urls |
| CTFontManagerUnregisterFontsForURL | function | CTFontManager.h | FontManager::unregister_fonts_for_url |
| CTFontManagerUnregisterFontsForURLs | function | CTFontManager.h | FontManager::unregister_fonts_for_urls |
| CTFrameGetFrameAttributes | function | CTFrame.h | CTFrame::has_frame_attributes |
| CTFrameGetLineOrigins | function | CTFrame.h | CTFrame::line_origins |
| CTFrameGetLines | function | CTFrame.h | CTFrame::line_origins, CTFrame::lines |
| CTFrameGetPath | function | CTFrame.h | CTFrame::path_bounding_box |
| CTFrameGetStringRange | function | CTFrame.h | CTFrame::string_range |
| CTFrameGetTypeID | function | CTFrame.h | frame_type_id |
| CTFrameGetVisibleStringRange | function | CTFrame.h | CTFrame::visible_string_range |
| CTFramesetterCreateFrame | function | CTFramesetter.h | CTFramesetter::create_frame_in_rect |
| CTFramesetterCreateWithAttributedString | function | CTFramesetter.h | CTFramesetter::create_with_attributed_string |
| CTFramesetterCreateWithTypesetter | function | CTFramesetter.h | CTFramesetter::create_with_typesetter |
| CTFramesetterGetTypeID | function | CTFramesetter.h | framesetter_type_id |
| CTFramesetterGetTypesetter | function | CTFramesetter.h | CTFramesetter::typesetter |
| CTFramesetterSuggestFrameSizeWithConstraints | function | CTFramesetter.h | CTFramesetter::suggest_frame_size_for_range |
| CTGlyphInfoCreateWithCharacterIdentifier | function | CTGlyphInfo.h | GlyphInfo::with_character_identifier |
| CTGlyphInfoCreateWithGlyph | function | CTGlyphInfo.h | GlyphInfo::with_glyph |
| CTGlyphInfoCreateWithGlyphName | function | CTGlyphInfo.h | GlyphInfo::with_glyph_name |
| CTGlyphInfoGetCharacterCollection | function | CTGlyphInfo.h | GlyphInfo::character_collection |
| CTGlyphInfoGetCharacterIdentifier | function | CTGlyphInfo.h | GlyphInfo::character_identifier |
| CTGlyphInfoGetGlyph | function | CTGlyphInfo.h | GlyphInfo::glyph |
| CTGlyphInfoGetGlyphName | function | CTGlyphInfo.h | GlyphInfo::glyph_name |
| CTGlyphInfoGetTypeID | function | CTGlyphInfo.h | glyph_info_type_id |
| CTLineCreateJustifiedLine | function | CTLine.h | CTLine::justified |
| CTLineCreateTruncatedLine | function | CTLine.h | CTLine::truncated |
| CTLineCreateWithAttributedString | function | CTLine.h | CTLine::create_with_attributed_string |
| CTLineGetBoundsWithOptions | function | CTLine.h | CTLine::bounds_with_options |
| CTLineGetGlyphCount | function | CTLine.h | CTLine::glyph_count |
| CTLineGetGlyphRuns | function | CTLine.h | CTLine::runs |
| CTLineGetImageBounds | function | CTLine.h | CTLine::image_bounds |
| CTLineGetOffsetForStringIndex | function | CTLine.h | CTLine::offset_for_string_index |
| CTLineGetPenOffsetForFlush | function | CTLine.h | CTLine::pen_offset_for_flush |
| CTLineGetStringIndexForPosition | function | CTLine.h | CTLine::string_index_for_position |
| CTLineGetStringRange | function | CTLine.h | CTLine::string_range |
| CTLineGetTrailingWhitespaceWidth | function | CTLine.h | CTLine::trailing_whitespace_width |
| CTLineGetTypeID | function | CTLine.h | line_type_id |
| CTLineGetTypographicBounds | function | CTLine.h | CTLine::typographic_bounds |
| CTParagraphStyleCreate | function | CTParagraphStyle.h | ParagraphStyle::new |
| CTParagraphStyleCreateCopy | function | CTParagraphStyle.h | ParagraphStyle::copy |
| CTParagraphStyleGetTypeID | function | CTParagraphStyle.h | paragraph_style_type_id |
| CTParagraphStyleGetValueForSpecifier | function | CTParagraphStyle.h | ParagraphStyle::value_for_specifier_json |
| CTRubyAnnotationCreate | function | CTRubyAnnotation.h | ct_ruby_annotation_create |
| CTRubyAnnotationCreateCopy | function | CTRubyAnnotation.h | RubyAnnotation::copy |
| CTRubyAnnotationCreateWithAttributes | function | CTRubyAnnotation.h | RubyAnnotation::with_attributes |
| CTRubyAnnotationGetAlignment | function | CTRubyAnnotation.h | RubyAnnotation::alignment |
| CTRubyAnnotationGetOverhang | function | CTRubyAnnotation.h | RubyAnnotation::overhang |
| CTRubyAnnotationGetSizeFactor | function | CTRubyAnnotation.h | RubyAnnotation::size_factor |
| CTRubyAnnotationGetTextForPosition | function | CTRubyAnnotation.h | RubyAnnotation::text_for_position |
| CTRubyAnnotationGetTypeID | function | CTRubyAnnotation.h | ruby_annotation_type_id |
| CTRunGetAdvances | function | CTRun.h | CTRun::advances |
| CTRunGetAttributes | function | CTRun.h | CTRun::attributes_json |
| CTRunGetBaseAdvancesAndOrigins | function | CTRun.h | CTRun::base_advances_and_origins |
| CTRunGetGlyphCount | function | CTRun.h | CTRun::advances, CTRun::base_advances_and_origins, CTRun::glyph_count, CTRun::glyphs, CTRun::positions, … |
| CTRunGetGlyphs | function | CTRun.h | CTRun::glyphs |
| CTRunGetImageBounds | function | CTRun.h | CTRun::image_bounds |
| CTRunGetPositions | function | CTRun.h | CTRun::positions |
| CTRunGetStatus | function | CTRun.h | CTRun::status |
| CTRunGetStringIndices | function | CTRun.h | CTRun::string_indices |
| CTRunGetStringRange | function | CTRun.h | CTRun::string_range |
| CTRunGetTextMatrix | function | CTRun.h | CTRun::text_matrix |
| CTRunGetTypeID | function | CTRun.h | run_type_id |
| CTRunGetTypographicBounds | function | CTRun.h | CTRun::typographic_bounds |
| CTTextTabCreate | function | CTTextTab.h | TextTab::new |
| CTTextTabGetAlignment | function | CTTextTab.h | TextTab::alignment |
| CTTextTabGetLocation | function | CTTextTab.h | TextTab::location |
| CTTextTabGetOptions | function | CTTextTab.h | TextTab::options_json |
| CTTextTabGetTypeID | function | CTTextTab.h | text_tab_type_id |
| CTTypesetterCreateLine | function | CTTypesetter.h | CTTypesetter::create_line |
| CTTypesetterCreateLineWithOffset | function | CTTypesetter.h | CTTypesetter::create_line_with_offset |
| CTTypesetterCreateWithAttributedString | function | CTTypesetter.h | CTTypesetter::create_with_attributed_string |
| CTTypesetterCreateWithAttributedStringAndOptions | function | CTTypesetter.h | CTTypesetter::create_with_options |
| CTTypesetterGetTypeID | function | CTTypesetter.h | typesetter_type_id |
| CTTypesetterSuggestClusterBreak | function | CTTypesetter.h | CTTypesetter::suggest_cluster_break |
| CTTypesetterSuggestClusterBreakWithOffset | function | CTTypesetter.h | CTTypesetter::suggest_cluster_break_with_offset |
| CTTypesetterSuggestLineBreak | function | CTTypesetter.h | CTTypesetter::suggest_line_break |
| CTTypesetterSuggestLineBreakWithOffset | function | CTTypesetter.h | CTTypesetter::suggest_line_break_with_offset |
| Symbol | Kind | Header | Wrapped by |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback | function | CTFontCollection.h | Comparator-callback collection API intentionally left raw-ffi only. | - |
| CTFontCollectionRef | typedef | CTFontCollection.h | Type definition (not directly wrapped) | - |
| CTFontCopyCharacterSet | function | CTFont.h | CFCharacterSet interop is intentionally left out of the safe surface. | - |
| CTFontCopyGraphicsFont | function | CTFont.h | CGFont ownership API intentionally left raw-ffi only. | - |
| CTFontCreatePathForGlyph | function | CTFont.h | CGPath/CoreGraphics path interop intentionally left raw-ffi only. | - |
| CTFontCreateWithGraphicsFont | function | CTFont.h | CGFont ownership API intentionally left raw-ffi only. | - |
| CTFontCreateWithPlatformFont | function | CTFont.h | Deprecated on macOS; intentionally excluded from the safe wrapper audit. | CT_DEPRECATED("ATS is deprecated", macos(10.5, 11.0)) |
| CTFontCreateWithQuickdrawInstance | function | CTFont.h | Deprecated on macOS; intentionally excluded from the safe wrapper audit. | CT_DEPRECATED("Quickdraw font references are deprecated", macos(10.5, 10.15)) |
| CTFontDescriptorMatchFontDescriptorsWithProgressHandler | function | CTFontDescriptor.h | Progress-handler matching API intentionally left raw-ffi only. | - |
| CTFontDescriptorRef | typedef | CTFontDescriptor.h | Type definition (not directly wrapped) | - |
| CTFontDrawGlyphs | function | CTFont.h | Direct CGContext drawing API intentionally left raw-ffi only. | - |
| CTFontDrawImageFromAdaptiveImageProviderAtPoint | function | CTFont.h | Adaptive-image-provider drawing API intentionally left raw-ffi only. | API_AVAILABLE(macos(15.0), ios(18.0), watchos(11.0), tvos(18.0)) |
| CTFontGetPlatformFont | function | CTFont.h | Deprecated on macOS; intentionally excluded from the safe wrapper audit. | CT_DEPRECATED("ATS is deprecated", macos(10.5, 11.0)) |
| CTFontGetTypographicBoundsForAdaptiveImageProvider | function | CTFont.h | Adaptive-image-provider metrics API intentionally left raw-ffi only. | API_AVAILABLE(macos(15.0), ios(18.0), watchos(11.0), tvos(18.0)) |
| CTFontManagerCompareFontFamilyNames | function | CTFontManager.h | Comparator helper intentionally omitted from the safe surface. | - |
| CTFontManagerCopyRegisteredFontDescriptors | function | CTFontManager.h | iOS-only API unavailable on macOS target. | CT_AVAILABLE(ios(13.0)) API_UNAVAILABLE(macos, watchos, tvos) |
| CTFontManagerCreateFontRequestRunLoopSource | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the safe wrapper audit. | CT_DEPRECATED("This functionality will be removed in a future release", macos(10.6, 11.0)) CT_UNAVAILABLE(ios, watchos, tvos) |
| CTFontManagerRegisterFontsWithAssetNames | function | CTFontManager.h | iOS-only asset-catalog API unavailable on macOS target. | CT_AVAILABLE(ios(13.0)) API_UNAVAILABLE(macos, watchos, tvos) |
| CTFontManagerRegisterGraphicsFont | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the safe wrapper audit. | CT_DEPRECATED("Use CTFontManagerCreateFontDescriptorsFromData or CTFontManagerRegisterFontsForURL", macos(10.8, 15), ios(4.1, 18), watchos(2, 11), tvos(9, 18)) |
| CTFontManagerRequestFonts | function | CTFontManager.h | iOS-only font-request API unavailable on macOS target. | CT_AVAILABLE(ios(13.0)) API_UNAVAILABLE(macos, watchos, tvos) |
| CTFontManagerUnregisterGraphicsFont | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the safe wrapper audit. | CT_DEPRECATED("Use the API corresponding to the one used to register the font", macos(10.8, 15), ios(4.1, 18), watchos(2, 11), tvos(9, 18)) |
| CTFontRef | typedef | CTFont.h | Type definition (not directly wrapped) | - |
| CTFrameDraw | function | CTFrame.h | Direct CGContext drawing API intentionally left raw-ffi only. | - |
| CTFrameRef | typedef | CTFrame.h | Type definition (not directly wrapped) | - |
| CTFramesetterRef | typedef | CTFramesetter.h | Type definition (not directly wrapped) | - |
| CTGlyphInfoRef | typedef | CTGlyphInfo.h | Type definition (not directly wrapped) | - |
| CTLineDraw | function | CTLine.h | Direct CGContext drawing API intentionally left raw-ffi only. | - |
| CTLineEnumerateCaretOffsets | function | CTLine.h | Caret-enumeration callback API intentionally left raw-ffi only. | - |
| CTLineRef | typedef | CTLine.h | Type definition (not directly wrapped) | - |
| CTMutableFontCollectionRef | typedef | CTFontCollection.h | Type definition (not directly wrapped) | - |
| CTParagraphStyleRef | typedef | CTParagraphStyle.h | Type definition (not directly wrapped) | - |
| CTRubyAnnotationRef | typedef | CTRubyAnnotation.h | Type definition (not directly wrapped) | - |
| CTRunDelegateCreate | function | CTRunDelegate.h | Low-level callback/refcon API intentionally left raw-ffi only. | - |
| CTRunDelegateGetRefCon | function | CTRunDelegate.h | Low-level callback/refcon API intentionally left raw-ffi only. | - |
| CTRunDelegateGetTypeID | function | CTRunDelegate.h | Low-level callback/refcon API intentionally left raw-ffi only. | - |
| CTRunDelegateRef | typedef | CTRunDelegate.h | Type definition (not directly wrapped) | - |
| CTRunDraw | function | CTRun.h | Direct CGContext drawing API intentionally left raw-ffi only. | - |
| CTRunGetAdvancesPtr | function | CTRun.h | Borrowed pointer fast-path omitted; use CTRun::advances instead. | - |
| CTRunGetGlyphsPtr | function | CTRun.h | Borrowed pointer fast-path omitted; use CTRun::glyphs instead. | - |
| CTRunGetPositionsPtr | function | CTRun.h | Borrowed pointer fast-path omitted; use CTRun::positions instead. | - |
| CTRunGetStringIndicesPtr | function | CTRun.h | Borrowed pointer fast-path omitted; use CTRun::string_indices instead. | - |
| CTRunRef | typedef | CTRun.h | Type definition (not directly wrapped) | - |
| CTTextTabRef | typedef | CTTextTab.h | Type definition (not directly wrapped) | - |
| CTTypesetterRef | typedef | CTTypesetter.h | Type definition (not directly wrapped) | - |
| CT_AVAILABLE | function | CTTextTab.h | Header macro artifact, not a callable API surface symbol. | - |
| CT_DEPRECATED | function | CTTypesetter.h | Header macro artifact, not a callable API surface symbol. | - |
| kCTAdaptiveImageProviderAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| kCTBackgroundColorAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineClassAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineClassHanging | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineClassIdeographicCentered | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineClassIdeographicHigh | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineClassIdeographicLow | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineClassMath | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineClassRoman | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineInfoAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineOffsetAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineOriginalFont | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineReferenceFont | constant | CTFont.h | Namespace constant (not directly wrapped) | - |
| kCTBaselineReferenceInfoAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| kCTCharacterShapeAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| kCTFontAttributeName | constant | CTStringAttributes.h | Namespace constant (not directly wrapped) | - |
| ... | ... | ... | ... | ... |
| +113 more |  |  |  |  |
