import CoreText
import CoreTextABI
import Foundation

private func traitsPayload(from value: Any?) -> CoreTextFontTraits {
    let dictionary = value as? NSDictionary
    return CoreTextFontTraits(
        symbolic_traits: (dictionary?[kCTFontSymbolicTrait] as? NSNumber)?.uint32Value ?? 0,
        weight: (dictionary?[kCTFontWeightTrait] as? NSNumber)?.doubleValue ?? 0,
        width: (dictionary?[kCTFontWidthTrait] as? NSNumber)?.doubleValue ?? 0,
        slant: (dictionary?[kCTFontSlantTrait] as? NSNumber)?.doubleValue ?? 0
    )
}

@_cdecl("ct_font_copy_traits")
public func ct_font_copy_traits(_ fontPtr: UnsafeMutableRawPointer?) -> CoreTextFontTraits {
    guard let fontPtr else {
        return CoreTextFontTraits()
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return traitsPayload(from: CTFontCopyTraits(font))
}

@_cdecl("ct_font_descriptor_copy_traits")
public func ct_font_descriptor_copy_traits(_ descriptorPtr: UnsafeMutableRawPointer?) -> CoreTextFontTraits {
    guard let descriptorPtr else {
        return CoreTextFontTraits()
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return traitsPayload(from: CTFontDescriptorCopyAttribute(descriptor, kCTFontTraitsAttribute))
}
