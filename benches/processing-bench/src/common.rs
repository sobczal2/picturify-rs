use std::fmt::Display;
use picturify_core::fast_image::FastImage;

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
    pub fn get_resolution(&self) -> (usize, usize) {
        match self {
            ImageResolution::P240 => (320, 240),
            ImageResolution::P480 => (640, 480),
            ImageResolution::P720 => (1280, 720),
        }
    }
    
    pub fn get_image(&self) -> FastImage {
        let (width, height) = self.get_resolution();
        FastImage::empty(width, height)
    }
    
    pub fn get_resolutions() -> Vec<ImageResolution> {
        vec![ImageResolution::P240, ImageResolution::P480, ImageResolution::P720]
    }
}