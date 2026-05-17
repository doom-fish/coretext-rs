import CoreFoundation
import CoreText
import Foundation

let kCTFontUIFontNone = CTFontUIFontType.none
let kCTFontUIFontUser = CTFontUIFontType.user
let kCTFontUIFontUserFixedPitch = CTFontUIFontType.userFixedPitch
let kCTFontUIFontSystem = CTFontUIFontType.system
let kCTFontUIFontEmphasizedSystem = CTFontUIFontType.emphasizedSystem
let kCTFontUIFontSmallSystem = CTFontUIFontType.smallSystem
let kCTFontUIFontSmallEmphasizedSystem = CTFontUIFontType.smallEmphasizedSystem
let kCTFontUIFontMiniSystem = CTFontUIFontType.miniSystem
let kCTFontUIFontMiniEmphasizedSystem = CTFontUIFontType.miniEmphasizedSystem
let kCTFontUIFontViews = CTFontUIFontType.views
let kCTFontUIFontApplication = CTFontUIFontType.application
let kCTFontUIFontLabel = CTFontUIFontType.label
let kCTFontUIFontMenuTitle = CTFontUIFontType.menuTitle
let kCTFontUIFontMenuItem = CTFontUIFontType.menuItem
let kCTFontUIFontMenuItemMark = CTFontUIFontType.menuItemMark
let kCTFontUIFontMenuItemCmdKey = CTFontUIFontType.menuItemCmdKey
let kCTFontUIFontWindowTitle = CTFontUIFontType.windowTitle
let kCTFontUIFontPushButton = CTFontUIFontType.pushButton
let kCTFontUIFontUtilityWindowTitle = CTFontUIFontType.utilityWindowTitle
let kCTFontUIFontAlertHeader = CTFontUIFontType.alertHeader
let kCTFontUIFontSystemDetail = CTFontUIFontType.systemDetail
let kCTFontUIFontEmphasizedSystemDetail = CTFontUIFontType.emphasizedSystemDetail
let kCTFontUIFontToolbar = CTFontUIFontType.toolbar
let kCTFontUIFontSmallToolbar = CTFontUIFontType.smallToolbar
let kCTFontUIFontMessage = CTFontUIFontType.message
let kCTFontUIFontPalette = CTFontUIFontType.palette
let kCTFontUIFontToolTip = CTFontUIFontType.toolTip
let kCTFontUIFontControlContent = CTFontUIFontType.controlContent

let kCTFontOrientationDefault = CTFontOrientation.default
let kCTFontOrientationHorizontal = CTFontOrientation.horizontal
let kCTFontOrientationVertical = CTFontOrientation.vertical

let kCTFontManagerScopeNone = CTFontManagerScope.none
let kCTFontManagerScopeProcess = CTFontManagerScope.process
let kCTFontManagerScopePersistent = CTFontManagerScope.persistent
let kCTFontManagerScopeSession = CTFontManagerScope.session

let kCTCharacterCollectionIdentityMapping = CTCharacterCollection.identityMapping
let kCTCharacterCollectionAdobeCNS1 = CTCharacterCollection.adobeCNS1
let kCTCharacterCollectionAdobeGB1 = CTCharacterCollection.adobeGB1
let kCTCharacterCollectionAdobeJapan1 = CTCharacterCollection.adobeJapan1
let kCTCharacterCollectionAdobeJapan2 = CTCharacterCollection.adobeJapan2
let kCTCharacterCollectionAdobeKorea1 = CTCharacterCollection.adobeKorea1

let kCTLineTruncationStart = CTLineTruncationType.start
let kCTLineTruncationEnd = CTLineTruncationType.end
let kCTLineTruncationMiddle = CTLineTruncationType.middle

let kCTTextAlignmentLeft = CTTextAlignment.left
let kCTTextAlignmentRight = CTTextAlignment.right
let kCTTextAlignmentCenter = CTTextAlignment.center
let kCTTextAlignmentJustified = CTTextAlignment.justified
let kCTTextAlignmentNatural = CTTextAlignment.natural

let kCTLineBreakByWordWrapping = CTLineBreakMode.byWordWrapping
let kCTLineBreakByCharWrapping = CTLineBreakMode.byCharWrapping
let kCTLineBreakByClipping = CTLineBreakMode.byClipping
let kCTLineBreakByTruncatingHead = CTLineBreakMode.byTruncatingHead
let kCTLineBreakByTruncatingTail = CTLineBreakMode.byTruncatingTail
let kCTLineBreakByTruncatingMiddle = CTLineBreakMode.byTruncatingMiddle

