use serde::Deserialize;

/// Bit constants from `CTFontSymbolicTraits`.
pub mod symbolic_traits {
    /// Wraps the `kCTFontTraitItalic` bit.
    pub const ITALIC: u32 = 1 << 0;
    /// Wraps the `kCTFontTraitBold` bit.
    pub const BOLD: u32 = 1 << 1;
    /// Wraps the `kCTFontTraitExpanded` bit.
    pub const EXPANDED: u32 = 1 << 5;
    /// Wraps the `kCTFontTraitCondensed` bit.
    pub const CONDENSED: u32 = 1 << 6;
    /// Wraps the `kCTFontTraitMonoSpace` bit.
    pub const MONOSPACE: u32 = 1 << 10;
    /// Wraps the `kCTFontTraitVertical` bit.
    pub const VERTICAL: u32 = 1 << 11;
    /// Wraps the `kCTFontTraitUIOptimized` bit.
    pub const UI_OPTIMIZED: u32 = 1 << 12;
    /// Wraps the color-glyph bit in `CTFontSymbolicTraits`.
    pub const COLOR_GLYPHS: u32 = 1 << 13;
    /// Wraps the composite bit in `CTFontSymbolicTraits`.
    pub const COMPOSITE: u32 = 1 << 14;
    /// Wraps the stylistic-class shift used by `CTFontSymbolicTraits`.
    pub const CLASS_MASK_SHIFT: u32 = 28;
    /// Wraps the stylistic-class mask used by `CTFontSymbolicTraits`.
    pub const CLASS_MASK: u32 = 15 << CLASS_MASK_SHIFT;
}

/// Normalized font trait values decoded from CoreText.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontTraits {
    /// Wraps the `symbolic_traits` value returned by `CTFontCopyTraits` and `CTFontGetSymbolicTraits`.
    pub symbolic_traits: u32,
    /// Wraps the `weight` value returned by `CTFontCopyTraits` and `CTFontGetSymbolicTraits`.
    pub weight: f64,
    /// Wraps the `width` value returned by `CTFontCopyTraits` and `CTFontGetSymbolicTraits`.
    pub width: f64,
    /// Wraps the `slant` value returned by `CTFontCopyTraits` and `CTFontGetSymbolicTraits`.
    pub slant: f64,
}

impl FontTraits {
    /// Checks bit flags returned by `CTFontGetSymbolicTraits`.
    #[must_use]
    pub const fn has(self, trait_bits: u32) -> bool {
        self.symbolic_traits & trait_bits == trait_bits
    }

    /// Extracts the stylistic class bits from `CTFontSymbolicTraits`.
    #[must_use]
    pub const fn stylistic_class(self) -> u32 {
        self.symbolic_traits & symbolic_traits::CLASS_MASK
    }
}

#[cfg(test)]
mod tests {
    use super::{symbolic_traits, FontTraits};

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < f64::EPSILON, "expected {left} to match {right}");
    }

    #[test]
    fn symbolic_trait_constants_are_stable() {
        assert_eq!(symbolic_traits::ITALIC, 1 << 0);
        assert_eq!(symbolic_traits::BOLD, 1 << 1);
        assert_eq!(symbolic_traits::EXPANDED, 1 << 5);
        assert_eq!(symbolic_traits::CLASS_MASK_SHIFT, 28);
        assert_eq!(symbolic_traits::CLASS_MASK, 15 << 28);
    }

    #[test]
    fn has_checks_requested_trait_bits() {
        let traits = FontTraits {
            symbolic_traits: symbolic_traits::ITALIC | symbolic_traits::BOLD,
            ..FontTraits::default()
        };

        assert!(traits.has(symbolic_traits::ITALIC));
        assert!(traits.has(symbolic_traits::ITALIC | symbolic_traits::BOLD));
        assert!(!traits.has(symbolic_traits::EXPANDED));
    }

    #[test]
    fn stylistic_class_masks_non_class_bits() {
        let class_bits = 3 << symbolic_traits::CLASS_MASK_SHIFT;
        let traits = FontTraits {
            symbolic_traits: class_bits | symbolic_traits::MONOSPACE,
            ..FontTraits::default()
        };

        assert_eq!(traits.stylistic_class(), class_bits);
    }

    #[test]
    fn default_traits_zero_out_numeric_fields() {
        let traits = FontTraits::default();

        assert_eq!(traits.symbolic_traits, 0);
        assert_close(traits.weight, 0.0);
        assert_close(traits.width, 0.0);
        assert_close(traits.slant, 0.0);
    }
}
