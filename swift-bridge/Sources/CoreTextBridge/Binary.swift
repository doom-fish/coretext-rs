import CoreFoundation
import CoreText
import CoreTextABI
import Foundation

func stringFromBytes(_ value: CoreTextBytes) -> String? {
    guard let data = value.data else {
        return nil
    }
    return String(bytes: UnsafeBufferPointer(start: data, count: value.len), encoding: .utf8)
}

func stringArray(_ values: CoreTextStrings) -> [String] {
    guard let data = values.data else {
        return []
    }
    return UnsafeBufferPointer(start: data, count: values.len).compactMap(stringFromBytes)
}

func urlArray(_ values: CoreTextStrings) -> [URL] {
    stringArray(values).map(URL.init(fileURLWithPath:))
}

private func copiedBytes(_ bytes: UnsafeBufferPointer<UInt8>) -> CoreTextBytes {
    guard !bytes.isEmpty else {
        return CoreTextBytes()
    }
    let data = UnsafeMutablePointer<UInt8>.allocate(capacity: bytes.count)
    data.initialize(from: bytes.baseAddress!, count: bytes.count)
    return CoreTextBytes(data: UnsafePointer(data), len: bytes.count)
}

private func copiedString(_ string: String) -> CoreTextBytes {
    Array(string.utf8).withUnsafeBufferPointer(copiedBytes)
}

private func binaryValue(_ value: CFTypeRef?, depth: Int = 0) -> CoreTextValue {
    var result = CoreTextValue()
    guard let value, depth <= 64 else {
        return result
    }
    let type = CFGetTypeID(value)
    if type == CFBooleanGetTypeID() {
        result.tag = UInt32(CoreTextBool)
        result.unsigned_value = (value as! NSNumber).boolValue ? 1 : 0
    } else if type == CFNumberGetTypeID() {
        let number = value as! NSNumber
        let encoding = String(cString: number.objCType)
        if encoding == "f" || encoding == "d" {
            result.tag = UInt32(CoreTextFloat)
            result.float_value = number.doubleValue
        } else if ["Q", "L", "I", "S", "C"].contains(encoding) {
            result.tag = UInt32(CoreTextUnsigned)
            result.unsigned_value = number.uint64Value
        } else {
            result.tag = UInt32(CoreTextSigned)
            result.signed_value = number.int64Value
        }
    } else if type == CFStringGetTypeID() {
        result.tag = UInt32(CoreTextString)
        result.bytes = copiedString(value as! String)
    } else if type == CFDataGetTypeID() {
        result.tag = UInt32(CoreTextData)
        result.bytes = (value as! Data).withUnsafeBytes { bytes in
            copiedBytes(bytes.bindMemory(to: UInt8.self))
        }
    } else if type == CFArrayGetTypeID() || type == CFSetGetTypeID() {
        let values = type == CFArrayGetTypeID()
            ? (value as! NSArray).map { $0 }
            : (value as! NSSet).allObjects
        result.tag = UInt32(CoreTextArray)
        result.count = values.count
        if !values.isEmpty {
            let elements = UnsafeMutablePointer<CoreTextValue>.allocate(capacity: values.count)
            for (index, entry) in values.enumerated() {
                elements.advanced(by: index).initialize(to: binaryValue(entry as CFTypeRef, depth: depth + 1))
            }
            result.elements = elements
        }
    } else if type == CFDictionaryGetTypeID() {
        let dictionary = value as! NSDictionary
        result.tag = UInt32(CoreTextDictionary)
        result.count = dictionary.count
        if dictionary.count > 0 {
            let elements = UnsafeMutablePointer<CoreTextValue>.allocate(capacity: dictionary.count)
            let keys = UnsafeMutablePointer<CoreTextBytes>.allocate(capacity: dictionary.count)
            for (index, entry) in dictionary.enumerated() {
                let key = (entry.key as? String) ?? String(describing: entry.key)
                keys.advanced(by: index).initialize(to: copiedString(key))
                elements.advanced(by: index).initialize(to: binaryValue(entry.value as CFTypeRef, depth: depth + 1))
            }
            result.elements = elements
            result.keys = keys
        }
    } else if type == CFURLGetTypeID() {
        let url = value as! URL
        result.tag = UInt32(CoreTextString)
        result.bytes = copiedString(url.isFileURL ? url.path : url.absoluteString)
    } else if type == CTFontDescriptorGetTypeID() {
        return binaryValue(CTFontDescriptorCopyAttributes(value as! CTFontDescriptor), depth: depth + 1)
    } else if type == CTFontGetTypeID() {
        return binaryValue(CTFontDescriptorCopyAttributes(CTFontCopyFontDescriptor(value as! CTFont)), depth: depth + 1)
    } else if type == CTTextTabGetTypeID() {
        let tab = value as! CTTextTab
        let fields: [String: Any] = [
            "alignment": Int(CTTextTabGetAlignment(tab).rawValue),
            "location": CTTextTabGetLocation(tab),
            "options": CTTextTabGetOptions(tab) ?? NSNull()
        ]
        return binaryValue(fields as CFDictionary, depth: depth + 1)
    }
    return result
}

