use image::{DynamicImage, GenericImage, GenericImageView};
use colors::hsl_conversions::{hsl_to_rgb, rgb_to_hsl};
use colors::hsv_conversions::{hsv_to_rgb, rgb_to_hsv};

pub  trait VirtualImage {
    fn get_alpha(&self, x: u32, y: u32) -> u8;
    fn set_alpha(&mut self, x: u32, y: u32, alpha: u8);
}

impl VirtualImage for image::DynamicImage {
    fn get_alpha(&self, x: u32, y: u32) -> u8 {
        let pixel = self.get_pixel(x, y);
        pixel[3]
    }

    fn set_alpha(&mut self, x: u32, y: u32, alpha: u8) {
        let mut pixel = self.get_pixel(x, y);
        pixel[3] = alpha as u8;
        self.put_pixel(x, y, pixel);
    }
}

pub  trait VirtualHSLImage {
    fn get_hsl(&self, x: u32, y: u32) -> (f32, f32, f32);
    fn set_hsl(&mut self, hue: f32, saturation: f32, lightness: f32, x: u32, y: u32);
}

impl VirtualHSLImage for DynamicImage {
    fn get_hsl(&self, x: u32, y: u32) -> (f32, f32, f32) {
        let pixel = self.get_pixel(x, y);
        let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);
        rgb_to_hsl(red, green, blue)
    }

    fn set_hsl(&mut self, hue: f32, saturation: f32, lightness: f32, x: u32, y: u32) {
        let (red, green, blue) = hsl_to_rgb(hue, saturation, lightness);
        let pixel = self.get_pixel(x, y);
        let new_pixel = image::Rgba([red, green, blue, pixel[3]]);
        self.put_pixel(x, y, new_pixel);
    }
}

pub  trait VirtualRGBImage {
    fn get_rgb(&self, x: u32, y: u32) -> (u8, u8, u8);
    fn set_rgb(&mut self, x: u32, y: u32, red: u8, green: u8, blue: u8);
}

impl VirtualRGBImage for DynamicImage {
    fn get_rgb(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let pixel = self.get_pixel(x, y);
        (pixel[0], pixel[1], pixel[2])
    }

    fn set_rgb(&mut self, x: u32, y: u32, red: u8, green: u8, blue: u8) {
        let pixel = self.get_pixel(x, y);
        let new_pixel = image::Rgba([red, green, blue, pixel[3]]);
        self.put_pixel(x, y, new_pixel);
    }
}

pub  trait VirtualHSVImage {
    fn get_hsv(&self, x: u32, y: u32) -> (f32, f32, f32);
    fn set_hsv(&mut self, x: u32, y: u32, hue: f32, saturation: f32, value: f32);
}

impl VirtualHSVImage for DynamicImage {
    fn get_hsv(&self, x: u32, y: u32) -> (f32, f32, f32) {
        let pixel = self.get_pixel(x, y);
        let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);
        let (hue, saturation, value) = rgb_to_hsv(red, green, blue);
        (hue, saturation, value)
    }

    fn set_hsv(&mut self, x: u32, y: u32, hue: f32, saturation: f32, value: f32) {
        let (red, green, blue) = hsv_to_rgb(hue, saturation, value);
        let pixel = self.get_pixel(x, y);
        let new_pixel = image::Rgba([red, green, blue, pixel[3]]);
        self.put_pixel(x, y, new_pixel);
    }
}