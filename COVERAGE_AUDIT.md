# coretext-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 479
VERIFIED: 468
GAPS: 0
EXEMPT: 11
COVERAGE_PCT: 100.00%

This audit covers the full public C surface of CoreText.framework, including `SFNTTypes.h` and `SFNTLayoutTypes.h`. Verified rows include both the default safe Swift-backed API and symbols exposed directly by the optional `raw-ffi` feature.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| CTFontCopyAvailableTables | function | CTFont.h | CTFont::available_tables |
| CTFontCopyDisplayName | function | CTFont.h | CTFont::display_name |
| CTFontCopyFamilyName | function | CTFont.h | CTFont::family_name, ffi::CTFontCopyFamilyName |
| CTFontCopyFeatureSettings | function | CTFont.h | CTFont::feature_settings |
| CTFontCopyFeatures | function | CTFont.h | CTFont::features |
| CTFontCopyFontDescriptor | function | CTFont.h | CTFont::descriptor |
| CTFontCopyFullName | function | CTFont.h | CTFont::full_name, ffi::CTFontCopyFullName |
| CTFontCopyLocalizedName | function | CTFont.h | CTFont::localized_name |
| CTFontCopyName | function | CTFont.h | CTFont::name |
| CTFontCopyNameForGlyph | function | CTFont.h | CTFont::name_for_glyph |
| CTFontCopyPostScriptName | function | CTFont.h | CTFont::postscript_name, ffi::CTFontCopyPostScriptName |
| CTFontCopySupportedLanguages | function | CTFont.h | CTFont::supported_languages |
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
| CTFontCreateWithName | function | CTFont.h | CTFont::new, ffi::CTFontCreateWithName |
| CTFontGetAdvancesForGlyphs | function | CTFont.h | CTFont::advances_for_glyphs |
| CTFontGetAscent | function | CTFont.h | CTFont::ascent, ffi::CTFontGetAscent |
| CTFontGetBoundingBox | function | CTFont.h | CTFont::bounding_box |
| CTFontGetBoundingRectsForGlyphs | function | CTFont.h | CTFont::bounding_rects_for_glyphs |
| CTFontGetCapHeight | function | CTFont.h | CTFont::cap_height |
| CTFontGetDescent | function | CTFont.h | CTFont::descent, ffi::CTFontGetDescent |
| CTFontGetGlyphCount | function | CTFont.h | CTFont::glyph_count, ffi::CTFontGetGlyphCount |
| CTFontGetGlyphWithName | function | CTFont.h | CTFont::glyph_with_name |
| CTFontGetGlyphsForCharacters | function | CTFont.h | CTFont::glyphs_for_string |
| CTFontGetLeading | function | CTFont.h | CTFont::leading, ffi::CTFontGetLeading |
| CTFontGetMatrix | function | CTFont.h | CTFont::matrix |
| CTFontGetOpticalBoundsForGlyphs | function | CTFont.h | CTFont::optical_bounds_for_glyphs |
| CTFontGetSize | function | CTFont.h | CTFont::size, ffi::CTFontGetSize |
| CTFontGetSlantAngle | function | CTFont.h | CTFont::slant_angle |
| CTFontGetSymbolicTraits | function | CTFont.h | CTFont::symbolic_traits |
| CTFontGetTypeID | function | CTFont.h | ffi::CTFontGetTypeID |
| CTFontGetUnderlinePosition | function | CTFont.h | CTFont::underline_position |
| CTFontGetUnderlineThickness | function | CTFont.h | CTFont::underline_thickness |
| CTFontGetUnitsPerEm | function | CTFont.h | CTFont::units_per_em |
| CTFontGetVerticalTranslationsForGlyphs | function | CTFont.h | CTFont::vertical_translations_for_glyphs |
| CTFontGetXHeight | function | CTFont.h | CTFont::x_height |
| CTFontHasTable | function | CTFont.h | CTFont::has_table |
| CTFontRef | opaque type | CTFont.h | CTFont, ffi::CTFontRef |
| CTFontUIFontType | enum | CTFont.h | UIFontType |
| kCTFontCopyrightNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontDescriptionNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontDesignerNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontDesignerURLNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontFamilyNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontFeatureSampleTextKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFeatureSelectorDefaultKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFeatureSelectorIdentifierKey | constant | CTFont.h | CTFont::feature_settings, CTFont::features, FontDescriptor::feature_settings, FontDescriptor::features |
| kCTFontFeatureSelectorNameKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFeatureSelectorSettingKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFeatureTooltipTextKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFeatureTypeExclusiveKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFeatureTypeIdentifierKey | constant | CTFont.h | CTFont::feature_settings, CTFont::features, FontDescriptor::feature_settings, FontDescriptor::features |
| kCTFontFeatureTypeNameKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFeatureTypeSelectorsKey | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontFullNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontLicenseNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontLicenseURLNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontManufacturerNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontOpenTypeFeatureTag | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontOpenTypeFeatureValue | constant | CTFont.h | CTFont::features, FontDescriptor::features |
| kCTFontPostScriptCIDNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontPostScriptNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontSampleTextNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontStyleNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontSubFamilyNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontTrademarkNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontUniqueNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontVariationAxisDefaultValueKey | constant | CTFont.h | CTFont::variation_axes, FontDescriptor::variation_axes |
| kCTFontVariationAxisHiddenKey | constant | CTFont.h | CTFont::variation_axes, FontDescriptor::variation_axes |
| kCTFontVariationAxisIdentifierKey | constant | CTFont.h | CTFont::variation_axes, FontDescriptor::variation_axes |
| kCTFontVariationAxisMaximumValueKey | constant | CTFont.h | CTFont::variation_axes, FontDescriptor::variation_axes |
| kCTFontVariationAxisMinimumValueKey | constant | CTFont.h | CTFont::variation_axes, FontDescriptor::variation_axes |
| kCTFontVariationAxisNameKey | constant | CTFont.h | CTFont::variation_axes, FontDescriptor::variation_axes |
| kCTFontVendorURLNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| kCTFontVersionNameKey | constant | CTFont.h | CTFont::localized_name, CTFont::name |
| CTFontCollectionCopyOptions | options | CTFontCollection.h | FontCollectionOptions |
| CTFontCollectionCopyQueryDescriptors | function | CTFontCollection.h | FontCollection::query_descriptors |
| CTFontCollectionCreateCopyWithFontDescriptors | function | CTFontCollection.h | FontCollection::copy_with_descriptors |
| CTFontCollectionCreateFromAvailableFonts | function | CTFontCollection.h | FontCollection::available_with_options |
| CTFontCollectionCreateMatchingFontDescriptors | function | CTFontCollection.h | FontCollection::matching_descriptors |
| CTFontCollectionCreateMatchingFontDescriptorsForFamily | function | CTFontCollection.h | FontCollection::matching_descriptors_for_family |
| CTFontCollectionCreateWithFontDescriptors | function | CTFontCollection.h | FontCollection::with_descriptors |
| CTFontCollectionRef | opaque type | CTFontCollection.h | FontCollection |
| kCTFontCollectionDisallowAutoActivationOption | constant | CTFontCollection.h | FontCollection::available_with_options, FontCollection::copy_with_descriptors, FontCollection::with_descriptors |
| kCTFontCollectionIncludeDisabledFontsOption | constant | CTFontCollection.h | FontCollection::available_with_options, FontCollection::copy_with_descriptors, FontCollection::with_descriptors |
| kCTFontCollectionRemoveDuplicatesOption | constant | CTFontCollection.h | FontCollection::available_with_options, FontCollection::copy_with_descriptors, FontCollection::with_descriptors |
| CTFontDescriptorCopyAttribute | function | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::display_name, FontDescriptor::family_name, FontDescriptor::feature_settings, FontDescriptor::features, … |
| CTFontDescriptorCreateCopyWithFamily | function | CTFontDescriptor.h | FontDescriptor::with_family |
| CTFontDescriptorCreateCopyWithFeature | function | CTFontDescriptor.h | FontDescriptor::with_feature |
| CTFontDescriptorCreateCopyWithSymbolicTraits | function | CTFontDescriptor.h | FontDescriptor::with_symbolic_traits |
| CTFontDescriptorCreateCopyWithVariation | function | CTFontDescriptor.h | FontDescriptor::with_variation |
| CTFontDescriptorCreateMatchingFontDescriptor | function | CTFontDescriptor.h | FontDescriptor::matching_descriptor |
| CTFontDescriptorCreateMatchingFontDescriptors | function | CTFontDescriptor.h | FontDescriptor::matching_descriptors |
| CTFontDescriptorCreateWithNameAndSize | function | CTFontDescriptor.h | FontDescriptor::new |
| CTFontDescriptorRef | opaque type | CTFontDescriptor.h | FontDescriptor |
| CTFontFormat | enum | CTFontDescriptor.h | FontFormat |
| CTFontOrientation | enum | CTFontDescriptor.h | FontOrientation |
| kCTFontDisplayNameAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::display_name |
| kCTFontDownloadableAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::is_downloadable |
| kCTFontEnabledAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::is_enabled |
| kCTFontFamilyNameAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::family_name |
| kCTFontFeatureSettingsAttribute | constant | CTFontDescriptor.h | FontDescriptor::feature_settings |
| kCTFontFeaturesAttribute | constant | CTFontDescriptor.h | FontDescriptor::features |
| kCTFontFormatAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::format |
| kCTFontNameAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::postscript_name |
| kCTFontOrientationAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::orientation |
| kCTFontSizeAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::size |
| kCTFontStyleNameAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::style_name |
| kCTFontTraitsAttribute | constant | CTFontDescriptor.h | FontDescriptor::traits |
| kCTFontURLAttribute | constant | CTFontDescriptor.h | FontDescriptor::attributes_json, FontDescriptor::url_path |
| kCTFontVariationAttribute | constant | CTFontDescriptor.h | FontDescriptor::variation_coordinates |
| kCTFontVariationAxesAttribute | constant | CTFontDescriptor.h | FontDescriptor::variation_axes |
| CTFontManagerAutoActivationSetting | enum | CTFontManager.h | AutoActivationSetting |
| CTFontManagerCopyAvailableFontFamilyNames | function | CTFontManager.h | FontManager::available_font_family_names |
| CTFontManagerCopyAvailableFontURLs | function | CTFontManager.h | FontManager::available_font_urls |
| CTFontManagerCopyAvailablePostScriptNames | function | CTFontManager.h | FontManager::available_postscript_names |
| CTFontManagerCreateFontDescriptorsFromURL | function | CTFontManager.h | FontManager::font_descriptors_from_url |
| CTFontManagerGetAutoActivationSetting | function | CTFontManager.h | FontManager::auto_activation_setting |
| CTFontManagerGetScopeForURL | function | CTFontManager.h | FontManager::scope_for_url |
| CTFontManagerIsSupportedFont | function | CTFontManager.h | FontManager::is_supported_font |
| CTFontManagerRegisterFontsForURL | function | CTFontManager.h | FontManager::register_fonts_for_url |
| CTFontManagerScope | enum | CTFontManager.h | FontManagerScope |
| CTFontManagerSetAutoActivationSetting | function | CTFontManager.h | FontManager::set_auto_activation_setting |
| CTFontManagerUnregisterFontsForURL | function | CTFontManager.h | FontManager::unregister_fonts_for_url |
| CTFontStylisticClass | options | CTFontTraits.h | FontTraits::stylistic_class |
| CTFontSymbolicTraits | options | CTFontTraits.h | FontTraits, symbolic_traits |
| kCTFontSlantTrait | constant | CTFontTraits.h | CTFont::traits, FontDescriptor::traits |
| kCTFontSymbolicTrait | constant | CTFontTraits.h | CTFont::traits, FontDescriptor::traits |
| kCTFontWeightTrait | constant | CTFontTraits.h | CTFont::traits, FontDescriptor::traits |
| kCTFontWidthTrait | constant | CTFontTraits.h | CTFont::traits, FontDescriptor::traits |
| CTFrameGetFrameAttributes | function | CTFrame.h | CTFrame::has_frame_attributes |
| CTFrameGetLineOrigins | function | CTFrame.h | CTFrame::line_origins, ffi::CTFrameGetLineOrigins |
| CTFrameGetLines | function | CTFrame.h | CTFrame::line_origins, CTFrame::lines, ffi::CTFrameGetLines |
| CTFrameGetPath | function | CTFrame.h | CTFrame::path_bounding_box |
| CTFrameGetStringRange | function | CTFrame.h | CTFrame::string_range, ffi::CTFrameGetStringRange |
| CTFrameGetTypeID | function | CTFrame.h | ffi::CTFrameGetTypeID |
| CTFrameGetVisibleStringRange | function | CTFrame.h | CTFrame::visible_string_range, ffi::CTFrameGetVisibleStringRange |
| CTFrameRef | opaque type | CTFrame.h | CTFrame, ffi::CTFrameRef |
| CTFramesetterCreateFrame | function | CTFramesetter.h | CTFramesetter::create_frame_in_rect, ffi::CTFramesetterCreateFrame |
| CTFramesetterCreateWithAttributedString | function | CTFramesetter.h | CTFramesetter::create_with_attributed_string, ffi::CTFramesetterCreateWithAttributedString |
| CTFramesetterCreateWithTypesetter | function | CTFramesetter.h | CTFramesetter::create_with_typesetter |
| CTFramesetterGetTypeID | function | CTFramesetter.h | ffi::CTFramesetterGetTypeID |
| CTFramesetterGetTypesetter | function | CTFramesetter.h | CTFramesetter::typesetter |
| CTFramesetterRef | opaque type | CTFramesetter.h | CTFramesetter, ffi::CTFramesetterRef |
| CTFramesetterSuggestFrameSizeWithConstraints | function | CTFramesetter.h | CTFramesetter::suggest_frame_size_for_range, ffi::CTFramesetterSuggestFrameSizeWithConstraints |
| CTCharacterCollection | enum | CTGlyphInfo.h | CharacterCollection |
| CTGlyphInfoCreateWithCharacterIdentifier | function | CTGlyphInfo.h | GlyphInfo::with_character_identifier |
| CTGlyphInfoCreateWithGlyph | function | CTGlyphInfo.h | GlyphInfo::with_glyph |
| CTGlyphInfoCreateWithGlyphName | function | CTGlyphInfo.h | GlyphInfo::with_glyph_name |
| CTGlyphInfoGetCharacterCollection | function | CTGlyphInfo.h | GlyphInfo::character_collection |
| CTGlyphInfoGetCharacterIdentifier | function | CTGlyphInfo.h | GlyphInfo::character_identifier |
| CTGlyphInfoGetGlyph | function | CTGlyphInfo.h | GlyphInfo::glyph |
| CTGlyphInfoGetGlyphName | function | CTGlyphInfo.h | GlyphInfo::glyph_name |
| CTGlyphInfoRef | opaque type | CTGlyphInfo.h | GlyphInfo |
| CTLineBoundsOptions | options | CTLine.h | bounds_options, ffi::CTLineBoundsOptions |
| CTLineCreateJustifiedLine | function | CTLine.h | CTLine::justified |
| CTLineCreateTruncatedLine | function | CTLine.h | CTLine::truncated |
| CTLineCreateWithAttributedString | function | CTLine.h | CTLine::create_with_attributed_string, ffi::CTLineCreateWithAttributedString |
| CTLineGetBoundsWithOptions | function | CTLine.h | CTLine::bounds_with_options, ffi::CTLineGetBoundsWithOptions |
| CTLineGetGlyphCount | function | CTLine.h | CTLine::glyph_count, ffi::CTLineGetGlyphCount |
| CTLineGetGlyphRuns | function | CTLine.h | CTLine::runs, ffi::CTLineGetGlyphRuns |
| CTLineGetImageBounds | function | CTLine.h | CTLine::image_bounds |
| CTLineGetOffsetForStringIndex | function | CTLine.h | CTLine::offset_for_string_index |
| CTLineGetPenOffsetForFlush | function | CTLine.h | CTLine::pen_offset_for_flush, ffi::CTLineGetPenOffsetForFlush |
| CTLineGetStringIndexForPosition | function | CTLine.h | CTLine::string_index_for_position |
| CTLineGetStringRange | function | CTLine.h | CTLine::string_range, ffi::CTLineGetStringRange |
| CTLineGetTrailingWhitespaceWidth | function | CTLine.h | CTLine::trailing_whitespace_width, ffi::CTLineGetTrailingWhitespaceWidth |
| CTLineGetTypeID | function | CTLine.h | ffi::CTLineGetTypeID |
| CTLineGetTypographicBounds | function | CTLine.h | CTLine::typographic_bounds, ffi::CTLineGetTypographicBounds |
| CTLineRef | opaque type | CTLine.h | CTLine, ffi::CTLineRef |
| CTLineTruncationType | enum | CTLine.h | LineTruncationType |
| CTLineBreakMode | enum | CTParagraphStyle.h | LineBreakMode |
| CTParagraphStyleCreate | function | CTParagraphStyle.h | ParagraphStyle::new, ffi::CTParagraphStyleCreate |
| CTParagraphStyleCreateCopy | function | CTParagraphStyle.h | ParagraphStyle::copy |
| CTParagraphStyleGetTypeID | function | CTParagraphStyle.h | ffi::CTParagraphStyleGetTypeID |
| CTParagraphStyleGetValueForSpecifier | function | CTParagraphStyle.h | ParagraphStyle::tab_stops, ffi::CTParagraphStyleGetValueForSpecifier |
| CTParagraphStyleRef | opaque type | CTParagraphStyle.h | ParagraphStyle, ffi::CTParagraphStyleRef |
| CTParagraphStyleSetting | struct | CTParagraphStyle.h | ffi::CTParagraphStyleSetting |
| CTParagraphStyleSpecifier | enum | CTParagraphStyle.h | ffi::CTParagraphStyleSpecifier |
| CTTextAlignment | enum | CTParagraphStyle.h | TextAlignment, ffi::CTTextAlignment |
| CTWritingDirection | enum | CTParagraphStyle.h | WritingDirection |
| CTRubyAlignment | enum | CTRubyAnnotation.h | RubyAlignment |
| CTRubyAnnotationCreateCopy | function | CTRubyAnnotation.h | RubyAnnotation::copy |
| CTRubyAnnotationGetAlignment | function | CTRubyAnnotation.h | RubyAnnotation::alignment |
| CTRubyAnnotationGetOverhang | function | CTRubyAnnotation.h | RubyAnnotation::overhang |
| CTRubyAnnotationGetSizeFactor | function | CTRubyAnnotation.h | RubyAnnotation::size_factor |
| CTRubyAnnotationGetTextForPosition | function | CTRubyAnnotation.h | RubyAnnotation::text_for_position |
| CTRubyAnnotationRef | opaque type | CTRubyAnnotation.h | RubyAnnotation |
| CTRubyOverhang | enum | CTRubyAnnotation.h | RubyOverhang |
| CTRubyPosition | enum | CTRubyAnnotation.h | RubyPosition |
| CTRunGetAdvances | function | CTRun.h | CTRun::advances, ffi::CTRunGetAdvances |
| CTRunGetAdvancesPtr | function | CTRun.h | ffi::CTRunGetAdvancesPtr |
| CTRunGetAttributes | function | CTRun.h | CTRun::attributes_json |
| CTRunGetBaseAdvancesAndOrigins | function | CTRun.h | CTRun::base_advances_and_origins |
| CTRunGetGlyphCount | function | CTRun.h | CTRun::advances, CTRun::base_advances_and_origins, CTRun::glyph_count, CTRun::glyphs, CTRun::positions, … |
| CTRunGetGlyphs | function | CTRun.h | CTRun::glyphs, ffi::CTRunGetGlyphs |
| CTRunGetGlyphsPtr | function | CTRun.h | ffi::CTRunGetGlyphsPtr |
| CTRunGetImageBounds | function | CTRun.h | CTRun::image_bounds |
| CTRunGetPositions | function | CTRun.h | CTRun::positions, ffi::CTRunGetPositions |
| CTRunGetPositionsPtr | function | CTRun.h | ffi::CTRunGetPositionsPtr |
| CTRunGetStatus | function | CTRun.h | CTRun::status, ffi::CTRunGetStatus |
| CTRunGetStringIndices | function | CTRun.h | CTRun::string_indices, ffi::CTRunGetStringIndices |
| CTRunGetStringIndicesPtr | function | CTRun.h | ffi::CTRunGetStringIndicesPtr |
| CTRunGetStringRange | function | CTRun.h | CTRun::string_range, ffi::CTRunGetStringRange |
| CTRunGetTextMatrix | function | CTRun.h | CTRun::text_matrix |
| CTRunGetTypeID | function | CTRun.h | ffi::CTRunGetTypeID |
| CTRunGetTypographicBounds | function | CTRun.h | CTRun::typographic_bounds, ffi::CTRunGetTypographicBounds |
| CTRunRef | opaque type | CTRun.h | CTRun, ffi::CTRunRef |
| CTRunStatus | options | CTRun.h | ffi::CTRunStatus, run_status |
| kCTFontAttributeName | constant | CTStringAttributes.h | AttributedString::new, ffi::kCTFontAttributeName |
| kCTParagraphStyleAttributeName | constant | CTStringAttributes.h | AttributedString::new, ffi::kCTParagraphStyleAttributeName |
| CTTextTabCreate | function | CTTextTab.h | TextTab::new |
| CTTextTabGetAlignment | function | CTTextTab.h | TextTab::alignment |
| CTTextTabGetLocation | function | CTTextTab.h | TextTab::location |
| CTTextTabRef | opaque type | CTTextTab.h | TextTab |
| CTTypesetterCreateLine | function | CTTypesetter.h | CTTypesetter::create_line |
| CTTypesetterCreateLineWithOffset | function | CTTypesetter.h | CTTypesetter::create_line_with_offset |
| CTTypesetterCreateWithAttributedString | function | CTTypesetter.h | CTTypesetter::create_with_attributed_string |
| CTTypesetterCreateWithAttributedStringAndOptions | function | CTTypesetter.h | CTTypesetter::create_with_options |
| CTTypesetterRef | opaque type | CTTypesetter.h | CTTypesetter |
| CTTypesetterSuggestClusterBreak | function | CTTypesetter.h | CTTypesetter::suggest_cluster_break |
| CTTypesetterSuggestClusterBreakWithOffset | function | CTTypesetter.h | CTTypesetter::suggest_cluster_break_with_offset |
| CTTypesetterSuggestLineBreak | function | CTTypesetter.h | CTTypesetter::suggest_line_break |
| CTTypesetterSuggestLineBreakWithOffset | function | CTTypesetter.h | CTTypesetter::suggest_line_break_with_offset |
| kCTTypesetterOptionAllowUnboundedLayout | constant | CTTypesetter.h | CTTypesetter::create_with_options |
| kCTTypesetterOptionForcedEmbeddingLevel | constant | CTTypesetter.h | CTTypesetter::create_with_options |
| CTFontCopyAttribute | function | CTFont.h | ffi::CTFontCopyAttribute |
| CTFontCopyCharacterSet | function | CTFont.h | ffi::CTFontCopyCharacterSet |
| CTFontCopyDefaultCascadeListForLanguages | function | CTFont.h | ffi::CTFontCopyDefaultCascadeListForLanguages |
| CTFontCopyGraphicsFont | function | CTFont.h | ffi::CTFontCopyGraphicsFont |
| CTFontCopyTable | function | CTFont.h | ffi::CTFontCopyTable |
| CTFontCreatePathForGlyph | function | CTFont.h | ffi::CTFontCreatePathForGlyph |
| CTFontCreateWithFontDescriptorAndOptions | function | CTFont.h | ffi::CTFontCreateWithFontDescriptorAndOptions |
| CTFontCreateWithGraphicsFont | function | CTFont.h | ffi::CTFontCreateWithGraphicsFont |
| CTFontCreateWithNameAndOptions | function | CTFont.h | ffi::CTFontCreateWithNameAndOptions |
| CTFontDrawGlyphs | function | CTFont.h | ffi::CTFontDrawGlyphs |
| CTFontDrawImageFromAdaptiveImageProviderAtPoint | function | CTFont.h | ffi::CTFontDrawImageFromAdaptiveImageProviderAtPoint; CTFont::draw_image_from_adaptive_image_provider_at_point |
| CTAdaptiveImageProviding | protocol | CTFont.h; CTRunDelegate.h | adaptive_image::{AdaptiveImageProviding, AdaptiveImageProvider, AdaptiveImageResponse} |
| CTFontGetLigatureCaretPositions | function | CTFont.h | ffi::CTFontGetLigatureCaretPositions |
| CTFontGetStringEncoding | function | CTFont.h | ffi::CTFontGetStringEncoding |
| CTFontGetTypographicBoundsForAdaptiveImageProvider | function | CTFont.h | ffi::CTFontGetTypographicBoundsForAdaptiveImageProvider; CTFont::typographic_bounds_for_adaptive_image_provider |
| CTFontOptions | options | CTFont.h | ffi::CTFontOptions |
| CTFontTableOptions | options | CTFont.h | ffi::CTFontTableOptions |
| kCTBaselineClassHanging | constant | CTFont.h | ffi::kCTBaselineClassHanging |
| kCTBaselineClassIdeographicCentered | constant | CTFont.h | ffi::kCTBaselineClassIdeographicCentered |
| kCTBaselineClassIdeographicHigh | constant | CTFont.h | ffi::kCTBaselineClassIdeographicHigh |
| kCTBaselineClassIdeographicLow | constant | CTFont.h | ffi::kCTBaselineClassIdeographicLow |
| kCTBaselineClassMath | constant | CTFont.h | ffi::kCTBaselineClassMath |
| kCTBaselineClassRoman | constant | CTFont.h | ffi::kCTBaselineClassRoman |
| kCTBaselineOriginalFont | constant | CTFont.h | ffi::kCTBaselineOriginalFont |
| kCTBaselineReferenceFont | constant | CTFont.h | ffi::kCTBaselineReferenceFont |
| CTFontCollectionCopyExclusionDescriptors | function | CTFontCollection.h | ffi::CTFontCollectionCopyExclusionDescriptors |
| CTFontCollectionCopyFontAttribute | function | CTFontCollection.h | ffi::CTFontCollectionCopyFontAttribute |
| CTFontCollectionCopyFontAttributes | function | CTFontCollection.h | ffi::CTFontCollectionCopyFontAttributes |
| CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback | function | CTFontCollection.h | ffi::CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback |
| CTFontCollectionCreateMatchingFontDescriptorsWithOptions | function | CTFontCollection.h | ffi::CTFontCollectionCreateMatchingFontDescriptorsWithOptions |
| CTFontCollectionCreateMutableCopy | function | CTFontCollection.h | ffi::CTFontCollectionCreateMutableCopy |
| CTFontCollectionGetTypeID | function | CTFontCollection.h | ffi::CTFontCollectionGetTypeID |
| CTFontCollectionSetExclusionDescriptors | function | CTFontCollection.h | ffi::CTFontCollectionSetExclusionDescriptors |
| CTFontCollectionSetQueryDescriptors | function | CTFontCollection.h | ffi::CTFontCollectionSetQueryDescriptors |
| CTMutableFontCollectionRef | opaque type | CTFontCollection.h | ffi::CTMutableFontCollectionRef |
| CTFontDescriptorCopyAttributes | function | CTFontDescriptor.h | ffi::CTFontDescriptorCopyAttributes |
| CTFontDescriptorCopyLocalizedAttribute | function | CTFontDescriptor.h | ffi::CTFontDescriptorCopyLocalizedAttribute |
| CTFontDescriptorCreateCopyWithAttributes | function | CTFontDescriptor.h | ffi::CTFontDescriptorCreateCopyWithAttributes |
| CTFontDescriptorCreateWithAttributes | function | CTFontDescriptor.h | ffi::CTFontDescriptorCreateWithAttributes |
| CTFontDescriptorGetTypeID | function | CTFontDescriptor.h | ffi::CTFontDescriptorGetTypeID |
| CTFontDescriptorMatchFontDescriptorsWithProgressHandler | function | CTFontDescriptor.h | ffi::CTFontDescriptorMatchFontDescriptorsWithProgressHandler |
| CTFontDescriptorMatchingState | enum | CTFontDescriptor.h | ffi::CTFontDescriptorMatchingState |
| kCTFontBaselineAdjustAttribute | constant | CTFontDescriptor.h | ffi::kCTFontBaselineAdjustAttribute |
| kCTFontCascadeListAttribute | constant | CTFontDescriptor.h | ffi::kCTFontCascadeListAttribute |
| kCTFontCharacterSetAttribute | constant | CTFontDescriptor.h | ffi::kCTFontCharacterSetAttribute |
| kCTFontDescriptorMatchingCurrentAssetSize | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingCurrentAssetSize |
| kCTFontDescriptorMatchingDescriptors | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingDescriptors |
| kCTFontDescriptorMatchingError | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingError |
| kCTFontDescriptorMatchingPercentage | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingPercentage |
| kCTFontDescriptorMatchingResult | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingResult |
| kCTFontDescriptorMatchingSourceDescriptor | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingSourceDescriptor |
| kCTFontDescriptorMatchingTotalAssetSize | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingTotalAssetSize |
| kCTFontDescriptorMatchingTotalDownloadedSize | constant | CTFontDescriptor.h | ffi::kCTFontDescriptorMatchingTotalDownloadedSize |
| kCTFontDownloadedAttribute | constant | CTFontDescriptor.h | ffi::kCTFontDownloadedAttribute |
| kCTFontFixedAdvanceAttribute | constant | CTFontDescriptor.h | ffi::kCTFontFixedAdvanceAttribute |
| kCTFontLanguagesAttribute | constant | CTFontDescriptor.h | ffi::kCTFontLanguagesAttribute |
| kCTFontMacintoshEncodingsAttribute | constant | CTFontDescriptor.h | ffi::kCTFontMacintoshEncodingsAttribute |
| kCTFontMatrixAttribute | constant | CTFontDescriptor.h | ffi::kCTFontMatrixAttribute |
| kCTFontOpticalSizeAttribute | constant | CTFontDescriptor.h | ffi::kCTFontOpticalSizeAttribute |
| kCTFontPriorityAttribute | constant | CTFontDescriptor.h | ffi::kCTFontPriorityAttribute |
| kCTFontRegistrationScopeAttribute | constant | CTFontDescriptor.h | ffi::kCTFontRegistrationScopeAttribute |
| CTFontManagerCompareFontFamilyNames | function | CTFontManager.h | ffi::CTFontManagerCompareFontFamilyNames |
| CTFontManagerCreateFontDescriptorFromData | function | CTFontManager.h | ffi::CTFontManagerCreateFontDescriptorFromData |
| CTFontManagerCreateFontDescriptorsFromData | function | CTFontManager.h | ffi::CTFontManagerCreateFontDescriptorsFromData |
| CTFontManagerEnableFontDescriptors | function | CTFontManager.h | ffi::CTFontManagerEnableFontDescriptors |
| CTFontManagerRegisterFontDescriptors | function | CTFontManager.h | ffi::CTFontManagerRegisterFontDescriptors |
| CTFontManagerRegisterFontURLs | function | CTFontManager.h | ffi::CTFontManagerRegisterFontURLs |
| CTFontManagerUnregisterFontDescriptors | function | CTFontManager.h | ffi::CTFontManagerUnregisterFontDescriptors |
| CTFontManagerUnregisterFontURLs | function | CTFontManager.h | ffi::CTFontManagerUnregisterFontURLs |
| kCTFontManagerBundleIdentifier | constant | CTFontManager.h | ffi::kCTFontManagerBundleIdentifier |
| kCTFontManagerRegisteredFontsChangedNotification | constant | CTFontManager.h | ffi::kCTFontManagerRegisteredFontsChangedNotification |
| CTFontManagerError | enum | CTFontManagerErrors.h | ffi::CTFontManagerError |
| kCTFontManagerErrorDomain | constant | CTFontManagerErrors.h | ffi::kCTFontManagerErrorDomain |
| kCTFontManagerErrorFontURLsKey | constant | CTFontManagerErrors.h | ffi::kCTFontManagerErrorFontURLsKey |
| CTFrameDraw | function | CTFrame.h | ffi::CTFrameDraw |
| CTFramePathFillRule | enum | CTFrame.h | ffi::CTFramePathFillRule |
| CTFrameProgression | enum | CTFrame.h | ffi::CTFrameProgression |
| kCTFrameClippingPathsAttributeName | constant | CTFrame.h | ffi::kCTFrameClippingPathsAttributeName |
| kCTFramePathClippingPathAttributeName | constant | CTFrame.h | ffi::kCTFramePathClippingPathAttributeName |
| kCTFramePathFillRuleAttributeName | constant | CTFrame.h | ffi::kCTFramePathFillRuleAttributeName |
| kCTFramePathWidthAttributeName | constant | CTFrame.h | ffi::kCTFramePathWidthAttributeName |
| kCTFrameProgressionAttributeName | constant | CTFrame.h | ffi::kCTFrameProgressionAttributeName |
| CTGlyphInfoGetTypeID | function | CTGlyphInfo.h | ffi::CTGlyphInfoGetTypeID |
| CTLineDraw | function | CTLine.h | ffi::CTLineDraw |
| CTLineEnumerateCaretOffsets | function | CTLine.h | ffi::CTLineEnumerateCaretOffsets |
| CTRubyAnnotationCreateWithAttributes | function | CTRubyAnnotation.h | ffi::CTRubyAnnotationCreateWithAttributes |
| CTRubyAnnotationGetTypeID | function | CTRubyAnnotation.h | ffi::CTRubyAnnotationGetTypeID |
| kCTRubyAnnotationScaleToFitAttributeName | constant | CTRubyAnnotation.h | ffi::kCTRubyAnnotationScaleToFitAttributeName |
| kCTRubyAnnotationSizeFactorAttributeName | constant | CTRubyAnnotation.h | ffi::kCTRubyAnnotationSizeFactorAttributeName |
| kCTRubyPositionCount | constant | CTRubyAnnotation.h | ffi::kCTRubyPositionCount |
| CTRunDraw | function | CTRun.h | ffi::CTRunDraw |
| CTRunDelegateCallbacks | struct | CTRunDelegate.h | ffi::CTRunDelegateCallbacks |
| CTRunDelegateCreate | function | CTRunDelegate.h | ffi::CTRunDelegateCreate |
| CTRunDelegateGetRefCon | function | CTRunDelegate.h | ffi::CTRunDelegateGetRefCon |
| CTRunDelegateGetTypeID | function | CTRunDelegate.h | ffi::CTRunDelegateGetTypeID |
| CTRunDelegateRef | opaque type | CTRunDelegate.h | ffi::CTRunDelegateRef |
| CTUnderlineStyle | options | CTStringAttributes.h | ffi::CTUnderlineStyle |
| CTUnderlineStyleModifiers | options | CTStringAttributes.h | ffi::CTUnderlineStyleModifiers |
| kCTAdaptiveImageProviderAttributeName | constant | CTStringAttributes.h | ffi::kCTAdaptiveImageProviderAttributeName |
| kCTBackgroundColorAttributeName | constant | CTStringAttributes.h | ffi::kCTBackgroundColorAttributeName |
| kCTBaselineClassAttributeName | constant | CTStringAttributes.h | ffi::kCTBaselineClassAttributeName |
| kCTBaselineInfoAttributeName | constant | CTStringAttributes.h | ffi::kCTBaselineInfoAttributeName |
| kCTBaselineOffsetAttributeName | constant | CTStringAttributes.h | ffi::kCTBaselineOffsetAttributeName |
| kCTBaselineReferenceInfoAttributeName | constant | CTStringAttributes.h | ffi::kCTBaselineReferenceInfoAttributeName |
| kCTForegroundColorAttributeName | constant | CTStringAttributes.h | ffi::kCTForegroundColorAttributeName |
| kCTForegroundColorFromContextAttributeName | constant | CTStringAttributes.h | ffi::kCTForegroundColorFromContextAttributeName |
| kCTGlyphInfoAttributeName | constant | CTStringAttributes.h | ffi::kCTGlyphInfoAttributeName |
| kCTHorizontalInVerticalFormsAttributeName | constant | CTStringAttributes.h | ffi::kCTHorizontalInVerticalFormsAttributeName |
| kCTKernAttributeName | constant | CTStringAttributes.h | ffi::kCTKernAttributeName |
| kCTLanguageAttributeName | constant | CTStringAttributes.h | ffi::kCTLanguageAttributeName |
| kCTLigatureAttributeName | constant | CTStringAttributes.h | ffi::kCTLigatureAttributeName |
| kCTRubyAnnotationAttributeName | constant | CTStringAttributes.h | ffi::kCTRubyAnnotationAttributeName |
| kCTRunDelegateAttributeName | constant | CTStringAttributes.h | ffi::kCTRunDelegateAttributeName |
| kCTStrokeColorAttributeName | constant | CTStringAttributes.h | ffi::kCTStrokeColorAttributeName |
| kCTStrokeWidthAttributeName | constant | CTStringAttributes.h | ffi::kCTStrokeWidthAttributeName |
| kCTSuperscriptAttributeName | constant | CTStringAttributes.h | ffi::kCTSuperscriptAttributeName |
| kCTTrackingAttributeName | constant | CTStringAttributes.h | ffi::kCTTrackingAttributeName |
| kCTUnderlineColorAttributeName | constant | CTStringAttributes.h | ffi::kCTUnderlineColorAttributeName |
| kCTUnderlineStyleAttributeName | constant | CTStringAttributes.h | ffi::kCTUnderlineStyleAttributeName |
| kCTVerticalFormsAttributeName | constant | CTStringAttributes.h | ffi::kCTVerticalFormsAttributeName |
| kCTWritingDirectionAttributeName | constant | CTStringAttributes.h | ffi::kCTWritingDirectionAttributeName |
| CTTextTabGetOptions | function | CTTextTab.h | ffi::CTTextTabGetOptions |
| CTTextTabGetTypeID | function | CTTextTab.h | ffi::CTTextTabGetTypeID |
| kCTTabColumnTerminatorsAttributeName | constant | CTTextTab.h | ffi::kCTTabColumnTerminatorsAttributeName |
| CTTypesetterGetTypeID | function | CTTypesetter.h | ffi::CTTypesetterGetTypeID |
| AnchorPoint | struct | SFNTLayoutTypes.h | ffi::AnchorPoint |
| AnchorPointTable | struct | SFNTLayoutTypes.h | ffi::AnchorPointTable |
| AnkrTable | struct | SFNTLayoutTypes.h | ffi::AnkrTable |
| BslnFormat0Part | struct | SFNTLayoutTypes.h | ffi::BslnFormat0Part |
| BslnFormat1Part | struct | SFNTLayoutTypes.h | ffi::BslnFormat1Part |
| BslnFormat2Part | struct | SFNTLayoutTypes.h | ffi::BslnFormat2Part |
| BslnFormat3Part | struct | SFNTLayoutTypes.h | ffi::BslnFormat3Part |
| BslnTable | struct | SFNTLayoutTypes.h | ffi::BslnTable |
| JustDirectionTable | struct | SFNTLayoutTypes.h | ffi::JustDirectionTable |
| JustPCAction | struct | SFNTLayoutTypes.h | ffi::JustPCAction |
| JustPCActionSubrecord | struct | SFNTLayoutTypes.h | ffi::JustPCActionSubrecord |
| JustPCConditionalAddAction | struct | SFNTLayoutTypes.h | ffi::JustPCConditionalAddAction |
| JustPCDecompositionAction | struct | SFNTLayoutTypes.h | ffi::JustPCDecompositionAction |
| JustPCDuctilityAction | struct | SFNTLayoutTypes.h | ffi::JustPCDuctilityAction |
| JustPCGlyphRepeatAddAction | struct | SFNTLayoutTypes.h | ffi::JustPCGlyphRepeatAddAction |
| JustPostcompTable | struct | SFNTLayoutTypes.h | ffi::JustPostcompTable |
| JustTable | struct | SFNTLayoutTypes.h | ffi::JustTable |
| JustWidthDeltaEntry | struct | SFNTLayoutTypes.h | ffi::JustWidthDeltaEntry |
| JustWidthDeltaGroup | struct | SFNTLayoutTypes.h | ffi::JustWidthDeltaGroup |
| KernIndexArrayHeader | struct | SFNTLayoutTypes.h | ffi::KernIndexArrayHeader |
| KernKerningPair | struct | SFNTLayoutTypes.h | ffi::KernKerningPair |
| KernOffsetTable | struct | SFNTLayoutTypes.h | ffi::KernOffsetTable |
| KernOrderedListEntry | struct | SFNTLayoutTypes.h | ffi::KernOrderedListEntry |
| KernOrderedListHeader | struct | SFNTLayoutTypes.h | ffi::KernOrderedListHeader |
| KernSimpleArrayHeader | struct | SFNTLayoutTypes.h | ffi::KernSimpleArrayHeader |
| KernStateEntry | struct | SFNTLayoutTypes.h | ffi::KernStateEntry |
| KernStateHeader | struct | SFNTLayoutTypes.h | ffi::KernStateHeader |
| KernSubtableHeader | struct | SFNTLayoutTypes.h | ffi::KernSubtableHeader |
| KernTableHeader | struct | SFNTLayoutTypes.h | ffi::KernTableHeader |
| KernVersion0Header | struct | SFNTLayoutTypes.h | ffi::KernVersion0Header |
| KernVersion0SubtableHeader | struct | SFNTLayoutTypes.h | ffi::KernVersion0SubtableHeader |
| KerxAnchorPointAction | struct | SFNTLayoutTypes.h | ffi::KerxAnchorPointAction |
| KerxControlPointAction | struct | SFNTLayoutTypes.h | ffi::KerxControlPointAction |
| KerxControlPointEntry | struct | SFNTLayoutTypes.h | ffi::KerxControlPointEntry |
| KerxControlPointHeader | struct | SFNTLayoutTypes.h | ffi::KerxControlPointHeader |
| KerxCoordinateAction | struct | SFNTLayoutTypes.h | ffi::KerxCoordinateAction |
| KerxIndexArrayHeader | struct | SFNTLayoutTypes.h | ffi::KerxIndexArrayHeader |
| KerxKerningPair | struct | SFNTLayoutTypes.h | ffi::KerxKerningPair |
| KerxOrderedListEntry | struct | SFNTLayoutTypes.h | ffi::KerxOrderedListEntry |
| KerxOrderedListHeader | struct | SFNTLayoutTypes.h | ffi::KerxOrderedListHeader |
| KerxSimpleArrayHeader | struct | SFNTLayoutTypes.h | ffi::KerxSimpleArrayHeader |
| KerxStateEntry | struct | SFNTLayoutTypes.h | ffi::KerxStateEntry |
| KerxStateHeader | struct | SFNTLayoutTypes.h | ffi::KerxStateHeader |
| KerxSubtableHeader | struct | SFNTLayoutTypes.h | ffi::KerxSubtableHeader |
| KerxTableHeader | struct | SFNTLayoutTypes.h | ffi::KerxTableHeader |
| LcarCaretClassEntry | struct | SFNTLayoutTypes.h | ffi::LcarCaretClassEntry |
| LcarCaretTable | struct | SFNTLayoutTypes.h | ffi::LcarCaretTable |
| LtagStringRange | struct | SFNTLayoutTypes.h | ffi::LtagStringRange |
| LtagTable | struct | SFNTLayoutTypes.h | ffi::LtagTable |
| MortChain | struct | SFNTLayoutTypes.h | ffi::MortChain |
| MortContextualSubtable | struct | SFNTLayoutTypes.h | ffi::MortContextualSubtable |
| MortFeatureEntry | struct | SFNTLayoutTypes.h | ffi::MortFeatureEntry |
| MortInsertionSubtable | struct | SFNTLayoutTypes.h | ffi::MortInsertionSubtable |
| MortLigatureSubtable | struct | SFNTLayoutTypes.h | ffi::MortLigatureSubtable |
| MortRearrangementSubtable | struct | SFNTLayoutTypes.h | ffi::MortRearrangementSubtable |
| MortSubtable | struct | SFNTLayoutTypes.h | ffi::MortSubtable |
| MortSwashSubtable | struct | SFNTLayoutTypes.h | ffi::MortSwashSubtable |
| MortTable | struct | SFNTLayoutTypes.h | ffi::MortTable |
| MorxChain | struct | SFNTLayoutTypes.h | ffi::MorxChain |
| MorxContextualSubtable | struct | SFNTLayoutTypes.h | ffi::MorxContextualSubtable |
| MorxInsertionSubtable | struct | SFNTLayoutTypes.h | ffi::MorxInsertionSubtable |
| MorxLigatureSubtable | struct | SFNTLayoutTypes.h | ffi::MorxLigatureSubtable |
| MorxRearrangementSubtable | struct | SFNTLayoutTypes.h | ffi::MorxRearrangementSubtable |
| MorxSubtable | struct | SFNTLayoutTypes.h | ffi::MorxSubtable |
| MorxTable | struct | SFNTLayoutTypes.h | ffi::MorxTable |
| OpbdSideValues | struct | SFNTLayoutTypes.h | ffi::OpbdSideValues |
| OpbdTable | struct | SFNTLayoutTypes.h | ffi::OpbdTable |
| PropLookupSegment | struct | SFNTLayoutTypes.h | ffi::PropLookupSegment |
| PropLookupSingle | struct | SFNTLayoutTypes.h | ffi::PropLookupSingle |
| PropTable | struct | SFNTLayoutTypes.h | ffi::PropTable |
| SFNTLookupArrayHeader | struct | SFNTLayoutTypes.h | ffi::SFNTLookupArrayHeader |
| SFNTLookupBinarySearchHeader | struct | SFNTLayoutTypes.h | ffi::SFNTLookupBinarySearchHeader |
| SFNTLookupSegment | struct | SFNTLayoutTypes.h | ffi::SFNTLookupSegment |
| SFNTLookupSegmentHeader | struct | SFNTLayoutTypes.h | ffi::SFNTLookupSegmentHeader |
| SFNTLookupSingle | struct | SFNTLayoutTypes.h | ffi::SFNTLookupSingle |
| SFNTLookupSingleHeader | struct | SFNTLayoutTypes.h | ffi::SFNTLookupSingleHeader |
| SFNTLookupTable | struct | SFNTLayoutTypes.h | ffi::SFNTLookupTable |
| SFNTLookupTrimmedArrayHeader | struct | SFNTLayoutTypes.h | ffi::SFNTLookupTrimmedArrayHeader |
| SFNTLookupVectorHeader | struct | SFNTLayoutTypes.h | ffi::SFNTLookupVectorHeader |
| STClassTable | struct | SFNTLayoutTypes.h | ffi::STClassTable |
| STEntryOne | struct | SFNTLayoutTypes.h | ffi::STEntryOne |
| STEntryTwo | struct | SFNTLayoutTypes.h | ffi::STEntryTwo |
| STEntryZero | struct | SFNTLayoutTypes.h | ffi::STEntryZero |
| STHeader | struct | SFNTLayoutTypes.h | ffi::STHeader |
| STXEntryOne | struct | SFNTLayoutTypes.h | ffi::STXEntryOne |
| STXEntryTwo | struct | SFNTLayoutTypes.h | ffi::STXEntryTwo |
| STXEntryZero | struct | SFNTLayoutTypes.h | ffi::STXEntryZero |
| STXHeader | struct | SFNTLayoutTypes.h | ffi::STXHeader |
| TrakTable | struct | SFNTLayoutTypes.h | ffi::TrakTable |
| TrakTableData | struct | SFNTLayoutTypes.h | ffi::TrakTableData |
| TrakTableEntry | struct | SFNTLayoutTypes.h | ffi::TrakTableEntry |
| FontVariation | struct | SFNTTypes.h | ffi::FontVariation |
| sfntCMapEncoding | struct | SFNTTypes.h | ffi::sfntCMapEncoding |
| sfntCMapExtendedSubHeader | struct | SFNTTypes.h | ffi::sfntCMapExtendedSubHeader |
| sfntCMapHeader | struct | SFNTTypes.h | ffi::sfntCMapHeader |
| sfntCMapSubHeader | struct | SFNTTypes.h | ffi::sfntCMapSubHeader |
| sfntDescriptorHeader | struct | SFNTTypes.h | ffi::sfntDescriptorHeader |
| sfntDirectory | struct | SFNTTypes.h | ffi::sfntDirectory |
| sfntDirectoryEntry | struct | SFNTTypes.h | ffi::sfntDirectoryEntry |
| sfntFeatureHeader | struct | SFNTTypes.h | ffi::sfntFeatureHeader |
| sfntFeatureName | struct | SFNTTypes.h | ffi::sfntFeatureName |
| sfntFontDescriptor | struct | SFNTTypes.h | ffi::sfntFontDescriptor |
| sfntFontFeatureSetting | struct | SFNTTypes.h | ffi::sfntFontFeatureSetting |
| sfntFontRunFeature | struct | SFNTTypes.h | ffi::sfntFontRunFeature |
| sfntInstance | struct | SFNTTypes.h | ffi::sfntInstance |
| sfntNameHeader | struct | SFNTTypes.h | ffi::sfntNameHeader |
| sfntNameRecord | struct | SFNTTypes.h | ffi::sfntNameRecord |
| sfntVariationAxis | struct | SFNTTypes.h | ffi::sfntVariationAxis |
| sfntVariationHeader | struct | SFNTTypes.h | ffi::sfntVariationHeader |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| CTFontCreateWithPlatformFont | function | CTFont.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("ATS is deprecated", macos(10.5, 11.0)) |
| CTFontCreateWithQuickdrawInstance | function | CTFont.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("Quickdraw font references are deprecated", macos(10.5, 10.15)) |
| CTFontGetPlatformFont | function | CTFont.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("ATS is deprecated", macos(10.5, 11.0)) |
| CTFontManagerCreateFontRequestRunLoopSource | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("This functionality will be removed in a future release", macos(10.6, 11.0)) |
| CTFontManagerRegisterFontsForURLs | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED_WITH_REPLACEMENT("CTFontManagerRegisterFontURLs", macos(10.6, 10.15), ios(4.1, 13.0), watchos(2.0, 6.0), tvos(9.0, 13.0)) |
| CTFontManagerRegisterGraphicsFont | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("Use CTFontManagerCreateFontDescriptorsFromData or CTFontManagerRegisterFontsForURL", macos(10.8, 15), ios(4.1, 18), watchos(2, 11), tvos(9, 18)) |
| CTFontManagerUnregisterFontsForURLs | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED_WITH_REPLACEMENT("CTFontManagerUnregisterFontURLs", macos(10.6, 10.15), ios(4.1, 13.0), watchos(2.0, 6.0), tvos(9.0, 13.0)) |
| CTFontManagerUnregisterGraphicsFont | function | CTFontManager.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("Use the API corresponding to the one used to register the font", macos(10.8, 15), ios(4.1, 18), watchos(2, 11), tvos(9, 18)) |
| kCTCharacterShapeAttributeName | constant | CTStringAttributes.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("Use feature type kCharacterShapeType with the appropriate selector", macos(10.5, 10.11), ios(3.2, 9.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| kCTTypesetterOptionDisableBidiProcessing | constant | CTTypesetter.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("Deprecated", macos(10.5, 10.8), ios(3.2, 6.0), watchos(2.0, 2.0), tvos(9.0, 9.0)) |
| CTGetCoreTextVersion | function | CoreText.h | Deprecated on macOS; intentionally excluded from the wrapper audit. | CT_DEPRECATED("Use -[NSProcessInfo operatingSystemVersion]", macos(10.5, 11.0), ios(3.2, 14.0), watchos(2.0, 7.0), tvos(9.0, 14.0)) |
