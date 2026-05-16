# coretext-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 478
VERIFIED: 234
GAPS: 233
EXEMPT: 11
COVERAGE_PCT: 50.11%

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

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| CTFontCopyAttribute | function | CTFont.h | Specific font accessors are wrapped, but generic attribute lookup is not. |
| CTFontCopyCharacterSet | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontCopyDefaultCascadeListForLanguages | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontCopyGraphicsFont | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontCopyTable | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontCreatePathForGlyph | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontCreateWithFontDescriptorAndOptions | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontCreateWithGraphicsFont | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontCreateWithNameAndOptions | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontDrawGlyphs | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontDrawImageFromAdaptiveImageProviderAtPoint | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontGetLigatureCaretPositions | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontGetStringEncoding | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontGetTypographicBoundsForAdaptiveImageProvider | function | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontOptions | options | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| CTFontTableOptions | options | CTFont.h | Advanced font graphics/cascade/table APIs are not wrapped. |
| kCTBaselineClassHanging | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| kCTBaselineClassIdeographicCentered | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| kCTBaselineClassIdeographicHigh | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| kCTBaselineClassIdeographicLow | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| kCTBaselineClassMath | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| kCTBaselineClassRoman | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| kCTBaselineOriginalFont | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| kCTBaselineReferenceFont | constant | CTFont.h | No public constant exposure or helper uses this SDK key. |
| CTFontCollectionCopyExclusionDescriptors | function | CTFontCollection.h | Collection mutation/full-attribute APIs are not wrapped. |
| CTFontCollectionCopyFontAttribute | function | CTFontCollection.h | Collection mutation/full-attribute APIs are not wrapped. |
| CTFontCollectionCopyFontAttributes | function | CTFontCollection.h | Collection mutation/full-attribute APIs are not wrapped. |
| CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback | function | CTFontCollection.h | Sorted collection matching callback API is not wrapped. |
| CTFontCollectionCreateMatchingFontDescriptorsWithOptions | function | CTFontCollection.h | Matching-descriptor options are not surfaced. |
| CTFontCollectionCreateMutableCopy | function | CTFontCollection.h | Collection mutation/full-attribute APIs are not wrapped. |
| CTFontCollectionGetTypeID | function | CTFontCollection.h | Type-ID helpers are not exposed outside raw-ffi. |
| CTFontCollectionSetExclusionDescriptors | function | CTFontCollection.h | Collection mutation/full-attribute APIs are not wrapped. |
| CTFontCollectionSetQueryDescriptors | function | CTFontCollection.h | Collection mutation/full-attribute APIs are not wrapped. |
| CTMutableFontCollectionRef | opaque type | CTFontCollection.h | No dedicated wrapper type is exposed. |
| CTFontDescriptorCopyAttributes | function | CTFontDescriptor.h | Specific descriptor getters are wrapped, but full attribute dictionaries are not. |
| CTFontDescriptorCopyLocalizedAttribute | function | CTFontDescriptor.h | Localized descriptor-attribute lookup is not wrapped. |
| CTFontDescriptorCreateCopyWithAttributes | function | CTFontDescriptor.h | No public wrapper method or raw-ffi declaration found. |
| CTFontDescriptorCreateWithAttributes | function | CTFontDescriptor.h | No generic descriptor constructor from an attribute dictionary. |
| CTFontDescriptorGetTypeID | function | CTFontDescriptor.h | No public wrapper method or raw-ffi declaration found. |
| CTFontDescriptorMatchFontDescriptorsWithProgressHandler | function | CTFontDescriptor.h | Descriptor matching/download progress callbacks are not exposed. |
| CTFontDescriptorMatchingState | enum | CTFontDescriptor.h | No Rust API for descriptor matching state callbacks. |
| kCTFontBaselineAdjustAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontCascadeListAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontCharacterSetAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingCurrentAssetSize | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingDescriptors | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingError | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingPercentage | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingResult | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingSourceDescriptor | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingTotalAssetSize | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDescriptorMatchingTotalDownloadedSize | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontDownloadedAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontFixedAdvanceAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontLanguagesAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontMacintoshEncodingsAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontMatrixAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontOpticalSizeAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontPriorityAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| kCTFontRegistrationScopeAttribute | constant | CTFontDescriptor.h | No public constant exposure or helper uses this SDK key. |
| CTFontManagerCompareFontFamilyNames | function | CTFontManager.h | No public wrapper method or raw-ffi declaration found. |
| CTFontManagerCreateFontDescriptorFromData | function | CTFontManager.h | Data-backed font descriptor creation is not wrapped. |
| CTFontManagerCreateFontDescriptorsFromData | function | CTFontManager.h | Data-backed font descriptor creation is not wrapped. |
| CTFontManagerEnableFontDescriptors | function | CTFontManager.h | Batch descriptor/URL registration APIs are not wrapped. |
| CTFontManagerRegisterFontDescriptors | function | CTFontManager.h | Batch descriptor/URL registration APIs are not wrapped. |
| CTFontManagerRegisterFontURLs | function | CTFontManager.h | Batch descriptor/URL registration APIs are not wrapped. |
| CTFontManagerUnregisterFontDescriptors | function | CTFontManager.h | Batch descriptor/URL registration APIs are not wrapped. |
| CTFontManagerUnregisterFontURLs | function | CTFontManager.h | Batch descriptor/URL registration APIs are not wrapped. |
| kCTFontManagerBundleIdentifier | constant | CTFontManager.h | No public constant exposure or helper uses this SDK key. |
| kCTFontManagerRegisteredFontsChangedNotification | constant | CTFontManager.h | No public constant exposure or helper uses this SDK key. |
| CTFontManagerError | enum | CTFontManagerErrors.h | FontManager methods surface string errors, not the error enum. |
| kCTFontManagerErrorDomain | constant | CTFontManagerErrors.h | No public constant exposure or helper uses this SDK key. |
| kCTFontManagerErrorFontURLsKey | constant | CTFontManagerErrors.h | No public constant exposure or helper uses this SDK key. |
| CTFrameDraw | function | CTFrame.h | Drawing APIs are omitted from the wrapper surface. |
| CTFramePathFillRule | enum | CTFrame.h | Frame progression/fill-rule options are not exposed. |
| CTFrameProgression | enum | CTFrame.h | Frame progression/fill-rule options are not exposed. |
| kCTFrameClippingPathsAttributeName | constant | CTFrame.h | No public constant exposure or helper uses this SDK key. |
| kCTFramePathClippingPathAttributeName | constant | CTFrame.h | No public constant exposure or helper uses this SDK key. |
| kCTFramePathFillRuleAttributeName | constant | CTFrame.h | No public constant exposure or helper uses this SDK key. |
| kCTFramePathWidthAttributeName | constant | CTFrame.h | No public constant exposure or helper uses this SDK key. |
| kCTFrameProgressionAttributeName | constant | CTFrame.h | No public constant exposure or helper uses this SDK key. |
| CTGlyphInfoGetTypeID | function | CTGlyphInfo.h | Type-ID helpers are not exposed outside raw-ffi. |
| CTLineDraw | function | CTLine.h | Drawing APIs are omitted from the wrapper surface. |
| CTLineEnumerateCaretOffsets | function | CTLine.h | No public wrapper method or raw-ffi declaration found. |
| CTRubyAnnotationCreateWithAttributes | function | CTRubyAnnotation.h | Per-position ruby attribute dictionaries are not wrapped. |
| CTRubyAnnotationGetTypeID | function | CTRubyAnnotation.h | No public wrapper method or raw-ffi declaration found. |
| kCTRubyAnnotationScaleToFitAttributeName | constant | CTRubyAnnotation.h | No public constant exposure or helper uses this SDK key. |
| kCTRubyAnnotationSizeFactorAttributeName | constant | CTRubyAnnotation.h | No public constant exposure or helper uses this SDK key. |
| kCTRubyPositionCount | constant | CTRubyAnnotation.h | No public constant exposure or helper uses this SDK key. |
| CTRunDraw | function | CTRun.h | Drawing APIs are omitted from the wrapper surface. |
| CTRunDelegateCallbacks | struct | CTRunDelegate.h | Run delegate callbacks and ref-con access are not wrapped. |
| CTRunDelegateCreate | function | CTRunDelegate.h | Run delegate callbacks and ref-con access are not wrapped. |
| CTRunDelegateGetRefCon | function | CTRunDelegate.h | Run delegate callbacks and ref-con access are not wrapped. |
| CTRunDelegateGetTypeID | function | CTRunDelegate.h | Run delegate callbacks and ref-con access are not wrapped. |
| CTRunDelegateRef | opaque type | CTRunDelegate.h | Run delegate callbacks and ref-con access are not wrapped. |
| CTUnderlineStyle | options | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| CTUnderlineStyleModifiers | options | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTAdaptiveImageProviderAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTBackgroundColorAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTBaselineClassAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTBaselineInfoAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTBaselineOffsetAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTBaselineReferenceInfoAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTForegroundColorAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTForegroundColorFromContextAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTGlyphInfoAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTHorizontalInVerticalFormsAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTKernAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTLanguageAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTLigatureAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTRubyAnnotationAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTRunDelegateAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTStrokeColorAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTStrokeWidthAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTSuperscriptAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTTrackingAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTUnderlineColorAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTUnderlineStyleAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTVerticalFormsAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| kCTWritingDirectionAttributeName | constant | CTStringAttributes.h | AttributedString only exposes font and paragraph-style attributes. |
| CTTextTabGetOptions | function | CTTextTab.h | TextTab only exposes alignment and location. |
| CTTextTabGetTypeID | function | CTTextTab.h | Type-ID helpers are not exposed outside raw-ffi. |
| kCTTabColumnTerminatorsAttributeName | constant | CTTextTab.h | No public constant exposure or helper uses this SDK key. |
| CTTypesetterGetTypeID | function | CTTypesetter.h | Type-ID helpers are not exposed outside raw-ffi. |
| AnchorPoint | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| AnchorPointTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| AnkrTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| BslnFormat0Part | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| BslnFormat1Part | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| BslnFormat2Part | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| BslnFormat3Part | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| BslnTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustDirectionTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustPCAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustPCActionSubrecord | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustPCConditionalAddAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustPCDecompositionAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustPCDuctilityAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustPCGlyphRepeatAddAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustPostcompTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustWidthDeltaEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| JustWidthDeltaGroup | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernIndexArrayHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernKerningPair | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernOffsetTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernOrderedListEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernOrderedListHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernSimpleArrayHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernStateEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernStateHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernSubtableHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernTableHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernVersion0Header | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KernVersion0SubtableHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxAnchorPointAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxControlPointAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxControlPointEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxControlPointHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxCoordinateAction | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxIndexArrayHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxKerningPair | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxOrderedListEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxOrderedListHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxSimpleArrayHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxStateEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxStateHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxSubtableHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| KerxTableHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| LcarCaretClassEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| LcarCaretTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| LtagStringRange | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| LtagTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortChain | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortContextualSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortFeatureEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortInsertionSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortLigatureSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortRearrangementSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortSwashSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MortTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MorxChain | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MorxContextualSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MorxInsertionSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MorxLigatureSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MorxRearrangementSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MorxSubtable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| MorxTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| OpbdSideValues | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| OpbdTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| PropLookupSegment | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| PropLookupSingle | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| PropTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupArrayHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupBinarySearchHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupSegment | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupSegmentHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupSingle | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupSingleHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupTrimmedArrayHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| SFNTLookupVectorHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STClassTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STEntryOne | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STEntryTwo | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STEntryZero | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STXEntryOne | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STXEntryTwo | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STXEntryZero | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| STXHeader | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| TrakTable | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| TrakTableData | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| TrakTableEntry | struct | SFNTLayoutTypes.h | No SFNT layout/table surface is exposed. |
| FontVariation | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntCMapEncoding | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntCMapExtendedSubHeader | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntCMapHeader | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntCMapSubHeader | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntDescriptorHeader | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntDirectory | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntDirectoryEntry | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntFeatureHeader | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntFeatureName | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntFontDescriptor | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntFontFeatureSetting | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntFontRunFeature | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntInstance | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntNameHeader | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntNameRecord | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntVariationAxis | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |
| sfntVariationHeader | struct | SFNTTypes.h | No SFNT layout/table surface is exposed. |

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
