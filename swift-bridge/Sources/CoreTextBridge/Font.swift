import CoreFoundation
import CoreGraphics
import CoreText
import Foundation

private func fontNameKey(_ raw: UInt32) -> CFString {
    switch raw {
    case 0: return kCTFontCopyrightNameKey
    case 1: return kCTFontFamilyNameKey
    case 2: return kCTFontSubFamilyNameKey
    case 3: return kCTFontStyleNameKey
    case 4: return kCTFontUniqueNameKey
    case 5: return kCTFontFullNameKey
    case 6: return kCTFontVersionNameKey
    case 7: return kCTFontPostScriptNameKey
    case 8: return kCTFontTrademarkNameKey
    case 9: return kCTFontManufacturerNameKey
    case 10: return kCTFontDesignerNameKey
    case 11: return kCTFontDescriptionNameKey
    case 12: return kCTFontVendorURLNameKey
    case 13: return kCTFontDesignerURLNameKey
    case 14: return kCTFontLicenseNameKey
    case 15: return kCTFontLicenseURLNameKey
    case 16: return kCTFontSampleTextNameKey
    case 17: return kCTFontPostScriptCIDNameKey
    default: return kCTFontFullNameKey
    }
}

private func uiFontType(_ raw: UInt32) -> CTFontUIFontType {
    switch raw {
    case UInt32.max: return kCTFontUIFontNone
    case 1: return kCTFontUIFontUserFixedPitch
    case 2: return kCTFontUIFontSystem
    case 3: return kCTFontUIFontEmphasizedSystem
    case 4: return kCTFontUIFontSmallSystem
    case 5: return kCTFontUIFontSmallEmphasizedSystem
    case 6: return kCTFontUIFontMiniSystem
    case 7: return kCTFontUIFontMiniEmphasizedSystem
    case 8: return kCTFontUIFontViews
    case 9: return kCTFontUIFontApplication
    case 10: return kCTFontUIFontLabel
    case 11: return kCTFontUIFontMenuTitle
    case 12: return kCTFontUIFontMenuItem
    case 13: return kCTFontUIFontMenuItemMark
    case 14: return kCTFontUIFontMenuItemCmdKey
    case 15: return kCTFontUIFontWindowTitle
    case 16: return kCTFontUIFontPushButton
    case 17: return kCTFontUIFontUtilityWindowTitle
    case 18: return kCTFontUIFontAlertHeader
    case 19: return kCTFontUIFontSystemDetail
    case 20: return kCTFontUIFontEmphasizedSystemDetail
    case 21: return kCTFontUIFontToolbar
    case 22: return kCTFontUIFontSmallToolbar
    case 23: return kCTFontUIFontMessage
    case 24: return kCTFontUIFontPalette
    case 25: return kCTFontUIFontToolTip
    case 26: return kCTFontUIFontControlContent
    default: return kCTFontUIFontUser
    }
}

private func fontOrientation(_ raw: UInt32) -> CTFontOrientation {
    switch raw {
    case 1: return kCTFontOrientationHorizontal
    case 2: return kCTFontOrientationVertical
    default: return kCTFontOrientationDefault
    }
}

private func availableTableTags(for font: CTFont) -> [UInt32] {
    guard let tables = CTFontCopyAvailableTables(font, CTFontTableOptions(rawValue: 0)) else {
        return []
    }
    let count = CFArrayGetCount(tables)
    guard count > 0 else {
        return []
    }
    return (0..<count).map { index in
        UInt32(UInt(bitPattern: CFArrayGetValueAtIndex(tables, index)))
    }
}

@_cdecl("ct_font_create_with_name")
func ct_font_create_with_name(_ name: UnsafePointer<CChar>?, _ size: Double) -> UnsafeMutableRawPointer? {
    guard let name = stringFromCString(name) else {
        return nil
    }
    let font = CTFontCreateWithName(name as CFString, CGFloat(size), nil)
    return retainBox(font)
}

