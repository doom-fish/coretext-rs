mod support;

use coretext::{font_collection_type_id, FontCollection, FontCollectionOptions, FontDescriptor};

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
    let attrs = attrs
        .as_array()
        .expect("one attribute dictionary per matching font");
    assert!(!attrs.is_empty());
    assert!(attrs.iter().all(serde_json::Value::is_object));
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

    let mut mutable = available.mutable_copy()?;
    mutable.set_query_descriptors(std::slice::from_ref(&descriptor));
    mutable.set_exclusion_descriptors(&[]);
    assert_eq!(mutable.as_font_collection().query_descriptors().len(), 1);
    Ok(())
}

#[test]
fn mutable_collections_are_uniquely_owned() -> Result<(), Box<dyn std::error::Error>> {
    let helvetica = FontDescriptor::new("Helvetica", 12.0)?;
    let times = FontDescriptor::new("Times-Roman", 12.0)?;
    let base = FontCollection::with_descriptors(
        &[helvetica.clone(), times.clone()],
        FontCollectionOptions::default(),
    )?;

    let mut mutable = base.mutable_copy()?;
    mutable.set_exclusion_descriptors(std::slice::from_ref(&times));
    let snapshot = mutable.as_font_collection();
    assert_eq!(snapshot.query_descriptors().len(), 2);
    assert_eq!(snapshot.exclusion_descriptors().len(), 1);

    let mut copy = mutable.clone();
    copy.set_query_descriptors(std::slice::from_ref(&helvetica));
    copy.set_exclusion_descriptors(&[]);
    assert_eq!(copy.as_font_collection().query_descriptors().len(), 1);
    assert!(copy.as_font_collection().exclusion_descriptors().is_empty());
    assert_eq!(mutable.as_font_collection().query_descriptors().len(), 2);
    assert_eq!(
        mutable.as_font_collection().exclusion_descriptors().len(),
        1
    );

    mutable.set_query_descriptors(std::slice::from_ref(&times));
    assert_eq!(snapshot.query_descriptors().len(), 2);
    assert_eq!(snapshot.exclusion_descriptors().len(), 1);

    let frozen = mutable.into_font_collection();
    assert_eq!(frozen.query_descriptors().len(), 1);
    assert_eq!(frozen.exclusion_descriptors().len(), 1);
    assert_eq!(base.query_descriptors().len(), 2);
    assert!(base.exclusion_descriptors().is_empty());
    Ok(())
}
