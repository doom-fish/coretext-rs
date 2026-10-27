import CoreText
import CoreTextABI
import Foundation

private func descriptorAttributesPayload(_ descriptor: CTFontDescriptor) -> [String: Any] {
    var payload: [String: Any] = [:]
    if let name = CTFontDescriptorCopyAttribute(descriptor, kCTFontNameAttribute) as? String {
        payload["name"] = name
    }
    if let displayName = CTFontDescriptorCopyAttribute(descriptor, kCTFontDisplayNameAttribute) as? String {
        payload["displayName"] = displayName
    }
    if let familyName = CTFontDescriptorCopyAttribute(descriptor, kCTFontFamilyNameAttribute) as? String {
        payload["familyName"] = familyName
    }
    if let styleName = CTFontDescriptorCopyAttribute(descriptor, kCTFontStyleNameAttribute) as? String {
        payload["styleName"] = styleName
    }
    if let size = CTFontDescriptorCopyAttribute(descriptor, kCTFontSizeAttribute) as? NSNumber {
        payload["size"] = size.doubleValue
    }
    if let format = CTFontDescriptorCopyAttribute(descriptor, kCTFontFormatAttribute) as? NSNumber {
        payload["format"] = format.uint32Value
    }
    if let orientation = CTFontDescriptorCopyAttribute(descriptor, kCTFontOrientationAttribute) as? NSNumber {
        payload["orientation"] = orientation.uint32Value
    }
    if let enabled = CTFontDescriptorCopyAttribute(descriptor, kCTFontEnabledAttribute) as? NSNumber {
        payload["enabled"] = enabled.boolValue
    }
    if let downloadable = CTFontDescriptorCopyAttribute(descriptor, kCTFontDownloadableAttribute) as? NSNumber {
        payload["downloadable"] = downloadable.boolValue
    }
    if let url = CTFontDescriptorCopyAttribute(descriptor, kCTFontURLAttribute) as? URL {
        payload["urlPath"] = url.path
    }
    return payload
}

@_cdecl("ct_font_descriptor_create")
public func ct_font_descriptor_create(_ name: UnsafePointer<CChar>?, _ size: Double) -> UnsafeMutableRawPointer? {
    guard let name = stringFromCString(name) else {
        return nil
    }
    return retainBox(CTFontDescriptorCreateWithNameAndSize(name as CFString, CGFloat(size)))
}

@_cdecl("ct_font_descriptor_copy_with_family")
public func ct_font_descriptor_copy_with_family(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ family: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr, let family = stringFromCString(family) else {
        return nil
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    guard let copy = CTFontDescriptorCreateCopyWithFamily(descriptor, family as CFString) else {
        return nil
    }
    return retainBox(copy)
}

@_cdecl("ct_font_descriptor_copy_with_symbolic_traits")
public func ct_font_descriptor_copy_with_symbolic_traits(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ traitValue: UInt32,
    _ traitMask: UInt32
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    guard let copy = CTFontDescriptorCreateCopyWithSymbolicTraits(
        descriptor,
        CTFontSymbolicTraits(rawValue: traitValue),
        CTFontSymbolicTraits(rawValue: traitMask)
    ) else {
        return nil
    }
    return retainBox(copy)
}

@_cdecl("ct_font_descriptor_copy_with_variation")
public func ct_font_descriptor_copy_with_variation(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ variationIdentifier: UInt32,
    _ variationValue: Double
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let copy = CTFontDescriptorCreateCopyWithVariation(
        descriptor,
        NSNumber(value: variationIdentifier),
        CGFloat(variationValue)
    )
    return retainBox(copy)
}

@_cdecl("ct_font_descriptor_copy_with_feature")
public func ct_font_descriptor_copy_with_feature(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ featureTypeIdentifier: Int64,
    _ featureSelectorIdentifier: Int64
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let copy = CTFontDescriptorCreateCopyWithFeature(
        descriptor,
        NSNumber(value: featureTypeIdentifier),
        NSNumber(value: featureSelectorIdentifier)
    )
    return retainBox(copy)
}

@_cdecl("ct_font_descriptor_create_matching_descriptor")
public func ct_font_descriptor_create_matching_descriptor(
    _ descriptorPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    guard let copy = CTFontDescriptorCreateMatchingFontDescriptor(descriptor, nil) else {
        return nil
    }
    return retainBox(copy)
}

@_cdecl("ct_font_descriptor_get_matching_descriptor_count")
public func ct_font_descriptor_get_matching_descriptor_count(_ descriptorPtr: UnsafeMutableRawPointer?) -> Int {
    guard let descriptorPtr else { return 0 }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontDescriptorCreateMatchingFontDescriptors(descriptor, nil)
    )
    return descriptors.count
}

@_cdecl("ct_font_descriptor_copy_matching_descriptors")
public func ct_font_descriptor_copy_matching_descriptors(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let descriptorPtr else { return 0 }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontDescriptorCreateMatchingFontDescriptors(descriptor, nil)
    )
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_descriptor_copy_postscript_name")
public func ct_font_descriptor_copy_postscript_name(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return duplicateCString(CTFontDescriptorCopyAttribute(descriptor, kCTFontNameAttribute) as? String)
}

