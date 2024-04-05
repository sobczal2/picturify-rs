use std::path::Path;

use image::image::Image;
use pixel::color::ColorChannel;
use pixel::pixel::{Pixel, PixelValueError, RGBAPixelValue};

#[derive(Debug)]
pub enum IOImageError {
    ImageError(image::ImageError),
    PixelValueError(PixelValueError),
}

pub fn read_image_from_file(file_path: &Path) -> Result<Image, IOImageError> {
    let img = image::open(file_path).unwrap();
    let img = img.to_rgba32f();
    let (width, height) = img.dimensions();
    let mut pixels = Vec::new();
    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];
            row.push(
                Pixel::RGBA(
                    RGBAPixelValue::new(r, g, b, a)
                        .map_err(|e| IOImageError::PixelValueError(e))?
                )
            );
        }
        pixels.push(row);
    }
    Ok(Image::from_2d_vec(pixels))
}

pub fn write_image_to_file(image: &Image, file_path: &Path) -> Result<(), IOImageError> {
    let width = image.size().width as usize;
    let height = image.size().height as usize;
    let mut img = image::RgbaImage::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let r = (pixel.get(ColorChannel::Red) * 255.0) as u8;
            let g = (pixel.get(ColorChannel::Green) * 255.0) as u8;
            let b = (pixel.get(ColorChannel::Blue) * 255.0) as u8;
            let a = (pixel.get(ColorChannel::Alpha) * 255.0) as u8;
            img.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
        }
    }
    img.save(file_path).map_err(|e| IOImageError::ImageError(e))?;
    Ok(())
}
