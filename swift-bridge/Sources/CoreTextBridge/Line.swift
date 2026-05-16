import CoreText
import Foundation

private func truncationType(_ raw: UInt32) -> CTLineTruncationType {
    switch raw {
    case 1: return kCTLineTruncationEnd
    case 2: return kCTLineTruncationMiddle
    default: return kCTLineTruncationStart
    }
}

@_cdecl("ct_line_create_with_attributed_string")
func ct_line_create_with_attributed_string(
    _ attributedStringPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let attributedStringPtr else { return nil }
    let attributedString: NSAttributedString = unbox(attributedStringPtr, as: NSAttributedString.self)
    return retainBox(CTLineCreateWithAttributedString(attributedString))
}

@_cdecl("ct_line_create_truncated_line")
func ct_line_create_truncated_line(
    _ linePtr: UnsafeMutableRawPointer?,
    _ width: Double,
    _ truncationTypeRaw: UInt32,
    _ truncationTokenPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let linePtr else { return nil }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    let truncationToken: CTLine? = truncationTokenPtr.map { unbox($0, as: CTLine.self) }
    guard let truncated = CTLineCreateTruncatedLine(line, width, truncationType(truncationTypeRaw), truncationToken) else {
        return nil
    }
    return retainBox(truncated)
}

@_cdecl("ct_line_create_justified_line")
func ct_line_create_justified_line(
    _ linePtr: UnsafeMutableRawPointer?,
    _ justificationFactor: Double,
    _ justificationWidth: Double
) -> UnsafeMutableRawPointer? {
    guard let linePtr else { return nil }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    guard let justified = CTLineCreateJustifiedLine(line, CGFloat(justificationFactor), justificationWidth) else {
        return nil
    }
    return retainBox(justified)
}

@_cdecl("ct_line_get_glyph_count")
func ct_line_get_glyph_count(_ linePtr: UnsafeMutableRawPointer?) -> Int {
    guard let linePtr else { return 0 }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetGlyphCount(line)
}

@_cdecl("ct_line_get_string_range")
func ct_line_get_string_range(_ linePtr: UnsafeMutableRawPointer?) -> CFRange {
    guard let linePtr else { return CFRange(location: 0, length: 0) }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetStringRange(line)
}

@_cdecl("ct_line_get_pen_offset_for_flush")
func ct_line_get_pen_offset_for_flush(
    _ linePtr: UnsafeMutableRawPointer?,
    _ flushFactor: Double,
    _ flushWidth: Double
) -> Double {
    guard let linePtr else { return 0 }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetPenOffsetForFlush(line, CGFloat(flushFactor), flushWidth)
}

@_cdecl("ct_line_get_typographic_bounds")
func ct_line_get_typographic_bounds(
    _ linePtr: UnsafeMutableRawPointer?,
    _ ascent: UnsafeMutablePointer<CGFloat>?,
    _ descent: UnsafeMutablePointer<CGFloat>?,
    _ leading: UnsafeMutablePointer<CGFloat>?
) -> Double {
    guard let linePtr else { return 0 }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetTypographicBounds(line, ascent, descent, leading)
}

@_cdecl("ct_line_get_bounds_with_options")
func ct_line_get_bounds_with_options(_ linePtr: UnsafeMutableRawPointer?, _ options: UInt64) -> CGRect {
    guard let linePtr else { return .null }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetBoundsWithOptions(line, CTLineBoundsOptions(rawValue: UInt(options)))
}

@_cdecl("ct_line_get_trailing_whitespace_width")
func ct_line_get_trailing_whitespace_width(_ linePtr: UnsafeMutableRawPointer?) -> Double {
    guard let linePtr else { return 0 }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetTrailingWhitespaceWidth(line)
}

@_cdecl("ct_line_get_image_bounds")
func ct_line_get_image_bounds(_ linePtr: UnsafeMutableRawPointer?) -> CGRect {
    guard let linePtr else { return .null }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetImageBounds(line, nil)
}

@_cdecl("ct_line_get_string_index_for_position")
func ct_line_get_string_index_for_position(
    _ linePtr: UnsafeMutableRawPointer?,
    _ position: CGPoint
) -> Int {
    guard let linePtr else { return kCFNotFound }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetStringIndexForPosition(line, position)
}

@_cdecl("ct_line_get_offset_for_string_index")
func ct_line_get_offset_for_string_index(
    _ linePtr: UnsafeMutableRawPointer?,
    _ charIndex: Int,
    _ secondaryOffset: UnsafeMutablePointer<CGFloat>?
) -> CGFloat {
    guard let linePtr else { return 0 }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    return CTLineGetOffsetForStringIndex(line, charIndex, secondaryOffset)
}

@_cdecl("ct_line_get_run_count")
func ct_line_get_run_count(_ linePtr: UnsafeMutableRawPointer?) -> Int {
    guard let linePtr else { return 0 }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    let runs: [CTRun] = typedArray(CTLineGetGlyphRuns(line))
    return runs.count
}

@_cdecl("ct_line_copy_runs")
func ct_line_copy_runs(
    _ linePtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let linePtr else { return 0 }
    let line: CTLine = unbox(linePtr, as: CTLine.self)
    let runs: [CTRun] = typedArray(CTLineGetGlyphRuns(line))
    return fillBoxedArray(runs, buffer: buffer, capacity: capacity)
}
