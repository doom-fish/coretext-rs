mod support;

use coretext::{
    font_type_id, symbolic_traits, CTFont, FontNameKey, FontOrientation, TextRange, UIFontType,
};

#[test]
fn font_surface_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let font = support::font();
    assert!((font.size() - 16.0).abs() < f64::EPSILON);
    assert_eq!(font.matrix(), coretext::CGAffineTransform::IDENTITY);
    assert!(!font.postscript_name()?.is_empty());
    assert!(!font.family_name()?.is_empty());
    assert!(!font.full_name()?.is_empty());
    assert!(!font.display_name()?.is_empty());
    assert_eq!(
        font.name(FontNameKey::Full).as_deref(),
        Some(font.full_name()?.as_str())
    );
    assert!(font.localized_name(FontNameKey::Family)?.is_some());
    assert!(font.ascent() > 0.0);
    assert!(font.units_per_em() > 0);
    assert!(font.glyph_count() > 0);
    assert!(font.bounding_box().size.width >= 0.0);
    assert!(font.underline_thickness() >= 0.0);
    assert!(font.cap_height() > 0.0);
    assert!(font.x_height() >= 0.0);
    assert!(!font.supported_languages()?.is_empty());
    assert_eq!(
        font.attribute_json("familyName")?.as_str(),
        Some(font.family_name()?.as_str())
    );
    let _ = font.string_encoding();
    let _ = font.default_cascade_list(&[])?;
    assert!(font_type_id() > 0);
    Ok(())
}

#[test]
fn font_traits_features_and_variations() -> Result<(), Box<dyn std::error::Error>> {
    let font = support::font();
    let traits = font.traits()?;
    assert_eq!(traits.symbolic_traits, font.symbolic_traits());
    let _ = traits.has(symbolic_traits::BOLD);

    let glyphs = font.glyphs_for_string("Hello")?;
    assert_eq!(glyphs.len(), 5);

    let glyph = support::first_glyph_for("A");
    if let Some(glyph_name) = font.name_for_glyph(glyph) {
        assert_eq!(font.glyph_with_name(&glyph_name)?, glyph);
    }

    let (overall_bounds, glyph_bounds) =
        font.bounding_rects_for_glyphs(FontOrientation::Horizontal, &[glyph]);
    assert_eq!(glyph_bounds.len(), 1);
    assert!(overall_bounds.size.width >= 0.0);

    let (_, optical_bounds) = font.optical_bounds_for_glyphs(&[glyph]);
    assert_eq!(optical_bounds.len(), 1);

    let (advance_total, advances) = font.advances_for_glyphs(FontOrientation::Horizontal, &[glyph]);
    assert_eq!(advances.len(), 1);
    assert!(advance_total >= 0.0);

    assert_eq!(font.vertical_translations_for_glyphs(&[glyph]).len(), 1);

    let features = font.features()?;
    if let Some(feature) = features.first() {
        assert!(!feature.name.is_empty());
    }
    let _ = font.feature_settings()?;

    let variation_axes = font.variation_axes()?;
    if let Some(axis) = variation_axes.first() {
        assert!(!axis.name.is_empty());
    }
    let _ = font.variation_coordinates()?;

    let tables = font.available_tables()?;
    if let Some(tag) = tables.first() {
        assert!(font.has_table(*tag));
        assert!(!font.table_data(*tag)?.is_empty());
    }

    let _ = font.ligature_caret_positions(support::first_glyph_for("fi"));

    Ok(())
}

#[test]
fn font_creation_helpers() -> Result<(), Box<dyn std::error::Error>> {
    let font = support::font();
    let descriptor = font.descriptor()?;
    let rebuilt = CTFont::from_descriptor(&descriptor, 18.0)?;
    assert!((rebuilt.size() - 18.0).abs() < f64::EPSILON);

    let ui_font = CTFont::ui_font(UIFontType::System, 13.0, None)?;
    assert!(ui_font.size() > 0.0);

    let family_copy = font.copy_with_family(16.0, &font.family_name()?)?;
    assert!(!family_copy.full_name()?.is_empty());

    let trait_copy = font.copy_with_symbolic_traits(
        font.size(),
        font.symbolic_traits(),
        font.symbolic_traits(),
    )?;
    assert!(trait_copy.glyph_count() > 0);

    let string_copy = font.font_for_string("Hello", TextRange::new(0, 5), None)?;
    assert!(string_copy.glyph_count() > 0);

    let attributed_copy = font.copy_with_attributes(14.0, Some(&descriptor))?;
    assert!((attributed_copy.size() - 14.0).abs() < f64::EPSILON);

    let with_options = CTFont::with_name_and_options("Helvetica", 15.0, 0)?;
    assert!((with_options.size() - 15.0).abs() < f64::EPSILON);

    let descriptor_options = CTFont::from_descriptor_with_options(&descriptor, 12.0, 0)?;
    assert!((descriptor_options.size() - 12.0).abs() < f64::EPSILON);
    Ok(())
}
