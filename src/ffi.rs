//! Raw C FFI declarations for CoreText, CoreFoundation, and CoreGraphics.
//!
//! All types here are `repr(C)` and match the ABI of the corresponding C headers.
//! Nothing in this module is safe to use directly.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs,
    clippy::upper_case_acronyms
)]

use core::ffi::{c_char, c_void};

pub use apple_cf::raw::{
    CFAllocatorRef, CFArrayRef, CFAttributedStringRef, CFCharacterSetRef, CFComparisonResult,
    CFDataRef, CFDictionaryRef, CFErrorRef, CFIndex, CFLocaleRef, CFNumberRef,
    CFOptionFlags, CFRange, CFSetRef, CFStringEncoding, CFStringRef, CFTypeID, CFTypeRef,
    CFURLRef, CGContextRef, CGFloat,
};

// ── CoreFoundation primitives ──────────────────────────────────────────────

/// NULL-based default allocator sentinel — identical to passing NULL.
pub const kCFAllocatorDefault: CFAllocatorRef = core::ptr::null();
pub type Boolean = u8;

pub const kCFStringEncodingUTF8: CFStringEncoding = 0x0800_0100;

// ── CoreGraphics geometry ──────────────────────────────────────────────────

pub type CGGlyph = u16;
pub type CGPathRef = *const c_void;

pub use apple_cf::cg::{CGAffineTransform, CGPoint, CGRect, CGSize};

// ── CoreText opaque references ─────────────────────────────────────────────

pub type CTFontRef = *const c_void;
pub type CTLineRef = *const c_void;
pub type CTRunRef = *const c_void;
pub type CTFrameRef = *const c_void;
pub type CTFramesetterRef = *const c_void;
pub type CTParagraphStyleRef = *const c_void;

// ── CTTextAlignment (CTTextAlignment.h / CTParagraphStyle.h) ──────────────

pub type CTTextAlignment = u8;
pub const kCTTextAlignmentLeft: CTTextAlignment = 0;
pub const kCTTextAlignmentRight: CTTextAlignment = 1;
pub const kCTTextAlignmentCenter: CTTextAlignment = 2;
pub const kCTTextAlignmentJustified: CTTextAlignment = 3;
pub const kCTTextAlignmentNatural: CTTextAlignment = 4;

// ── CTParagraphStyleSpecifier (CTParagraphStyle.h) ────────────────────────

pub type CTParagraphStyleSpecifier = u32;
pub const kCTParagraphStyleSpecifierAlignment: CTParagraphStyleSpecifier = 0;

/// `CTParagraphStyleSetting` — specifier/value pair passed to `CTParagraphStyleCreate`.
#[repr(C)]
pub struct CTParagraphStyleSetting {
    pub spec: CTParagraphStyleSpecifier,
    pub valueSize: usize,
    pub value: *const c_void,
}

// ── CTRunStatus (CTRun.h) ──────────────────────────────────────────────────

pub type CTRunStatus = u32;
pub const kCTRunStatusNoStatus: CTRunStatus = 0;
pub const kCTRunStatusRightToLeft: CTRunStatus = 1 << 0;
pub const kCTRunStatusNonMonotonic: CTRunStatus = 1 << 1;
pub const kCTRunStatusHasNonIdentityMatrix: CTRunStatus = 1 << 2;

// ── CTLineBoundsOptions (CTLine.h) ────────────────────────────────────────

pub type CTLineBoundsOptions = u64;
pub const kCTLineBoundsExcludeTypographicLeading: CTLineBoundsOptions = 1 << 0;
pub const kCTLineBoundsExcludeTypographicShifts: CTLineBoundsOptions = 1 << 1;
pub const kCTLineBoundsUseHangingPunctuation: CTLineBoundsOptions = 1 << 2;
pub const kCTLineBoundsUseGlyphPathBounds: CTLineBoundsOptions = 1 << 3;
pub const kCTLineBoundsUseOpticalBounds: CTLineBoundsOptions = 1 << 4;
pub const kCTLineBoundsIncludeLanguageExtents: CTLineBoundsOptions = 1 << 5;

// ── CF global constants ────────────────────────────────────────────────────
// These are struct-valued globals; we only ever take their address.
extern "C" {
    pub static kCFTypeDictionaryKeyCallBacks: u8;
    pub static kCFTypeDictionaryValueCallBacks: u8;
}

// CFStringRef-valued globals (pointer-sized, read directly).
extern "C" {
    pub static kCTFontAttributeName: CFStringRef;
    pub static kCTParagraphStyleAttributeName: CFStringRef;
}

// ── CoreFoundation ────────────────────────────────────────────────────────

