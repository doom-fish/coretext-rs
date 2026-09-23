mod support;

use apple_cf::cg::CGContext;
use coretext::{
    AttributedString, CGAffineTransform, CGPoint, CGRect, CTFramesetter, CTLine, CoreTextError,
    FontOrientation, PathElement, TextRange,
};

fn painted_pixels(context: &CGContext) -> usize {
    unsafe { context.as_bytes() }
        .chunks_exact(4)
        .filter(|pixel| pixel[3] != 0)
        .count()
}

#[test]
fn line_draw_paints_the_context_at_the_text_position() -> Result<(), Box<dyn std::error::Error>> {
    let attributed = AttributedString::new("Draw me", &support::font(), None)?;
    let line = CTLine::create_with_attributed_string(&attributed)?;

    let context = CGContext::new_rgba8(160, 40)?;
    assert_eq!(painted_pixels(&context), 0);
    line.draw(&context, CGPoint::new(4.0, 12.0));
    let painted = painted_pixels(&context);
    assert!(painted > 0);

    let clipped = CGContext::new_rgba8(160, 40)?;
    line.draw(&clipped, CGPoint::new(400.0, 12.0));
    assert_eq!(painted_pixels(&clipped), 0);
    Ok(())
}

#[test]
fn frame_draw_paints_the_context() -> Result<(), Box<dyn std::error::Error>> {
    let attributed = AttributedString::new("Frames draw every line", &support::font(), None)?;
    let framesetter = CTFramesetter::create_with_attributed_string(&attributed)?;
    let frame = framesetter
        .create_frame_in_rect(CGRect::new(0.0, 0.0, 120.0, 80.0), TextRange::new(0, 0))?;
    let context = CGContext::new_rgba8(120, 80)?;
    frame.draw(&context);
    assert!(painted_pixels(&context) > 0);
    Ok(())
}

#[test]
fn font_draw_glyphs_requires_matching_positions() -> Result<(), Box<dyn std::error::Error>> {
    let font = support::font();
    let glyphs = font.glyphs_for_string("Hi")?;
    let context = CGContext::new_rgba8(64, 32)?;

    let mismatch = font.draw_glyphs(&glyphs, &[CGPoint::new(2.0, 8.0)], &context);
    assert!(matches!(
        mismatch,
        Err(CoreTextError::LengthMismatch {
            expected: 2,
            actual: 1
        })
    ));
    font.draw_glyphs(&[], &[], &context)?;
    assert_eq!(painted_pixels(&context), 0);

    font.draw_glyphs(
        &glyphs,
        &[CGPoint::new(2.0, 8.0), CGPoint::new(14.0, 8.0)],
        &context,
    )?;
    assert!(painted_pixels(&context) > 0);
    Ok(())
}

#[test]
fn glyph_paths_match_glyph_bounds() {
    let font = support::font();
    let glyph = support::first_glyph_for("A");
    let path = font.path_for_glyph(glyph, None).expect("A has an outline");
    assert!(!path.is_empty());

    let (_, rects) = font.bounding_rects_for_glyphs(FontOrientation::Horizontal, &[glyph]);
    let bounds = path.path_bounding_box();
    assert!((bounds.size.width - rects[0].size.width).abs() < 0.01);
    assert!((bounds.size.height - rects[0].size.height).abs() < 0.01);

    let elements = path.elements();
    assert!(matches!(elements.first(), Some(PathElement::MoveTo(_))));
    assert!(elements.contains(&PathElement::CloseSubpath));

    let doubled = CGAffineTransform {
        a: 2.0,
        b: 0.0,
        c: 0.0,
        d: 2.0,
        tx: 0.0,
        ty: 0.0,
    };
    let scaled = font
        .path_for_glyph(glyph, Some(&doubled))
        .expect("A has an outline");
    assert!((scaled.path_bounding_box().size.width / bounds.size.width - 2.0).abs() < 0.001);
    assert_eq!(scaled.elements().len(), elements.len());

    let space = support::first_glyph_for(" ");
    assert!(font.path_for_glyph(space, None).is_none());
}

#[test]
fn caret_offsets_cover_both_edges_of_each_character() -> Result<(), Box<dyn std::error::Error>> {
    let attributed = AttributedString::new("ab", &support::font(), None)?;
    let line = CTLine::create_with_attributed_string(&attributed)?;
    let carets = line.caret_offsets();

    let indices: Vec<isize> = carets.iter().map(|caret| caret.string_index).collect();
    let leading: Vec<bool> = carets.iter().map(|caret| caret.leading_edge).collect();
    assert_eq!(indices, [0, 0, 1, 1]);
    assert_eq!(leading, [true, false, true, false]);
    assert!(carets
        .windows(2)
        .all(|pair| pair[0].offset <= pair[1].offset));
    assert!(carets[0].offset.abs() < f64::EPSILON);
    assert!((carets[3].offset - line.typographic_bounds().width).abs() < 0.01);
    Ok(())
}
