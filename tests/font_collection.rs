mod support;

use coretext::{FontCollection, FontCollectionOptions};

#[test]
fn font_collection_queries() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = support::font().descriptor()?;
    let options = FontCollectionOptions {
        remove_duplicates: true,
        include_disabled_fonts: false,
        disallow_auto_activation: false,
    };

    let collection = FontCollection::with_descriptors(std::slice::from_ref(&descriptor), options)?;
    assert_eq!(collection.query_descriptors().len(), 1);
    assert!(!collection.matching_descriptors().is_empty());

    let available = FontCollection::available()?;
    assert!(!available.matching_descriptors().is_empty());

    let copy = available.copy_with_descriptors(std::slice::from_ref(&descriptor), options)?;
    assert!(!copy.matching_descriptors().is_empty());

    let family_name = descriptor.family_name().expect("family name");
    assert!(!available
        .matching_descriptors_for_family(&family_name)?
        .is_empty());
    Ok(())
}
