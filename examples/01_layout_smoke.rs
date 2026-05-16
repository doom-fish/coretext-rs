//! Layout pipeline example covering paragraph styles, text tabs, lines, runs,
//! typesetters, framesetters, and frames.
use coretext::{
    bounds_options, AttributedString, CGRect, CGSize, CTFramesetter, CTTypesetter, LineBreakMode,
    ParagraphStyle, ParagraphStyleOptions, TextAlignment, TextRange, TextTab, TypesetterOptions,
    WritingDirection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = coretext::CTFont::new("Helvetica", 20.0)?;
    let tab = TextTab::new(TextAlignment::Left, 32.0)?;

    let paragraph_style = ParagraphStyle::new(&ParagraphStyleOptions {
        alignment: Some(TextAlignment::Center),
        text_tabs: vec![tab],
        default_tab_interval: Some(32.0),
        line_break_mode: Some(LineBreakMode::TruncatingTail),
        base_writing_direction: Some(WritingDirection::LeftToRight),
        ..ParagraphStyleOptions::default()
    })?;

    let text = "Tabs\tmake CoreText layout easier to inspect.";
    let attributed = AttributedString::new(text, &font, Some(&paragraph_style))?;

    let typesetter = CTTypesetter::create_with_options(
        &attributed,
        TypesetterOptions {
            allow_unbounded_layout: true,
            forced_embedding_level: Some(0),
        },
    )?;
    let break_index = typesetter.suggest_line_break(0, 260.0);
    let line = typesetter.create_line(TextRange::new(0, break_index))?;
    let line_bounds = line.typographic_bounds();
    println!(
        "line glyphs={} width={:.2} truncated={}",
        line.glyph_count(),
        line_bounds.width,
        line.truncated(
            line_bounds.width / 2.0,
            coretext::LineTruncationType::End,
            None
        )
        .is_some()
    );

    for (index, run) in line.runs().iter().enumerate() {
        println!(
            "run[{index}] glyphs={} width={:.2} attrs={}",
            run.glyph_count(),
            run.typographic_bounds().width,
            run.attributes_json()?.is_object()
        );
    }

    let framesetter = CTFramesetter::create_with_typesetter(&typesetter)?;
    let (suggested, fit_range) =
        framesetter.suggest_frame_size_with_constraints(CGSize::new(260.0, f64::INFINITY));
    let frame = framesetter.create_frame_in_rect(
        CGRect::new(0.0, 0.0, suggested.width + 24.0, 120.0),
        fit_range,
    )?;
    println!(
        "frame lines={} visible={:?} optical-width={:.2}",
        frame.lines().len(),
        frame.visible_string_range(),
        line.bounds_with_options(bounds_options::USE_OPTICAL_BOUNDS)
            .size
            .width,
    );
    println!("tab stop count={}", paragraph_style.tab_stops().len());
    Ok(())
}
