//! Descriptor, collection, and manager example.
use coretext::{FontCollection, FontCollectionOptions, FontDescriptor, FontManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = FontDescriptor::new("Helvetica", 16.0)?;
    println!(
        "descriptor family={:?} display={:?} matches={}",
        descriptor.family_name(),
        descriptor.display_name(),
        descriptor.matching_descriptors().len()
    );

    let collection = FontCollection::with_descriptors(
        std::slice::from_ref(&descriptor),
        FontCollectionOptions {
            remove_duplicates: true,
            include_disabled_fonts: false,
            disallow_auto_activation: false,
        },
    )?;
    println!(
        "collection query={} matching={}",
        collection.query_descriptors().len(),
        collection.matching_descriptors().len()
    );

    let urls = FontManager::available_font_urls()?;
    let path = urls.first().expect("installed font URL");
    println!(
        "manager postscript-names={} family-names={} path={} supported={} descriptors={}",
        FontManager::available_postscript_names()?.len(),
        FontManager::available_font_family_names()?.len(),
        path,
        FontManager::is_supported_font(path)?,
        FontManager::font_descriptors_from_url(path)?.len()
    );
    Ok(())
}
