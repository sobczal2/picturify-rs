use crate::pixel::traits::RgbaU8Pixel;
use image::Rgba;

pub trait PixelMath {
    fn add(&self, other: Self) -> Self;
    fn add_assign(&mut self, other: Self);
    fn sub(&self, other: Self) -> Self;
    fn sub_assign(&mut self, other: Self);
    fn add_scalar(&self, scalar: f32) -> Self;
    fn sub_scalar(&self, scalar: f32) -> Self;
    fn mul_scalar(&self, scalar: f32) -> Self;
    fn div_scalar(&self, scalar: f32) -> Self;
}

impl PixelMath for Rgba<u8> {
    fn add(&self, other: Self) -> Self {
        Rgba([
            self[0].saturating_add(other[0]),
            self[1].saturating_add(other[1]),
            self[2].saturating_add(other[2]),
            self[3].saturating_add(other[3]),
        ])
    }

    fn add_assign(&mut self, other: Self) {
        self.set_red_u8(self.red_u8().saturating_add(other.red_u8()));
        self.set_green_u8(self.green_u8().saturating_add(other.green_u8()));
        self.set_blue_u8(self.blue_u8().saturating_add(other.blue_u8()));
        self.set_alpha_u8(self.alpha_u8().saturating_add(other.alpha_u8()));
    }

    fn sub(&self, other: Self) -> Self {
        Rgba([
            self.red_u8().saturating_sub(other.red_u8()),
            self.green_u8().saturating_sub(other.green_u8()),
            self.blue_u8().saturating_sub(other.blue_u8()),
            self.alpha_u8().saturating_sub(other.alpha_u8()),
        ])
    }

    fn sub_assign(&mut self, other: Self) {
        self.set_red_u8(self.red_u8().saturating_sub(other.red_u8()));
        self.set_green_u8(self.green_u8().saturating_sub(other.green_u8()));
        self.set_blue_u8(self.blue_u8().saturating_sub(other.blue_u8()));
        self.set_alpha_u8(self.alpha_u8().saturating_sub(other.alpha_u8()));
    }

    fn add_scalar(&self, scalar: f32) -> Self {
        let scaled_scalar = (scalar * 255.0).round() as u8;
        Rgba([
            self.red_u8().saturating_add(scaled_scalar),
            self.green_u8().saturating_add(scaled_scalar),
            self.blue_u8().saturating_add(scaled_scalar),
            self.alpha_u8().saturating_add(scaled_scalar),
        ])
    }

    fn sub_scalar(&self, scalar: f32) -> Self {
        let scaled_scalar = (scalar * 255.0).round() as u8;
        Rgba([
            self.red_u8().saturating_sub(scaled_scalar),
            self.green_u8().saturating_sub(scaled_scalar),
            self.blue_u8().saturating_sub(scaled_scalar),
            self.alpha_u8().saturating_sub(scaled_scalar),
        ])
    }

    fn mul_scalar(&self, scalar: f32) -> Self {
        let scaled_scalar = (scalar * 255.0).round() as u8;
        Rgba([
            self.red_u8().saturating_mul(scaled_scalar),
            self.green_u8().saturating_mul(scaled_scalar),
            self.blue_u8().saturating_mul(scaled_scalar),
            self.alpha_u8().saturating_mul(scaled_scalar),
        ])
    }

    fn div_scalar(&self, scalar: f32) -> Self {
        let scaled_scalar = (scalar * 255.0).round() as u8;
        Rgba([
            self.red_u8().saturating_div(scaled_scalar),
            self.green_u8().saturating_div(scaled_scalar),
            self.blue_u8().saturating_div(scaled_scalar),
            self.alpha_u8().saturating_div(scaled_scalar),
        ])
    }
}
