mod support;

use coretext::{font_collection_type_id, FontCollection, FontCollectionOptions};

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
    assert!(collection.exclusion_descriptors().is_empty());
    assert!(!collection.font_attribute_json("familyName")?.is_null());
    let attrs = collection.font_attributes_json(&["familyName", "name"])?;
    assert!(attrs.is_array() || attrs.is_object());
    assert!(!collection.matching_descriptors_with_options(options).is_empty());
    assert!(font_collection_type_id() > 0);

    let available = FontCollection::available()?;
    assert!(!available.matching_descriptors().is_empty());

    let copy = available.copy_with_descriptors(std::slice::from_ref(&descriptor), options)?;
    assert!(!copy.matching_descriptors().is_empty());

    let family_name = descriptor.family_name().expect("family name");
    assert!(!available
        .matching_descriptors_for_family(&family_name)?
        .is_empty());

    let mutable = available.mutable_copy()?;
    mutable.set_query_descriptors(std::slice::from_ref(&descriptor));
    mutable.set_exclusion_descriptors(&[]);
    Ok(())
}
