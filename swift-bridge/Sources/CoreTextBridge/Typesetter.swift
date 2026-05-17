import CoreText
import Foundation

private struct TypesetterOptionsPayload: Decodable {
    let allowUnboundedLayout: Bool
    let forcedEmbeddingLevel: Int64?
}

private func typesetterOptions(_ json: UnsafePointer<CChar>?) -> CFDictionary? {
    guard let payload = decodeJSON(json, as: TypesetterOptionsPayload.self) else {
        return nil
    }
    var options: [CFString: Any] = [:]
    if payload.allowUnboundedLayout {
        options[kCTTypesetterOptionAllowUnboundedLayout] = true
    }
    if let forcedEmbeddingLevel = payload.forcedEmbeddingLevel {
        options[kCTTypesetterOptionForcedEmbeddingLevel] = NSNumber(value: forcedEmbeddingLevel)
    }
    if options.isEmpty {
        return nil
    }
    return options as CFDictionary
}

@_cdecl("ct_typesetter_create_with_attributed_string")
func ct_typesetter_create_with_attributed_string(
    _ attributedStringPtr: UnsafeMutableRawPointer?,
    _ optionsJSON: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let attributedStringPtr else { return nil }
    let attributedString: NSAttributedString = unbox(attributedStringPtr, as: NSAttributedString.self)
    guard let typesetter = CTTypesetterCreateWithAttributedStringAndOptions(attributedString, typesetterOptions(optionsJSON)) else {
        return nil
    }
    return retainBox(typesetter)
}

@_cdecl("ct_typesetter_create_line")
func ct_typesetter_create_line(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ stringRange: CFRange
) -> UnsafeMutableRawPointer? {
    guard let typesetterPtr else { return nil }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return retainBox(CTTypesetterCreateLine(typesetter, stringRange))
}

@_cdecl("ct_typesetter_create_line_with_offset")
func ct_typesetter_create_line_with_offset(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ stringRange: CFRange,
    _ offset: Double
) -> UnsafeMutableRawPointer? {
    guard let typesetterPtr else { return nil }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return retainBox(CTTypesetterCreateLineWithOffset(typesetter, stringRange, offset))
}

@_cdecl("ct_typesetter_suggest_line_break")
func ct_typesetter_suggest_line_break(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ startIndex: Int,
    _ width: Double
) -> Int {
    guard let typesetterPtr else { return 0 }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return CTTypesetterSuggestLineBreak(typesetter, startIndex, width)
}

@_cdecl("ct_typesetter_suggest_line_break_with_offset")
func ct_typesetter_suggest_line_break_with_offset(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ startIndex: Int,
    _ width: Double,
    _ offset: Double
) -> Int {
    guard let typesetterPtr else { return 0 }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return CTTypesetterSuggestLineBreakWithOffset(typesetter, startIndex, width, offset)
}

@_cdecl("ct_typesetter_suggest_cluster_break")
func ct_typesetter_suggest_cluster_break(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ startIndex: Int,
    _ width: Double
) -> Int {
    guard let typesetterPtr else { return 0 }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return CTTypesetterSuggestClusterBreak(typesetter, startIndex, width)
}

@_cdecl("ct_typesetter_suggest_cluster_break_with_offset")
func ct_typesetter_suggest_cluster_break_with_offset(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ startIndex: Int,
    _ width: Double,
    _ offset: Double
) -> Int {
    guard let typesetterPtr else { return 0 }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return CTTypesetterSuggestClusterBreakWithOffset(typesetter, startIndex, width, offset)
}

@_cdecl("ct_typesetter_get_type_id")
func ct_typesetter_get_type_id() -> UInt64 {
    UInt64(CTTypesetterGetTypeID())
}
