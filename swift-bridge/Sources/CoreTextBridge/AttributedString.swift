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
