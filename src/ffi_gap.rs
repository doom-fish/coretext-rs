// Additional raw FFI declarations covering the remaining CoreText audit surface.

pub type CFOptionFlags = usize;
pub type CFComparisonResult = CFIndex;
pub type CFStringEncoding = u32;
pub type CGColorRef = *const c_void;
pub type CGContextRef = *const c_void;
pub type CGFontRef = *const c_void;
pub type CTFontCollectionRef = *const c_void;
pub type CTMutableFontCollectionRef = *const c_void;
pub type CTFontDescriptorRef = *const c_void;
pub type CTGlyphInfoRef = *const c_void;
pub type CTRubyAnnotationRef = *const c_void;
pub type CTTextTabRef = *const c_void;
pub type CTTypesetterRef = *const c_void;
pub type CTRunDelegateRef = *const c_void;
pub type UniChar = u16;
pub type FourCharCode = u32;
pub type Fixed = i32;

pub const kCFCompareLessThan: CFComparisonResult = -1;
pub const kCFCompareEqualTo: CFComparisonResult = 0;
pub const kCFCompareGreaterThan: CFComparisonResult = 1;

pub type CTFontOptions = CFOptionFlags;
pub const kCTFontOptionsDefault: CTFontOptions = 0;
pub const kCTFontOptionsPreventAutoActivation: CTFontOptions = 1 << 0;
pub const kCTFontOptionsPreventAutoDownload: CTFontOptions = 1 << 1;
pub const kCTFontOptionsPreferSystemFont: CTFontOptions = 1 << 2;

pub type CTFontTableTag = FourCharCode;
pub type CTFontTableOptions = u32;
pub const kCTFontTableOptionNoOptions: CTFontTableOptions = 0;
pub const kCTFontTableOptionExcludeSynthetic: CTFontTableOptions = 1 << 0;

pub type CTFontCollectionCopyOptions = u32;
pub const kCTFontCollectionCopyDefaultOptions: CTFontCollectionCopyOptions = 0;
pub const kCTFontCollectionCopyUnique: CTFontCollectionCopyOptions = 1 << 0;
pub const kCTFontCollectionCopyStandardSort: CTFontCollectionCopyOptions = 1 << 1;

pub type CTFontDescriptorMatchingState = u32;
pub const kCTFontDescriptorMatchingDidBegin: CTFontDescriptorMatchingState = 0;
pub const kCTFontDescriptorMatchingDidFinish: CTFontDescriptorMatchingState = 1;
pub const kCTFontDescriptorMatchingWillBeginQuerying: CTFontDescriptorMatchingState = 2;
pub const kCTFontDescriptorMatchingStalled: CTFontDescriptorMatchingState = 3;
pub const kCTFontDescriptorMatchingWillBeginDownloading: CTFontDescriptorMatchingState = 4;
pub const kCTFontDescriptorMatchingDownloading: CTFontDescriptorMatchingState = 5;
pub const kCTFontDescriptorMatchingDidFinishDownloading: CTFontDescriptorMatchingState = 6;
pub const kCTFontDescriptorMatchingDidMatch: CTFontDescriptorMatchingState = 7;
pub const kCTFontDescriptorMatchingDidFailWithError: CTFontDescriptorMatchingState = 8;

pub type CTFontManagerError = CFIndex;
pub const kCTFontManagerErrorFileNotFound: CTFontManagerError = 101;
pub const kCTFontManagerErrorInsufficientPermissions: CTFontManagerError = 102;
pub const kCTFontManagerErrorUnrecognizedFormat: CTFontManagerError = 103;
pub const kCTFontManagerErrorInvalidFontData: CTFontManagerError = 104;
pub const kCTFontManagerErrorAlreadyRegistered: CTFontManagerError = 105;
pub const kCTFontManagerErrorExceededResourceLimit: CTFontManagerError = 106;
pub const kCTFontManagerErrorAssetNotFound: CTFontManagerError = 107;
pub const kCTFontManagerErrorNotRegistered: CTFontManagerError = 201;
pub const kCTFontManagerErrorInUse: CTFontManagerError = 202;
pub const kCTFontManagerErrorSystemRequired: CTFontManagerError = 203;
pub const kCTFontManagerErrorRegistrationFailed: CTFontManagerError = 301;
pub const kCTFontManagerErrorMissingEntitlement: CTFontManagerError = 302;
pub const kCTFontManagerErrorInsufficientInfo: CTFontManagerError = 303;
pub const kCTFontManagerErrorCancelledByUser: CTFontManagerError = 304;
pub const kCTFontManagerErrorDuplicatedName: CTFontManagerError = 305;
pub const kCTFontManagerErrorInvalidFilePath: CTFontManagerError = 306;
pub const kCTFontManagerErrorUnsupportedScope: CTFontManagerError = 307;

