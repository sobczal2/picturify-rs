use palette::LinSrgba;

use crate::pixel::traits::{RgbaF32Pixel, RgbaU8Pixel};

impl RgbaU8Pixel for LinSrgba {
    #[inline(always)]
    fn red_u8(&self) -> u8 {
        (self.red * 255.0).round() as u8
    }
    #[inline(always)]
    fn green_u8(&self) -> u8 {
        (self.green * 255.0).round() as u8
    }
    #[inline(always)]
    fn blue_u8(&self) -> u8 {
        (self.blue * 255.0).round() as u8
    }
    #[inline(always)]
    fn alpha_u8(&self) -> u8 {
        (self.alpha * 255.0).round() as u8
    }
    #[inline(always)]
    fn set_red_u8(&mut self, value: u8) {
        self.red = value as f32 / 255.0;
    }
    #[inline(always)]
    fn set_green_u8(&mut self, value: u8) {
        self.green = value as f32 / 255.0;
    }
    #[inline(always)]
    fn set_blue_u8(&mut self, value: u8) {
        self.blue = value as f32 / 255.0;
    }
    #[inline(always)]
    fn set_alpha_u8(&mut self, value: u8) {
        self.alpha = value as f32 / 255.0;
    }
}

impl RgbaF32Pixel for LinSrgba {
    #[inline(always)]
    fn red_f32(&self) -> f32 {
        self.red
    }
    #[inline(always)]
    fn green_f32(&self) -> f32 {
        self.green
    }
    #[inline(always)]
    fn blue_f32(&self) -> f32 {
        self.blue
    }
    #[inline(always)]
    fn alpha_f32(&self) -> f32 {
        self.alpha
    }
    #[inline(always)]
    fn set_red_f32(&mut self, value: f32) {
        self.red = value;
    }
    #[inline(always)]
    fn set_green_f32(&mut self, value: f32) {
        self.green = value;
    }
    #[inline(always)]
    fn set_blue_f32(&mut self, value: f32) {
        self.blue = value;
    }
    #[inline(always)]
    fn set_alpha_f32(&mut self, value: f32) {
        self.alpha = value;
    }
}
