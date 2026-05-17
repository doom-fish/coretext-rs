import CoreText
import Foundation

private func textAlignment(_ raw: UInt8) -> CTTextAlignment {
    switch raw {
    case 0: return kCTTextAlignmentLeft
    case 1: return kCTTextAlignmentRight
    case 2: return kCTTextAlignmentCenter
    case 3: return kCTTextAlignmentJustified
    default: return kCTTextAlignmentNatural
    }
}

private func textAlignmentRaw(_ alignment: CTTextAlignment) -> UInt8 {
    switch alignment {
    case kCTTextAlignmentLeft: return 0
    case kCTTextAlignmentRight: return 1
    case kCTTextAlignmentCenter: return 2
    case kCTTextAlignmentJustified: return 3
    default: return 4
    }
}

@_cdecl("ct_text_tab_create")
func ct_text_tab_create(_ alignment: UInt8, _ location: Double) -> UnsafeMutableRawPointer? {
    retainBox(CTTextTabCreate(textAlignment(alignment), location, nil))
}

@_cdecl("ct_text_tab_get_alignment")
func ct_text_tab_get_alignment(_ textTabPtr: UnsafeMutableRawPointer?) -> UInt8 {
    guard let textTabPtr else { return 4 }
    let textTab: CTTextTab = unbox(textTabPtr, as: CTTextTab.self)
    return textAlignmentRaw(CTTextTabGetAlignment(textTab))
}

@_cdecl("ct_text_tab_get_location")
func ct_text_tab_get_location(_ textTabPtr: UnsafeMutableRawPointer?) -> Double {
    guard let textTabPtr else { return 0 }
    let textTab: CTTextTab = unbox(textTabPtr, as: CTTextTab.self)
    return CTTextTabGetLocation(textTab)
}

@_cdecl("ct_text_tab_get_options_json")
func ct_text_tab_get_options_json(_ textTabPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let textTabPtr else {
        return duplicateCString("null")
    }
    let textTab: CTTextTab = unbox(textTabPtr, as: CTTextTab.self)
    return cfToJSONCString(CTTextTabGetOptions(textTab))
}

@_cdecl("ct_text_tab_get_type_id")
func ct_text_tab_get_type_id() -> UInt64 {
    UInt64(CTTextTabGetTypeID())
}