@_cdecl("ct_font_descriptor_copy_display_name")
public func ct_font_descriptor_copy_display_name(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return duplicateCString(CTFontDescriptorCopyAttribute(descriptor, kCTFontDisplayNameAttribute) as? String)
}

@_cdecl("ct_font_descriptor_copy_family_name")
public func ct_font_descriptor_copy_family_name(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return duplicateCString(CTFontDescriptorCopyAttribute(descriptor, kCTFontFamilyNameAttribute) as? String)
}

@_cdecl("ct_font_descriptor_copy_style_name")
public func ct_font_descriptor_copy_style_name(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return duplicateCString(CTFontDescriptorCopyAttribute(descriptor, kCTFontStyleNameAttribute) as? String)
}

@_cdecl("ct_font_descriptor_copy_url_path")
public func ct_font_descriptor_copy_url_path(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else { return nil }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    let url = CTFontDescriptorCopyAttribute(descriptor, kCTFontURLAttribute) as? URL
    return duplicateCString(url?.path)
}

@_cdecl("ct_font_descriptor_get_size")
public func ct_font_descriptor_get_size(_ descriptorPtr: UnsafeMutableRawPointer?) -> Double {
    guard let descriptorPtr else { return 0 }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return (CTFontDescriptorCopyAttribute(descriptor, kCTFontSizeAttribute) as? NSNumber)?.doubleValue ?? 0
}

@_cdecl("ct_font_descriptor_get_orientation")
public func ct_font_descriptor_get_orientation(_ descriptorPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let descriptorPtr else { return 0 }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return (CTFontDescriptorCopyAttribute(descriptor, kCTFontOrientationAttribute) as? NSNumber)?.uint32Value ?? 0
}

@_cdecl("ct_font_descriptor_get_format")
public func ct_font_descriptor_get_format(_ descriptorPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let descriptorPtr else { return 0 }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return (CTFontDescriptorCopyAttribute(descriptor, kCTFontFormatAttribute) as? NSNumber)?.uint32Value ?? 0
}

@_cdecl("ct_font_descriptor_is_enabled")
public func ct_font_descriptor_is_enabled(_ descriptorPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let descriptorPtr else { return false }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return (CTFontDescriptorCopyAttribute(descriptor, kCTFontEnabledAttribute) as? NSNumber)?.boolValue ?? false
}

@_cdecl("ct_font_descriptor_is_downloadable")
public func ct_font_descriptor_is_downloadable(_ descriptorPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let descriptorPtr else { return false }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return (CTFontDescriptorCopyAttribute(descriptor, kCTFontDownloadableAttribute) as? NSNumber)?.boolValue ?? false
}

@_cdecl("ct_font_descriptor_copy_attributes")
public func ct_font_descriptor_copy_attributes(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let descriptorPtr else {
        return binaryResult([:] as CFDictionary)
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return binaryResult(descriptorAttributesPayload(descriptor) as CFDictionary)
}

@_cdecl("ct_font_descriptor_copy_attribute")
public func ct_font_descriptor_copy_attribute(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ attributeName: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let descriptorPtr, let attributeName = stringFromCString(attributeName) else {
        return binaryResult(nil)
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return binaryResult(CTFontDescriptorCopyAttribute(descriptor, fontAttributeName(attributeName)))
}

@_cdecl("ct_font_descriptor_copy_localized_attribute")
public func ct_font_descriptor_copy_localized_attribute(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ attributeName: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let descriptorPtr, let attributeName = stringFromCString(attributeName) else {
        return binaryResult(nil)
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return binaryResult(
        CTFontDescriptorCopyLocalizedAttribute(descriptor, fontAttributeName(attributeName), nil)
    )
}

@_cdecl("ct_font_descriptor_create_copy_with_attributes")
public func ct_font_descriptor_create_copy_with_attributes(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ attributesValue: UnsafePointer<CoreTextValue>?
) -> UnsafeMutableRawPointer? {
    guard let descriptorPtr,
          let attributes = dictionaryFromValue(attributesValue, keyTransform: fontAttributeName)
    else {
        return nil
    }
    let descriptor: CTFontDescriptor = unbox(descriptorPtr, as: CTFontDescriptor.self)
    return retainBox(CTFontDescriptorCreateCopyWithAttributes(descriptor, attributes))
}

@_cdecl("ct_font_descriptor_create_with_attributes")
public func ct_font_descriptor_create_with_attributes(
    _ attributesValue: UnsafePointer<CoreTextValue>?
) -> UnsafeMutableRawPointer? {
    guard let attributes = dictionaryFromValue(attributesValue, keyTransform: fontAttributeName) else {
        return nil
    }
    return retainBox(CTFontDescriptorCreateWithAttributes(attributes))
}

@_cdecl("ct_font_descriptor_get_type_id")
public func ct_font_descriptor_get_type_id() -> UInt64 {
    UInt64(CTFontDescriptorGetTypeID())
}
