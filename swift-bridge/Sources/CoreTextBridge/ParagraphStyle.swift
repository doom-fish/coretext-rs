import CoreFoundation
import CoreText
import Foundation

private final class PointerOwner {
    private var cleanup: [() -> Void] = []

    func make<T>(_ value: T) -> UnsafeMutablePointer<T> {
        let pointer = UnsafeMutablePointer<T>.allocate(capacity: 1)
        pointer.initialize(to: value)
        cleanup.append {
            pointer.deinitialize(count: 1)
            pointer.deallocate()
        }
        return pointer
    }

    deinit {
        cleanup.forEach { $0() }
    }
}

private struct ParagraphStyleOptionsPayload: Decodable {
    let alignment: UInt8?
    let firstLineHeadIndent: Double?
    let headIndent: Double?
    let tailIndent: Double?
    let defaultTabInterval: Double?
    let lineBreakMode: UInt8?
    let lineHeightMultiple: Double?
    let maximumLineHeight: Double?
    let minimumLineHeight: Double?
    let paragraphSpacing: Double?
    let paragraphSpacingBefore: Double?
    let baseWritingDirection: Int8?
    let maximumLineSpacing: Double?
    let minimumLineSpacing: Double?
    let lineSpacingAdjustment: Double?
    let lineBoundsOptions: UInt64?
}

private func textAlignment(_ raw: UInt8) -> CTTextAlignment {
    switch raw {
    case 0: return kCTTextAlignmentLeft
    case 1: return kCTTextAlignmentRight
    case 2: return kCTTextAlignmentCenter
    case 3: return kCTTextAlignmentJustified
    default: return kCTTextAlignmentNatural
    }
}

private func textAlignmentRaw(_ value: CTTextAlignment) -> UInt8 {
    switch value {
    case kCTTextAlignmentLeft: return 0
    case kCTTextAlignmentRight: return 1
    case kCTTextAlignmentCenter: return 2
    case kCTTextAlignmentJustified: return 3
    default: return 4
    }
}

private func lineBreakMode(_ raw: UInt8) -> CTLineBreakMode {
    switch raw {
    case 1: return kCTLineBreakByCharWrapping
    case 2: return kCTLineBreakByClipping
    case 3: return kCTLineBreakByTruncatingHead
    case 4: return kCTLineBreakByTruncatingTail
    case 5: return kCTLineBreakByTruncatingMiddle
    default: return kCTLineBreakByWordWrapping
    }
}

private func lineBreakModeRaw(_ value: CTLineBreakMode) -> UInt8 {
    switch value {
    case kCTLineBreakByCharWrapping: return 1
    case kCTLineBreakByClipping: return 2
    case kCTLineBreakByTruncatingHead: return 3
    case kCTLineBreakByTruncatingTail: return 4
    case kCTLineBreakByTruncatingMiddle: return 5
    default: return 0
    }
}

private func writingDirection(_ raw: Int8) -> CTWritingDirection {
    switch raw {
    case 0: return kCTWritingDirectionLeftToRight
    case 1: return kCTWritingDirectionRightToLeft
    default: return kCTWritingDirectionNatural
    }
}

private func writingDirectionRaw(_ value: CTWritingDirection) -> Int8 {
    switch value {
    case kCTWritingDirectionLeftToRight: return 0
    case kCTWritingDirectionRightToLeft: return 1
    default: return -1
    }
}

private func paragraphStyleValue<T>(
    _ paragraphStyle: CTParagraphStyle,
    spec: CTParagraphStyleSpecifier,
    default defaultValue: T,
    type: T.Type = T.self
) -> T {
    var value = defaultValue
    _ = CTParagraphStyleGetValueForSpecifier(
        paragraphStyle,
        spec,
        MemoryLayout<T>.size,
        &value
    )
    return value
}

