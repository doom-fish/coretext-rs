mod support;

use coretext::{
    bounds_options, AttributedString, CGPoint, CGRect, CGSize, CTFramesetter, CTLine, CTTypesetter,
    LineBreakMode, LineTruncationType, ParagraphStyle, ParagraphStyleOptions, TextAlignment,
    TextRange, TextTab, TypesetterOptions, WritingDirection,
};

#[test]
fn paragraph_style_and_text_tabs_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let tab = TextTab::new(TextAlignment::Left, 24.0)?;
    assert_eq!(tab.alignment(), TextAlignment::Left);
    assert!((tab.location() - 24.0).abs() < f64::EPSILON);

    let style = ParagraphStyle::new(&ParagraphStyleOptions {
        alignment: Some(TextAlignment::Center),
        first_line_head_indent: Some(8.0),
        head_indent: Some(8.0),
        tail_indent: Some(-8.0),
        text_tabs: vec![tab],
        default_tab_interval: Some(24.0),
        line_break_mode: Some(LineBreakMode::TruncatingTail),
        base_writing_direction: Some(WritingDirection::LeftToRight),
        ..ParagraphStyleOptions::default()
    })?;

    assert_eq!(style.alignment(), TextAlignment::Center);
    assert_eq!(style.line_break_mode(), LineBreakMode::TruncatingTail);
    assert_eq!(
        style.base_writing_direction(),
        WritingDirection::LeftToRight
    );
    assert_eq!(style.tab_stops().len(), 1);
    assert!((style.tab_stops()[0].location() - 24.0).abs() < f64::EPSILON);
    assert_eq!(style.copy()?.alignment(), TextAlignment::Center);
    Ok(())
}

#[test]
fn line_run_typesetter_framesetter_and_frame() -> Result<(), Box<dyn std::error::Error>> {
    let attr = support::attributed_string();
    let typesetter = CTTypesetter::create_with_options(
        &attr,
        TypesetterOptions {
            allow_unbounded_layout: true,
            forced_embedding_level: Some(0),
        },
    )?;
    let break_index = typesetter.suggest_line_break(0, 220.0);
    assert!(break_index > 0);
    assert!(typesetter.suggest_cluster_break(0, 220.0) > 0);
    assert!(typesetter.suggest_line_break_with_offset(0, 220.0, 4.0) > 0);
    assert!(typesetter.suggest_cluster_break_with_offset(0, 220.0, 4.0) > 0);

    let line = typesetter.create_line(TextRange::new(0, break_index))?;
    assert!(line.glyph_count() > 0);
    assert!(line.string_index_for_position(CGPoint::new(0.0, 0.0)) >= 0);
    let (primary, secondary) = line.offset_for_string_index(0);
    assert!(primary >= 0.0);
    assert!(secondary >= 0.0);

    let line_with_offset =
        typesetter.create_line_with_offset(TextRange::new(0, break_index), 4.0)?;
    assert!(line_with_offset.glyph_count() > 0);

    let bounds = line.typographic_bounds();
    assert!(bounds.width > 0.0);
    assert!(
        line.bounds_with_options(bounds_options::USE_OPTICAL_BOUNDS)
            .size
            .width
            >= 0.0
    );
    assert!(line.image_bounds().size.width >= 0.0);
    assert!(line.trailing_whitespace_width() >= 0.0);
    assert!(line.pen_offset_for_flush(0.5, bounds.width + 40.0) >= 0.0);
    assert!(line.justified(1.0, bounds.width + 20.0).is_some());

    let token_attr = AttributedString::new("…", &support::font(), None)?;
    let token = CTLine::create_with_attributed_string(&token_attr)?;
    assert!(line
        .truncated(bounds.width / 2.0, LineTruncationType::End, Some(&token))
        .is_some());

    let runs = line.runs();
    assert!(!runs.is_empty());
    let run = &runs[0];
    assert!(run.glyph_count() > 0);
    assert_eq!(run.glyphs().len(), run.positions().len());
    assert_eq!(run.glyphs().len(), run.advances().len());
    assert_eq!(run.glyphs().len(), run.string_indices().len());
    assert!(run.typographic_bounds().width >= 0.0);
    assert!(run.image_bounds().size.width >= 0.0);
    let _ = run.text_matrix();
    let _ = run.attributes_json()?;
    let (base_advances, origins) = run.base_advances_and_origins();
    assert_eq!(base_advances.len(), origins.len());

    let framesetter = CTFramesetter::create_with_typesetter(&typesetter)?;
    assert!(framesetter.typesetter()?.suggest_line_break(0, 220.0) > 0);
    let (suggested, fit_range) = framesetter.suggest_frame_size_for_range(
        TextRange::new(0, break_index),
        CGSize::new(220.0, f64::INFINITY),
    );
    assert!(suggested.width > 0.0);
    let frame = framesetter.create_frame_in_rect(
        CGRect::new(0.0, 0.0, suggested.width + 24.0, 120.0),
        fit_range,
    )?;
    assert!(!frame.lines().is_empty());
    assert_eq!(frame.lines().len(), frame.line_origins().len());
    assert!(frame.path_bounding_box().size.width >= 0.0);
    let _ = frame.has_frame_attributes();
    Ok(())
}
