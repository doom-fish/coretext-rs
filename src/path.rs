use crate::bridge;
use crate::common::{impl_handle, impl_thread_safe};
use crate::types::{CGPoint, CGRect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathElement {
    MoveTo(CGPoint),
    LineTo(CGPoint),
    QuadCurveTo {
        control: CGPoint,
        to: CGPoint,
    },
    CurveTo {
        control1: CGPoint,
        control2: CGPoint,
        to: CGPoint,
    },
    CloseSubpath,
}

pub struct GlyphPath {
    raw: bridge::Handle,
}

impl_handle!(GlyphPath);
impl_thread_safe!(GlyphPath);

impl GlyphPath {
    #[must_use]
    pub fn bounding_box(&self) -> CGRect {
        unsafe { bridge::ct_path_get_bounding_box(self.raw) }
    }

    #[must_use]
    pub fn path_bounding_box(&self) -> CGRect {
        unsafe { bridge::ct_path_get_path_bounding_box(self.raw) }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        unsafe { bridge::ct_path_is_empty(self.raw) }
    }

    #[must_use]
    pub fn elements(&self) -> Vec<PathElement> {
        let count = unsafe {
            bridge::ct_path_copy_elements(self.raw, std::ptr::null_mut(), std::ptr::null_mut(), 0)
        };
        let Ok(len) = usize::try_from(count) else {
            return Vec::new();
        };
        let Some(point_len) = len.checked_mul(3) else {
            return Vec::new();
        };
        if len == 0 {
            return Vec::new();
        }
        let mut kinds = vec![0_u8; len];
        let mut points = vec![CGPoint::default(); point_len];
        let written = unsafe {
            bridge::ct_path_copy_elements(self.raw, kinds.as_mut_ptr(), points.as_mut_ptr(), count)
        };
        let written = usize::try_from(written).unwrap_or(0).min(len);
        kinds
            .iter()
            .zip(points.chunks_exact(3))
            .take(written)
            .filter_map(|(kind, points)| match kind {
                0 => Some(PathElement::MoveTo(points[0])),
                1 => Some(PathElement::LineTo(points[0])),
                2 => Some(PathElement::QuadCurveTo {
                    control: points[0],
                    to: points[1],
                }),
                3 => Some(PathElement::CurveTo {
                    control1: points[0],
                    control2: points[1],
                    to: points[2],
                }),
                4 => Some(PathElement::CloseSubpath),
                _ => None,
            })
            .collect()
    }
}
