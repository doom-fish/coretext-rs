import CoreText
import Foundation

private struct FontCollectionOptionsPayload: Decodable {
    let removeDuplicates: Bool
    let includeDisabledFonts: Bool
    let disallowAutoActivation: Bool
}

private func fontCollectionOptions(_ json: UnsafePointer<CChar>?) -> CFDictionary? {
    guard let payload = decodeJSON(json, as: FontCollectionOptionsPayload.self) else {
        return nil
    }

    var options: [CFString: Any] = [:]
    if payload.removeDuplicates {
        options[kCTFontCollectionRemoveDuplicatesOption] = true
    }
    if payload.includeDisabledFonts {
        options[kCTFontCollectionIncludeDisabledFontsOption] = true
    }
    if payload.disallowAutoActivation {
        options[kCTFontCollectionDisallowAutoActivationOption] = true
    }
    if options.isEmpty {
        return nil
    }
    return options as CFDictionary
}

@_cdecl("ct_font_collection_create_available")
func ct_font_collection_create_available(_ optionsJSON: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    retainBox(CTFontCollectionCreateFromAvailableFonts(fontCollectionOptions(optionsJSON)))
}

@_cdecl("ct_font_collection_create_with_descriptors")
func ct_font_collection_create_with_descriptors(
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int,
    _ optionsJSON: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    let collection = CTFontCollectionCreateWithFontDescriptors(
        descriptors as CFArray,
        fontCollectionOptions(optionsJSON)
    )
    return retainBox(collection)
}

@_cdecl("ct_font_collection_copy_with_descriptors")
func ct_font_collection_copy_with_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int,
    _ optionsJSON: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let collectionPtr else { return nil }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    let copy = CTFontCollectionCreateCopyWithFontDescriptors(
        collection,
        descriptors as CFArray,
        fontCollectionOptions(optionsJSON)
    )
    return retainBox(copy)
}

@_cdecl("ct_font_collection_get_query_descriptor_count")
func ct_font_collection_get_query_descriptor_count(_ collectionPtr: UnsafeMutableRawPointer?) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCopyQueryDescriptors(collection))
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_query_descriptors")
func ct_font_collection_copy_query_descriptors(
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
func ct_font_collection_get_matching_descriptor_count(_ collectionPtr: UnsafeMutableRawPointer?) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCreateMatchingFontDescriptors(collection))
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_matching_descriptors")
func ct_font_collection_copy_matching_descriptors(
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
func ct_font_collection_get_matching_descriptors_for_family_count(
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
func ct_font_collection_copy_matching_descriptors_for_family(
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
func ct_font_collection_get_exclusion_descriptor_count(_ collectionPtr: UnsafeMutableRawPointer?) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCopyExclusionDescriptors(collection))
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_exclusion_descriptors")
func ct_font_collection_copy_exclusion_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(CTFontCollectionCopyExclusionDescriptors(collection))
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_collection_copy_font_attribute_json")
func ct_font_collection_copy_font_attribute_json(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ attributeName: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let collectionPtr, let attributeName = stringFromCString(attributeName) else {
        return duplicateCString("null")
    }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    return cfToJSONCString(
        CTFontCollectionCopyFontAttribute(collection, fontAttributeName(attributeName), [])
    )
}

@_cdecl("ct_font_collection_copy_font_attributes_json")
func ct_font_collection_copy_font_attributes_json(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ attributeNamesJSON: UnsafePointer<CChar>?
) -> UnsafeMutablePointer<CChar>? {
    guard let collectionPtr else {
        return duplicateCString("[]")
    }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let attributeNames = stringArrayFromJSON(attributeNamesJSON).map(fontAttributeName)
    return cfToJSONCString(
        CTFontCollectionCopyFontAttributes(collection, NSSet(array: attributeNames), [])
    )
}

@_cdecl("ct_font_collection_matching_with_options_count")
func ct_font_collection_matching_with_options_count(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ optionsJSON: UnsafePointer<CChar>?
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontCollectionCreateMatchingFontDescriptorsWithOptions(collection, fontCollectionOptions(optionsJSON))
    )
    return descriptors.count
}

@_cdecl("ct_font_collection_copy_matching_with_options")
func ct_font_collection_copy_matching_with_options(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ optionsJSON: UnsafePointer<CChar>?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let collectionPtr else { return 0 }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontCollectionCreateMatchingFontDescriptorsWithOptions(collection, fontCollectionOptions(optionsJSON))
    )
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_collection_create_mutable_copy")
func ct_font_collection_create_mutable_copy(_ collectionPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let collectionPtr else { return nil }
    let collection: CTFontCollection = unbox(collectionPtr, as: CTFontCollection.self)
    let copy: CTMutableFontCollection = CTFontCollectionCreateMutableCopy(collection)
    return retainBox(copy)
}

@_cdecl("ct_font_collection_get_type_id")
func ct_font_collection_get_type_id() -> UInt64 {
    UInt64(CTFontCollectionGetTypeID())
}

@_cdecl("ct_font_collection_set_exclusion_descriptors")
func ct_font_collection_set_exclusion_descriptors(
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
func ct_font_collection_set_query_descriptors(
    _ collectionPtr: UnsafeMutableRawPointer?,
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int
) {
    guard let collectionPtr else { return }
    let collection: CTMutableFontCollection = unbox(collectionPtr, as: CTMutableFontCollection.self)
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    CTFontCollectionSetQueryDescriptors(collection, descriptors.isEmpty ? nil : descriptors as CFArray)
}
