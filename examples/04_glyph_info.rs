//! Glyph info example.
use coretext::{CTFont, GlyphInfo};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = CTFont::new("Helvetica", 18.0)?;
    let glyph = font.glyphs_for_string("A")?[0];
    let glyph_info = GlyphInfo::with_glyph(glyph, &font, "A")?;
    println!(
        "glyph={} name={:?} collection={:?}",
        glyph_info.glyph(),
        glyph_info.glyph_name(),
        glyph_info.character_collection()
    );
    Ok(())
}
