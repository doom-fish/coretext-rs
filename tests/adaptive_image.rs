use apple_cf::cg::CGContext;
use coretext::{
    AdaptiveImageProvider, AdaptiveImageProviding, AdaptiveImageResponse, CGPoint, CGSize, CTFont,
};

struct SolidAdaptiveImage;

impl AdaptiveImageProviding for SolidAdaptiveImage {
    fn image_for_proposed_size(
        &self,
        _proposed_size: CGSize,
        _scale_factor: f64,
    ) -> Option<AdaptiveImageResponse> {
        let image_size = 12.0;
        let context = CGContext::new_rgba8(12, 12).ok()?;
        context.set_rgb_fill_color(1.0, 0.0, 0.0, 1.0);
        context.fill_rect(0.0, 0.0, image_size, image_size);
        let image = context.snapshot_to_image()?;
        Some(AdaptiveImageResponse {
            image,
            image_offset: CGPoint::new(0.0, 0.0),
            image_size: CGSize::new(image_size, image_size),
        })
    }
}

#[test]
fn adaptive_image_provider_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let font = CTFont::new("Helvetica", 18.0)?;
    let provider = AdaptiveImageProvider::new(SolidAdaptiveImage)?;

    let bounds = font.typographic_bounds_for_adaptive_image_provider(Some(&provider));
    assert!(bounds.size.width >= 0.0);
    assert!(bounds.size.height >= 0.0);

    let context = CGContext::new_rgba8(48, 48)?;
    font.draw_image_from_adaptive_image_provider_at_point(
        &provider,
        CGPoint::new(4.0, 20.0),
        &context,
    );
    assert!(context.as_bytes().iter().any(|value| *value != 0));
    Ok(())
}
