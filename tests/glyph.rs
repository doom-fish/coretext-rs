mod support;

use coretext::{CharacterCollection, GlyphInfo};

#[test]
fn glyph_info_round_trips_font_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let font = support::font();
    let glyph = support::first_glyph_for("A");

    let glyph_info = GlyphInfo::with_glyph(glyph, &font, "A")?;
    assert_eq!(glyph_info.glyph(), glyph);

    if let Some(glyph_name) = font.name_for_glyph(glyph) {
        let glyph_by_name = GlyphInfo::with_glyph_name(&glyph_name, &font, "A")?;
        assert_eq!(
            glyph_by_name.glyph_name().as_deref(),
            Some(glyph_name.as_str())
        );
        let _ = glyph_by_name.glyph();
    }

    let character_identifier = glyph_info.character_identifier();
    if character_identifier != 0 {
        let glyph_by_identifier = GlyphInfo::with_character_identifier(
            character_identifier,
            CharacterCollection::IdentityMapping,
            "A",
        )?;
        assert_eq!(
            glyph_by_identifier.character_collection(),
            CharacterCollection::IdentityMapping
        );
    }

    Ok(())
}
