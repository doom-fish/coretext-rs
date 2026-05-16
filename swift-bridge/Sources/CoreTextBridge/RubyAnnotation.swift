import CoreText
import Foundation

private func rubyAlignment(_ raw: UInt8) -> CTRubyAlignment {
    switch raw {
    case 1: return kCTRubyAlignmentStart
    case 2: return kCTRubyAlignmentCenter
    case 3: return kCTRubyAlignmentEnd
    case 4: return kCTRubyAlignmentDistributeLetter
    case 5: return kCTRubyAlignmentDistributeSpace
    case 6: return kCTRubyAlignmentLineEdge
    default: return kCTRubyAlignmentAuto
    }
}

private func rubyAlignmentRaw(_ value: CTRubyAlignment) -> UInt8 {
    switch value {
    case kCTRubyAlignmentStart: return 1
    case kCTRubyAlignmentCenter: return 2
    case kCTRubyAlignmentEnd: return 3
    case kCTRubyAlignmentDistributeLetter: return 4
    case kCTRubyAlignmentDistributeSpace: return 5
    case kCTRubyAlignmentLineEdge: return 6
    default: return 0
    }
}

private func rubyOverhang(_ raw: UInt8) -> CTRubyOverhang {
    switch raw {
    case 1: return kCTRubyOverhangStart
    case 2: return kCTRubyOverhangEnd
    case 3: return kCTRubyOverhangNone
    default: return kCTRubyOverhangAuto
    }
}

private func rubyOverhangRaw(_ value: CTRubyOverhang) -> UInt8 {
    switch value {
    case kCTRubyOverhangStart: return 1
    case kCTRubyOverhangEnd: return 2
    case kCTRubyOverhangNone: return 3
    default: return 0
    }
}

private func rubyPosition(_ raw: UInt8) -> CTRubyPosition {
    switch raw {
    case 1: return kCTRubyPositionAfter
    case 2: return kCTRubyPositionInterCharacter
    case 3: return kCTRubyPositionInline
    default: return kCTRubyPositionBefore
    }
}

private func rubyText(_ value: UnsafePointer<CChar>?) -> CFString? {
    guard let value else { return nil }
    return CFStringCreateWithCString(nil, value, CFStringBuiltInEncodings.UTF8.rawValue)
}

@_cdecl("ct_ruby_annotation_create")
func ct_ruby_annotation_create(
    _ alignment: UInt8,
    _ overhang: UInt8,
    _ sizeFactor: Double,
    _ beforeText: UnsafePointer<CChar>?,
    _ afterText: UnsafePointer<CChar>?,
    _ interCharacterText: UnsafePointer<CChar>?,
    _ inlineText: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    var texts: [Unmanaged<CFString>?] = [
        rubyText(beforeText).map(Unmanaged.passRetained),
        rubyText(afterText).map(Unmanaged.passRetained),
        rubyText(interCharacterText).map(Unmanaged.passRetained),
        rubyText(inlineText).map(Unmanaged.passRetained)
    ]
    defer {
        for case let value? in texts {
            value.release()
        }
    }
    let annotation = texts.withUnsafeMutableBufferPointer { buffer in
        CTRubyAnnotationCreate(
            rubyAlignment(alignment),
            rubyOverhang(overhang),
            CGFloat(sizeFactor),
            buffer.baseAddress!
        )
    }
    return retainBox(annotation)
}

@_cdecl("ct_ruby_annotation_copy")
func ct_ruby_annotation_copy(_ rubyAnnotationPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let rubyAnnotationPtr else { return nil }
    let rubyAnnotation: CTRubyAnnotation = unbox(rubyAnnotationPtr, as: CTRubyAnnotation.self)
    return retainBox(CTRubyAnnotationCreateCopy(rubyAnnotation))
}

@_cdecl("ct_ruby_annotation_get_alignment")
func ct_ruby_annotation_get_alignment(_ rubyAnnotationPtr: UnsafeMutableRawPointer?) -> UInt8 {
    guard let rubyAnnotationPtr else { return 0 }
    let rubyAnnotation: CTRubyAnnotation = unbox(rubyAnnotationPtr, as: CTRubyAnnotation.self)
    return rubyAlignmentRaw(CTRubyAnnotationGetAlignment(rubyAnnotation))
}

@_cdecl("ct_ruby_annotation_get_overhang")
func ct_ruby_annotation_get_overhang(_ rubyAnnotationPtr: UnsafeMutableRawPointer?) -> UInt8 {
    guard let rubyAnnotationPtr else { return 0 }
    let rubyAnnotation: CTRubyAnnotation = unbox(rubyAnnotationPtr, as: CTRubyAnnotation.self)
    return rubyOverhangRaw(CTRubyAnnotationGetOverhang(rubyAnnotation))
}

@_cdecl("ct_ruby_annotation_get_size_factor")
func ct_ruby_annotation_get_size_factor(_ rubyAnnotationPtr: UnsafeMutableRawPointer?) -> Double {
    guard let rubyAnnotationPtr else { return 0 }
    let rubyAnnotation: CTRubyAnnotation = unbox(rubyAnnotationPtr, as: CTRubyAnnotation.self)
    return CTRubyAnnotationGetSizeFactor(rubyAnnotation)
}

@_cdecl("ct_ruby_annotation_copy_text_for_position")
func ct_ruby_annotation_copy_text_for_position(
    _ rubyAnnotationPtr: UnsafeMutableRawPointer?,
    _ position: UInt8
) -> UnsafeMutablePointer<CChar>? {
    guard let rubyAnnotationPtr else { return nil }
    let rubyAnnotation: CTRubyAnnotation = unbox(rubyAnnotationPtr, as: CTRubyAnnotation.self)
    return duplicateCString(CTRubyAnnotationGetTextForPosition(rubyAnnotation, rubyPosition(position)) as String?)
}
