use palette::{LinSrgba, Srgba};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct SerializablePixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<SerializablePixel> for image::Rgba<u8> {
    fn from(pixel: SerializablePixel) -> Self {
        Self {
            0: [pixel.r, pixel.g, pixel.b, pixel.a],
        }
    }
}

impl From<image::Rgba<u8>> for SerializablePixel {
    fn from(pixel: image::Rgba<u8>) -> Self {
        Self {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
            a: pixel[3],
        }
    }
}

impl From<Srgba> for SerializablePixel {
    fn from(pixel: Srgba) -> Self {
        Self {
            r: (pixel.red * 255.0) as u8,
            g: (pixel.green * 255.0) as u8,
            b: (pixel.blue * 255.0) as u8,
            a: (pixel.alpha * 255.0) as u8,
        }
    }
}

impl From<SerializablePixel> for Srgba {
    fn from(pixel: SerializablePixel) -> Self {
        Self::new(
            pixel.r as f32 / 255.0,
            pixel.g as f32 / 255.0,
            pixel.b as f32 / 255.0,
            pixel.a as f32 / 255.0,
        )
    }
}

impl From<LinSrgba> for SerializablePixel {
    fn from(pixel: LinSrgba) -> Self {
        let srgba: Srgba = pixel.into();
        srgba.into()
    }
}

impl From<SerializablePixel> for LinSrgba {
    fn from(pixel: SerializablePixel) -> Self {
        let srgba: Srgba = pixel.into();
        srgba.into()
    }
}

impl SerializablePixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
