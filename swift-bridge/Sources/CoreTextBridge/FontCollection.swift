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
