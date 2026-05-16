//! Ruby annotation example.
use coretext::{RubyAlignment, RubyAnnotation, RubyOverhang, RubyPosition};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ruby = RubyAnnotation::new(
        RubyAlignment::Center,
        RubyOverhang::Auto,
        0.5,
        [Some("ふ"), Some("ご"), None, Some("inline")],
    )?;
    println!(
        "alignment={:?} overhang={:?} before={:?} after={:?} inline={:?}",
        ruby.alignment(),
        ruby.overhang(),
        ruby.text_for_position(RubyPosition::Before),
        ruby.text_for_position(RubyPosition::After),
        ruby.text_for_position(RubyPosition::Inline),
    );
    Ok(())
}
