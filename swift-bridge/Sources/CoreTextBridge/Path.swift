import CoreGraphics
import CoreText
import Foundation

@_cdecl("ct_font_create_path_for_glyph")
func ct_font_create_path_for_glyph(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ glyph: UInt16,
    _ transform: UnsafePointer<CGAffineTransform>?
) -> UnsafeMutableRawPointer? {
    guard let fontPtr else { return nil }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    guard let path = CTFontCreatePathForGlyph(font, glyph, transform) else {
        return nil
    }
    return retainBox(path)
}

@_cdecl("ct_font_draw_glyphs")
func ct_font_draw_glyphs(
    _ fontPtr: UnsafeMutableRawPointer?,
    _ glyphs: UnsafePointer<CGGlyph>?,
    _ positions: UnsafePointer<CGPoint>?,
    _ count: Int,
    _ contextPtr: UnsafeMutableRawPointer?
) {
    guard let fontPtr, let glyphs, let positions, count > 0, let contextPtr else { return }
    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    let context = Unmanaged<CGContext>.fromOpaque(contextPtr).takeUnretainedValue()
    CTFontDrawGlyphs(font, glyphs, positions, count, context)
}

@_cdecl("ct_path_get_bounding_box")
func ct_path_get_bounding_box(_ pathPtr: UnsafeMutableRawPointer?) -> CGRect {
    guard let pathPtr else { return .null }
    let path: CGPath = unbox(pathPtr, as: CGPath.self)
    return path.boundingBox
}

@_cdecl("ct_path_get_path_bounding_box")
func ct_path_get_path_bounding_box(_ pathPtr: UnsafeMutableRawPointer?) -> CGRect {
    guard let pathPtr else { return .null }
    let path: CGPath = unbox(pathPtr, as: CGPath.self)
    return path.boundingBoxOfPath
}

@_cdecl("ct_path_is_empty")
func ct_path_is_empty(_ pathPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let pathPtr else { return true }
    let path: CGPath = unbox(pathPtr, as: CGPath.self)
    return path.isEmpty
}

@_cdecl("ct_path_copy_elements")
func ct_path_copy_elements(
    _ pathPtr: UnsafeMutableRawPointer?,
    _ kinds: UnsafeMutablePointer<UInt8>?,
    _ points: UnsafeMutablePointer<CGPoint>?,
    _ capacity: Int
) -> Int {
    guard let pathPtr else { return 0 }
    let path: CGPath = unbox(pathPtr, as: CGPath.self)
    var elements: [(kind: UInt8, points: [CGPoint])] = []
    path.applyWithBlock { elementPointer in
        let element = elementPointer.pointee
        let pointCount: Int
        switch element.type {
        case .moveToPoint, .addLineToPoint:
            pointCount = 1
        case .addQuadCurveToPoint:
            pointCount = 2
        case .addCurveToPoint:
            pointCount = 3
        case .closeSubpath:
            pointCount = 0
        @unknown default:
            return
        }
        let elementPoints = (0..<pointCount).map { element.points[$0] }
        elements.append((UInt8(truncatingIfNeeded: element.type.rawValue), elementPoints))
    }
    guard let kinds, let points else {
        return elements.count
    }
    let count = min(max(capacity, 0), elements.count)
    for index in 0..<count {
        kinds[index] = elements[index].kind
        for (offset, point) in elements[index].points.enumerated() {
            points[index * 3 + offset] = point
        }
    }
    return count
}
