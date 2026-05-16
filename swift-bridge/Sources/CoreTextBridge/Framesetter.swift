import CoreGraphics
import CoreText
import Foundation

@_cdecl("ct_framesetter_create_with_attributed_string")
func ct_framesetter_create_with_attributed_string(
    _ attributedStringPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let attributedStringPtr else { return nil }
    let attributedString: NSAttributedString = unbox(attributedStringPtr, as: NSAttributedString.self)
    return retainBox(CTFramesetterCreateWithAttributedString(attributedString))
}

@_cdecl("ct_framesetter_create_with_typesetter")
func ct_framesetter_create_with_typesetter(_ typesetterPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let typesetterPtr else { return nil }
    let typesetter: CTTypesetter = unbox(typesetterPtr, as: CTTypesetter.self)
    return retainBox(CTFramesetterCreateWithTypesetter(typesetter))
}

@_cdecl("ct_framesetter_copy_typesetter")
func ct_framesetter_copy_typesetter(_ framesetterPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let framesetterPtr else { return nil }
    let framesetter: CTFramesetter = unbox(framesetterPtr, as: CTFramesetter.self)
    return retainBox(CTFramesetterGetTypesetter(framesetter))
}

@_cdecl("ct_framesetter_suggest_frame_size")
func ct_framesetter_suggest_frame_size(
    _ framesetterPtr: UnsafeMutableRawPointer?,
    _ stringRange: CFRange,
    _ constraints: CGSize,
    _ fitRange: UnsafeMutablePointer<CFRange>?
) -> CGSize {
    guard let framesetterPtr else { return .zero }
    let framesetter: CTFramesetter = unbox(framesetterPtr, as: CTFramesetter.self)
    return CTFramesetterSuggestFrameSizeWithConstraints(framesetter, stringRange, nil, constraints, fitRange)
}

@_cdecl("ct_framesetter_create_frame_in_rect")
func ct_framesetter_create_frame_in_rect(
    _ framesetterPtr: UnsafeMutableRawPointer?,
    _ stringRange: CFRange,
    _ rect: CGRect
) -> UnsafeMutableRawPointer? {
    guard let framesetterPtr else { return nil }
    let framesetter: CTFramesetter = unbox(framesetterPtr, as: CTFramesetter.self)
    let path = CGPath(rect: rect, transform: nil)
    return retainBox(CTFramesetterCreateFrame(framesetter, stringRange, path, nil))
}
