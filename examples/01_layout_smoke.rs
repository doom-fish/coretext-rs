//! Smoke test: lay out "Hello, CoreText!" and verify key measurements.
use coretext::{
    AttributedString, CGRect, CGSize, CTFont, CTFramesetter, CTLine, ParagraphStyle, TextAlignment,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── Font ──────────────────────────────────────────────────────────────────
    let font = CTFont::new("Helvetica", 24.0)?;
    println!(
        "✅ CTFont(\"{}\", size={}) postscript=\"{}\" family=\"{}\"",
        font.full_name()?,
        font.size(),
        font.postscript_name()?,
        font.family_name()?,
    );
    println!(
        "   ascent={:.4}  descent={:.4}  leading={:.4}  glyphs={}",
        font.ascent(),
        font.descent(),
        font.leading(),
        font.glyph_count(),
    );

    // ── Paragraph style ───────────────────────────────────────────────────────
    let para = ParagraphStyle::with_alignment(TextAlignment::Center)?;
    assert_eq!(para.alignment(), TextAlignment::Center);
    println!("✅ ParagraphStyle alignment={:?}", para.alignment());

    // ── Attributed string ─────────────────────────────────────────────────────
    let text = "Hello, CoreText!";
    let attr = AttributedString::new(text, &font, Some(&para))?;
    println!("✅ AttributedString created ({} chars)", text.len());

    // ── Line ──────────────────────────────────────────────────────────────────
    let line = CTLine::create_with_attributed_string(&attr)?;
    let bounds = line.typographic_bounds();
    println!(
        "✅ CTLine  glyphs={}  range={:?}  width={:.4}  ascent={:.4}  descent={:.4}",
        line.glyph_count(),
        line.string_range(),
        bounds.width,
        bounds.ascent,
        bounds.descent,
    );

    let runs = line.runs();
    println!("   runs={}", runs.len());
    for (i, run) in runs.iter().enumerate() {
        let rb = run.typographic_bounds();
        println!(
            "   run[{i}] glyphs={}  status={}  range={:?}  width={:.4}",
            run.glyph_count(),
            run.status(),
            run.string_range(),
            rb.width,
        );
        println!(
            "   run[{i}] positions[0]={:?}  advances[0]={:?}  indices[0]={:?}",
            run.positions().first(),
            run.advances().first(),
            run.string_indices().first(),
        );
    }

    // ── Framesetter ───────────────────────────────────────────────────────────
    let setter = CTFramesetter::create_with_attributed_string(&attr)?;
    let (suggested, fit_range) =
        setter.suggest_frame_size_with_constraints(CGSize::new(f64::INFINITY, f64::INFINITY));
    println!("✅ CTFramesetter  suggested={suggested:?}  fit_range={fit_range:?}");

    // ── Frame ─────────────────────────────────────────────────────────────────
    let rect = CGRect::new(0.0, 0.0, suggested.width + 40.0, 200.0);
    let frame = setter.create_frame_in_rect(rect, fit_range)?;
    let frame_lines = frame.lines();
    let origins = frame.line_origins();
    println!(
        "✅ CTFrame  string_range={:?}  visible={:?}  lines={}",
        frame.string_range(),
        frame.visible_string_range(),
        frame_lines.len(),
    );
    for (i, origin) in origins.iter().enumerate() {
        println!("   line_origin[{i}] = ({:.4}, {:.4})", origin.x, origin.y);
    }

    println!("\n✅ coretext layout OK");
    Ok(())
}
