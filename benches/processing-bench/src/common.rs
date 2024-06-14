use std::fmt::Display;

use picturify_core::core::fast_image::FastImage;
use picturify_core::geometry::size::Size;

#[derive(Debug, Clone, Copy)]
pub enum ImageResolution {
    P240,
    P480,
    P720,
}

impl Display for ImageResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageResolution::P240 => write!(f, "240P"),
            ImageResolution::P480 => write!(f, "480P"),
            ImageResolution::P720 => write!(f, "720P"),
        }
    }
}

impl ImageResolution {
    pub fn get_resolution(&self) -> Size {
        match self {
            ImageResolution::P240 => (320, 240).into(),
            ImageResolution::P480 => (640, 480).into(),
            ImageResolution::P720 => (1280, 720).into(),
        }
    }

    pub fn get_image(&self) -> FastImage {
        FastImage::empty(self.get_resolution())
    }

    pub fn get_resolutions() -> Vec<ImageResolution> {
        vec![
            ImageResolution::P240,
            ImageResolution::P480,
            ImageResolution::P720,
        ]
    }
}
