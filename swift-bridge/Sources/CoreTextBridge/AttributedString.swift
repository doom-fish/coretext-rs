import CoreFoundation
import CoreText
import Foundation

@_cdecl("ct_attributed_string_create_with_runs")
public func ct_attributed_string_create_with_runs(
    _ texts: UnsafePointer<UnsafePointer<UInt8>?>?,
    _ lengths: UnsafePointer<Int>?,
    _ fonts: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    guard count >= 0 else { return nil }
    guard let attributed = CFAttributedStringCreateMutable(nil, 0) else { return nil }
    if count > 0 {
        guard let texts, let lengths, let fonts else { return nil }
        for index in 0..<count {
            guard let text = texts[index], lengths[index] >= 0, let fontPtr = fonts[index] else {
                return nil
            }
            let font: CTFont = unbox(fontPtr, as: CTFont.self)
            guard let substring = CFStringCreateWithBytes(
                nil,
                text,
                lengths[index],
                CFStringBuiltInEncodings.UTF8.rawValue,
                false
            ) else {
                return nil
            }
            let start = CFAttributedStringGetLength(attributed)
            CFAttributedStringReplaceString(
                attributed,
                CFRange(location: start, length: 0),
                substring
            )
            let length = CFAttributedStringGetLength(attributed) - start
            if length > 0 {
                CFAttributedStringSetAttribute(
                    attributed,
                    CFRange(location: start, length: length),
                    kCTFontAttributeName,
                    font
                )
            }
        }
    }
    return retainBox(attributed as NSAttributedString)
}

@_cdecl("ct_attributed_string_create")
public func ct_attributed_string_create(
    _ text: UnsafePointer<CChar>?,
    _ fontPtr: UnsafeMutableRawPointer?,
    _ paragraphStylePtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let text = stringFromCString(text), let fontPtr else {
        return nil
    }

    let font: CTFont = unbox(fontPtr, as: CTFont.self)
    var attributes: [NSAttributedString.Key: Any] = [
        NSAttributedString.Key(rawValue: kCTFontAttributeName as String): font
    ]
    if let paragraphStylePtr {
        let paragraphStyle: CTParagraphStyle = unbox(paragraphStylePtr, as: CTParagraphStyle.self)
        attributes[NSAttributedString.Key(rawValue: kCTParagraphStyleAttributeName as String)] = paragraphStyle
    }

    let attributed = NSAttributedString(string: text, attributes: attributes)
    return retainBox(attributed)
}

@_cdecl("ct_attributed_string_get_length")
public func ct_attributed_string_get_length(_ attributedStringPtr: UnsafeMutableRawPointer?) -> Int {
    guard let attributedStringPtr else { return 0 }
    let attributed: NSAttributedString = unbox(attributedStringPtr, as: NSAttributedString.self)
    return attributed.length
}
