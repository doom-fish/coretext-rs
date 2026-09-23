#![cfg(feature = "raw-ffi")]

use core::mem::{align_of, size_of};

use coretext::ffi;

#[test]
fn raw_ffi_gap_symbols_compile() {
    assert_eq!(ffi::kCTUnderlineStyleSingle, 0x01);
    assert_eq!(ffi::kCTUnderlinePatternDashDot, 0x0300);
    assert_eq!(ffi::kCTFontOptionsPreventAutoActivation, 1);
    assert_eq!(ffi::kCTFontDescriptorMatchingDidBegin, 0);
    assert_eq!(ffi::kCTFontManagerErrorAlreadyRegistered, 105);
    assert_eq!(ffi::kCTFrameProgressionRightToLeft, 1);
    assert!(!unsafe { ffi::kCTTabColumnTerminatorsAttributeName }.is_null());
    assert!(!unsafe { ffi::kCTBaselineClassRoman }.is_null());

    let _: unsafe extern "C" fn(ffi::CTFontRef, ffi::CFStringRef) -> ffi::CFTypeRef =
        ffi::CTFontCopyAttribute;
    let _: unsafe extern "C" fn(ffi::CTFontDescriptorRef) -> ffi::CFDictionaryRef =
        ffi::CTFontDescriptorCopyAttributes;
    let _: unsafe extern "C" fn() -> ffi::CFTypeID = ffi::CTFontCollectionGetTypeID;
    let _: unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void) -> ffi::CFComparisonResult =
        ffi::CTFontManagerCompareFontFamilyNames;
    let _: unsafe extern "C" fn(ffi::CTTextTabRef) -> ffi::CFDictionaryRef = ffi::CTTextTabGetOptions;

    assert_eq!(size_of::<ffi::AnchorPoint>(), 4);
    assert_eq!(size_of::<ffi::sfntDirectoryEntry>(), 16);
    assert!(size_of::<ffi::KernTableHeader>() > 0);
    assert!(size_of::<ffi::KerxSubtableHeader>() > 0);
    assert!(size_of::<ffi::CTRunDelegateCallbacks>() > 0);
    assert!(align_of::<ffi::sfntDirectory>() <= 2);
    assert!(align_of::<ffi::KernSubtableHeader>() <= 2);
}
