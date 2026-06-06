use core::{ffi::c_void, mem, ptr};

use apple_cf::cg::CGImage;

use crate::bridge;
use crate::error::{CoreTextError, CoreTextResult};
use crate::types::{CGPoint, CGSize};

/// Result produced by an [`AdaptiveImageProviding`] callback.
#[derive(Debug)]
pub struct AdaptiveImageResponse {
    /// Image to draw for the requested glyph.
    pub image: CGImage,
    /// Offset of the image relative to the proposed layout rect.
    pub image_offset: CGPoint,
    /// Size of the image in points.
    pub image_size: CGSize,
}

/// Rust callback trait mirroring CoreText's `CTAdaptiveImageProviding` protocol.
pub trait AdaptiveImageProviding: Send + Sync {
    /// Returns the image that should be drawn for the requested size and scale.
    fn image_for_proposed_size(
        &self,
        proposed_size: CGSize,
        scale_factor: f64,
    ) -> Option<AdaptiveImageResponse>;
}

/// Owned bridge object that conforms to CoreText's `CTAdaptiveImageProviding` protocol.
pub struct AdaptiveImageProvider {
    raw: bridge::Handle,
}

impl Drop for AdaptiveImageProvider {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { bridge::ct_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl AdaptiveImageProvider {
    /// Creates a bridged adaptive-image provider backed by a Rust callback trait.
    pub fn new<P>(provider: P) -> CoreTextResult<Self>
    where
        P: AdaptiveImageProviding + 'static,
    {
        let boxed: Box<Box<dyn AdaptiveImageProviding>> = Box::new(Box::new(provider));
        let refcon = Box::into_raw(boxed).cast::<c_void>();
        let raw = unsafe {
            bridge::ct_adaptive_image_provider_create(
                refcon,
                adaptive_image_callback,
                adaptive_image_release,
            )
        };
        if raw.is_null() {
            unsafe {
                drop(Box::from_raw(refcon.cast::<Box<dyn AdaptiveImageProviding>>()));
            }
            return Err(CoreTextError::Bridge(
                "ct_adaptive_image_provider_create returned NULL".to_string(),
            ));
        }
        Ok(Self { raw })
    }

    pub(crate) const fn as_raw(&self) -> bridge::Handle {
        self.raw
    }
}

unsafe extern "C" fn adaptive_image_callback(
    refcon: *mut c_void,
    proposed_size: CGSize,
    scale_factor: f64,
    out_image_offset: *mut CGPoint,
    out_image_size: *mut CGSize,
) -> *mut c_void {
    let provider = unsafe { &*refcon.cast::<Box<dyn AdaptiveImageProviding>>() };
    // A panic unwinding across the C ABI into CoreText is undefined behaviour;
    // contain any panic from the user trait method and report "no image".
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let Some(response) = provider
            .as_ref()
            .image_for_proposed_size(proposed_size, scale_factor)
        else {
            return ptr::null_mut();
        };

        if !out_image_offset.is_null() {
            unsafe { *out_image_offset = response.image_offset };
        }
        if !out_image_size.is_null() {
            unsafe { *out_image_size = response.image_size };
        }

        let image = response.image;
        let raw = image.as_ptr();
        mem::forget(image);
        raw
    }))
    .unwrap_or(ptr::null_mut())
}

unsafe extern "C" fn adaptive_image_release(refcon: *mut c_void) {
    if !refcon.is_null() {
        // This callback is invoked from the Swift bridge's `deinit` across the C
        // ABI; a panic in the user trait's `Drop` unwinding into Swift is
        // undefined behaviour, so contain it here (mirrors adaptive_image_callback).
        let boxed = unsafe { Box::from_raw(refcon.cast::<Box<dyn AdaptiveImageProviding>>()) };
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(boxed)));
    }
}