let kCTWritingDirectionNatural = CTWritingDirection.natural
let kCTWritingDirectionLeftToRight = CTWritingDirection.leftToRight
let kCTWritingDirectionRightToLeft = CTWritingDirection.rightToLeft

let kCTParagraphStyleSpecifierAlignment = CTParagraphStyleSpecifier.alignment
let kCTParagraphStyleSpecifierFirstLineHeadIndent = CTParagraphStyleSpecifier.firstLineHeadIndent
let kCTParagraphStyleSpecifierHeadIndent = CTParagraphStyleSpecifier.headIndent
let kCTParagraphStyleSpecifierTailIndent = CTParagraphStyleSpecifier.tailIndent
let kCTParagraphStyleSpecifierTabStops = CTParagraphStyleSpecifier.tabStops
let kCTParagraphStyleSpecifierDefaultTabInterval = CTParagraphStyleSpecifier.defaultTabInterval
let kCTParagraphStyleSpecifierLineBreakMode = CTParagraphStyleSpecifier.lineBreakMode
let kCTParagraphStyleSpecifierLineHeightMultiple = CTParagraphStyleSpecifier.lineHeightMultiple
let kCTParagraphStyleSpecifierMaximumLineHeight = CTParagraphStyleSpecifier.maximumLineHeight
let kCTParagraphStyleSpecifierMinimumLineHeight = CTParagraphStyleSpecifier.minimumLineHeight
let kCTParagraphStyleSpecifierParagraphSpacing = CTParagraphStyleSpecifier.paragraphSpacing
let kCTParagraphStyleSpecifierParagraphSpacingBefore = CTParagraphStyleSpecifier.paragraphSpacingBefore
let kCTParagraphStyleSpecifierBaseWritingDirection = CTParagraphStyleSpecifier.baseWritingDirection
let kCTParagraphStyleSpecifierMaximumLineSpacing = CTParagraphStyleSpecifier.maximumLineSpacing
let kCTParagraphStyleSpecifierMinimumLineSpacing = CTParagraphStyleSpecifier.minimumLineSpacing
let kCTParagraphStyleSpecifierLineSpacingAdjustment = CTParagraphStyleSpecifier.lineSpacingAdjustment
let kCTParagraphStyleSpecifierLineBoundsOptions = CTParagraphStyleSpecifier.lineBoundsOptions

let kCTRubyAlignmentAuto = CTRubyAlignment.auto
let kCTRubyAlignmentStart = CTRubyAlignment.start
let kCTRubyAlignmentCenter = CTRubyAlignment.center
let kCTRubyAlignmentEnd = CTRubyAlignment.end
let kCTRubyAlignmentDistributeLetter = CTRubyAlignment.distributeLetter
let kCTRubyAlignmentDistributeSpace = CTRubyAlignment.distributeSpace
let kCTRubyAlignmentLineEdge = CTRubyAlignment.lineEdge

let kCTRubyOverhangAuto = CTRubyOverhang.auto
let kCTRubyOverhangStart = CTRubyOverhang.start
let kCTRubyOverhangEnd = CTRubyOverhang.end
let kCTRubyOverhangNone = CTRubyOverhang.none

let kCTRubyPositionBefore = CTRubyPosition.before
let kCTRubyPositionAfter = CTRubyPosition.after
let kCTRubyPositionInterCharacter = CTRubyPosition.interCharacter
let kCTRubyPositionInline = CTRubyPosition.inline

final class Box<T> {
    let value: T

    init(_ value: T) {
        self.value = value
    }
}

@inline(__always)
func retainBox<T>(_ value: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(Box(value)).toOpaque()
}

@inline(__always)
func unbox<T>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    let typed = ptr.assumingMemoryBound(to: Box<T>.self)
    return Unmanaged<Box<T>>.fromOpaque(UnsafeRawPointer(typed)).takeUnretainedValue().value
}

@inline(__always)
func duplicateCString(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else {
        return nil
    }
    return strdup(value)
}

@inline(__always)
func stringFromCString(_ value: UnsafePointer<CChar>?) -> String? {
    guard let value else {
        return nil
    }
    return String(cString: value)
}

@inline(__always)
func decodeJSON<T: Decodable>(_ value: UnsafePointer<CChar>?, as type: T.Type) -> T? {
    guard let string = stringFromCString(value) else {
        return nil
    }
    return try? JSONDecoder().decode(type, from: Data(string.utf8))
}

