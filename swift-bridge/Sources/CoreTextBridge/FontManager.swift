import CoreText
import Dispatch
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

private func fontManagerErrorMessages(_ errors: CFArray?) -> [String] {
    guard let errors else { return [] }
    return (errors as NSArray).map { value in
        (value as? NSError)?.localizedDescription ?? String(describing: value)
    }
}

private func collectFontManagerErrors(
    _ body: (@escaping (CFArray, Bool) -> Bool) -> Void
) -> [String] {
    var messages: [String] = []
    let semaphore = DispatchSemaphore(value: 0)
    body { errors, done in
        messages.append(contentsOf: fontManagerErrorMessages(errors))
        if done {
            semaphore.signal()
        }
        return true
    }
    _ = semaphore.wait(timeout: .now() + .seconds(5))
    return Array(Set(messages)).sorted()
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

@_cdecl("ct_font_manager_copy_registered_descriptor_count")
func ct_font_manager_copy_registered_descriptor_count(_ scope: UInt32, _ enabled: Bool) -> Int {
    #if os(macOS)
    let _ = scope
    let _ = enabled
    return 0
    #else
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontManagerCopyRegisteredFontDescriptors(fontManagerScope(scope), enabled)
    )
    return descriptors.count
    #endif
}

@_cdecl("ct_font_manager_copy_registered_descriptors")
func ct_font_manager_copy_registered_descriptors(
    _ scope: UInt32,
    _ enabled: Bool,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    #if os(macOS)
    let _ = scope
    let _ = enabled
    return 0
    #else
    let descriptors: [CTFontDescriptor] = typedArray(
        CTFontManagerCopyRegisteredFontDescriptors(fontManagerScope(scope), enabled)
    )
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
    #endif
}

@_cdecl("ct_font_manager_create_descriptor_from_data")
func ct_font_manager_create_descriptor_from_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) -> UnsafeMutableRawPointer? {
    guard let bytes, length > 0 else { return nil }
    let data = Data(bytes: bytes, count: length) as CFData
    guard let descriptor = CTFontManagerCreateFontDescriptorFromData(data) else {
        return nil
    }
    return retainBox(descriptor)
}

@_cdecl("ct_font_manager_create_descriptors_from_data_count")
func ct_font_manager_create_descriptors_from_data_count(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) -> Int {
    guard let bytes, length > 0 else { return 0 }
    let data = Data(bytes: bytes, count: length) as CFData
    let descriptors: [CTFontDescriptor] = typedArray(CTFontManagerCreateFontDescriptorsFromData(data))
    return descriptors.count
}

@_cdecl("ct_font_manager_create_descriptors_from_data")
func ct_font_manager_create_descriptors_from_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ capacity: Int
) -> Int {
    guard let bytes, length > 0 else { return 0 }
    let data = Data(bytes: bytes, count: length) as CFData
    let descriptors: [CTFontDescriptor] = typedArray(CTFontManagerCreateFontDescriptorsFromData(data))
    return fillBoxedArray(descriptors, buffer: buffer, capacity: capacity)
}

@_cdecl("ct_font_manager_enable_font_descriptors")
func ct_font_manager_enable_font_descriptors(
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int,
    _ enable: Bool
) {
    #if os(macOS)
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    CTFontManagerEnableFontDescriptors(descriptors as CFArray, enable)
    #else
    let _ = descriptorHandles
    let _ = descriptorCount
    let _ = enable
    #endif
}

@_cdecl("ct_font_manager_register_font_descriptors")
func ct_font_manager_register_font_descriptors(
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int,
    _ scope: UInt32,
    _ enabled: Bool
) -> UnsafeMutablePointer<CChar>? {
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    let errors = collectFontManagerErrors { handler in
        CTFontManagerRegisterFontDescriptors(descriptors as CFArray, fontManagerScope(scope), enabled, handler)
    }
    return jsonCString(errors)
}

@_cdecl("ct_font_manager_register_font_urls")
func ct_font_manager_register_font_urls(
    _ urlPathsJSON: UnsafePointer<CChar>?,
    _ scope: UInt32,
    _ enabled: Bool
) -> UnsafeMutablePointer<CChar>? {
    let urls = urlArrayFromJSONPaths(urlPathsJSON)
    let errors = collectFontManagerErrors { handler in
        CTFontManagerRegisterFontURLs(urls as CFArray, fontManagerScope(scope), enabled, handler)
    }
    return jsonCString(errors)
}

@_cdecl("ct_font_manager_register_fonts_for_urls")
func ct_font_manager_register_fonts_for_urls(
    _ urlPathsJSON: UnsafePointer<CChar>?,
    _ scope: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    let urls = urlArrayFromJSONPaths(urlPathsJSON)
    let errors = collectFontManagerErrors { handler in
        CTFontManagerRegisterFontURLs(urls as CFArray, fontManagerScope(scope), true, handler)
    }
    errorOut?.pointee = jsonCString(errors)
    return errors.isEmpty
}

@_cdecl("ct_font_manager_register_fonts_with_asset_names")
func ct_font_manager_register_fonts_with_asset_names(
    _ assetNamesJSON: UnsafePointer<CChar>?,
    _ scope: UInt32,
    _ enabled: Bool
) -> Bool {
    #if os(macOS)
    let _ = assetNamesJSON
    let _ = scope
    let _ = enabled
    return false
    #else
    let names = stringArrayFromJSON(assetNamesJSON)
    let errors = collectFontManagerErrors { handler in
        CTFontManagerRegisterFontsWithAssetNames(names as CFArray, nil, fontManagerScope(scope), enabled, handler)
    }
    return errors.isEmpty
    #endif
}

@_cdecl("ct_font_manager_unregister_font_descriptors")
func ct_font_manager_unregister_font_descriptors(
    _ descriptorHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ descriptorCount: Int,
    _ scope: UInt32
) -> UnsafeMutablePointer<CChar>? {
    let descriptors: [CTFontDescriptor] = handlesToValues(descriptorHandles, count: descriptorCount)
    let errors = collectFontManagerErrors { handler in
        CTFontManagerUnregisterFontDescriptors(descriptors as CFArray, fontManagerScope(scope), handler)
    }
    return jsonCString(errors)
}

@_cdecl("ct_font_manager_unregister_font_urls")
func ct_font_manager_unregister_font_urls(
    _ urlPathsJSON: UnsafePointer<CChar>?,
    _ scope: UInt32
) -> UnsafeMutablePointer<CChar>? {
    let urls = urlArrayFromJSONPaths(urlPathsJSON)
    let errors = collectFontManagerErrors { handler in
        CTFontManagerUnregisterFontURLs(urls as CFArray, fontManagerScope(scope), handler)
    }
    return jsonCString(errors)
}

@_cdecl("ct_font_manager_unregister_fonts_for_urls")
func ct_font_manager_unregister_fonts_for_urls(
    _ urlPathsJSON: UnsafePointer<CChar>?,
    _ scope: UInt32,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
    let urls = urlArrayFromJSONPaths(urlPathsJSON)
    let errors = collectFontManagerErrors { handler in
        CTFontManagerUnregisterFontURLs(urls as CFArray, fontManagerScope(scope), handler)
    }
    errorOut?.pointee = jsonCString(errors)
    return errors.isEmpty
}