@_cdecl("ct_font_create_with_descriptor")
func ct_font_create_with_descriptor(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ size: Double
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr else {
        return nil
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let font = CTFontCreateWithFontDescriptor(descriptor, CGFloat(size), nil)
    return retainBox(font)
}

@_cdecl("ct_font_create_ui_font")
func ct_font_create_ui_font(
    _ uiType: UInt32,
    _ size: Double,
    _ language: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let font = CTFontCreateUIFontForLanguage(
        uiFontType(uiType),
        CGFloat(size),
        stringFromCString(language) as CFString?
    ) else {
        return nil
    }
    return retainBox(font)
}

@_cdecl("ct_font_copy_with_attributes")
func ct_font_copy_with_attributes(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ size: Double,
    _ descriptorPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let fontPtr else {
        return nil
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let descriptor: CTFontDescriptor? = descriptorPtr.map { unbox($0, as: CTFontDescriptor.self) }
    return retainBox(CTFontCreateCopyWithAttributes(font, CGFloat(size), nil, descriptor))
}

@_cdecl("ct_font_copy_with_family")
func ct_font_copy_with_family(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ size: Double,
    _ family: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let fontPtr, let family = stringFromCString(family) else {
        return nil
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    guard let copy = CTFontCreateCopyWithFamily(font, CGFloat(size), nil, family as CFString) else {
        return nil
    }
    return retainBox(copy)
}

@_cdecl("ct_font_copy_with_symbolic_traits")
func ct_font_copy_with_symbolic_traits(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ size: Double,
    _ traitValue: UInt32,
    _ traitMask: UInt32
) -> UnsafeMutableRawPointer? {
    guard let fontPtr else {
        return nil
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    guard let copy = CTFontCreateCopyWithSymbolicTraits(
        font,
        CGFloat(size),
        nil,
        CTFontSymbolicTraits(rawValue: traitValue),
        CTFontSymbolicTraits(rawValue: traitMask)
    ) else {
        return nil
    }
    return retainBox(copy)
}

@_cdecl("ct_font_create_for_string")
func ct_font_create_for_string(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ string: UnsafePointer<CChar>?,
    _ range: CFRange,
    _ language: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let fontPtr, let string = stringFromCString(string) else {
        return nil
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let language = stringFromCString(language) as CFString?
    let copy = CTFontCreateForStringWithLanguage(font, string as CFString, range, language)
    return retainBox(copy)
}

@_cdecl("ct_font_copy_descriptor")
func ct_font_copy_descriptor(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let fontPtr else {
        return nil
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return retainBox(CTFontCopyFontDescriptor(font))
}

@_cdecl("ct_font_get_size")
func ct_font_get_size(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetSize(font)
}

@_cdecl("ct_font_get_matrix")
func ct_font_get_matrix(_ fontPtr: UnsafeMutableRawPointer?) -> CGAffineTransform {
    guard let fontPtr else { return .identity }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetMatrix(font)
}

@_cdecl("ct_font_get_symbolic_traits")
func ct_font_get_symbolic_traits(_ fontPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetSymbolicTraits(font).rawValue
}

@_cdecl("ct_font_copy_postscript_name")
func ct_font_copy_postscript_name(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return duplicateCString(CTFontCopyPostScriptName(font) as String)
}

@_cdecl("ct_font_copy_family_name")
func ct_font_copy_family_name(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return duplicateCString(CTFontCopyFamilyName(font) as String)
}

@_cdecl("ct_font_copy_full_name")
func ct_font_copy_full_name(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return duplicateCString(CTFontCopyFullName(font) as String)
}

@_cdecl("ct_font_copy_display_name")
func ct_font_copy_display_name(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return duplicateCString(CTFontCopyDisplayName(font) as String)
}

@_cdecl("ct_font_copy_name")
func ct_font_copy_name(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ key: UInt32
) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return duplicateCString(CTFontCopyName(font, fontNameKey(key)) as String?)
}

@_cdecl("ct_font_copy_localized_name")
func ct_font_copy_localized_name(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ key: UInt32
) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return duplicateCString(CTFontCopyLocalizedName(font, fontNameKey(key), nil) as String?)
}

@_cdecl("ct_font_get_ascent")
func ct_font_get_ascent(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetAscent(font)
}

@_cdecl("ct_font_get_descent")
func ct_font_get_descent(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetDescent(font)
}

@_cdecl("ct_font_get_leading")
func ct_font_get_leading(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetLeading(font)
}

@_cdecl("ct_font_get_units_per_em")
func ct_font_get_units_per_em(_ fontPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetUnitsPerEm(font)
}

@_cdecl("ct_font_get_glyph_count")
func ct_font_get_glyph_count(_ fontPtr: UnsafeMutableRawPointer?) -> Int {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetGlyphCount(font)
}

@_cdecl("ct_font_get_bounding_box")
func ct_font_get_bounding_box(_ fontPtr: UnsafeMutableRawPointer?) -> CGRect {
    guard let fontPtr else { return .null }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetBoundingBox(font)
}

@_cdecl("ct_font_get_underline_position")
func ct_font_get_underline_position(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetUnderlinePosition(font)
}

@_cdecl("ct_font_get_underline_thickness")
func ct_font_get_underline_thickness(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetUnderlineThickness(font)
}

@_cdecl("ct_font_get_slant_angle")
func ct_font_get_slant_angle(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetSlantAngle(font)
}

@_cdecl("ct_font_get_cap_height")
func ct_font_get_cap_height(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetCapHeight(font)
}

@_cdecl("ct_font_get_x_height")
func ct_font_get_x_height(_ fontPtr: UnsafeMutableRawPointer?) -> Double {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetXHeight(font)
}

@_cdecl("ct_font_copy_supported_languages_json")
func ct_font_copy_supported_languages_json(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else {
        return jsonCString([String]())
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let languages = (CTFontCopySupportedLanguages(font) as NSArray) as? [String] ?? []
    return jsonCString(languages)
}

@_cdecl("ct_font_get_glyphs_for_characters")
func ct_font_get_glyphs_for_characters(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ characters: UnsafePointer<UniChar>?,
    _ glyphs: UnsafeMutablePointer<CGGlyph>?,
    _ count: Int
) -> Bool {
    guard let fontPtr, let characters, let glyphs, count > 0 else {
        return false
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetGlyphsForCharacters(font, characters, glyphs, count)
}

@_cdecl("ct_font_get_glyph_with_name")
func ct_font_get_glyph_with_name(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ glyphName: UnsafePointer<CChar>?
) -> CGGlyph {
    guard let fontPtr, let glyphName = stringFromCString(glyphName) else {
        return 0
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetGlyphWithName(font, glyphName as CFString)
}

@_cdecl("ct_font_copy_name_for_glyph")
func ct_font_copy_name_for_glyph(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ glyph: CGGlyph
) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return duplicateCString(CTFontCopyNameForGlyph(font, glyph) as String?)
}

@_cdecl("ct_font_get_bounding_rects_for_glyphs")
func ct_font_get_bounding_rects_for_glyphs(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ orientation: UInt32,
    _ glyphs: UnsafePointer<CGGlyph>?,
    _ boundingRects: UnsafeMutablePointer<CGRect>?,
    _ count: Int
) -> CGRect {
    guard let fontPtr, let glyphs else { return .null }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetBoundingRectsForGlyphs(
        font,
        fontOrientation(orientation),
        glyphs,
        boundingRects,
        count
    )
}

@_cdecl("ct_font_get_optical_bounds_for_glyphs")
func ct_font_get_optical_bounds_for_glyphs(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ glyphs: UnsafePointer<CGGlyph>?,
    _ boundingRects: UnsafeMutablePointer<CGRect>?,
    _ count: Int
) -> CGRect {
    guard let fontPtr, let glyphs else { return .null }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetOpticalBoundsForGlyphs(font, glyphs, boundingRects, count, 0)
}

@_cdecl("ct_font_get_advances_for_glyphs")
func ct_font_get_advances_for_glyphs(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ orientation: UInt32,
    _ glyphs: UnsafePointer<CGGlyph>?,
    _ advances: UnsafeMutablePointer<CGSize>?,
    _ count: Int
) -> Double {
    guard let fontPtr, let glyphs else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetAdvancesForGlyphs(font, fontOrientation(orientation), glyphs, advances, count)
}

@_cdecl("ct_font_get_vertical_translations_for_glyphs")
func ct_font_get_vertical_translations_for_glyphs(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ glyphs: UnsafePointer<CGGlyph>?,
    _ translations: UnsafeMutablePointer<CGSize>?,
    _ count: Int
) {
    guard let fontPtr, let glyphs, let translations, count > 0 else { return }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    CTFontGetVerticalTranslationsForGlyphs(font, glyphs, translations, count)
}

@_cdecl("ct_font_copy_available_tables_json")
func ct_font_copy_available_tables_json(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else {
        return jsonCString([UInt32]())
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return jsonCString(availableTableTags(for: font))
}

@_cdecl("ct_font_has_table")
func ct_font_has_table(_ fontPtr: UnsafeMutableRawPointer?, _ tag: UInt32) -> Bool {
    guard let fontPtr else { return false }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontHasTable(font, CTFontTableTag(tag))
}

@_cdecl("ct_font_copy_attribute_json")
func ct_font_copy_attribute_json(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ attributeName: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr, let attributeName = stringFromCString(attributeName) else {
        return duplicateCString("null")
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return cfToJSONCString(CTFontCopyAttribute(font, fontAttributeName(attributeName)))
}

@_cdecl("ct_font_copy_default_cascade_list_count")
func ct_font_copy_default_cascade_list_count(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ languagesJSON: UnsafePointer<CChar>?
) -> Int {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let languages = stringArrayFromJSON(languagesJSON)
    let cascade: [CTFontDescriptor] = typedArray(
        CTFontCopyDefaultCascadeListForLanguages(font, languages.isEmpty ? nil : languages as CFArray)
    )
    return cascade.count
}

@_cdecl("ct_font_copy_default_cascade_list")
func ct_font_copy_default_cascade_list(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ languagesJSON: UnsafePointer<CChar>?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let languages = stringArrayFromJSON(languagesJSON)
    let cascade: [CTFontDescriptor] = typedArray(
        CTFontCopyDefaultCascadeListForLanguages(font, languages.isEmpty ? nil : languages as CFArray)
    )
    return fillBoxedArray(cascade, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_copy_table_bytes")
func ct_font_copy_table_bytes(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ tag: UInt32,
    _ outLen: UnsafeMutablePointer<Int>?
) -> UnsafeMutablePointer<UInt8>? {
    outLen?.pointee = 0
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    guard let tableData = CTFontCopyTable(font, CTFontTableTag(tag), []) as Data? else {
        return nil
    }
    let length = tableData.count
    outLen?.pointee = length
    guard length > 0 else {
        return nil
    }
    guard let raw = malloc(length) else {
        return nil
    }
    let buffer = raw.assumingMemoryBound(to: UInt8.self)
    tableData.copyBytes(to: buffer, count: length)
    return buffer
}

@_cdecl("ct_font_create_with_descriptor_and_options")
func ct_font_create_with_descriptor_and_options(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ size: Double,
    _ options: UInt32
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return retainBox(
        CTFontCreateWithFontDescriptorAndOptions(
            descriptor,
            CGFloat(size),
            nil,
            CTFontOptions(rawValue: UInt(options))
        )
    )
}

@_cdecl("ct_font_create_with_name_and_options")
func ct_font_create_with_name_and_options(
    _ name: UnsafePointer<CChar>?,
    _ size: Double,
    _ options: UInt32
) -> UnsafeMutableRawPointer? {
    guard let name = stringFromCString(name) else { return nil }
    return retainBox(
        CTFontCreateWithNameAndOptions(
            name as CFString,
            CGFloat(size),
            nil,
            CTFontOptions(rawValue: UInt(options))
        )
    )
}

@_cdecl("ct_font_get_ligature_caret_positions")
func ct_font_get_ligature_caret_positions(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ glyph: CGGlyph,
    _ buffer: UnsafeMutablePointer<CGFloat>?,
    _ maxPositions: Int
) -> Int {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetLigatureCaretPositions(font, glyph, buffer, maxPositions)
}

@_cdecl("ct_font_get_string_encoding")
func ct_font_get_string_encoding(_ fontPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let fontPtr else { return 0 }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return CTFontGetStringEncoding(font)
}

@_cdecl("ct_font_get_type_id")
func ct_font_get_type_id() -> UInt64 {
    UInt64(CTFontGetTypeID())
}
