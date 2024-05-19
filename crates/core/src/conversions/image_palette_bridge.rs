use image::Rgba;
use palette::{LinSrgba, Srgba};

#[inline(always)]
pub fn lin_srgba_to_rgba(lin_srgba: LinSrgba) -> Rgba<u8> {
    let srgba: Srgba = lin_srgba.into();
    srgba_to_rgba(srgba)
}

#[inline(always)]
pub fn srgba_to_rgba(srgba: Srgba) -> Rgba<u8> {
    let r = (srgba.red * 255.0).round() as u8;
    let g = (srgba.green * 255.0).round() as u8;
    let b = (srgba.blue * 255.0).round() as u8;
    let a = (srgba.alpha * 255.0).round() as u8;

    Rgba([r, g, b, a])
}

#[inline(always)]
pub fn rgba_to_lin_srgba(rgba: Rgba<u8>) -> LinSrgba {
    let srgba = rgba_to_srgba(rgba);
    srgba.into_linear()
}

#[inline(always)]
pub fn rgba_to_srgba(rgba: Rgba<u8>) -> Srgba {
    let r = rgba[0] as f32 / 255.0;
    let g = rgba[1] as f32 / 255.0;
    let b = rgba[2] as f32 / 255.0;
    let a = rgba[3] as f32 / 255.0;

    Srgba::new(r, g, b, a)
}