pub type CTFrameProgression = u32;
pub const kCTFrameProgressionTopToBottom: CTFrameProgression = 0;
pub const kCTFrameProgressionRightToLeft: CTFrameProgression = 1;
pub const kCTFrameProgressionLeftToRight: CTFrameProgression = 2;

pub type CTFramePathFillRule = u32;
pub const kCTFramePathFillEvenOdd: CTFramePathFillRule = 0;
pub const kCTFramePathFillWindingNumber: CTFramePathFillRule = 1;

pub type CTUnderlineStyle = i32;
pub const kCTUnderlineStyleNone: CTUnderlineStyle = 0x00;
pub const kCTUnderlineStyleSingle: CTUnderlineStyle = 0x01;
pub const kCTUnderlineStyleThick: CTUnderlineStyle = 0x02;
pub const kCTUnderlineStyleDouble: CTUnderlineStyle = 0x09;

pub type CTUnderlineStyleModifiers = i32;
pub const kCTUnderlinePatternSolid: CTUnderlineStyleModifiers = 0x0000;
pub const kCTUnderlinePatternDot: CTUnderlineStyleModifiers = 0x0100;
pub const kCTUnderlinePatternDash: CTUnderlineStyleModifiers = 0x0200;
pub const kCTUnderlinePatternDashDot: CTUnderlineStyleModifiers = 0x0300;
pub const kCTUnderlinePatternDashDotDot: CTUnderlineStyleModifiers = 0x0400;

pub type CTRubyPosition = u8;
pub const kCTRubyPositionCount: CTRubyPosition = 4;

pub type CTFontCollectionSortDescriptorsCallback = Option<
    unsafe extern "C" fn(
        first: CTFontDescriptorRef,
        second: CTFontDescriptorRef,
        refCon: *mut c_void,
    ) -> CFComparisonResult,
>;
pub type CTFontDescriptorProgressHandler = *const c_void;
pub type CTFontManagerRegistrationHandler = *const c_void;
pub type CTLineCaretEnumerationBlock = *const c_void;

pub type CTRunDelegateDeallocateCallback = Option<unsafe extern "C" fn(refCon: *mut c_void)>;
pub type CTRunDelegateGetAscentCallback = Option<unsafe extern "C" fn(refCon: *mut c_void) -> CGFloat>;
pub type CTRunDelegateGetDescentCallback = Option<unsafe extern "C" fn(refCon: *mut c_void) -> CGFloat>;
pub type CTRunDelegateGetWidthCallback = Option<unsafe extern "C" fn(refCon: *mut c_void) -> CGFloat>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CTRunDelegateCallbacks {
    pub version: CFIndex,
    pub dealloc: CTRunDelegateDeallocateCallback,
    pub getAscent: CTRunDelegateGetAscentCallback,
    pub getDescent: CTRunDelegateGetDescentCallback,
    pub getWidth: CTRunDelegateGetWidthCallback,
}

pub const kCTRunDelegateVersion1: CFIndex = 1;
pub const kCTRunDelegateCurrentVersion: CFIndex = kCTRunDelegateVersion1;

pub type SFNTLookupTableFormat = u16;
pub type SFNTLookupValue = u16;
pub type STClass = u8;
pub type STXStateIndex = u16;
pub type JustPCActionType = u16;
pub type JustificationFlags = u16;
pub type OpbdTableFormat = u16;
pub type PropCharProperties = u16;
pub type KernSubtableInfo = u16;
pub type KernKerningValue = i16;
pub type KernArrayOffset = u16;
pub type KerxSubtableCoverage = u32;
pub type KerxArrayOffset = u32;
pub type BslnTableFormat = u16;
pub type MortSubtableMaskFlags = u32;

