#![cfg(feature = "raw-ffi")]

use core::mem::{align_of, size_of};

use coretext::ffi;

#[test]
fn raw_ffi_gap_symbols_compile() {
    let _ = ffi::kCTUnderlineStyleSingle;
    let _ = ffi::kCTUnderlinePatternDashDot;
    let _ = ffi::kCTFontOptionsPreventAutoActivation;
    let _ = ffi::kCTFontDescriptorMatchingDidBegin;
    let _ = ffi::kCTFontManagerErrorAlreadyRegistered;
    let _ = ffi::kCTFrameProgressionRightToLeft;
    let _ = unsafe { ffi::kCTTabColumnTerminatorsAttributeName };
    let _ = unsafe { ffi::kCTBaselineClassRoman };

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
