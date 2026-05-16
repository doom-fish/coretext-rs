mod support;

use coretext::{FontDescriptor, FontFormat, FontOrientation};

#[test]
fn descriptor_surface_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = FontDescriptor::new("Helvetica", 16.0)?;
    assert!((descriptor.size() - 16.0).abs() < f64::EPSILON);
    assert_eq!(descriptor.orientation(), FontOrientation::Default);
    assert!(descriptor.family_name().is_some());
    assert!(descriptor.display_name().is_some());
    assert!(descriptor.matching_descriptor().is_some());
    assert!(!descriptor.matching_descriptors().is_empty());
    let attributes = descriptor.attributes_json()?;
    assert!(attributes.is_object());
    assert!(matches!(
        descriptor.format(),
        FontFormat::Unrecognized
            | FontFormat::OpenTypePostScript
            | FontFormat::OpenTypeTrueType
            | FontFormat::TrueType
            | FontFormat::PostScript
            | FontFormat::Bitmap
    ));
    Ok(())
}

#[test]
fn descriptor_copy_helpers_and_metadata() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = support::font().descriptor()?;
    let family_name = descriptor.family_name().expect("family name");
    let family_copy = descriptor.with_family(&family_name)?;
    assert_eq!(
        family_copy.family_name().as_deref(),
        Some(family_name.as_str())
    );

    let symbolic_copy = descriptor.with_symbolic_traits(
        support::font().symbolic_traits(),
        support::font().symbolic_traits(),
    )?;
    assert!(symbolic_copy.attributes_json()?.is_object());

    let features = descriptor.features()?;
    let feature_settings = descriptor.feature_settings()?;
    let variation_axes = descriptor.variation_axes()?;
    let variation_coordinates = descriptor.variation_coordinates()?;
    let traits = descriptor.traits()?;
    let _ = traits.symbolic_traits;

    if let Some(feature) = features.first() {
        if let Some(selector) = feature.selectors.first() {
            let feature_copy =
                descriptor.with_feature(feature.type_identifier, selector.identifier)?;
            assert!(feature_copy.attributes_json()?.is_object());
        }
    }

    if let Some(axis) = variation_axes.first() {
        let variation_copy = descriptor.with_variation(axis.identifier, axis.default_value)?;
        assert!(variation_copy.attributes_json()?.is_object());
    }

    let _ = feature_settings;
    let _ = variation_coordinates;
    Ok(())
}
