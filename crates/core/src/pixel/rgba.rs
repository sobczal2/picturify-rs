use image::Rgba;

use crate::pixel::traits::{RgbaF32Pixel, RgbaU8Pixel};

impl RgbaU8Pixel for Rgba<u8> {
    #[inline(always)]
    fn red_u8(&self) -> u8 {
        self.0[0]
    }
    #[inline(always)]
    fn green_u8(&self) -> u8 {
        self.0[1]
    }
    #[inline(always)]
    fn blue_u8(&self) -> u8 {
        self.0[2]
    }
    #[inline(always)]
    fn alpha_u8(&self) -> u8 {
        self.0[3]
    }
    #[inline(always)]
    fn set_red_u8(&mut self, value: u8) {
        self.0[0] = value;
    }
    #[inline(always)]
    fn set_green_u8(&mut self, value: u8) {
        self.0[1] = value;
    }
    #[inline(always)]
    fn set_blue_u8(&mut self, value: u8) {
        self.0[2] = value;
    }
    #[inline(always)]
    fn set_alpha_u8(&mut self, value: u8) {
        self.0[3] = value;
    }
}

impl RgbaF32Pixel for Rgba<u8> {
    #[inline(always)]
    fn red_f32(&self) -> f32 {
        self.0[0] as f32 / 255.0
    }
    #[inline(always)]
    fn green_f32(&self) -> f32 {
        self.0[1] as f32 / 255.0
    }
    #[inline(always)]
    fn blue_f32(&self) -> f32 {
        self.0[2] as f32 / 255.0
    }
    #[inline(always)]
    fn alpha_f32(&self) -> f32 {
        self.0[3] as f32 / 255.0
    }
    #[inline(always)]
    fn set_red_f32(&mut self, value: f32) {
        self.0[0] = (value * 255.0) as u8;
    }
    #[inline(always)]
    fn set_green_f32(&mut self, value: f32) {
        self.0[1] = (value * 255.0) as u8;
    }
    #[inline(always)]
    fn set_blue_f32(&mut self, value: f32) {
        self.0[2] = (value * 255.0) as u8;
    }
    #[inline(always)]
    fn set_alpha_f32(&mut self, value: f32) {
        self.0[3] = (value * 255.0) as u8;
    }
}