// SFNTTypes.h / SFNTLayoutTypes.h structs and unions.
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntDirectoryEntry {
    pub tableTag: u32,
    pub checkSum: u32,
    pub offset: u32,
    pub length: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntDirectory {
    pub format: u32,
    pub numOffsets: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub table: [sfntDirectoryEntry; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntCMapSubHeader {
    pub format: u16,
    pub length: u16,
    pub languageID: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntCMapExtendedSubHeader {
    pub format: u16,
    pub reserved: u16,
    pub length: u32,
    pub language: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntCMapEncoding {
    pub platformID: u16,
    pub scriptID: u16,
    pub offset: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntCMapHeader {
    pub version: u16,
    pub numTables: u16,
    pub encoding: [sfntCMapEncoding; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntNameRecord {
    pub platformID: u16,
    pub scriptID: u16,
    pub languageID: u16,
    pub nameID: u16,
    pub length: u16,
    pub offset: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntNameHeader {
    pub format: u16,
    pub count: u16,
    pub stringOffset: u16,
    pub rec: [sfntNameRecord; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntVariationAxis {
    pub axisTag: u32,
    pub minValue: i32,
    pub defaultValue: i32,
    pub maxValue: i32,
    pub flags: i16,
    pub nameID: i16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntInstance {
    pub nameID: i16,
    pub flags: i16,
    pub coord: [i32; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntVariationHeader {
    pub version: i32,
    pub offsetToData: u16,
    pub countSizePairs: u16,
    pub axisCount: u16,
    pub axisSize: u16,
    pub instanceCount: u16,
    pub instanceSize: u16,
    pub axis: [sfntVariationAxis; 1],
    pub instance: [sfntInstance; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntFontDescriptor {
    pub name: u32,
    pub value: i32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntDescriptorHeader {
    pub version: i32,
    pub descriptorCount: i32,
    pub descriptor: [sfntFontDescriptor; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntFeatureName {
    pub featureType: u16,
    pub settingCount: u16,
    pub offsetToSettings: i32,
    pub featureFlags: u16,
    pub nameID: i16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntFontFeatureSetting {
    pub setting: u16,
    pub nameID: i16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntFontRunFeature {
    pub featureType: u16,
    pub setting: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct sfntFeatureHeader {
    pub version: i32,
    pub featureNameCount: u16,
    pub featureSetCount: u16,
    pub reserved: i32,
    pub names: [sfntFeatureName; 1],
    pub settings: [sfntFontFeatureSetting; 1],
    pub runs: [sfntFontRunFeature; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct FontVariation {
    pub name: u32,
    pub value: i32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupBinarySearchHeader {
    pub unitSize: u16,
    pub nUnits: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupArrayHeader {
    pub lookupValues: [SFNTLookupValue; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupTrimmedArrayHeader {
    pub firstGlyph: u16,
    pub count: u16,
    pub valueArray: [SFNTLookupValue; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupVectorHeader {
    pub valueSize: u16,
    pub firstGlyph: u16,
    pub count: u16,
    pub values: [u8; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupSegment {
    pub lastGlyph: u16,
    pub firstGlyph: u16,
    pub value: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupSegmentHeader {
    pub binSearch: SFNTLookupBinarySearchHeader,
    pub segments: [SFNTLookupSegment; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupSingle {
    pub glyph: u16,
    pub value: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupSingleHeader {
    pub binSearch: SFNTLookupBinarySearchHeader,
    pub entries: [SFNTLookupSingle; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union SFNTLookupFormatSpecificHeader {
    pub theArray: SFNTLookupArrayHeader,
    pub segment: SFNTLookupSegmentHeader,
    pub single: SFNTLookupSingleHeader,
    pub trimmedArray: SFNTLookupTrimmedArrayHeader,
    pub vector: SFNTLookupVectorHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct SFNTLookupTable {
    pub format: SFNTLookupTableFormat,
    pub fsHeader: SFNTLookupFormatSpecificHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STHeader {
    pub filler: u8,
    pub nClasses: STClass,
    pub classTableOffset: u16,
    pub stateArrayOffset: u16,
    pub entryTableOffset: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STClassTable {
    pub firstGlyph: u16,
    pub nGlyphs: u16,
    pub classes: [STClass; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STEntryZero {
    pub newState: u16,
    pub flags: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STEntryOne {
    pub newState: u16,
    pub flags: u16,
    pub offset1: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STEntryTwo {
    pub newState: u16,
    pub flags: u16,
    pub offset1: u16,
    pub offset2: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STXHeader {
    pub nClasses: u32,
    pub classTableOffset: u32,
    pub stateArrayOffset: u32,
    pub entryTableOffset: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STXEntryZero {
    pub newState: STXStateIndex,
    pub flags: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STXEntryOne {
    pub newState: STXStateIndex,
    pub flags: u16,
    pub index1: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct STXEntryTwo {
    pub newState: STXStateIndex,
    pub flags: u16,
    pub index1: u16,
    pub index2: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct LcarCaretClassEntry {
    pub count: u16,
    pub partials: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct LcarCaretTable {
    pub version: i32,
    pub format: u16,
    pub lookup: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustPCDecompositionAction {
    pub lowerLimit: i32,
    pub upperLimit: i32,
    pub order: u16,
    pub count: u16,
    pub glyphs: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustPCConditionalAddAction {
    pub substThreshold: i32,
    pub addGlyph: u16,
    pub substGlyph: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustPCDuctilityAction {
    pub ductilityAxis: u32,
    pub minimumLimit: i32,
    pub noStretchValue: i32,
    pub maximumLimit: i32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustPCGlyphRepeatAddAction {
    pub flags: u16,
    pub glyph: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustPCActionSubrecord {
    pub theClass: u16,
    pub theType: JustPCActionType,
    pub length: u32,
    pub data: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustPCAction {
    pub actionCount: u32,
    pub actions: [JustPCActionSubrecord; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustWidthDeltaEntry {
    pub justClass: u32,
    pub beforeGrowLimit: i32,
    pub beforeShrinkLimit: i32,
    pub afterGrowLimit: i32,
    pub afterShrinkLimit: i32,
    pub growFlags: JustificationFlags,
    pub shrinkFlags: JustificationFlags,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustWidthDeltaGroup {
    pub count: u32,
    pub entries: [JustWidthDeltaEntry; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustPostcompTable {
    pub lookupTable: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustDirectionTable {
    pub justClass: u16,
    pub widthDeltaClusters: u16,
    pub postcomp: u16,
    pub lookup: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct JustTable {
    pub version: i32,
    pub format: u16,
    pub horizHeaderOffset: u16,
    pub vertHeaderOffset: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct OpbdSideValues {
    pub leftSideShift: i16,
    pub topSideShift: i16,
    pub rightSideShift: i16,
    pub bottomSideShift: i16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct OpbdTable {
    pub version: i32,
    pub format: OpbdTableFormat,
    pub lookupTable: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortRearrangementSubtable {
    pub header: STHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortContextualSubtable {
    pub header: STHeader,
    pub substitutionTableOffset: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortLigatureSubtable {
    pub header: STHeader,
    pub ligatureActionTableOffset: u16,
    pub componentTableOffset: u16,
    pub ligatureTableOffset: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortSwashSubtable {
    pub lookup: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortInsertionSubtable {
    pub header: STHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union MortSpecificSubtable {
    pub rearrangement: MortRearrangementSubtable,
    pub contextual: MortContextualSubtable,
    pub ligature: MortLigatureSubtable,
    pub swash: MortSwashSubtable,
    pub insertion: MortInsertionSubtable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortSubtable {
    pub length: u16,
    pub coverage: u16,
    pub flags: MortSubtableMaskFlags,
    pub u: MortSpecificSubtable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortFeatureEntry {
    pub featureType: u16,
    pub featureSelector: u16,
    pub enableFlags: MortSubtableMaskFlags,
    pub disableFlags: MortSubtableMaskFlags,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortChain {
    pub defaultFlags: MortSubtableMaskFlags,
    pub length: u32,
    pub nFeatures: u16,
    pub nSubtables: u16,
    pub featureEntries: [MortFeatureEntry; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MortTable {
    pub version: i32,
    pub nChains: u32,
    pub chains: [MortChain; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MorxRearrangementSubtable {
    pub header: STXHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MorxContextualSubtable {
    pub header: STXHeader,
    pub substitutionTableOffset: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MorxLigatureSubtable {
    pub header: STXHeader,
    pub ligatureActionTableOffset: u32,
    pub componentTableOffset: u32,
    pub ligatureTableOffset: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MorxInsertionSubtable {
    pub header: STXHeader,
    pub insertionGlyphTableOffset: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union MorxSpecificSubtable {
    pub rearrangement: MorxRearrangementSubtable,
    pub contextual: MorxContextualSubtable,
    pub ligature: MorxLigatureSubtable,
    pub swash: MortSwashSubtable,
    pub insertion: MorxInsertionSubtable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MorxSubtable {
    pub length: u32,
    pub coverage: u32,
    pub flags: MortSubtableMaskFlags,
    pub u: MorxSpecificSubtable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MorxChain {
    pub defaultFlags: MortSubtableMaskFlags,
    pub length: u32,
    pub nFeatures: u32,
    pub nSubtables: u32,
    pub featureEntries: [MortFeatureEntry; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct MorxTable {
    pub version: i32,
    pub nChains: u32,
    pub chains: [MorxChain; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct PropTable {
    pub version: i32,
    pub format: u16,
    pub defaultProps: PropCharProperties,
    pub lookup: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct PropLookupSegment {
    pub lastGlyph: u16,
    pub firstGlyph: u16,
    pub value: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct PropLookupSingle {
    pub glyph: u16,
    pub props: PropCharProperties,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct TrakTableEntry {
    pub track: i32,
    pub nameTableIndex: u16,
    pub sizesOffset: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct TrakTableData {
    pub nTracks: u16,
    pub nSizes: u16,
    pub sizeTableOffset: u32,
    pub trakTable: [TrakTableEntry; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct TrakTable {
    pub version: i32,
    pub format: u16,
    pub horizOffset: u16,
    pub vertOffset: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernVersion0Header {
    pub version: u16,
    pub nTables: u16,
    pub firstSubtable: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernTableHeader {
    pub version: i32,
    pub nTables: i32,
    pub firstSubtable: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernKerningPair {
    pub left: u16,
    pub right: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernOrderedListEntry {
    pub pair: KernKerningPair,
    pub value: KernKerningValue,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernOrderedListHeader {
    pub nPairs: u16,
    pub searchRange: u16,
    pub entrySelector: u16,
    pub rangeShift: u16,
    pub table: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernStateHeader {
    pub header: STHeader,
    pub valueTable: u16,
    pub firstTable: [u8; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernStateEntry {
    pub newState: u16,
    pub flags: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernOffsetTable {
    pub firstGlyph: u16,
    pub nGlyphs: u16,
    pub offsetTable: [KernArrayOffset; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernSimpleArrayHeader {
    pub rowWidth: u16,
    pub leftOffsetTable: u16,
    pub rightOffsetTable: u16,
    pub theArray: KernArrayOffset,
    pub firstTable: [u16; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernIndexArrayHeader {
    pub glyphCount: u16,
    pub kernValueCount: u8,
    pub leftClassCount: u8,
    pub rightClassCount: u8,
    pub flags: u8,
    pub kernValue: [i16; 1],
    pub leftClass: [u8; 1],
    pub rightClass: [u8; 1],
    pub kernIndex: [u8; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union KernFormatSpecificHeader {
    pub orderedList: KernOrderedListHeader,
    pub stateTable: KernStateHeader,
    pub simpleArray: KernSimpleArrayHeader,
    pub indexArray: KernIndexArrayHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernVersion0SubtableHeader {
    pub version: u16,
    pub length: u16,
    pub stInfo: KernSubtableInfo,
    pub fsHeader: KernFormatSpecificHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KernSubtableHeader {
    pub length: i32,
    pub stInfo: KernSubtableInfo,
    pub tupleIndex: i16,
    pub fsHeader: KernFormatSpecificHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxTableHeader {
    pub version: i32,
    pub nTables: u32,
    pub firstSubtable: [u32; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxKerningPair {
    pub left: u16,
    pub right: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxOrderedListEntry {
    pub pair: KerxKerningPair,
    pub value: KernKerningValue,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxOrderedListHeader {
    pub nPairs: u32,
    pub searchRange: u32,
    pub entrySelector: u32,
    pub rangeShift: u32,
    pub table: [u32; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxStateHeader {
    pub header: STXHeader,
    pub valueTable: u32,
    pub firstTable: [u8; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxStateEntry {
    pub newState: u16,
    pub flags: u16,
    pub valueIndex: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxControlPointHeader {
    pub header: STXHeader,
    pub flags: u32,
    pub firstTable: [u8; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxControlPointEntry {
    pub newState: u16,
    pub flags: u16,
    pub actionIndex: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxControlPointAction {
    pub markControlPoint: u16,
    pub currControlPoint: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxAnchorPointAction {
    pub markAnchorPoint: u16,
    pub currAnchorPoint: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxCoordinateAction {
    pub markX: u16,
    pub markY: u16,
    pub currX: u16,
    pub currY: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxSimpleArrayHeader {
    pub rowWidth: u32,
    pub leftOffsetTable: u32,
    pub rightOffsetTable: u32,
    pub theArray: KerxArrayOffset,
    pub firstTable: [u32; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxIndexArrayHeader {
    pub flags: u32,
    pub rowCount: u16,
    pub columnCount: u16,
    pub rowIndexTableOffset: u32,
    pub columnIndexTableOffset: u32,
    pub kerningArrayOffset: u32,
    pub kerningVectorOffset: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union KerxFormatSpecificHeader {
    pub orderedList: KerxOrderedListHeader,
    pub stateTable: KerxStateHeader,
    pub simpleArray: KerxSimpleArrayHeader,
    pub indexArray: KerxIndexArrayHeader,
    pub controlPoint: KerxControlPointHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct KerxSubtableHeader {
    pub length: u32,
    pub stInfo: KerxSubtableCoverage,
    pub tupleCount: u32,
    pub fsHeader: KerxFormatSpecificHeader,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct BslnFormat0Part {
    pub deltas: [i16; 32],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct BslnFormat1Part {
    pub deltas: [i16; 32],
    pub mappingData: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct BslnFormat2Part {
    pub stdGlyph: u16,
    pub ctlPoints: [i16; 32],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct BslnFormat3Part {
    pub stdGlyph: u16,
    pub ctlPoints: [i16; 32],
    pub mappingData: SFNTLookupTable,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union BslnFormatUnion {
    pub fmt0Part: BslnFormat0Part,
    pub fmt1Part: BslnFormat1Part,
    pub fmt2Part: BslnFormat2Part,
    pub fmt3Part: BslnFormat3Part,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct BslnTable {
    pub version: i32,
    pub format: BslnTableFormat,
    pub defaultBaseline: u16,
    pub parts: BslnFormatUnion,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct AnchorPoint {
    pub x: i16,
    pub y: i16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct AnchorPointTable {
    pub nPoints: u32,
    pub points: [AnchorPoint; 1],
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct AnkrTable {
    pub version: u16,
    pub flags: u16,
    pub lookupTableOffset: u32,
    pub anchorPointTableOffset: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct LtagStringRange {
    pub offset: u16,
    pub length: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct LtagTable {
    pub version: u32,
    pub flags: u32,
    pub numTags: u32,
    pub tagRange: [LtagStringRange; 1],
}

extern "C" {
    pub static kCTBaselineClassRoman: CFStringRef;
    pub static kCTBaselineClassIdeographicCentered: CFStringRef;
    pub static kCTBaselineClassIdeographicLow: CFStringRef;
    pub static kCTBaselineClassIdeographicHigh: CFStringRef;
    pub static kCTBaselineClassHanging: CFStringRef;
    pub static kCTBaselineClassMath: CFStringRef;
    pub static kCTBaselineReferenceFont: CFStringRef;
    pub static kCTBaselineOriginalFont: CFStringRef;
    pub static kCTFontBaselineAdjustAttribute: CFStringRef;
    pub static kCTFontCascadeListAttribute: CFStringRef;
    pub static kCTFontCharacterSetAttribute: CFStringRef;
    pub static kCTFontDescriptorMatchingCurrentAssetSize: CFStringRef;
    pub static kCTFontDescriptorMatchingDescriptors: CFStringRef;
    pub static kCTFontDescriptorMatchingError: CFStringRef;
    pub static kCTFontDescriptorMatchingPercentage: CFStringRef;
    pub static kCTFontDescriptorMatchingResult: CFStringRef;
    pub static kCTFontDescriptorMatchingSourceDescriptor: CFStringRef;
    pub static kCTFontDescriptorMatchingTotalAssetSize: CFStringRef;
    pub static kCTFontDescriptorMatchingTotalDownloadedSize: CFStringRef;
    pub static kCTFontDownloadedAttribute: CFStringRef;
    pub static kCTFontFixedAdvanceAttribute: CFStringRef;
    pub static kCTFontLanguagesAttribute: CFStringRef;
    pub static kCTFontMacintoshEncodingsAttribute: CFStringRef;
    pub static kCTFontMatrixAttribute: CFStringRef;
    pub static kCTFontOpticalSizeAttribute: CFStringRef;
    pub static kCTFontPriorityAttribute: CFStringRef;
    pub static kCTFontRegistrationScopeAttribute: CFStringRef;
    pub static kCTFontManagerBundleIdentifier: CFStringRef;
    pub static kCTFontManagerRegisteredFontsChangedNotification: CFStringRef;
    pub static kCTFontManagerErrorDomain: CFStringRef;
    pub static kCTFontManagerErrorFontURLsKey: CFStringRef;
    pub static kCTFrameClippingPathsAttributeName: CFStringRef;
    pub static kCTFramePathClippingPathAttributeName: CFStringRef;
    pub static kCTFramePathFillRuleAttributeName: CFStringRef;
    pub static kCTFramePathWidthAttributeName: CFStringRef;
    pub static kCTFrameProgressionAttributeName: CFStringRef;
    pub static kCTRubyAnnotationScaleToFitAttributeName: CFStringRef;
    pub static kCTRubyAnnotationSizeFactorAttributeName: CFStringRef;
    pub static kCTAdaptiveImageProviderAttributeName: CFStringRef;
    pub static kCTBackgroundColorAttributeName: CFStringRef;
    pub static kCTBaselineClassAttributeName: CFStringRef;
    pub static kCTBaselineInfoAttributeName: CFStringRef;
    pub static kCTBaselineOffsetAttributeName: CFStringRef;
    pub static kCTBaselineReferenceInfoAttributeName: CFStringRef;
    pub static kCTForegroundColorAttributeName: CFStringRef;
    pub static kCTForegroundColorFromContextAttributeName: CFStringRef;
    pub static kCTGlyphInfoAttributeName: CFStringRef;
    pub static kCTHorizontalInVerticalFormsAttributeName: CFStringRef;
    pub static kCTKernAttributeName: CFStringRef;
    pub static kCTLanguageAttributeName: CFStringRef;
    pub static kCTLigatureAttributeName: CFStringRef;
    pub static kCTRubyAnnotationAttributeName: CFStringRef;
    pub static kCTRunDelegateAttributeName: CFStringRef;
    pub static kCTStrokeColorAttributeName: CFStringRef;
    pub static kCTStrokeWidthAttributeName: CFStringRef;
    pub static kCTSuperscriptAttributeName: CFStringRef;
    pub static kCTTrackingAttributeName: CFStringRef;
    pub static kCTUnderlineColorAttributeName: CFStringRef;
    pub static kCTUnderlineStyleAttributeName: CFStringRef;
    pub static kCTVerticalFormsAttributeName: CFStringRef;
    pub static kCTWritingDirectionAttributeName: CFStringRef;
    pub static kCTTabColumnTerminatorsAttributeName: CFStringRef;
}

extern "C" {
    pub fn CTFontCopyAttribute(font: CTFontRef, attribute: CFStringRef) -> CFTypeRef;
    pub fn CTFontCopyCharacterSet(font: CTFontRef) -> CFCharacterSetRef;
    pub fn CTFontCopyDefaultCascadeListForLanguages(
        font: CTFontRef,
        languagePrefList: CFArrayRef,
    ) -> CFArrayRef;
    pub fn CTFontCreatePathForGlyph(
        font: CTFontRef,
        glyph: CGGlyph,
        matrix: *const CGAffineTransform,
    ) -> CGPathRef;
    pub fn CTFontCreateWithFontDescriptorAndOptions(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        options: CTFontOptions,
    ) -> CTFontRef;
    pub fn CTFontCreateWithGraphicsFont(
        graphicsFont: CGFontRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    pub fn CTFontCreateWithNameAndOptions(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
        options: CTFontOptions,
    ) -> CTFontRef;
    pub fn CTFontCopyGraphicsFont(
        font: CTFontRef,
        attributes: *mut CTFontDescriptorRef,
    ) -> CGFontRef;
    pub fn CTFontCopyTable(
        font: CTFontRef,
        table: CTFontTableTag,
        options: CTFontTableOptions,
    ) -> CFDataRef;
    pub fn CTFontDrawGlyphs(
        font: CTFontRef,
        glyphs: *const CGGlyph,
        positions: *const CGPoint,
        count: usize,
        context: CGContextRef,
    );
    pub fn CTFontGetLigatureCaretPositions(
        font: CTFontRef,
        glyph: CGGlyph,
        positions: *mut CGFloat,
        maxPositions: CFIndex,
    ) -> CFIndex;
    pub fn CTFontGetStringEncoding(font: CTFontRef) -> CFStringEncoding;
    pub fn CTFontGetTypographicBoundsForAdaptiveImageProvider(
        font: CTFontRef,
        provider: CFTypeRef,
    ) -> CGRect;
    pub fn CTFontDrawImageFromAdaptiveImageProviderAtPoint(
        font: CTFontRef,
        provider: CFTypeRef,
        point: CGPoint,
        context: CGContextRef,
    );

    pub fn CTFontCollectionGetTypeID() -> CFTypeID;
    pub fn CTFontCollectionCreateMutableCopy(
        original: CTFontCollectionRef,
    ) -> CTMutableFontCollectionRef;
    pub fn CTFontCollectionCopyQueryDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
    pub fn CTFontCollectionSetQueryDescriptors(
        collection: CTMutableFontCollectionRef,
        descriptors: CFArrayRef,
    );
    pub fn CTFontCollectionCopyExclusionDescriptors(collection: CTFontCollectionRef) -> CFArrayRef;
    pub fn CTFontCollectionSetExclusionDescriptors(
        collection: CTMutableFontCollectionRef,
        descriptors: CFArrayRef,
    );
    pub fn CTFontCollectionCreateMatchingFontDescriptorsSortedWithCallback(
        collection: CTFontCollectionRef,
        sortCallback: CTFontCollectionSortDescriptorsCallback,
        refCon: *mut c_void,
    ) -> CFArrayRef;
    pub fn CTFontCollectionCreateMatchingFontDescriptorsWithOptions(
        collection: CTFontCollectionRef,
        options: CFDictionaryRef,
    ) -> CFArrayRef;
    pub fn CTFontCollectionCopyFontAttribute(
        collection: CTFontCollectionRef,
        attributeName: CFStringRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;
    pub fn CTFontCollectionCopyFontAttributes(
        collection: CTFontCollectionRef,
        attributeNames: CFSetRef,
        options: CTFontCollectionCopyOptions,
    ) -> CFArrayRef;

    pub fn CTFontDescriptorGetTypeID() -> CFTypeID;
    pub fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateCopyWithAttributes(
        original: CTFontDescriptorRef,
        attributes: CFDictionaryRef,
    ) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCopyAttributes(descriptor: CTFontDescriptorRef) -> CFDictionaryRef;
    pub fn CTFontDescriptorCopyLocalizedAttribute(
        descriptor: CTFontDescriptorRef,
        attribute: CFStringRef,
        language: *mut CFStringRef,
    ) -> CFTypeRef;
    pub fn CTFontDescriptorMatchFontDescriptorsWithProgressHandler(
        descriptors: CFArrayRef,
        mandatoryAttributes: CFSetRef,
        progressBlock: CTFontDescriptorProgressHandler,
    ) -> bool;

    pub fn CTFontManagerCompareFontFamilyNames(
        family1: *const c_void,
        family2: *const c_void,
        context: *mut c_void,
    ) -> CFComparisonResult;
    pub fn CTFontManagerCreateFontDescriptorFromData(data: CFDataRef) -> CTFontDescriptorRef;
    pub fn CTFontManagerCreateFontDescriptorsFromData(data: CFDataRef) -> CFArrayRef;
    pub fn CTFontManagerRegisterFontDescriptors(
        fontDescriptors: CFArrayRef,
        scope: u32,
        enabled: bool,
        registrationHandler: CTFontManagerRegistrationHandler,
    );
    pub fn CTFontManagerRegisterFontURLs(
        fontURLs: CFArrayRef,
        scope: u32,
        enabled: bool,
        registrationHandler: CTFontManagerRegistrationHandler,
    );
    pub fn CTFontManagerUnregisterFontDescriptors(
        fontDescriptors: CFArrayRef,
        scope: u32,
        registrationHandler: CTFontManagerRegistrationHandler,
    );
    pub fn CTFontManagerUnregisterFontURLs(
        fontURLs: CFArrayRef,
        scope: u32,
        registrationHandler: CTFontManagerRegistrationHandler,
    );
    pub fn CTFontManagerEnableFontDescriptors(descriptors: CFArrayRef, enable: bool);

    pub fn CTFrameDraw(frame: CTFrameRef, context: CGContextRef);

    pub fn CTGlyphInfoGetTypeID() -> CFTypeID;

    pub fn CTLineDraw(line: CTLineRef, context: CGContextRef);
    pub fn CTLineEnumerateCaretOffsets(line: CTLineRef, block: CTLineCaretEnumerationBlock);

    pub fn CTRubyAnnotationGetTypeID() -> CFTypeID;
    pub fn CTRubyAnnotationCreateWithAttributes(
        alignment: u8,
        overhang: u8,
        position: u8,
        string: CFStringRef,
        attributes: CFDictionaryRef,
    ) -> CTRubyAnnotationRef;

    pub fn CTRunDraw(run: CTRunRef, context: CGContextRef, range: CFRange);
    pub fn CTRunDelegateGetTypeID() -> CFTypeID;
    pub fn CTRunDelegateCreate(
        callbacks: *const CTRunDelegateCallbacks,
        refCon: *mut c_void,
    ) -> CTRunDelegateRef;
    pub fn CTRunDelegateGetRefCon(runDelegate: CTRunDelegateRef) -> *mut c_void;

    pub fn CTTextTabGetTypeID() -> CFTypeID;
    pub fn CTTextTabGetOptions(tab: CTTextTabRef) -> CFDictionaryRef;

    pub fn CTTypesetterGetTypeID() -> CFTypeID;
}
