mod support;

use coretext::{
    bounds_options, frame_type_id, framesetter_type_id, line_type_id, paragraph_style_type_id,
    run_type_id, text_tab_type_id, typesetter_type_id, AttributedString, CGPoint, CGRect, CGSize,
    CTFramesetter, CTLine, CTTypesetter, CoreTextError, CoreTextResult, LineBreakMode,
    LineTruncationType, ParagraphStyle, ParagraphStyleOptions, TextAlignment, TextRange, TextTab,
    TypesetterOptions, WritingDirection,
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
    let tab_options = style.tab_stops()[0].options_json()?;
    assert!(tab_options.is_null());
    let alignment = style.value_for_specifier_json(0)?;
    assert_eq!(alignment.as_u64(), Some(2));
    assert_eq!(style.copy()?.alignment(), TextAlignment::Center);
    assert!(text_tab_type_id() > 0);
    assert!(paragraph_style_type_id() > 0);
    Ok(())
}

#[test]
fn line_run_typesetter_framesetter_and_frame() -> Result<(), Box<dyn std::error::Error>> {
    let attr = support::attributed_string();
    assert!(typesetter_type_id() > 0);
    assert!(line_type_id() > 0);
    assert!(run_type_id() > 0);
    assert!(framesetter_type_id() > 0);
    assert!(frame_type_id() > 0);
    let typesetter = CTTypesetter::create_with_options(
        &attr,
        TypesetterOptions {
            allow_unbounded_layout: true,
            forced_embedding_level: Some(0),
        },
    )?;
    let break_index = typesetter.suggest_line_break(0, 220.0)?;
    assert!(break_index > 0);
    assert!(typesetter.suggest_cluster_break(0, 220.0)? > 0);
    assert!(typesetter.suggest_line_break_with_offset(0, 220.0, 4.0)? > 0);
    assert!(typesetter.suggest_cluster_break_with_offset(0, 220.0, 4.0)? > 0);

    let line = typesetter.create_line(TextRange::new(0, break_index))?;
    assert!(line.glyph_count() > 0);
    assert_eq!(line.string_range(), TextRange::new(0, break_index));
    assert_eq!(line.string_index_for_position(CGPoint::new(0.0, 0.0)), 0);
    let (primary, secondary) = line.offset_for_string_index(0);
    assert!(primary.abs() < f64::EPSILON);
    assert!(secondary.abs() < f64::EPSILON);

    let line_with_offset =
        typesetter.create_line_with_offset(TextRange::new(0, break_index), 4.0)?;
    assert!(line_with_offset.glyph_count() > 0);

    let bounds = line.typographic_bounds();
    assert!(bounds.width > 0.0);
    assert!(
        line.bounds_with_options(bounds_options::USE_OPTICAL_BOUNDS)
            .size
            .width
            > 0.0
    );
    let image_bounds = line.image_bounds();
    assert!(image_bounds.size.width > 0.0 && image_bounds.size.width <= bounds.width);
    let trailing = line.trailing_whitespace_width();
    assert!(
        trailing > 0.0,
        "the suggested break keeps the space after the last word"
    );
    let pen_offset = line.pen_offset_for_flush(0.5, bounds.width + 40.0);
    assert!((pen_offset - (40.0 + trailing) / 2.0).abs() < 0.01);
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
    assert!(run.typographic_bounds().width > 0.0);
    assert!(run.image_bounds().size.width > 0.0);
    assert_eq!(run.text_matrix(), coretext::CGAffineTransform::IDENTITY);
    let attributes = run.attributes_json()?;
    assert!(attributes["keys"]
        .as_array()
        .is_some_and(|keys| keys.iter().any(|key| key == "NSFont")));
    let (base_advances, origins) = run.base_advances_and_origins();
    assert_eq!(base_advances.len(), origins.len());
    assert_eq!(base_advances.len(), run.glyphs().len());

    let framesetter = CTFramesetter::create_with_typesetter(&typesetter)?;
    assert_eq!(
        framesetter.typesetter()?.suggest_line_break(0, 220.0)?,
        break_index
    );
    let (suggested, fit_range) = framesetter.suggest_frame_size_for_range(
        TextRange::new(0, break_index),
        CGSize::new(220.0, f64::INFINITY),
    )?;
    assert!(suggested.width > 0.0);
    assert_eq!(fit_range, TextRange::new(0, break_index));
    let frame = framesetter.create_frame_in_rect(
        CGRect::new(0.0, 0.0, suggested.width + 24.0, 120.0),
        fit_range,
    )?;
    assert!(!frame.lines().is_empty());
    assert_eq!(frame.lines().len(), frame.line_origins().len());
    let path_bounds = frame.path_bounding_box();
    assert!((path_bounds.size.width - (suggested.width + 24.0)).abs() < f64::EPSILON);
    assert!((path_bounds.size.height - 120.0).abs() < f64::EPSILON);
    assert!(!frame.has_frame_attributes());
    Ok(())
}

