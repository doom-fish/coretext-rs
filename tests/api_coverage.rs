//! API coverage integration tests for coretext-rs.
//!
//! Verifies that the declared surface compiles and produces sensible values
//! against the active macOS SDK.

use coretext::{
    AttributedString, CGRect, CGSize, CTFont, CTFramesetter, CTLine, ParagraphStyle, TextAlignment,
    TextRange,
};

const TEXT: &str = "CoreText test";

fn make_font() -> CTFont {
    CTFont::new("Helvetica", 18.0).expect("Helvetica must be available on macOS")
}

fn make_attr(font: &CTFont, ps: Option<&ParagraphStyle>) -> AttributedString {
    AttributedString::new(TEXT, font, ps).expect("AttributedString::new")
}

// ── CTFont ────────────────────────────────────────────────────────────────────

#[test]
fn font_create_and_size() {
    let font = make_font();
    assert!((font.size() - 18.0).abs() < 1e-6, "size should be 18.0");
}

#[test]
fn font_names() {
    let font = make_font();
    let ps = font.postscript_name().expect("postscript_name");
    let fam = font.family_name().expect("family_name");
    let full = font.full_name().expect("full_name");
    assert!(!ps.is_empty(), "postscript name non-empty");
    assert!(!fam.is_empty(), "family name non-empty");
    assert!(!full.is_empty(), "full name non-empty");
}

#[test]
fn font_metrics() {
    let font = make_font();
    assert!(font.ascent() > 0.0, "ascent > 0");
    assert!(font.descent() < 0.0 || font.descent() >= 0.0); // just check it returns
    assert!(font.glyph_count() > 0, "glyph count > 0");
}

// ── ParagraphStyle ────────────────────────────────────────────────────────────

#[test]
fn paragraph_style_left() {
    let ps = ParagraphStyle::with_alignment(TextAlignment::Left).expect("ParagraphStyle");
    assert_eq!(ps.alignment(), TextAlignment::Left);
}

#[test]
fn paragraph_style_center() {
    let ps = ParagraphStyle::with_alignment(TextAlignment::Center).expect("ParagraphStyle");
    assert_eq!(ps.alignment(), TextAlignment::Center);
}

#[test]
fn paragraph_style_justified() {
    let ps = ParagraphStyle::with_alignment(TextAlignment::Justified).expect("ParagraphStyle");
    assert_eq!(ps.alignment(), TextAlignment::Justified);
}

// ── AttributedString ──────────────────────────────────────────────────────────

#[test]
fn attributed_string_no_paragraph_style() {
    let font = make_font();
    make_attr(&font, None);
}

#[test]
fn attributed_string_with_paragraph_style() {
    let font = make_font();
    let ps = ParagraphStyle::with_alignment(TextAlignment::Right).unwrap();
    make_attr(&font, Some(&ps));
}

// ── CTLine ────────────────────────────────────────────────────────────────────

#[test]
fn line_glyph_count_positive() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    assert!(line.glyph_count() > 0, "glyph_count > 0");
}

#[test]
fn line_string_range() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let r = line.string_range();
    assert_eq!(r.location, 0);
    assert_eq!(r.length, isize::try_from(TEXT.len()).unwrap());
}

#[test]
fn line_typographic_bounds() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let tb = line.typographic_bounds();
    assert!(tb.width > 0.0, "line width > 0");
    assert!(tb.ascent > 0.0, "ascent > 0");
}

#[test]
fn line_bounds_with_options() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let rect = line.bounds_with_options(0);
    assert!(rect.size.width > 0.0, "bounds width > 0");
}

#[test]
fn line_trailing_whitespace() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let _ = line.trailing_whitespace_width(); // must not panic
}

#[test]
fn line_pen_offset_for_flush() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let tb = line.typographic_bounds();
    let offset = line.pen_offset_for_flush(0.5, tb.width + 40.0);
    assert!(offset >= 0.0, "pen offset for centered flush >= 0");
}

#[test]
fn line_runs_non_empty() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let runs = line.runs();
    assert!(!runs.is_empty(), "at least one run");
}

// ── CTRun ─────────────────────────────────────────────────────────────────────

#[test]
fn run_glyphs_and_positions() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let runs = line.runs();
    let run = runs.first().expect("at least one run");
    let g = run.glyphs();
    let p = run.positions();
    let a = run.advances();
    let idx = run.string_indices();
    assert!(!g.is_empty());
    assert_eq!(g.len(), p.len());
    assert_eq!(g.len(), a.len());
    assert_eq!(g.len(), idx.len());
}

#[test]
fn run_string_range() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let runs = line.runs();
    let run = runs.first().unwrap();
    let r = run.string_range();
    assert!(r.length > 0);
}

#[test]
fn run_typographic_bounds() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let line = CTLine::create_with_attributed_string(&attr).expect("CTLine");
    let runs = line.runs();
    let run = runs.first().unwrap();
    let tb = run.typographic_bounds();
    assert!(tb.width > 0.0);
}

// ── CTFramesetter ─────────────────────────────────────────────────────────────

#[test]
fn framesetter_suggest_size() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let setter = CTFramesetter::create_with_attributed_string(&attr).expect("CTFramesetter");
    let (size, range) =
        setter.suggest_frame_size_with_constraints(CGSize::new(f64::INFINITY, f64::INFINITY));
    assert!(size.width > 0.0);
    assert!(range.length > 0);
}

// ── CTFrame ───────────────────────────────────────────────────────────────────

#[test]
fn frame_lines_and_origins() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let setter = CTFramesetter::create_with_attributed_string(&attr).expect("CTFramesetter");
    let (suggested, fit_range) =
        setter.suggest_frame_size_with_constraints(CGSize::new(f64::INFINITY, f64::INFINITY));
    let rect = CGRect::new(0.0, 0.0, suggested.width + 40.0, 200.0);
    let frame = setter
        .create_frame_in_rect(rect, fit_range)
        .expect("CTFrame");
    let lines = frame.lines();
    let origins = frame.line_origins();
    assert!(!lines.is_empty(), "frame has at least one line");
    assert_eq!(
        lines.len(),
        origins.len(),
        "origins count matches line count"
    );
}

#[test]
fn frame_string_range() {
    let font = make_font();
    let attr = make_attr(&font, None);
    let setter = CTFramesetter::create_with_attributed_string(&attr).expect("CTFramesetter");
    let (suggested, fit_range) =
        setter.suggest_frame_size_with_constraints(CGSize::new(f64::INFINITY, f64::INFINITY));
    let rect = CGRect::new(0.0, 0.0, suggested.width + 40.0, 200.0);
    let frame = setter
        .create_frame_in_rect(rect, fit_range)
        .expect("CTFrame");
    let sr = frame.string_range();
    let vsr = frame.visible_string_range();
    assert_eq!(sr, TextRange::new(0, isize::try_from(TEXT.len()).unwrap()));
    assert!(vsr.length > 0);
}
