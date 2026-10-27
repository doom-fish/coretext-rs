import CoreText
import CoreTextABI
import Foundation

private func variationAxisPayloads(from value: Any?) -> [[String: Any]] {
    guard let array = value as? NSArray else {
        return []
    }
    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return [
            "identifier": (dictionary[kCTFontVariationAxisIdentifierKey] as? NSNumber)?.uint32Value ?? 0,
            "minimumValue": (dictionary[kCTFontVariationAxisMinimumValueKey] as? NSNumber)?.doubleValue ?? 0,
            "maximumValue": (dictionary[kCTFontVariationAxisMaximumValueKey] as? NSNumber)?.doubleValue ?? 0,
            "defaultValue": (dictionary[kCTFontVariationAxisDefaultValueKey] as? NSNumber)?.doubleValue ?? 0,
            "name": dictionary[kCTFontVariationAxisNameKey] as? String ?? "",
            "hidden": (dictionary[kCTFontVariationAxisHiddenKey] as? NSNumber)?.boolValue ?? false
        ]
    }
}

private func variationCoordinatePayloads(from value: Any?) -> [[String: Any]] {
    guard let dictionary = value as? NSDictionary else {
        return []
    }

    var payloads: [(UInt32, Double)] = []
    for case let key as NSNumber in dictionary.allKeys {
        guard let number = dictionary[key] as? NSNumber else {
            continue
        }
        payloads.append((key.uint32Value, number.doubleValue))
    }
    return payloads.sorted { $0.0 < $1.0 }.map { ["identifier": $0.0, "value": $0.1] }
}

@_cdecl("ct_font_copy_variation_axes")
public func ct_font_copy_variation_axes(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let fontPtr else {
        return binaryResult([] as CFArray)
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return binaryResult(variationAxisPayloads(from: CTFontCopyVariationAxes(font)) as CFArray)
}

@_cdecl("ct_font_copy_variation")
public func ct_font_copy_variation(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let fontPtr else {
        return binaryResult([] as CFArray)
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return binaryResult(variationCoordinatePayloads(from: CTFontCopyVariation(font)) as CFArray)
}

@_cdecl("ct_font_descriptor_copy_variation_axes")
public func ct_font_descriptor_copy_variation_axes(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let descriptorPtr else {
        return binaryResult([] as CFArray)
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let value = CTFontDescriptorCopyAttribute(descriptor, kCTFontVariationAxesAttribute)
    return binaryResult(variationAxisPayloads(from: value) as CFArray)
}

@_cdecl("ct_font_descriptor_copy_variation")
public func ct_font_descriptor_copy_variation(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let descriptorPtr else {
        return binaryResult([] as CFArray)
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let value = CTFontDescriptorCopyAttribute(descriptor, kCTFontVariationAttribute)
    return binaryResult(variationCoordinatePayloads(from: value) as CFArray)
}
