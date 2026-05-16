import CoreGraphics
import CoreText
import Foundation

@_cdecl("ct_frame_get_string_range")
func ct_frame_get_string_range(_ framePtr: UnsafeMutableRawPointer?) -> CFRange {
    guard let framePtr else { return CFRange(location: 0, length: 0) }
    let frame: CTFrame = unbox(framePtr, as: CTFrame.self)
    return CTFrameGetStringRange(frame)
}

@_cdecl("ct_frame_get_visible_string_range")
func ct_frame_get_visible_string_range(_ framePtr: UnsafeMutableRawPointer?) -> CFRange {
    guard let framePtr else { return CFRange(location: 0, length: 0) }
    let frame: CTFrame = unbox(framePtr, as: CTFrame.self)
    return CTFrameGetVisibleStringRange(frame)
}

@_cdecl("ct_frame_copy_path_bounding_box")
func ct_frame_copy_path_bounding_box(_ framePtr: UnsafeMutableRawPointer?) -> CGRect {
    guard let framePtr else { return .null }
    let frame: CTFrame = unbox(framePtr, as: CTFrame.self)
    return CTFrameGetPath(frame).boundingBox
}

@_cdecl("ct_frame_has_frame_attributes")
func ct_frame_has_frame_attributes(_ framePtr: UnsafeMutableRawPointer?) -> Bool {
    guard let framePtr else { return false }
    let frame: CTFrame = unbox(framePtr, as: CTFrame.self)
    return CTFrameGetFrameAttributes(frame) != nil
}

@_cdecl("ct_frame_get_line_count")
func ct_frame_get_line_count(_ framePtr: UnsafeMutableRawPointer?) -> Int {
    guard let framePtr else { return 0 }
    let frame: CTFrame = unbox(framePtr, as: CTFrame.self)
    let lines: [CTLine] = typedArray(CTFrameGetLines(frame))
    return lines.count
}

@_cdecl("ct_frame_copy_lines")
func ct_frame_copy_lines(
    _ framePtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let framePtr else { return 0 }
    let frame: CTFrame = unbox(framePtr, as: CTFrame.self)
    let lines: [CTLine] = typedArray(CTFrameGetLines(frame))
    return fillBoxedArray(lines, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_frame_copy_line_origins")
func ct_frame_copy_line_origins(
    _ framePtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<CGPoint>?,
    _ capacity: Int
) -> Int {
    guard let framePtr, let buffer, capacity > 0 else { return 0 }
    let frame: CTFrame = unbox(framePtr, as: CTFrame.self)
    let lines: [CTLine] = typedArray(CTFrameGetLines(frame))
    CTFrameGetLineOrigins(frame, CFRange(location: 0, length: capacity), buffer)
    return min(capacity, lines.count)
}