private func paragraphStyleTextTabs(_ paragraphStyle: CTParagraphStyle) -> [CTTextTab] {
    var tabStops: Unmanaged<CFArray>?
    let ok = CTParagraphStyleGetValueForSpecifier(
        paragraphStyle,
        kCTParagraphStyleSpecifierTabStops,
        MemoryLayout<Unmanaged<CFArray>?>.size,
        &tabStops
    )
    guard ok, let tabStops else {
        return []
    }

    let array = tabStops.takeUnretainedValue()
    let count = CFArrayGetCount(array)
    guard count > 0 else {
        return []
    }

    return (0..<count).map { index in
        unsafeBitCast(CFArrayGetValueAtIndex(array, index), to: CTTextTab.self)
    }
}

@_cdecl("ct_paragraph_style_create")
func ct_paragraph_style_create(
    _ optionsJSON: UnsafePointer<CChar>?,
    _ textTabHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ tabCount: Int
) -> UnsafeMutableRawPointer? {
    let payload = decodeJSON(optionsJSON, as: ParagraphStyleOptionsPayload.self)
    let textTabs: [CTTextTab] = handlesToValues(textTabHandles, count: tabCount)
    let owner = PointerOwner()
    var settings: [CTParagraphStyleSetting] = []

    if let alignment = payload?.alignment {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierAlignment,
                valueSize: MemoryLayout<CTTextAlignment>.size,
                value: owner.make(textAlignment(alignment))
            )
        )
    }
    if let firstLineHeadIndent = payload?.firstLineHeadIndent {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierFirstLineHeadIndent,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(firstLineHeadIndent))
            )
        )
    }
    if let headIndent = payload?.headIndent {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierHeadIndent,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(headIndent))
            )
        )
    }
    if let tailIndent = payload?.tailIndent {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierTailIndent,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(tailIndent))
            )
        )
    }
    if !textTabs.isEmpty {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierTabStops,
                valueSize: MemoryLayout<CFArray>.size,
                value: owner.make(textTabs as CFArray)
            )
        )
    }
    if let defaultTabInterval = payload?.defaultTabInterval {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierDefaultTabInterval,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(defaultTabInterval))
            )
        )
    }
    if let lineBreakModeValue = payload?.lineBreakMode {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierLineBreakMode,
                valueSize: MemoryLayout<CTLineBreakMode>.size,
                value: owner.make(lineBreakMode(lineBreakModeValue))
            )
        )
    }
    if let lineHeightMultiple = payload?.lineHeightMultiple {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierLineHeightMultiple,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(lineHeightMultiple))
            )
        )
    }
    if let maximumLineHeight = payload?.maximumLineHeight {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierMaximumLineHeight,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(maximumLineHeight))
            )
        )
    }
    if let minimumLineHeight = payload?.minimumLineHeight {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierMinimumLineHeight,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(minimumLineHeight))
            )
        )
    }
    if let paragraphSpacing = payload?.paragraphSpacing {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierParagraphSpacing,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(paragraphSpacing))
            )
        )
    }
    if let paragraphSpacingBefore = payload?.paragraphSpacingBefore {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierParagraphSpacingBefore,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(paragraphSpacingBefore))
            )
        )
    }
    if let baseWritingDirection = payload?.baseWritingDirection {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierBaseWritingDirection,
                valueSize: MemoryLayout<CTWritingDirection>.size,
                value: owner.make(writingDirection(baseWritingDirection))
            )
        )
    }
    if let maximumLineSpacing = payload?.maximumLineSpacing {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierMaximumLineSpacing,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(maximumLineSpacing))
            )
        )
    }
    if let minimumLineSpacing = payload?.minimumLineSpacing {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierMinimumLineSpacing,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(minimumLineSpacing))
            )
        )
    }
    if let lineSpacingAdjustment = payload?.lineSpacingAdjustment {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierLineSpacingAdjustment,
                valueSize: MemoryLayout<CGFloat>.size,
                value: owner.make(CGFloat(lineSpacingAdjustment))
            )
        )
    }
    if let lineBoundsOptions = payload?.lineBoundsOptions {
        settings.append(
            CTParagraphStyleSetting(
                spec: kCTParagraphStyleSpecifierLineBoundsOptions,
                valueSize: MemoryLayout<CTLineBoundsOptions>.size,
                value: owner.make(CTLineBoundsOptions(rawValue: UInt(lineBoundsOptions)))
            )
        )
    }

    let paragraphStyle = CTParagraphStyleCreate(settings, settings.count)
    return retainBox(paragraphStyle)
}

