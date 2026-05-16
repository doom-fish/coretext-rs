import CoreText
import Foundation

private func characterCollection(_ raw: UInt16) -> CTCharacterCollection {
    switch raw {
    case 1: return kCTCharacterCollectionAdobeCNS1
    case 2: return kCTCharacterCollectionAdobeGB1
    case 3: return kCTCharacterCollectionAdobeJapan1
    case 4: return kCTCharacterCollectionAdobeJapan2
    case 5: return kCTCharacterCollectionAdobeKorea1
    default: return kCTCharacterCollectionIdentityMapping
    }
}

@_cdecl("ct_glyph_info_create_with_glyph_name")
func ct_glyph_info_create_with_glyph_name(
    _ glyphName: UnsafePointer<CChar>?,
    _ fontPtr: UnsafeMutableRawPointer?,
    _ baseString: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let glyphName = stringFromCString(glyphName),
          let fontPtr,
          let baseString = stringFromCString(baseString)
    else {
        return nil
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    guard let glyphInfo = CTGlyphInfoCreateWithGlyphName(glyphName as CFString, font, baseString as CFString) else {
        return nil
    }
    return retainBox(glyphInfo)
}

@_cdecl("ct_glyph_info_create_with_glyph")
func ct_glyph_info_create_with_glyph(
    _ glyph: CGGlyph,
    _ fontPtr: UnsafeMutableRawPointer?,
    _ baseString: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let fontPtr, let baseString = stringFromCString(baseString) else {
        return nil
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    guard let glyphInfo = CTGlyphInfoCreateWithGlyph(glyph, font, baseString as CFString) else {
        return nil
    }
    return retainBox(glyphInfo)
}

@_cdecl("ct_glyph_info_create_with_character_identifier")
func ct_glyph_info_create_with_character_identifier(
    _ characterIdentifier: UInt16,
    _ collection: UInt16,
    _ baseString: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let baseString = stringFromCString(baseString) else {
        return nil
    }
    guard let glyphInfo = CTGlyphInfoCreateWithCharacterIdentifier(
        CGFontIndex(characterIdentifier),
        characterCollection(collection),
        baseString as CFString
    ) else {
        return nil
    }
    return retainBox(glyphInfo)
}

@_cdecl("ct_glyph_info_copy_glyph_name")
func ct_glyph_info_copy_glyph_name(_ glyphInfoPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let glyphInfoPtr else { return nil }
    let glyphInfo: CTGlyphInfo = unbox(glyphInfoPtr, as: CTGlyphInfo.self)
    return duplicateCString(CTGlyphInfoGetGlyphName(glyphInfo) as String?)
}

@_cdecl("ct_glyph_info_get_glyph")
func ct_glyph_info_get_glyph(_ glyphInfoPtr: UnsafeMutableRawPointer?) -> CGGlyph {
    guard let glyphInfoPtr else { return 0 }
    let glyphInfo: CTGlyphInfo = unbox(glyphInfoPtr, as: CTGlyphInfo.self)
    return CTGlyphInfoGetGlyph(glyphInfo)
}

@_cdecl("ct_glyph_info_get_character_identifier")
func ct_glyph_info_get_character_identifier(_ glyphInfoPtr: UnsafeMutableRawPointer?) -> UInt16 {
    guard let glyphInfoPtr else { return 0 }
    let glyphInfo: CTGlyphInfo = unbox(glyphInfoPtr, as: CTGlyphInfo.self)
    return UInt16(CTGlyphInfoGetCharacterIdentifier(glyphInfo))
}

@_cdecl("ct_glyph_info_get_character_collection")
func ct_glyph_info_get_character_collection(_ glyphInfoPtr: UnsafeMutableRawPointer?) -> UInt16 {
    guard let glyphInfoPtr else { return 0 }
    let glyphInfo: CTGlyphInfo = unbox(glyphInfoPtr, as: CTGlyphInfo.self)
    return CTGlyphInfoGetCharacterCollection(glyphInfo).rawValue
}
