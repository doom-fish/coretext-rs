import CoreText
import Foundation

private func fontManagerScope(_ raw: UInt32) -> CTFontManagerScope {
    switch raw {
    case 1: return kCTFontManagerScopeProcess
    case 2: return kCTFontManagerScopePersistent
    case 3: return kCTFontManagerScopeSession
    default: return kCTFontManagerScopeNone
    }
}

private func fontDescriptors(from urlPath: String) -> [CTFontDescriptor] {
    let url = URL(fileURLWithPath: urlPath) as CFURL
    return typedArray(CTFontManagerCreateFontDescriptorsFromURL(url))
}

@_cdecl("ct_font_manager_copy_available_postscript_names_json")
func ct_font_manager_copy_available_postscript_names_json() -> UnsafeMutablePointer<CChar>? {
    let names = (CTFontManagerCopyAvailablePostScriptNames() as NSArray) as? [String] ?? []
    return jsonCString(names)
}

@_cdecl("ct_font_manager_copy_available_font_family_names_json")
func ct_font_manager_copy_available_font_family_names_json() -> UnsafeMutablePointer<CChar>? {
    let names = (CTFontManagerCopyAvailableFontFamilyNames() as NSArray) as? [String] ?? []
    return jsonCString(names)
}

@_cdecl("ct_font_manager_copy_available_font_urls_json")
func ct_font_manager_copy_available_font_urls_json() -> UnsafeMutablePointer<CChar>? {
    let urls = ((CTFontManagerCopyAvailableFontURLs() as NSArray?) ?? [])
        .compactMap { ($0 as? URL)?.path }
    return jsonCString(urls)
}

@_cdecl("ct_font_manager_get_descriptor_count_for_url")
func ct_font_manager_get_descriptor_count_for_url(_ urlPath: UnsafePointer<CChar>?) -> Int {
    guard let urlPath = stringFromCString(urlPath) else { return 0 }
    return fontDescriptors(from: urlPath).count
}

@_cdecl("ct_font_manager_copy_descriptors_from_url")
func ct_font_manager_copy_descriptors_from_url(
    _ urlPath: UnsafePointer<CChar>?,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let urlPath = stringFromCString(urlPath) else { return 0 }
    return fillBoxedArray(fontDescriptors(from: urlPath), buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_manager_is_supported_font")
func ct_font_manager_is_supported_font(_ urlPath: UnsafePointer<CChar>?) -> Bool {
    guard let urlPath = stringFromCString(urlPath) else { return false }
    return CTFontManagerIsSupportedFont(URL(fileURLWithPath: urlPath) as CFURL)
}

@_cdecl("ct_font_manager_register_fonts_for_url")
func ct_font_manager_register_fonts_for_url(
    _ urlPath: UnsafePointer<CChar>?,
    _ scope: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let urlPath = stringFromCString(urlPath) else { return false }
    var error: Unmanaged<CFError>?
    let ok = CTFontManagerRegisterFontsForURL(
        URL(fileURLWithPath: urlPath) as CFURL,
        fontManagerScope(scope),
        &error
    )
    errorOut?.pointee = duplicateCString(error?.takeRetainedValue().localizedDescription)
    return ok
}

@_cdecl("ct_font_manager_unregister_fonts_for_url")
func ct_font_manager_unregister_fonts_for_url(
    _ urlPath: UnsafePointer<CChar>?,
    _ scope: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    guard let urlPath = stringFromCString(urlPath) else { return false }
    var error: Unmanaged<CFError>?
    let ok = CTFontManagerUnregisterFontsForURL(
        URL(fileURLWithPath: urlPath) as CFURL,
        fontManagerScope(scope),
        &error
    )
    errorOut?.pointee = duplicateCString(error?.takeRetainedValue().localizedDescription)
    return ok
}

@_cdecl("ct_font_manager_get_scope_for_url")
func ct_font_manager_get_scope_for_url(_ urlPath: UnsafePointer<CChar>?) -> UInt32 {
    guard let urlPath = stringFromCString(urlPath) else { return 0 }
    return CTFontManagerGetScopeForURL(URL(fileURLWithPath: urlPath) as CFURL).rawValue
}

@_cdecl("ct_font_manager_get_auto_activation_setting")
func ct_font_manager_get_auto_activation_setting(_ bundleIdentifier: UnsafePointer<CChar>?) -> UInt32 {
    let bundleIdentifier = stringFromCString(bundleIdentifier) as CFString?
    return CTFontManagerGetAutoActivationSetting(bundleIdentifier).rawValue
}

@_cdecl("ct_font_manager_set_auto_activation_setting")
func ct_font_manager_set_auto_activation_setting(
    _ bundleIdentifier: UnsafePointer<CChar>?,
    _ setting: UInt32
) {
    let bundleIdentifier = stringFromCString(bundleIdentifier) as CFString?
    CTFontManagerSetAutoActivationSetting(
        bundleIdentifier,
        CTFontManagerAutoActivationSetting(rawValue: setting) ?? .default
    )
}
