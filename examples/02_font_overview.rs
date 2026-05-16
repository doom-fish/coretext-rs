//! Font-centric example covering descriptors, traits, features, variations, and tables.
use coretext::{CTFont, FontNameKey, FontOrientation, UIFontType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = CTFont::new("Helvetica", 18.0)?;
    let descriptor = font.descriptor()?;
    let ui_font = CTFont::ui_font(UIFontType::System, 13.0, None)?;
    let glyph = font.glyphs_for_string("A")?[0];

    println!(
        "font={} postscript={} family={} ui-size={}",
        font.full_name()?,
        font.postscript_name()?,
        font.family_name()?,
        ui_font.size()
    );
    println!(
        "descriptor family={:?} attrs={}",
        descriptor.family_name(),
        descriptor.attributes_json()?.is_object()
    );
    println!(
        "traits={:?} features={} variation-axes={} tables={}",
        font.traits()?,
        font.features()?.len(),
        font.variation_axes()?.len(),
        font.available_tables()?.len()
    );
    println!(
        "glyph={} name={:?} full-name={:?} bounds={:?} advances={:?}",
        glyph,
        font.name_for_glyph(glyph),
        font.name(FontNameKey::Full),
        font.bounding_rects_for_glyphs(FontOrientation::Horizontal, &[glyph])
            .1,
        font.advances_for_glyphs(FontOrientation::Horizontal, &[glyph])
            .1,
    );
    Ok(())
}
