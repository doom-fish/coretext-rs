import CoreText
import CoreTextABI
import Foundation

private func fontCollectionOptions(_ value: UnsafePointer<CoreTextCollectionOptions>?) -> CFDictionary? {
    guard let payload = value?.pointee else {
        return nil
    }

    var options: [CFString: Any] = [:]
    if payload.remove_duplicates {
        options[kCTFontCollectionRemoveDuplicatesOption] = true
    }
    if payload.include_disabled_fonts {
        options[kCTFontCollectionIncludeDisabledFontsOption] = true
    }
    if payload.disallow_auto_activation {
        options[kCTFontCollectionDisallowAutoActivationOption] = true
    }
    if options.isEmpty {
        return nil
    }
    return options as CFDictionary
}

@_cdecl("ct_font_collection_create_available")
public func ct_font_collection_create_available(_ options: UnsafePointer<CoreTextCollectionOptions>?) -> UnsafeMutableRawPointer? {
    retainBox(CTFontCollectionCreateFromAvailableFonts(fontCollectionOptions(options)))
}

@_cdecl("ct_font_collection_create_for_family")
public func ct_font_collection_create_for_family(_ familyName: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let familyName = stringFromCString(familyName) else { return nil }
    let attributes: [CFString: Any] = [kCTFontFamilyNameAttribute: familyName as CFString]
    let descriptor = CTFontDescriptorCreateWithAttributes(attributes as CFDictionary)
    let options: [CFString: Any] = [kCTFontCollectionRemoveDuplicatesOption: true]
    let collection = CTFontCollectionCreateWithFontDescriptors(
        [descriptor] as CFArray, options as CFDictionary
    )
    guard let matching = CTFontCollectionCreateMatchingFontDescriptors(collection),
          CFArrayGetCount(matching) > 0 else {
        return nil
    }
    return retainBox(collection)
}

@_cdecl("ct_font_collection_create_with_descriptors")
public func ct_font_collection_create_with_descriptors(
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int,
    _ options: UnsafePointer<CoreTextCollectionOptions>?
) -> UnsafeMutableRawPointer? {
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    let collection = CTFontCollectionCreateWithFontDescriptors(
        descriptors as CFArray,
        fontCollectionOptions(options)
    )
    return retainBox(collection)
}

@_cdecl("ct_font_collection_copy_with_descriptors")
public func ct_font_collection_copy_with_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int,
    _ options: UnsafePointer<CoreTextCollectionOptions>?
) -> UnsafeMutableRawPointer? {
    guard let collectionPtr else { return nil }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    let copy = CTFontCollectionCreateCopyWithFontDescriptors(
        collection,
        descriptors as CFArray,
        fontCollectionOptions(options)
    )
    return retainBox(copy)
}

@_cdecl("ct_font_collection_get_query_descriptor_count")
public func ct_font_collection_get_query_descriptor_count(_ collectionPtr: UnsafeMutableRawPointer?) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCopyQueryDescriptors(collection))
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_query_descriptors")
public func ct_font_collection_copy_query_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCopyQueryDescriptors(collection))
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_collection_get_matching_descriptor_count")
public func ct_font_collection_get_matching_descriptor_count(_ collectionPtr: UnsafeMutableRawPointer?) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCreateMatchingFontDescriptors(collection))
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_matching_descriptors")
public func ct_font_collection_copy_matching_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCreateMatchingFontDescriptors(collection))
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_collection_get_matching_descriptors_for_family_count")
public func ct_font_collection_get_matching_descriptors_for_family_count(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ familyName: UnsafePointer<CChar>?
) -> Int {
    guard let collectionPtr, let familyName = stringFromCString(familyName) else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontCollectionCreateMatchingFontDescriptorsForFamily(collection, familyName as CFString, nil)
    )
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_matching_descriptors_for_family")
public func ct_font_collection_copy_matching_descriptors_for_family(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ familyName: UnsafePointer<CChar>?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let collectionPtr, let familyName = stringFromCString(familyName) else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontCollectionCreateMatchingFontDescriptorsForFamily(collection, familyName as CFString, nil)
    )
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_collection_get_exclusion_descriptor_count")
public func ct_font_collection_get_exclusion_descriptor_count(_ collectionPtr: UnsafeMutableRawPointer?) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCopyExclusionDescriptors(collection))
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_exclusion_descriptors")
public func ct_font_collection_copy_exclusion_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCopyExclusionDescriptors(collection))
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_collection_copy_font_attribute")
public func ct_font_collection_copy_font_attribute(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ attributeName: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let collectionPtr, let attributeName = stringFromCString(attributeName) else {
        return binaryResult(nil)
    }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    return binaryResult(
        CTFontCollectionCopyFontAttribute(collection, fontAttributeName(attributeName), [])
    )
}

@_cdecl("ct_font_collection_copy_font_attributes")
public func ct_font_collection_copy_font_attributes(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ names: CoreTextStrings
) -> UnsafeMutablePointer<CoreTextValue>? {
    guard let collectionPtr else {
        return binaryResult([] as CFArray)
    }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let attributeNames = stringArray(names).map(fontAttributeName)
    return binaryResult(
        CTFontCollectionCopyFontAttributes(collection, NSSet(array: attributeNames), [])
    )
}

@_cdecl("ct_font_collection_matching_with_options_count")
public func ct_font_collection_matching_with_options_count(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ options: UnsafePointer<CoreTextCollectionOptions>?
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontCollectionCreateMatchingFontDescriptorsWithOptions(collection, fontCollectionOptions(options))
    )
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_matching_with_options")
public func ct_font_collection_copy_matching_with_options(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ options: UnsafePointer<CoreTextCollectionOptions>?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontCollectionCreateMatchingFontDescriptorsWithOptions(collection, fontCollectionOptions(options))
    )
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_collection_create_mutable_copy")
public func ct_font_collection_create_mutable_copy(_ collectionPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let collectionPtr else { return nil }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let copy: CTMutableFontCollection = CTFontCollectionCreateMutableCopy(collection)
    return retainBox(copy)
}

@_cdecl("ct_font_collection_get_type_id")
public func ct_font_collection_get_type_id() -> UInt64 {
    UInt64(CTFontCollectionGetTypeID())
}

@_cdecl("ct_font_collection_set_exclusion_descriptors")
public func ct_font_collection_set_exclusion_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int
) {
    guard let collectionPtr else { return }
    let collection: CTMutableFontCollection = unbox(collectionPtr, as: CTMutableFontCollection.self)
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    CTFontCollectionSetExclusionDescriptors(collection, descriptors.isEmpty ? nil : descriptors as CFArray)
}

@_cdecl("ct_font_collection_set_query_descriptors")
public func ct_font_collection_set_query_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int
) {
    guard let collectionPtr else { return }
    let collection: CTMutableFontCollection = unbox(collectionPtr, as: CTMutableFontCollection.self)
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    CTFontCollectionSetQueryDescriptors(collection, descriptors.isEmpty ? nil : descriptors as CFArray)
}
