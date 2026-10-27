import CoreText
import CoreTextABI
import Foundation

private func typesetterOptions(_ value: UnsafePointer<CoreTextTypesetterOptions>?) -> CFDictionary? {
    guard let payload = value?.pointee else {
        return nil
    }
    var options: [CFString: Any] = [:]
    if payload.allow_unbounded_layout {
        options[kCTTypesetterOptionAllowUnboundedLayout] = true
    }
    if payload.has_forced_embedding_level {
        options[kCTTypesetterOptionForcedEmbeddingLevel] = NSNumber(value: payload.forced_embedding_level)
    }
    if options.isEmpty {
        return nil
    }
    return options as CFDictionary
}

@_cdecl("ct_typesetter_create_with_attributed_string")
public func ct_typesetter_create_with_attributed_string(
    _ attributedStringPtr: UnsafeMutableRawPointer?,
    _ options: UnsafePointer<CoreTextTypesetterOptions>?
) -> UnsafeMutableRawPointer? {
    guard let attributedStringPtr else { return nil }
    let attributedString: NSAttributedString = unbox(attributedStringPtr, as: NSAttributedString.self)
    guard let typesetter = CTTypesetterCreateWithAttributedStringAndOptions(attributedString, typesetterOptions(options)) else {
        return nil
    }
    return retainBox(typesetter)
}

@_cdecl("ct_typesetter_create_line")
public func ct_typesetter_create_line(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ stringRange: CFRange
) -> UnsafeMutableRawPointer? {
    guard let typesetterPtr else { return nil }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return retainBox(CTTypesetterCreateLine(typesetter, stringRange))
}

@_cdecl("ct_typesetter_create_line_with_offset")
public func ct_typesetter_create_line_with_offset(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ stringRange: CFRange,
    _ offset: Double
) -> UnsafeMutableRawPointer? {
    guard let typesetterPtr else { return nil }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return retainBox(CTTypesetterCreateLineWithOffset(typesetter, stringRange, offset))
}

@_cdecl("ct_typesetter_suggest_line_break")
public func ct_typesetter_suggest_line_break(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ startIndex: Int,
    _ width: Double
) -> Int {
    guard let typesetterPtr else { return 0 }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return CTTypesetterSuggestLineBreak(typesetter, startIndex, width)
}

@_cdecl("ct_typesetter_suggest_line_break_with_offset")
public func ct_typesetter_suggest_line_break_with_offset(
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
public func ct_typesetter_suggest_cluster_break(
    _ typesetterPtr: UnsafeMutableRawPointer?,
    _ startIndex: Int,
    _ width: Double
) -> Int {
    guard let typesetterPtr else { return 0 }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return CTTypesetterSuggestClusterBreak(typesetter, startIndex, width)
}

@_cdecl("ct_typesetter_suggest_cluster_break_with_offset")
public func ct_typesetter_suggest_cluster_break_with_offset(
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
public func ct_typesetter_get_type_id() -> UInt64 {
    UInt64(CTTypesetterGetTypeID())
}
