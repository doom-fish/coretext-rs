import CoreText
import CoreTextABI
import Foundation

private func featureSelectorPayloads(from value: Any?) -> [[String: Any]] {
    guard let array = value as? NSArray else {
        return []
    }

    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return [
            "identifier": (dictionary[kCTFontFeatureSelectorIdentifierKey] as? NSNumber)?.int64Value ?? 0,
            "name": dictionary[kCTFontFeatureSelectorNameKey] as? String ?? "",
            "isDefault": (dictionary[kCTFontFeatureSelectorDefaultKey] as? NSNumber)?.boolValue ?? false,
            "isEnabled": (dictionary[kCTFontFeatureSelectorSettingKey] as? NSNumber)?.boolValue ?? false,
            "sampleText": dictionary[kCTFontFeatureSampleTextKey] as? String as Any? ?? NSNull(),
            "tooltipText": dictionary[kCTFontFeatureTooltipTextKey] as? String as Any? ?? NSNull(),
            "openTypeTag": dictionary[kCTFontOpenTypeFeatureTag] as? String as Any? ?? NSNull(),
            "openTypeValue": dictionary[kCTFontOpenTypeFeatureValue] as? NSNumber ?? NSNull()
        ]
    }
}

private func featurePayloads(from value: Any?) -> [[String: Any]] {
    guard let array = value as? NSArray else {
        return []
    }

    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return [
            "typeIdentifier": (dictionary[kCTFontFeatureTypeIdentifierKey] as? NSNumber)?.int64Value ?? 0,
            "name": dictionary[kCTFontFeatureTypeNameKey] as? String ?? "",
            "exclusive": (dictionary[kCTFontFeatureTypeExclusiveKey] as? NSNumber)?.boolValue ?? false,
            "selectors": featureSelectorPayloads(from: dictionary[kCTFontFeatureTypeSelectorsKey]),
            "openTypeTag": dictionary[kCTFontOpenTypeFeatureTag] as? String as Any? ?? NSNull(),
            "openTypeValue": dictionary[kCTFontOpenTypeFeatureValue] as? NSNumber ?? NSNull(),
            "sampleText": dictionary[kCTFontFeatureSampleTextKey] as? String as Any? ?? NSNull(),
            "tooltipText": dictionary[kCTFontFeatureTooltipTextKey] as? String as Any? ?? NSNull()
        ]
    }
}

private func featureSettingPayloads(from value: Any?) -> [[String: Any]] {
    guard let array = value as? NSArray else {
        return []
    }

    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return [
            "typeIdentifier": (dictionary[kCTFontFeatureTypeIdentifierKey] as? NSNumber)?.int64Value ?? 0,
            "selectorIdentifier": (dictionary[kCTFontFeatureSelectorIdentifierKey] as? NSNumber)?.int64Value ?? 0
        ]
    }
}

@_cdecl("ct_font_copy_features")
public func ct_font_copy_features(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let fontPtr else {
        return binaryResult([] as CFArray)
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return binaryResult(featurePayloads(from: CTFontCopyFeatures(font)) as CFArray)
}

@_cdecl("ct_font_copy_feature_settings")
public func ct_font_copy_feature_settings(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let fontPtr else {
        return binaryResult([] as CFArray)
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return binaryResult(featureSettingPayloads(from: CTFontCopyFeatureSettings(font)) as CFArray)
}

@_cdecl("ct_font_descriptor_copy_features")
public func ct_font_descriptor_copy_features(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let descriptorPtr else {
        return binaryResult([] as CFArray)
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return binaryResult(
        featurePayloads(from: CTFontDescriptorCopyAttribute(descriptor, kCTFontFeaturesAttribute)) as CFArray
    )
}

@_cdecl("ct_font_descriptor_copy_feature_settings")
public func ct_font_descriptor_copy_feature_settings(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let descriptorPtr else {
        return binaryResult([] as CFArray)
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return binaryResult(
        featureSettingPayloads(
            from: CTFontDescriptorCopyAttribute(descriptor, kCTFontFeatureSettingsAttribute)
        ) as CFArray
    )
}