@inline(__always)
func jsonCString<T: Encodable>(_ value: T) -> UnsafeMutablePointer<CChar>? {
    guard let data = try? JSONEncoder().encode(value),
          let string = String(data: data, encoding: .utf8)
    else {
        return nil
    }
    return duplicateCString(string)
}

@inline(__always)
func typedArray<T>(_ value: CFArray?) -> [T] {
    guard let value else {
        return []
    }
    return (value as NSArray).compactMap { $0 as? T }
}

@inline(__always)
func fillBoxedArray<T>(
    _ values: [T],
    buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    capacity: Int
) -> Int {
    guard let buffer else {
        return values.count
    }

    let count = min(values.count, capacity)
    guard count > 0 else {
        return 0
    }

    for index in 0..<count {
        buffer[index] = retainBox(values[index])
    }
    return count
}

@inline(__always)
func handlesToValues<T>(
    _ handles: UnsafePointer<UnsafeMutableRawPointer?>?,
    count: Int,
    as _: T.Type = T.self
) -> [T] {
    guard let handles, count > 0 else {
        return []
    }

    return (0..<count).compactMap { index in
        guard let handle = handles[index] else {
            return nil
        }
        return unbox(handle, as: T.self)
    }
}

@inline(__always)
func jsonObjectFromCString(_ value: UnsafePointer<CChar>?) -> Any? {
    guard let string = stringFromCString(value) else {
        return nil
    }
    return try? JSONSerialization.jsonObject(with: Data(string.utf8))
}

