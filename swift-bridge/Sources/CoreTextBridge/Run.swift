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
func ct_run_get_glyph_count(_ runPtr: UnsafeMutableRawPointer?) -> Int {
    guard let runPtr else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetGlyphCount(run)
}

@_cdecl("ct_run_copy_attributes_json")
func ct_run_copy_attributes_json(_ runPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let runPtr else {
        return jsonCString(RunAttributesPayload(attributeCount: 0, keys: []))
    }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return jsonCString(runAttributesPayload(run))
}

@_cdecl("ct_run_get_status")
func ct_run_get_status(_ runPtr: UnsafeMutableRawPointer?) -> UInt32 {
    guard let runPtr else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetStatus(run).rawValue
}

@_cdecl("ct_run_copy_glyphs")
func ct_run_copy_glyphs(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CGGlyph>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    CTRunGetGlyphs(run, CFRange(location: 0, length: capacity), buffer)
    return min(capacity, CTRunGetGlyphCount(run))
}

@_cdecl("ct_run_copy_positions")
func ct_run_copy_positions(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CGPoint>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    CTRunGetPositions(run, CFRange(location: 0, length: capacity), buffer)
    return min(capacity, CTRunGetGlyphCount(run))
}

@_cdecl("ct_run_copy_advances")
func ct_run_copy_advances(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CGSize>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    CTRunGetAdvances(run, CFRange(location: 0, length: capacity), buffer)
    return min(capacity, CTRunGetGlyphCount(run))
}

@_cdecl("ct_run_copy_string_indices")
func ct_run_copy_string_indices(
    _ runPtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CFIndex>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, let buffer, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    CTRunGetStringIndices(run, CFRange(location: 0, length: capacity), buffer)
    return min(capacity, CTRunGetGlyphCount(run))
}

@_cdecl("ct_run_get_string_range")
func ct_run_get_string_range(_ runPtr: UnsafeMutableRawPointer?) -> CFRange {
    guard let runPtr else { return CFRange(location: 0, length: 0) }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetStringRange(run)
}

@_cdecl("ct_run_get_typographic_bounds")
func ct_run_get_typographic_bounds(
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
func ct_run_get_image_bounds(_ runPtr: UnsafeMutableRawPointer?) -> CGRect {
    guard let runPtr else { return .null }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetImageBounds(run, nil, CFRange(location: 0, length: 0))
}

@_cdecl("ct_run_get_text_matrix")
func ct_run_get_text_matrix(_ runPtr: UnsafeMutableRawPointer?) -> CGAffineTransform {
    guard let runPtr else { return .identity }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    return CTRunGetTextMatrix(run)
}

@_cdecl("ct_run_copy_base_advances_and_origins")
func ct_run_copy_base_advances_and_origins(
    _ runPtr: UnsafeMutableRawPointer?,
    _ advances: UnsafeMutablePointer<CGSize>?,
    _ origins: UnsafeMutablePointer<CGPoint>?,
    _ capacity: Int
) -> Int {
    guard let runPtr, capacity > 0 else { return 0 }
    let run: CTRun = unbox(runPtr, as: CTRun.self)
    CTRunGetBaseAdvancesAndOrigins(run, CFRange(location: 0, length: capacity), advances, origins)
    return min(capacity, CTRunGetGlyphCount(run))
}

@_cdecl("ct_run_get_type_id")
func ct_run_get_type_id() -> UInt64 {
    UInt64(CTRunGetTypeID())
}