extern "C" {
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        cStr: *const c_char,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
    pub fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
    pub fn CFStringGetMaximumSizeForEncoding(
        length: CFIndex,
        encoding: CFStringEncoding,
    ) -> CFIndex;
    pub fn CFStringGetCString(
        theString: CFStringRef,
        buffer: *mut c_char,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
    pub fn CFDictionaryCreate(
        allocator: CFAllocatorRef,
        keys: *const CFTypeRef,
        values: *const CFTypeRef,
        numValues: CFIndex,
        keyCallBacks: *const u8,
        valueCallBacks: *const u8,
    ) -> CFDictionaryRef;
    pub fn CFAttributedStringCreate(
        alloc: CFAllocatorRef,
        str: CFStringRef,
        attributes: CFDictionaryRef,
    ) -> CFAttributedStringRef;
    pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    pub fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> CFTypeRef;
}

// ── CoreGraphics ──────────────────────────────────────────────────────────

extern "C" {
    pub fn CGPathCreateWithRect(rect: CGRect, transform: *const CGAffineTransform) -> CGPathRef;
    pub fn CGPathRelease(path: CGPathRef);
}

// ── CoreText ──────────────────────────────────────────────────────────────

extern "C" {
    // CTFont
    pub fn CTFontGetTypeID() -> CFTypeID;
    pub fn CTFontCreateWithName(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;
    pub fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    pub fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
    pub fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
    pub fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
    pub fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetGlyphCount(font: CTFontRef) -> CFIndex;

    // CTParagraphStyle
    pub fn CTParagraphStyleGetTypeID() -> CFTypeID;
    pub fn CTParagraphStyleCreate(
        settings: *const CTParagraphStyleSetting,
        settingCount: CFIndex,
    ) -> CTParagraphStyleRef;
    pub fn CTParagraphStyleGetValueForSpecifier(
        style: CTParagraphStyleRef,
        spec: CTParagraphStyleSpecifier,
        valueBufferSize: usize,
        valueBuffer: *mut c_void,
    ) -> Boolean;

    // CTLine
    pub fn CTLineGetTypeID() -> CFTypeID;
    pub fn CTLineCreateWithAttributedString(attrString: CFAttributedStringRef) -> CTLineRef;
    pub fn CTLineGetGlyphCount(line: CTLineRef) -> CFIndex;
    pub fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
    pub fn CTLineGetStringRange(line: CTLineRef) -> CFRange;
    pub fn CTLineGetPenOffsetForFlush(
        line: CTLineRef,
        flushFactor: CGFloat,
        flushWidth: f64,
    ) -> f64;
    pub fn CTLineGetTypographicBounds(
        line: CTLineRef,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> f64;
    pub fn CTLineGetBoundsWithOptions(line: CTLineRef, options: CTLineBoundsOptions) -> CGRect;
    pub fn CTLineGetTrailingWhitespaceWidth(line: CTLineRef) -> f64;

    // CTRun
    pub fn CTRunGetTypeID() -> CFTypeID;
    pub fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
    pub fn CTRunGetStatus(run: CTRunRef) -> CTRunStatus;
    pub fn CTRunGetGlyphsPtr(run: CTRunRef) -> *const CGGlyph;
    pub fn CTRunGetGlyphs(run: CTRunRef, range: CFRange, buffer: *mut CGGlyph);
    pub fn CTRunGetPositionsPtr(run: CTRunRef) -> *const CGPoint;
    pub fn CTRunGetPositions(run: CTRunRef, range: CFRange, buffer: *mut CGPoint);
    pub fn CTRunGetAdvancesPtr(run: CTRunRef) -> *const CGSize;
    pub fn CTRunGetAdvances(run: CTRunRef, range: CFRange, buffer: *mut CGSize);
    pub fn CTRunGetStringIndicesPtr(run: CTRunRef) -> *const CFIndex;
    pub fn CTRunGetStringIndices(run: CTRunRef, range: CFRange, buffer: *mut CFIndex);
    pub fn CTRunGetStringRange(run: CTRunRef) -> CFRange;
    pub fn CTRunGetTypographicBounds(
        run: CTRunRef,
        range: CFRange,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> f64;

    // CTFramesetter
    pub fn CTFramesetterGetTypeID() -> CFTypeID;
    pub fn CTFramesetterCreateWithAttributedString(
        attrString: CFAttributedStringRef,
    ) -> CTFramesetterRef;
    pub fn CTFramesetterCreateFrame(
        framesetter: CTFramesetterRef,
        stringRange: CFRange,
        path: CGPathRef,
        frameAttributes: CFDictionaryRef,
    ) -> CTFrameRef;
    pub fn CTFramesetterSuggestFrameSizeWithConstraints(
        framesetter: CTFramesetterRef,
        stringRange: CFRange,
        frameAttributes: CFDictionaryRef,
        constraints: CGSize,
        fitRange: *mut CFRange,
    ) -> CGSize;

    // CTFrame
    pub fn CTFrameGetTypeID() -> CFTypeID;
    pub fn CTFrameGetStringRange(frame: CTFrameRef) -> CFRange;
    pub fn CTFrameGetVisibleStringRange(frame: CTFrameRef) -> CFRange;
    pub fn CTFrameGetLines(frame: CTFrameRef) -> CFArrayRef;
    pub fn CTFrameGetLineOrigins(frame: CTFrameRef, range: CFRange, origins: *mut CGPoint);
}

include!("ffi_gap.rs");
