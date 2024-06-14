pub trait RgbaU8Pixel {
    fn red_u8(&self) -> u8;
    fn green_u8(&self) -> u8;
    fn blue_u8(&self) -> u8;
    fn alpha_u8(&self) -> u8;
    fn set_red_u8(&mut self, value: u8);
    fn set_green_u8(&mut self, value: u8);
    fn set_blue_u8(&mut self, value: u8);
    fn set_alpha_u8(&mut self, value: u8);
    #[inline(always)]
    fn set_red_clamped_u8(&mut self, value: u8) {
        self.set_red_u8(value.clamp(0, 255));
    }
    #[inline(always)]
    fn set_green_clamped_u8(&mut self, value: u8) {
        self.set_green_u8(value.clamp(0, 255));
    }
    #[inline(always)]
    fn set_blue_clamped_u8(&mut self, value: u8) {
        self.set_blue_u8(value.clamp(0, 255));
    }
    #[inline(always)]
    fn set_alpha_clamped_u8(&mut self, value: u8) {
        self.set_alpha_u8(value.clamp(0, 255));
    }
}

pub trait RgbaF32Pixel {
    fn red_f32(&self) -> f32;
    fn green_f32(&self) -> f32;
    fn blue_f32(&self) -> f32;
    fn alpha_f32(&self) -> f32;
    fn set_red_f32(&mut self, value: f32);
    fn set_green_f32(&mut self, value: f32);
    fn set_blue_f32(&mut self, value: f32);
    fn set_alpha_f32(&mut self, value: f32);
    #[inline(always)]
    fn set_red_clamped_f32(&mut self, value: f32) {
        self.set_red_f32(value.clamp(0.0, 1.0));
    }
    #[inline(always)]
    fn set_green_clamped_f32(&mut self, value: f32) {
        self.set_green_f32(value.clamp(0.0, 1.0));
    }
    #[inline(always)]
    fn set_blue_clamped_f32(&mut self, value: f32) {
        self.set_blue_f32(value.clamp(0.0, 1.0));
    }
    #[inline(always)]
    fn set_alpha_clamped_f32(&mut self, value: f32) {
        self.set_alpha_f32(value.clamp(0.0, 1.0));
    }
}