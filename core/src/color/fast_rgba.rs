use image::Rgba;

pub struct FastRgba {
    array: [f32; 4],
}

impl From<&mut Rgba<f32>> for FastRgba {
    fn from(rgba: &mut Rgba<f32>) -> FastRgba {
        FastRgba {
            array: rgba.0,
        }
    }
}