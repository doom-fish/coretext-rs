import CoreText
import Foundation

private struct FontFeatureSelectorPayload: Codable {
    let identifier: Int64
    let name: String
    let isDefault: Bool
    let isEnabled: Bool
    let sampleText: String?
    let tooltipText: String?
    let openTypeTag: String?
    let openTypeValue: Int64?
}

private struct FontFeaturePayload: Codable {
    let typeIdentifier: Int64
    let name: String
    let exclusive: Bool
    let selectors: [FontFeatureSelectorPayload]
    let openTypeTag: String?
    let openTypeValue: Int64?
    let sampleText: String?
    let tooltipText: String?
}

private struct FontFeatureSettingPayload: Codable {
    let typeIdentifier: Int64
    let selectorIdentifier: Int64
}

private func featureSelectorPayloads(from value: Any?) -> [FontFeatureSelectorPayload] {
    guard let array = value as? NSArray else {
        return []
    }

    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return FontFeatureSelectorPayload(
            identifier: (dictionary[kCTFontFeatureSelectorIdentifierKey] as? NSNumber)?.int64Value ?? 0,
            name: dictionary[kCTFontFeatureSelectorNameKey] as? String ?? "",
            isDefault: (dictionary[kCTFontFeatureSelectorDefaultKey] as? NSNumber)?.boolValue ?? false,
            isEnabled: (dictionary[kCTFontFeatureSelectorSettingKey] as? NSNumber)?.boolValue ?? false,
            sampleText: dictionary[kCTFontFeatureSampleTextKey] as? String,
            tooltipText: dictionary[kCTFontFeatureTooltipTextKey] as? String,
            openTypeTag: dictionary[kCTFontOpenTypeFeatureTag] as? String,
            openTypeValue: (dictionary[kCTFontOpenTypeFeatureValue] as? NSNumber)?.int64Value
        )
    }
}

private func featurePayloads(from value: Any?) -> [FontFeaturePayload] {
    guard let array = value as? NSArray else {
        return []
    }

    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return FontFeaturePayload(
            typeIdentifier: (dictionary[kCTFontFeatureTypeIdentifierKey] as? NSNumber)?.int64Value ?? 0,
            name: dictionary[kCTFontFeatureTypeNameKey] as? String ?? "",
            exclusive: (dictionary[kCTFontFeatureTypeExclusiveKey] as? NSNumber)?.boolValue ?? false,
            selectors: featureSelectorPayloads(from: dictionary[kCTFontFeatureTypeSelectorsKey]),
            openTypeTag: dictionary[kCTFontOpenTypeFeatureTag] as? String,
            openTypeValue: (dictionary[kCTFontOpenTypeFeatureValue] as? NSNumber)?.int64Value,
            sampleText: dictionary[kCTFontFeatureSampleTextKey] as? String,
            tooltipText: dictionary[kCTFontFeatureTooltipTextKey] as? String
        )
    }
}

private func featureSettingPayloads(from value: Any?) -> [FontFeatureSettingPayload] {
    guard let array = value as? NSArray else {
        return []
    }

    return array.compactMap { item in
        guard let dictionary = item as? NSDictionary else {
            return nil
        }
        return FontFeatureSettingPayload(
            typeIdentifier: (dictionary[kCTFontFeatureTypeIdentifierKey] as? NSNumber)?.int64Value ?? 0,
            selectorIdentifier: (dictionary[kCTFontFeatureSelectorIdentifierKey] as? NSNumber)?.int64Value ?? 0
        )
    }
}

@_cdecl("ct_font_copy_features_json")
func ct_font_copy_features_json(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else {
        return jsonCString([FontFeaturePayload]())
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return jsonCString(featurePayloads(from: CTFontCopyFeatures(font)))
}

@_cdecl("ct_font_copy_feature_settings_json")
func ct_font_copy_feature_settings_json(_ fontPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let fontPtr else {
        return jsonCString([FontFeatureSettingPayload]())
    }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    return jsonCString(featureSettingPayloads(from: CTFontCopyFeatureSettings(font)))
}

@_cdecl("ct_font_descriptor_copy_features_json")
func ct_font_descriptor_copy_features_json(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else {
        return jsonCString([FontFeaturePayload]())
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return jsonCString(
        featurePayloads(from: CTFontDescriptorCopyAttribute(descriptor, kCTFontFeaturesAttribute))
    )
}

@_cdecl("ct_font_descriptor_copy_feature_settings_json")
func ct_font_descriptor_copy_feature_settings_json(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else {
        return jsonCString([FontFeatureSettingPayload]())
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return jsonCString(
        featureSettingPayloads(
            from: CTFontDescriptorCopyAttribute(descriptor, kCTFontFeatureSettingsAttribute)
        )
    )
}