func binaryResult(_ value: Any?) -> UnsafeMutablePointer<CoreTextValue> {
    let result = UnsafeMutablePointer<CoreTextValue>.allocate(capacity: 1)
    result.initialize(to: binaryValue(value as CFTypeRef?))
    return result
}

private func releaseValue(_ value: CoreTextValue) {
    if let data = value.bytes.data {
        UnsafeMutablePointer(mutating: data).deallocate()
    }
    if let elements = value.elements {
        for index in 0..<value.count {
            releaseValue(elements[index])
        }
        elements.deinitialize(count: value.count)
        elements.deallocate()
    }
    if let keys = value.keys {
        for index in 0..<value.count {
            if let data = keys[index].data {
                UnsafeMutablePointer(mutating: data).deallocate()
            }
        }
        keys.deinitialize(count: value.count)
        keys.deallocate()
    }
}

@_cdecl("ct_value_release")
public func ct_value_release(_ value: UnsafeMutablePointer<CoreTextValue>?) {
    guard let value else {
        return
    }
    releaseValue(value.pointee)
    value.deinitialize(count: 1)
    value.deallocate()
}

private func nativeValue(
    _ value: CoreTextValue,
    depth: Int = 0,
    keyTransform: (String) -> CFString
) -> Any? {
    guard depth <= 64 else {
        return nil
    }
    switch value.tag {
    case UInt32(CoreTextNull):
        return NSNull()
    case UInt32(CoreTextBool):
        return NSNumber(value: value.unsigned_value != 0)
    case UInt32(CoreTextSigned):
        return NSNumber(value: value.signed_value)
    case UInt32(CoreTextUnsigned):
        return NSNumber(value: value.unsigned_value)
    case UInt32(CoreTextFloat):
        return NSNumber(value: value.float_value)
    case UInt32(CoreTextString):
        return stringFromBytes(value.bytes)
    case UInt32(CoreTextData):
        if value.bytes.len == 0 {
            return Data()
        }
        guard let data = value.bytes.data else {
            return nil
        }
        return Data(bytes: data, count: value.bytes.len)
    case UInt32(CoreTextArray), UInt32(CoreTextDictionary):
        if value.count == 0 {
            return value.tag == UInt32(CoreTextArray) ? [] as [Any] : [:] as [CFString: Any]
        }
        guard let elements = value.elements else {
            return nil
        }
        var values: [Any] = []
        for index in 0..<value.count {
            guard let entry = nativeValue(elements[index], depth: depth + 1, keyTransform: keyTransform) else {
                return nil
            }
            values.append(entry)
        }
        if value.tag == UInt32(CoreTextArray) {
            return values
        }
        guard let keys = value.keys else {
            return nil
        }
        var dictionary: [CFString: Any] = [:]
        for index in 0..<value.count {
            guard let key = stringFromBytes(keys[index]) else {
                return nil
            }
            dictionary[keyTransform(key)] = values[index]
        }
        return dictionary
    default:
        return nil
    }
}

func dictionaryFromValue(
    _ value: UnsafePointer<CoreTextValue>?,
    keyTransform: (String) -> CFString = { $0 as CFString }
) -> CFDictionary? {
    guard let value,
          let dictionary = nativeValue(value.pointee, keyTransform: keyTransform) as? [CFString: Any] else {
        return nil
    }
    return dictionary as CFDictionary
}