@_cdecl("ct_paragraph_style_copy")
func ct_paragraph_style_copy(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let paragraphStylePtr else { return nil }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return retainBox(CTParagraphStyleCreateCopy(paragraphStyle))
}

@_cdecl("ct_paragraph_style_get_alignment")
func ct_paragraph_style_get_alignment(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> UInt8 {
    guard let paragraphStylePtr else { return 4 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return textAlignmentRaw(paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierAlignment, default: kCTTextAlignmentNatural))
}

@_cdecl("ct_paragraph_style_get_first_line_head_indent")
func ct_paragraph_style_get_first_line_head_indent(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierFirstLineHeadIndent, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_head_indent")
func ct_paragraph_style_get_head_indent(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierHeadIndent, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_tail_indent")
func ct_paragraph_style_get_tail_indent(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierTailIndent, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_default_tab_interval")
func ct_paragraph_style_get_default_tab_interval(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierDefaultTabInterval, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_line_break_mode")
func ct_paragraph_style_get_line_break_mode(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> UInt8 {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return lineBreakModeRaw(paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierLineBreakMode, default: kCTLineBreakByWordWrapping))
}

@_cdecl("ct_paragraph_style_get_line_height_multiple")
func ct_paragraph_style_get_line_height_multiple(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierLineHeightMultiple, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_maximum_line_height")
func ct_paragraph_style_get_maximum_line_height(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierMaximumLineHeight, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_minimum_line_height")
func ct_paragraph_style_get_minimum_line_height(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierMinimumLineHeight, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_paragraph_spacing")
func ct_paragraph_style_get_paragraph_spacing(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierParagraphSpacing, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_paragraph_spacing_before")
func ct_paragraph_style_get_paragraph_spacing_before(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierParagraphSpacingBefore, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_base_writing_direction")
func ct_paragraph_style_get_base_writing_direction(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Int8 {
    guard let paragraphStylePtr else { return -1 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return writingDirectionRaw(paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierBaseWritingDirection, default: kCTWritingDirectionNatural))
}

@_cdecl("ct_paragraph_style_get_maximum_line_spacing")
func ct_paragraph_style_get_maximum_line_spacing(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierMaximumLineSpacing, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_minimum_line_spacing")
func ct_paragraph_style_get_minimum_line_spacing(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierMinimumLineSpacing, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_line_spacing_adjustment")
func ct_paragraph_style_get_line_spacing_adjustment(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Double {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleValue(paragraphStyle, spec: kCTParagraphStyleSpecifierLineSpacingAdjustment, default: CGFloat(0))
}

@_cdecl("ct_paragraph_style_get_line_bounds_options")
func ct_paragraph_style_get_line_bounds_options(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    let value: CTLineBoundsOptions = paragraphStyleValue(
        paragraphStyle,
        spec: kCTParagraphStyleSpecifierLineBoundsOptions,
        default: CTLineBoundsOptions(rawValue: 0)
    )
    return UInt64(value.rawValue)
}

@_cdecl("ct_paragraph_style_get_text_tab_count")
func ct_paragraph_style_get_text_tab_count(_ paragraphStylePtr: UnsafeMutableRawPointer?) -> Int {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return paragraphStyleTextTabs(paragraphStyle).count
}

@_cdecl("ct_paragraph_style_copy_text_tabs")
func ct_paragraph_style_copy_text_tabs(
    _ paragraphStylePtr: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let paragraphStylePtr else { return 0 }
    let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
    return fillBoxedArray(
        paragraphStyleTextTabs(paragraphStyle),
        buffer: buffer,
        capacity: capacity
    )
}
