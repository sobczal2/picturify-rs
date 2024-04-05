use pixel::color::ColorSpace;
use crate::common::size::Size2dI32;
use crate::pixel::pixel::{Pixel, RGBPixelValue};

pub struct Image {
    pixels: Vec<Vec<Pixel>>,
    size: Size2dI32,
}

impl Image {
    pub fn new(size: Size2dI32) -> Image {
        let pixels = vec![vec![Pixel::RGB(RGBPixelValue::default()); size.width as usize]; size.height as usize];
        Image {
            pixels,
            size,
        }
    }

    pub fn new_with_pixel(size: Size2dI32, pixel: Pixel) -> Image {
        let pixels = vec![vec![pixel; size.width as usize]; size.height as usize];
        Image {
            pixels,
            size,
        }
    }

    pub fn from_2d_vec(pixels: Vec<Vec<Pixel>>) -> Image {
        Image {
            size: Size2dI32 {
                width: pixels[0].len() as i32,
                height: pixels.len() as i32,
            },
            pixels,
        }
    }

    pub fn from_1d_vec(pixels: Vec<Pixel>, size: Size2dI32) -> Image {
        let mut image = Image::new(size);
        for y in 0..size.height {
            for x in 0..size.width {
                image.set_pixel(x as usize, y as usize, pixels[(y * size.width + x) as usize]);
            }
        }
        image
    }

    pub fn from_image(image: &Image) -> Image {
        Image {
            size: image.size,
            pixels: image.pixels.clone(),
        }
    }


    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.pixels[y][x]
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.pixels[y][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y][x] = pixel;
    }

    pub fn size(&self) -> Size2dI32 {
        self.size
    }
    
    pub  fn change_color_space(&mut self, color_space: ColorSpace) {
        for y in 0..self.size.height as usize {
            for x in 0..self.size.width as usize {
                let pixel = self.get_pixel_mut(x, y);
                *pixel = pixel.to_color_space(color_space);
            }
        }
    }
}
