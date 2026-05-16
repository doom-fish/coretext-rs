mod support;

use coretext::{AutoActivationSetting, FontManager, FontManagerScope};

#[test]
fn font_manager_listing_and_lookup() -> Result<(), Box<dyn std::error::Error>> {
    assert!(!FontManager::available_postscript_names()?.is_empty());
    assert!(!FontManager::available_font_family_names()?.is_empty());
    assert!(!FontManager::available_font_urls()?.is_empty());

    let font_url = support::first_font_url();
    assert!(FontManager::is_supported_font(&font_url)?);
    assert!(!FontManager::font_descriptors_from_url(&font_url)?.is_empty());
    assert!(matches!(
        FontManager::scope_for_url(&font_url)?,
        FontManagerScope::None
            | FontManagerScope::Process
            | FontManagerScope::Persistent
            | FontManagerScope::Session
    ));
    assert!(matches!(
        FontManager::auto_activation_setting(None),
        AutoActivationSetting::Default
            | AutoActivationSetting::Disabled
            | AutoActivationSetting::Enabled
            | AutoActivationSetting::PromptUser
    ));
    Ok(())
}
