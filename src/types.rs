/// Core Graphics geometry types re-exported from `apple-cf`.
pub use apple_cf::cg::{CGAffineTransform, CGPoint, CGRect, CGSize};
/// Re-exports `CFRange` for CoreText range parameters.
pub use apple_cf::raw::CFRange;

use crate::error::{CoreTextError, CoreTextResult};

/// A range of UTF-16 code units — location and length within a string, as in `CFRange`.
///
/// Offsets count UTF-16 code units (what `NSString` and `CFString` index by), not bytes
/// and not `char`s: `"é"` is one unit and `"😀"` is two. Use `str::encode_utf16` to
/// convert. Wrappers that take a range check it against the string before calling
/// CoreText and return [`CoreTextError::RangeOutOfBounds`](crate::CoreTextError) otherwise.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TextRange {
    /// Start offset in UTF-16 code units; matches the `location` field of `CFRange`.
    pub location: isize,
    /// Number of UTF-16 code units; matches the `length` field of `CFRange`.
    pub length: isize,
}

impl TextRange {
    /// Creates a `CFRange`-style span for CoreText string APIs.
    #[inline]
    pub const fn new(location: isize, length: isize) -> Self {
        Self { location, length }
    }

    pub(crate) fn checked_cf_range(self, string_length: usize) -> CoreTextResult<CFRange> {
        let out_of_bounds = CoreTextError::RangeOutOfBounds {
            range: self,
            string_length,
        };
        let Ok(limit) = isize::try_from(string_length) else {
            return Err(out_of_bounds);
        };
        match self.location.checked_add(self.length) {
            Some(end) if self.location >= 0 && self.length >= 0 && end <= limit => {
                Ok(CFRange::from(self))
            }
            _ => Err(out_of_bounds),
        }
    }
}

pub(crate) fn checked_string_index(index: isize, string_length: usize) -> CoreTextResult<isize> {
    match isize::try_from(string_length) {
        Ok(limit) if (0..=limit).contains(&index) => Ok(index),
        _ => Err(CoreTextError::IndexOutOfBounds {
            index,
            string_length,
        }),
    }
}

pub(crate) fn utf16_length(text: &str) -> usize {
    text.encode_utf16().count()
}

impl From<CFRange> for TextRange {
    fn from(value: CFRange) -> Self {
        Self::new(
            isize::try_from(value.location)
                .expect("CFRange::location must fit in isize on supported targets"),
            isize::try_from(value.length)
                .expect("CFRange::length must fit in isize on supported targets"),
        )
    }
}

impl From<TextRange> for CFRange {
    fn from(value: TextRange) -> Self {
        Self {
            location: i64::try_from(value.location)
                .expect("TextRange::location must fit in CFIndex on supported targets"),
            length: i64::try_from(value.length)
                .expect("TextRange::length must fit in CFIndex on supported targets"),
        }
    }
}

/// Width, ascent, descent, and leading for a line or run.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TypographicBounds {
    /// Wraps the `width` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub width: f64,
    /// Wraps the `ascent` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub ascent: f64,
    /// Wraps the `descent` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub descent: f64,
    /// Wraps the `leading` value returned by `CTLineGetTypographicBounds` and `CTRunGetTypographicBounds`.
    pub leading: f64,
}

#[cfg(test)]
mod tests {
    use super::{checked_string_index, utf16_length, CFRange, TextRange, TypographicBounds};
    use crate::error::CoreTextError;

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < f64::EPSILON, "expected {left} to match {right}");
    }

    #[test]
    fn text_range_new_sets_location_and_length() {
        let range = TextRange::new(3, 5);

        assert_eq!(range.location, 3);
        assert_eq!(range.length, 5);
    }

    #[test]
    fn text_range_round_trips_through_cfrange() {
        let range = TextRange::new(7, 9);
        let cf_range: CFRange = range.into();
        let round_trip = TextRange::from(cf_range);

        assert_eq!(round_trip, range);
    }

    #[test]
    fn checked_cf_range_accepts_ranges_inside_the_string() {
        for (location, length) in [(0, 0), (0, 5), (2, 3), (5, 0)] {
            let range = TextRange::new(location, length);
            let cf_range = range.checked_cf_range(5).expect("range inside the string");
            assert_eq!(TextRange::from(cf_range), range);
        }
    }

    #[test]
    fn checked_cf_range_rejects_negative_overflowing_and_out_of_bounds_ranges() {
        for (location, length) in [
            (-1, 1),
            (0, -1),
            (0, 6),
            (6, 0),
            (4, 2),
            (1, isize::MAX),
            (isize::MAX, isize::MAX),
            (isize::MIN, 0),
        ] {
            let range = TextRange::new(location, length);
            assert!(
                matches!(
                    range.checked_cf_range(5),
                    Err(CoreTextError::RangeOutOfBounds { range: rejected, string_length: 5 })
                        if rejected == range
                ),
                "{range:?}"
            );
        }
    }

    #[test]
    fn checked_string_index_allows_the_end_of_the_string() {
        assert_eq!(checked_string_index(0, 0).ok(), Some(0));
        assert_eq!(checked_string_index(3, 3).ok(), Some(3));
        for index in [-1, 4, isize::MAX, isize::MIN] {
            assert!(matches!(
                checked_string_index(index, 3),
                Err(CoreTextError::IndexOutOfBounds { index: rejected, string_length: 3 })
                    if rejected == index
            ));
        }
    }

    #[test]
    fn utf16_length_counts_code_units_not_bytes() {
        assert_eq!(utf16_length(""), 0);
        assert_eq!(utf16_length("abc"), 3);
        assert_eq!(utf16_length("é"), 1);
        assert_eq!(utf16_length("😀"), 2);
        assert_eq!("😀".len(), 4);
    }

    #[test]
    fn typographic_bounds_default_is_zeroed() {
        let bounds = TypographicBounds::default();

        assert_close(bounds.width, 0.0);
        assert_close(bounds.ascent, 0.0);
        assert_close(bounds.descent, 0.0);
        assert_close(bounds.leading, 0.0);
    }
}
