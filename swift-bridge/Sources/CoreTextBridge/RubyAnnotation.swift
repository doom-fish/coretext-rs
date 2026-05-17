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

private struct RubyAnnotationAttributesPayload: Decodable {
    let alignment: UInt8
    let overhang: UInt8
    let sizeFactor: Double
    let texts: [String?]
}

@_cdecl("ct_ruby_annotation_create_with_attributes_json")
func ct_ruby_annotation_create_with_attributes_json(
    _ attrsJSON: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let payload = decodeJSON(attrsJSON, as: RubyAnnotationAttributesPayload.self) else {
        return nil
    }

    var texts = Array(payload.texts.prefix(4))
    while texts.count < 4 {
        texts.append(nil)
    }

    let populated = texts.enumerated().compactMap { index, text in
        text.map { (index, $0) }
    }
    if populated.count == 1, let (index, text) = populated.first {
        let attributes: [CFString: Any] = [
            kCTRubyAnnotationSizeFactorAttributeName: NSNumber(value: payload.sizeFactor)
        ]
        let annotation = CTRubyAnnotationCreateWithAttributes(
            rubyAlignment(payload.alignment),
            rubyOverhang(payload.overhang),
            rubyPosition(UInt8(index)),
            text as CFString,
            attributes as CFDictionary
        )
        return retainBox(annotation)
    }

    var rubyTexts: [Unmanaged<CFString>?] = texts.map { text in
        text.map { Unmanaged.passRetained($0 as CFString) }
    }
    defer {
        for case let value? in rubyTexts {
            value.release()
        }
    }
    let annotation = rubyTexts.withUnsafeMutableBufferPointer { buffer in
        CTRubyAnnotationCreate(
            rubyAlignment(payload.alignment),
            rubyOverhang(payload.overhang),
            CGFloat(payload.sizeFactor),
            buffer.baseAddress!
        )
    }
    return retainBox(annotation)
}

@_cdecl("ct_ruby_annotation_get_type_id")
func ct_ruby_annotation_get_type_id() -> UInt64 {
    UInt64(CTRubyAnnotationGetTypeID())
}
