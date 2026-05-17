use coretext::{ruby_annotation_type_id, RubyAlignment, RubyAnnotation, RubyOverhang, RubyPosition};

#[test]
fn ruby_annotation_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let annotation = RubyAnnotation::new(
        RubyAlignment::Center,
        RubyOverhang::Auto,
        0.5,
        [Some("ふ"), Some("ご"), None, Some("inline")],
    )?;

    assert_eq!(annotation.alignment(), RubyAlignment::Center);
    assert_eq!(annotation.overhang(), RubyOverhang::Auto);
    assert!((annotation.size_factor() - 0.5).abs() < f64::EPSILON);
    assert_eq!(
        annotation
            .text_for_position(RubyPosition::Before)
            .as_deref(),
        Some("ふ")
    );
    assert_eq!(
        annotation.text_for_position(RubyPosition::After).as_deref(),
        Some("ご")
    );
    assert_eq!(
        annotation
            .copy()?
            .text_for_position(RubyPosition::Inline)
            .as_deref(),
        Some("inline")
    );

    let attributed = RubyAnnotation::with_attributes(
        RubyAlignment::Center,
        RubyOverhang::Auto,
        0.5,
        [Some("ふ"), Some("ご"), None, Some("inline")],
    )?;
    assert_eq!(attributed.alignment(), RubyAlignment::Center);
    assert_eq!(
        attributed.text_for_position(RubyPosition::Before).as_deref(),
        Some("ふ")
    );
    assert!(ruby_annotation_type_id() > 0);
    Ok(())
}
