#![allow(dead_code)]

use std::path::Path;

use coretext::{
    AttributedString, CGRect, CGSize, CTFont, CTFrame, CTFramesetter, CTLine, CTTypesetter,
    FontManager, LineBreakMode, ParagraphStyle, ParagraphStyleOptions, TextAlignment, TextRange,
    TextTab, TypesetterOptions, WritingDirection,
};

pub const SAMPLE_TEXT: &str = "Tabs\tand ruby make CoreText layout interesting.";

pub fn font() -> CTFont {
    CTFont::new("Helvetica", 16.0).expect("Helvetica must be available on macOS")
}

pub fn text_tab() -> TextTab {
    TextTab::new(TextAlignment::Left, 24.0).expect("CTTextTab")
}

pub fn paragraph_style() -> ParagraphStyle {
    ParagraphStyle::new(&ParagraphStyleOptions {
        alignment: Some(TextAlignment::Center),
        first_line_head_indent: Some(12.0),
        head_indent: Some(12.0),
        tail_indent: Some(-12.0),
        text_tabs: vec![text_tab()],
        default_tab_interval: Some(24.0),
        line_break_mode: Some(LineBreakMode::TruncatingTail),
        base_writing_direction: Some(WritingDirection::LeftToRight),
        ..ParagraphStyleOptions::default()
    })
    .expect("CTParagraphStyle")
}

pub fn attributed_string() -> AttributedString {
    AttributedString::new(SAMPLE_TEXT, &font(), Some(&paragraph_style()))
        .expect("NSAttributedString")
}

pub fn typesetter() -> CTTypesetter {
    CTTypesetter::create_with_options(
        &attributed_string(),
        TypesetterOptions {
            allow_unbounded_layout: true,
            forced_embedding_level: Some(0),
        },
    )
    .expect("CTTypesetter")
}

pub fn line() -> CTLine {
    let typesetter = typesetter();
    let break_index = typesetter.suggest_line_break(0, 220.0);
    typesetter
        .create_line(TextRange::new(0, break_index))
        .expect("CTLine")
}

pub fn framesetter() -> CTFramesetter {
    CTFramesetter::create_with_attributed_string(&attributed_string()).expect("CTFramesetter")
}

pub fn frame() -> CTFrame {
    let framesetter = framesetter();
    let (size, fit_range) =
        framesetter.suggest_frame_size_with_constraints(CGSize::new(220.0, f64::INFINITY));
    framesetter
        .create_frame_in_rect(CGRect::new(0.0, 0.0, size.width + 24.0, 120.0), fit_range)
        .expect("CTFrame")
}

pub fn first_font_url() -> String {
    FontManager::available_font_urls()
        .expect("font urls")
        .into_iter()
        .find(|value| Path::new(value).exists())
        .expect("expected at least one installed font URL")
}

pub fn first_glyph_for(text: &str) -> u16 {
    font()
        .glyphs_for_string(text)
        .expect("glyphs for string")
        .into_iter()
        .find(|glyph| *glyph != 0)
        .expect("expected at least one glyph")
}