private let fontAttributeAliases: [String: CFString] = [
    "name": kCTFontNameAttribute,
    "kCTFontNameAttribute": kCTFontNameAttribute,
    kCTFontNameAttribute as String: kCTFontNameAttribute,
    "displayName": kCTFontDisplayNameAttribute,
    "kCTFontDisplayNameAttribute": kCTFontDisplayNameAttribute,
    kCTFontDisplayNameAttribute as String: kCTFontDisplayNameAttribute,
    "familyName": kCTFontFamilyNameAttribute,
    "kCTFontFamilyNameAttribute": kCTFontFamilyNameAttribute,
    kCTFontFamilyNameAttribute as String: kCTFontFamilyNameAttribute,
    "styleName": kCTFontStyleNameAttribute,
    "kCTFontStyleNameAttribute": kCTFontStyleNameAttribute,
    kCTFontStyleNameAttribute as String: kCTFontStyleNameAttribute,
    "traits": kCTFontTraitsAttribute,
    "kCTFontTraitsAttribute": kCTFontTraitsAttribute,
    kCTFontTraitsAttribute as String: kCTFontTraitsAttribute,
    "variation": kCTFontVariationAttribute,
    "kCTFontVariationAttribute": kCTFontVariationAttribute,
    kCTFontVariationAttribute as String: kCTFontVariationAttribute,
    "variationAxes": kCTFontVariationAxesAttribute,
    "kCTFontVariationAxesAttribute": kCTFontVariationAxesAttribute,
    kCTFontVariationAxesAttribute as String: kCTFontVariationAxesAttribute,
    "size": kCTFontSizeAttribute,
    "kCTFontSizeAttribute": kCTFontSizeAttribute,
    kCTFontSizeAttribute as String: kCTFontSizeAttribute,
    "matrix": kCTFontMatrixAttribute,
    "kCTFontMatrixAttribute": kCTFontMatrixAttribute,
    kCTFontMatrixAttribute as String: kCTFontMatrixAttribute,
    "cascadeList": kCTFontCascadeListAttribute,
    "kCTFontCascadeListAttribute": kCTFontCascadeListAttribute,
    kCTFontCascadeListAttribute as String: kCTFontCascadeListAttribute,
    "characterSet": kCTFontCharacterSetAttribute,
    "kCTFontCharacterSetAttribute": kCTFontCharacterSetAttribute,
    kCTFontCharacterSetAttribute as String: kCTFontCharacterSetAttribute,
    "languages": kCTFontLanguagesAttribute,
    "kCTFontLanguagesAttribute": kCTFontLanguagesAttribute,
    kCTFontLanguagesAttribute as String: kCTFontLanguagesAttribute,
    "baselineAdjust": kCTFontBaselineAdjustAttribute,
    "kCTFontBaselineAdjustAttribute": kCTFontBaselineAdjustAttribute,
    kCTFontBaselineAdjustAttribute as String: kCTFontBaselineAdjustAttribute,
    "macintoshEncodings": kCTFontMacintoshEncodingsAttribute,
    "kCTFontMacintoshEncodingsAttribute": kCTFontMacintoshEncodingsAttribute,
    kCTFontMacintoshEncodingsAttribute as String: kCTFontMacintoshEncodingsAttribute,
    "features": kCTFontFeaturesAttribute,
    "kCTFontFeaturesAttribute": kCTFontFeaturesAttribute,
    kCTFontFeaturesAttribute as String: kCTFontFeaturesAttribute,
    "featureSettings": kCTFontFeatureSettingsAttribute,
    "kCTFontFeatureSettingsAttribute": kCTFontFeatureSettingsAttribute,
    kCTFontFeatureSettingsAttribute as String: kCTFontFeatureSettingsAttribute,
    "fixedAdvance": kCTFontFixedAdvanceAttribute,
    "kCTFontFixedAdvanceAttribute": kCTFontFixedAdvanceAttribute,
    kCTFontFixedAdvanceAttribute as String: kCTFontFixedAdvanceAttribute,
    "orientation": kCTFontOrientationAttribute,
    "kCTFontOrientationAttribute": kCTFontOrientationAttribute,
    kCTFontOrientationAttribute as String: kCTFontOrientationAttribute,
    "format": kCTFontFormatAttribute,
    "kCTFontFormatAttribute": kCTFontFormatAttribute,
    kCTFontFormatAttribute as String: kCTFontFormatAttribute,
    "registrationScope": kCTFontRegistrationScopeAttribute,
    "kCTFontRegistrationScopeAttribute": kCTFontRegistrationScopeAttribute,
    kCTFontRegistrationScopeAttribute as String: kCTFontRegistrationScopeAttribute,
    "priority": kCTFontPriorityAttribute,
    "kCTFontPriorityAttribute": kCTFontPriorityAttribute,
    kCTFontPriorityAttribute as String: kCTFontPriorityAttribute,
    "enabled": kCTFontEnabledAttribute,
    "kCTFontEnabledAttribute": kCTFontEnabledAttribute,
    kCTFontEnabledAttribute as String: kCTFontEnabledAttribute,
    "downloadable": kCTFontDownloadableAttribute,
    "kCTFontDownloadableAttribute": kCTFontDownloadableAttribute,
    kCTFontDownloadableAttribute as String: kCTFontDownloadableAttribute,
    "downloaded": kCTFontDownloadedAttribute,
    "kCTFontDownloadedAttribute": kCTFontDownloadedAttribute,
    kCTFontDownloadedAttribute as String: kCTFontDownloadedAttribute,
    "opticalSize": kCTFontOpticalSizeAttribute,
    "kCTFontOpticalSizeAttribute": kCTFontOpticalSizeAttribute,
    kCTFontOpticalSizeAttribute as String: kCTFontOpticalSizeAttribute,
    "url": kCTFontURLAttribute,
    "urlPath": kCTFontURLAttribute,
    "kCTFontURLAttribute": kCTFontURLAttribute,
    kCTFontURLAttribute as String: kCTFontURLAttribute,
]

func fontAttributeName(_ name: String) -> CFString {
    fontAttributeAliases[name] ?? (name as CFString)
}

private let stringAttributeAliases: [String: CFString] = [
    "font": kCTFontAttributeName,
    "kCTFontAttributeName": kCTFontAttributeName,
    kCTFontAttributeName as String: kCTFontAttributeName,
    "sizeFactor": kCTRubyAnnotationSizeFactorAttributeName,
    "kCTRubyAnnotationSizeFactorAttributeName": kCTRubyAnnotationSizeFactorAttributeName,
    kCTRubyAnnotationSizeFactorAttributeName as String: kCTRubyAnnotationSizeFactorAttributeName,
    "scaleToFit": kCTRubyAnnotationScaleToFitAttributeName,
    "kCTRubyAnnotationScaleToFitAttributeName": kCTRubyAnnotationScaleToFitAttributeName,
    kCTRubyAnnotationScaleToFitAttributeName as String: kCTRubyAnnotationScaleToFitAttributeName,
]

func stringAttributeName(_ name: String) -> CFString {
    stringAttributeAliases[name] ?? (name as CFString)
}

