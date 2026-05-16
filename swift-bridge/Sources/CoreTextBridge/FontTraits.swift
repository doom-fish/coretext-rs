import CoreText
import Foundation

private struct FontTraitsPayload: Codable {
    let symbolicTraits: UInt32
    let weight: Double
    let width: Double
    let slant: Double
}

private func traitsPayload(from value: Any?) -> FontTraitsPayload {
    let dictionary = value as? NSDictionary
    return FontTraitsPayload(
        symbolicTraits: (dictionary?[kCTFontSymbolicTrait] as? NSNumber)?.uint32Value ?? 0,
        weight: (dictionary?[kCTFontWeightTrait] as? NSNumber)?.doubleValue ?? 0,
        width: (dictionary?[kCTFontWidthTrait] as? NSNumber)?.doubleValue ?? 0,
        slant: (dictionary?[kCTFontSlantTrait] as? NSNumber)?.doubleValue ?? 0
    )
}

@_cdecl("ct_font_copy_traits_json")
func ct_font_copy_traits_json(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else {
        return jsonCString(FontTraitsPayload(symbolicTraits: 0, weight: 0, width: 0, slant: 0))
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return jsonCString(traitsPayload(from: CTFontCopyTraits(font)))
}

@_cdecl("ct_font_descriptor_copy_traits_json")
func ct_font_descriptor_copy_traits_json(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else {
        return jsonCString(FontTraitsPayload(symbolicTraits: 0, weight: 0, width: 0, slant: 0))
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return jsonCString(
        traitsPayload(from: CTFontDescriptorCopyAttribute(descriptor, kCTFontTraitsAttribute))
    )
}
