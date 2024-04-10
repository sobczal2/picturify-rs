use image::Rgba;
use palette::Srgba;

#[inline(always)]
pub fn cord_1d_to_2d(cord: usize, width: usize) -> (usize, usize) {
    (cord % width, cord / width)
}

#[inline(always)]
pub fn cord_2d_to_1d(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

#[inline(always)]
pub fn image_rgba_to_palette_srgba(image: Rgba<u8>) -> Srgba {
    Srgba::new(
        image[0] as f32 / 255.0,
        image[1] as f32 / 255.0,
        image[2] as f32 / 255.0,
        image[3] as f32 / 255.0,
    )
}

#[inline(always)]
pub fn palette_srgba_to_image_rgba(palette: Srgba) -> Rgba<u8> {
    Rgba([
        (palette.red * 255.0) as u8,
        (palette.green * 255.0) as u8,
        (palette.blue * 255.0) as u8,
        (palette.alpha * 255.0) as u8,
    ])
}