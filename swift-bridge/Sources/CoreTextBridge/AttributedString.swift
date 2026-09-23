import CoreText
import Foundation

@_cdecl("ct_attributed_string_create")
func ct_attributed_string_create(
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
func ct_attributed_string_get_length(_ attributedStringPtr: UnsafeMutableRawPointer?) -> Int {
    guard let attributedStringPtr else { return 0 }
    let attributed: NSAttributedString = unbox(attributedStringPtr, as: NSAttributedString.self)
    return attributed.length
}
