import CoreText
import Foundation

private struct FontVariationAxisPayload: Codable {
    let identifier: UInt32
    let minimumValue: Double
    let maximumValue: Double
    let defaultValue: Double
    let name: String
    let hidden: Bool
}

private struct FontVariationCoordinatePayload: Codable {
    let identifier: UInt32
    let value: Double
}

private func variationAxisPayloads(from value: Any?) -> [FontVariationAxisPayload] {
    guard let array = value as? NSArray else {
        return []
    }
    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return FontVariationAxisPayload(
            identifier: (dictionary[kCTFontVariationAxisIdentifierKey] as? NSNumber)?.uint32Value ?? 0,
            minimumValue: (dictionary[kCTFontVariationAxisMinimumValueKey] as? NSNumber)?.doubleValue ?? 0,
            maximumValue: (dictionary[kCTFontVariationAxisMaximumValueKey] as? NSNumber)?.doubleValue ?? 0,
            defaultValue: (dictionary[kCTFontVariationAxisDefaultValueKey] as? NSNumber)?.doubleValue ?? 0,
            name: dictionary[kCTFontVariationAxisNameKey] as? String ?? "",
            hidden: (dictionary[kCTFontVariationAxisHiddenKey] as? NSNumber)?.boolValue ?? false
        )
    }
}

private func variationCoordinatePayloads(from value: Any?) -> [FontVariationCoordinatePayload] {
    guard let dictionary = value as? NSDictionary else {
        return []
    }

    var payloads: [FontVariationCoordinatePayload] = []
    for case let key as NSNumber in dictionary.allKeys {
        guard let number = dictionary[key] as? NSNumber else {
            continue
        }
        payloads.append(
            FontVariationCoordinatePayload(
                identifier: key.uint32Value,
                value: number.doubleValue
            )
        )
    }
    return payloads.sorted { $0.identifier < $1.identifier }
}

@_cdecl("ct_font_copy_variation_axes_json")
func ct_font_copy_variation_axes_json(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else {
        return jsonCString([FontVariationAxisPayload]())
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return jsonCString(variationAxisPayloads(from: CTFontCopyVariationAxes(font)))
}

@_cdecl("ct_font_copy_variation_json")
func ct_font_copy_variation_json(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else {
        return jsonCString([FontVariationCoordinatePayload]())
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return jsonCString(variationCoordinatePayloads(from: CTFontCopyVariation(font)))
}

@_cdecl("ct_font_descriptor_copy_variation_axes_json")
func ct_font_descriptor_copy_variation_axes_json(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else {
        return jsonCString([FontVariationAxisPayload]())
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let value = CTFontDescriptorCopyAttribute(descriptor, kCTFontVariationAxesAttribute)
    return jsonCString(variationAxisPayloads(from: value))
}

@_cdecl("ct_font_descriptor_copy_variation_json")
func ct_font_descriptor_copy_variation_json(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else {
        return jsonCString([FontVariationCoordinatePayload]())
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let value = CTFontDescriptorCopyAttribute(descriptor, kCTFontVariationAttribute)
    return jsonCString(variationCoordinatePayloads(from: value))
}
