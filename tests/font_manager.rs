mod support;

use std::fs;
use std::path::PathBuf;

use coretext::{AutoActivationSetting, CoreTextError, FontManager, FontManagerScope};

fn missing_font() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("coretext-rs-missing-font.ttf")
}

#[test]
fn font_manager_listing_and_lookup() -> Result<(), Box<dyn std::error::Error>> {
    assert!(!FontManager::available_postscript_names()?.is_empty());
    assert!(!FontManager::available_font_family_names()?.is_empty());
    assert!(!FontManager::available_font_urls()?.is_empty());

    let font_url = support::first_font_url();
    assert!(FontManager::is_supported_font(&font_url)?);
    assert!(!FontManager::font_descriptors_from_url(&font_url)?.is_empty());

    let manifest = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
    assert!(!FontManager::is_supported_font(manifest)?);
    assert!(FontManager::font_descriptors_from_url(manifest)?.is_empty());
    assert_eq!(
        FontManager::scope_for_url(manifest)?,
        FontManagerScope::None
    );
    assert_eq!(
        FontManager::auto_activation_setting(Some("fish.doom.coretext-rs.unregistered-bundle")),
        AutoActivationSetting::Default
    );

    let data = fs::read(&font_url)?;
    assert!(FontManager::font_descriptor_from_data(&data)?
        .family_name()
        .is_some());
    assert!(!FontManager::font_descriptors_from_data(&data).is_empty());
    assert!(FontManager::font_descriptors_from_data(b"not a font").is_empty());
    Ok(())
}

#[test]
fn registering_a_missing_font_reports_an_error() {
    let missing = missing_font();
    assert!(!missing.exists());
    let results = [
        FontManager::register_fonts_for_url(&missing, FontManagerScope::Process),
        FontManager::register_font_urls(&[&missing], FontManagerScope::Process, true),
        FontManager::register_fonts_for_urls(&[&missing], FontManagerScope::Process),
        FontManager::unregister_font_urls(&[&missing], FontManagerScope::Process),
        FontManager::unregister_fonts_for_urls(&[&missing], FontManagerScope::Process),
    ];
    for result in results {
        match result {
            Err(CoreTextError::Bridge(message)) => assert!(!message.is_empty()),
            other => panic!("expected a font manager error, got {other:?}"),
        }
    }
}

#[test]
fn empty_registration_requests_succeed_without_calling_core_text() {
    let none: [&str; 0] = [];
    assert!(FontManager::register_font_urls(&none, FontManagerScope::Process, true).is_ok());
    assert!(FontManager::unregister_font_urls(&none, FontManagerScope::Process).is_ok());
    assert!(FontManager::register_font_descriptors(&[], FontManagerScope::Process, true).is_ok());
    assert!(FontManager::unregister_font_descriptors(&[], FontManagerScope::Process).is_ok());
}

#[test]
#[ignore = "changes the enabled state of an installed font in the user's font registry"]
fn enable_font_descriptors_accepts_installed_fonts() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read(support::first_font_url())?;
    let descriptors = FontManager::font_descriptors_from_data(&data);
    FontManager::enable_font_descriptors(&descriptors[..1], true);
    Ok(())
}
