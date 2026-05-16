use serde::Deserialize;

/// Bit constants from `CTFontSymbolicTraits`.
pub mod symbolic_traits {
    pub const ITALIC: u32 = 1 << 0;
    pub const BOLD: u32 = 1 << 1;
    pub const EXPANDED: u32 = 1 << 5;
    pub const CONDENSED: u32 = 1 << 6;
    pub const MONOSPACE: u32 = 1 << 10;
    pub const VERTICAL: u32 = 1 << 11;
    pub const UI_OPTIMIZED: u32 = 1 << 12;
    pub const COLOR_GLYPHS: u32 = 1 << 13;
    pub const COMPOSITE: u32 = 1 << 14;
    pub const CLASS_MASK_SHIFT: u32 = 28;
    pub const CLASS_MASK: u32 = 15 << CLASS_MASK_SHIFT;
}

/// Normalized font trait values decoded from CoreText.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontTraits {
    pub symbolic_traits: u32,
    pub weight: f64,
    pub width: f64,
    pub slant: f64,
}

impl FontTraits {
    #[must_use]
    pub const fn has(self, trait_bits: u32) -> bool {
        self.symbolic_traits & trait_bits == trait_bits
    }

    #[must_use]
    pub const fn stylistic_class(self) -> u32 {
        self.symbolic_traits & symbolic_traits::CLASS_MASK
    }
}
