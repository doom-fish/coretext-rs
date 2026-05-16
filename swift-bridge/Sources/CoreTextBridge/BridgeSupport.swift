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
