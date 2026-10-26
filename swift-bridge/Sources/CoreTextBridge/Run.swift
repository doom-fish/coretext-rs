import CoreText
import Foundation

private struct RunAttributesPayload: Codable {
    let attributeCount: Int
    let keys: [String]
}

private func runAttributesPayload(_ run: CTRun) -> RunAttributesPayload {
    let attributes = CTRunGetAttributes(run) as NSDictionary
    let keys = attributes.allKeys.compactMap { $0 as? String }
    return RunAttributesPayload(attributeCount: attributes.count, keys: keys.sorted())
}

@_cdecl("ct_run_get_glyph_count")
public func ct_run_get_glyph_count(_ runPtr: UnsafeMutableRawPointer?) -> Int {
    guard let runPtr else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetGlyphCount(run)
}

@_cdecl("ct_run_copy_attributes_json")
public func ct_run_copy_attributes_json(_ runPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let runPtr else {
        return jsonCString(RunAttributesPayload(attributeCount: 0, keys: []))
    }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return jsonCString(runAttributesPayload(run))
}

@_cdecl("ct_run_copy_font")
public func ct_run_copy_font(_ runPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let runPtr else { return nil }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    guard let value = (CTRunGetAttributes(run) as NSDictionary)[kCTFontAttributeName],
          CFGetTypeID(value as CFTypeRef) == CTFontGetTypeID() else {
        return nil
    }
    return retainBox(value as! CTFont)
}

@_cdecl("ct_run_get_status")
public func ct_run_get_status(_ runPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let runPtr else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetStatus(run).rawValue
}

@_cdecl("ct_run_copy_glyphs")
public func ct_run_copy_glyphs(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CGGlyph>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    let count = min(capacity, CTRunGetGlyphCount(run))
    CTRunGetGlyphs(run, CFRange(location: 0, length: count), buffer)
    return count
}

@_cdecl("ct_run_copy_positions")
public func ct_run_copy_positions(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CGPoint>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    let count = min(capacity, CTRunGetGlyphCount(run))
    CTRunGetPositions(run, CFRange(location: 0, length: count), buffer)
    return count
}

@_cdecl("ct_run_copy_advances")
public func ct_run_copy_advances(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CGSize>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    let count = min(capacity, CTRunGetGlyphCount(run))
    CTRunGetAdvances(run, CFRange(location: 0, length: count), buffer)
    return count
}

@_cdecl("ct_run_copy_string_indices")
public func ct_run_copy_string_indices(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CFIndex>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    let count = min(capacity, CTRunGetGlyphCount(run))
    CTRunGetStringIndices(run, CFRange(location: 0, length: count), buffer)
    return count
}

@_cdecl("ct_run_get_string_range")
public func ct_run_get_string_range(_ runPtr: UnsafeMutableRawPointer?) -> CFRange {
    guard let runPtr else { return CFRange(location: 0, length: 0) }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetStringRange(run)
}

@_cdecl("ct_run_get_typographic_bounds")
public func ct_run_get_typographic_bounds(
    _ runPtr: UnsafeMutableRawPointer?,
    _ ascent: UnsafeMutablePointer<CGFloat>?,
    _ descent: UnsafeMutablePointer<CGFloat>?,
    _ leading: UnsafeMutablePointer<CGFloat>?
) -> Double {
    guard let runPtr else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetTypographicBounds(run, CFRange(location: 0, length: 0), ascent, descent, leading)
}

@_cdecl("ct_run_get_image_bounds")
public func ct_run_get_image_bounds(_ runPtr: UnsafeMutableRawPointer?) -> CGRect {
    guard let runPtr else { return .null }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetImageBounds(run, nil, CFRange(location: 0, length: 0))
}

@_cdecl("ct_run_get_text_matrix")
public func ct_run_get_text_matrix(_ runPtr: UnsafeMutableRawPointer?) -> CGAffineTransform {
    guard let runPtr else { return .identity }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetTextMatrix(run)
}

@_cdecl("ct_run_copy_base_advances_and_origins")
public func ct_run_copy_base_advances_and_origins(
    _ runPtr: UnsafeMutableRawPointer?,
    _ advances: UnsafeMutablePointer<CGSize>?,
    _ origins: UnsafeMutablePointer<CGPoint>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    let count = min(capacity, CTRunGetGlyphCount(run))
    CTRunGetBaseAdvancesAndOrigins(run, CFRange(location: 0, length: count), advances, origins)
    return count
}

@_cdecl("ct_run_get_type_id")
public func ct_run_get_type_id() -> UInt64 {
    UInt64(CTRunGetTypeID())
}