fn layout_string(text: &str) -> AttributedString {
    AttributedString::new(text, &support::font(), None).expect("attributed string")
}

fn is_range_error(result: &Result<impl Sized, CoreTextError>, expected: TextRange) -> bool {
    matches!(
        result,
        Err(CoreTextError::RangeOutOfBounds { range, .. }) if *range == expected
    )
}

#[test]
fn typesetter_rejects_ranges_outside_the_utf16_string() -> Result<(), Box<dyn std::error::Error>> {
    let text = "héllo 😀";
    let length = isize::try_from(text.encode_utf16().count())?;
    assert_eq!(length, 8);
    let typesetter = CTTypesetter::create_with_attributed_string(&layout_string(text))?;

    assert_eq!(
        typesetter
            .create_line(TextRange::new(0, length))?
            .string_range(),
        TextRange::new(0, length)
    );
    assert_eq!(
        typesetter
            .create_line(TextRange::new(length, 0))?
            .glyph_count(),
        0
    );

    let byte_length = isize::try_from(text.len())?;
    for range in [
        TextRange::new(0, byte_length),
        TextRange::new(0, length + 1),
        TextRange::new(length + 1, 0),
        TextRange::new(-1, 2),
        TextRange::new(0, -3),
        TextRange::new(3, isize::MAX),
        TextRange::new(isize::MAX, 1),
    ] {
        assert!(
            is_range_error(&typesetter.create_line(range), range),
            "{range:?}"
        );
        assert!(
            is_range_error(&typesetter.create_line_with_offset(range, 2.0), range),
            "{range:?}"
        );
    }
    Ok(())
}

#[test]
fn typesetter_rejects_break_indices_outside_the_string() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Break me";
    let length = isize::try_from(text.encode_utf16().count())?;
    let typesetter = CTTypesetter::create_with_attributed_string(&layout_string(text))?;

    assert_eq!(typesetter.suggest_line_break(length, 100.0)?, 0);
    assert_eq!(typesetter.suggest_cluster_break(length, 100.0)?, 0);
    for index in [-1, length + 1, isize::MAX, isize::MIN] {
        let expected = |result: CoreTextResult<isize>| {
            matches!(
                result,
                Err(CoreTextError::IndexOutOfBounds { index: rejected, string_length: 8 })
                    if rejected == index
            )
        };
        assert!(expected(typesetter.suggest_line_break(index, 100.0)));
        assert!(expected(
            typesetter.suggest_line_break_with_offset(index, 100.0, 1.0)
        ));
        assert!(expected(typesetter.suggest_cluster_break(index, 100.0)));
        assert!(expected(
            typesetter.suggest_cluster_break_with_offset(index, 100.0, 1.0)
        ));
    }
    Ok(())
}

#[test]
fn framesetter_rejects_ranges_outside_the_string() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Frame me";
    let length = isize::try_from(text.encode_utf16().count())?;
    let framesetter = CTFramesetter::create_with_attributed_string(&layout_string(text))?;
    let rect = CGRect::new(0.0, 0.0, 200.0, 200.0);
    let constraints = CGSize::new(200.0, f64::INFINITY);

    assert_eq!(
        framesetter
            .create_frame_in_rect(rect, TextRange::new(2, length - 2))?
            .string_range(),
        TextRange::new(2, length - 2)
    );
    for range in [
        TextRange::new(-4, 1),
        TextRange::new(0, length + 5),
        TextRange::new(length + 5, 1),
        TextRange::new(1, isize::MAX),
    ] {
        assert!(is_range_error(
            &framesetter.create_frame_in_rect(rect, range),
            range
        ));
        assert!(is_range_error(
            &framesetter.suggest_frame_size_for_range(range, constraints),
            range
        ));
    }

    let derived = framesetter.typesetter()?;
    assert!(is_range_error(
        &derived.create_line(TextRange::new(0, length + 1)),
        TextRange::new(0, length + 1)
    ));
    let rebuilt = CTFramesetter::create_with_typesetter(&derived)?;
    assert!(is_range_error(
        &rebuilt.create_frame_in_rect(rect, TextRange::new(length, 1)),
        TextRange::new(length, 1)
    ));
    Ok(())
}

#[test]
fn font_for_string_validates_utf16_ranges() -> Result<(), Box<dyn std::error::Error>> {
    let font = support::font();
    let text = "日本語 text";
    let length = isize::try_from(text.encode_utf16().count())?;

    assert!(
        font.font_for_string(text, TextRange::new(0, 3), None)?
            .glyph_count()
            > 0
    );
    assert!(font
        .font_for_string(text, TextRange::new(length, 0), None)
        .is_ok());
    for range in [
        TextRange::new(0, isize::try_from(text.len())?),
        TextRange::new(length + 5, 1),
        TextRange::new(-1, 1),
        TextRange::new(2, isize::MAX),
    ] {
        assert!(is_range_error(
            &font.font_for_string(text, range, None),
            range
        ));
    }
    Ok(())
}