func cfFromJSON(
    _ value: Any,
    keyTransform: (String) -> CFString = { $0 as CFString }
) -> CFTypeRef? {
    switch value {
    case let string as String:
        return string as CFTypeRef
    case let number as NSNumber:
        return number as CFTypeRef
    case _ as NSNull:
        return NSNull()
    case let array as [Any]:
        return array.compactMap { cfFromJSON($0, keyTransform: keyTransform) } as CFArray
    case let dictionary as [String: Any]:
        var result: [CFString: Any] = [:]
        for (key, entry) in dictionary {
            if let converted = cfFromJSON(entry, keyTransform: keyTransform) {
                result[keyTransform(key)] = converted
            }
        }
        return result as CFDictionary
    default:
        return nil
    }
}

func dictionaryFromJSON(
    _ json: UnsafePointer<CChar>?,
    keyTransform: (String) -> CFString = { $0 as CFString }
) -> CFDictionary? {
    guard let object = jsonObjectFromCString(json) as? [String: Any],
          let converted = cfFromJSON(object, keyTransform: keyTransform)
    else {
        return nil
    }
    return unsafeBitCast(converted, to: CFDictionary.self)
}

func stringArrayFromJSON(_ json: UnsafePointer<CChar>?) -> [String] {
    decodeJSON(json, as: [String].self) ?? []
}

func urlArrayFromJSONPaths(_ json: UnsafePointer<CChar>?) -> [URL] {
    stringArrayFromJSON(json).map(URL.init(fileURLWithPath:))
}

func cfToJSON(_ value: CFTypeRef?) -> Any {
    guard let value else {
        return NSNull()
    }

    let typeID = CFGetTypeID(value)
    if typeID == CFNullGetTypeID() {
        return NSNull()
    }
    if typeID == CFBooleanGetTypeID() {
        return (value as! NSNumber).boolValue
    }
    if typeID == CFNumberGetTypeID() {
        return (value as! NSNumber).doubleValue
    }
    if typeID == CFStringGetTypeID() {
        return value as! String
    }
    if typeID == CFArrayGetTypeID() {
        return (value as! NSArray).map { cfToJSON($0 as CFTypeRef) }
    }
    if typeID == CFDictionaryGetTypeID() {
        var result: [String: Any] = [:]
        (value as! NSDictionary).forEach { key, entry in
            let mappedKey = (key as? String) ?? String(describing: key)
            result[mappedKey] = cfToJSON(entry as CFTypeRef)
        }
        return result
    }
    if typeID == CFSetGetTypeID() {
        return (value as! NSSet).allObjects.map { cfToJSON($0 as CFTypeRef) }
    }
    if typeID == CFURLGetTypeID() {
        let url = value as! URL
        return url.isFileURL ? url.path : url.absoluteString
    }
    if typeID == CFDataGetTypeID() {
        return (value as! Data).base64EncodedString()
    }
    if typeID == CTFontDescriptorGetTypeID() {
        return cfToJSON(CTFontDescriptorCopyAttributes(value as! CTFontDescriptor))
    }
    if typeID == CTFontGetTypeID() {
        return cfToJSON(CTFontDescriptorCopyAttributes(CTFontCopyFontDescriptor(value as! CTFont)))
    }
    if typeID == CTTextTabGetTypeID() {
        let tab = value as! CTTextTab
        return [
            "alignment": Int(CTTextTabGetAlignment(tab).rawValue),
            "location": CTTextTabGetLocation(tab),
            "options": cfToJSON(CTTextTabGetOptions(tab))
        ]
    }
    return NSNull()
}

func cfToJSONCString(_ value: CFTypeRef?) -> UnsafeMutablePointer<CChar>? {
    guard let data = try? JSONSerialization.data(withJSONObject: cfToJSON(value), options: [.fragmentsAllowed]),
          let string = String(data: data, encoding: .utf8)
    else {
        return nil
    }
    return duplicateCString(string)
}

@_cdecl("ct_retain")
func ct_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else {
        return nil
    }
    let typed = handle.assumingMemoryBound(to: UInt8.self)
    return Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(typed)).retain().toOpaque()
}

@_cdecl("ct_release")
func ct_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else {
        return
    }
    let typed = handle.assumingMemoryBound(to: UInt8.self)
    Unmanaged<AnyObject>.fromOpaque(UnsafeRawPointer(typed)).release()
}

@_cdecl("ct_string_release")
func ct_string_release(_ value: UnsafeMutablePointer<CChar>?) {
    free(value)
}
