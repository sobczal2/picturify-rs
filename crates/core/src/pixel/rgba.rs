use image::Rgba;

pub trait RgbaF32Pixel {
    fn red_f32(&self) -> f32;
    fn green_f32(&self) -> f32;
    fn blue_f32(&self) -> f32;
    fn alpha_f32(&self) -> f32;
    fn set_red_f32(&mut self, value: f32);
    fn set_green_f32(&mut self, value: f32);
    fn set_blue_f32(&mut self, value: f32);
    fn set_alpha_f32(&mut self, value: f32);
    fn set_red_clamped_f32(&mut self, value: f32) {
        self.set_red_f32(value.max(0.0).min(1.0));
    }
    fn set_green_clamped_f32(&mut self, value: f32) {
        self.set_green_f32(value.max(0.0).min(1.0));
    }
    fn set_blue_clamped_f32(&mut self, value: f32) {
        self.set_blue_f32(value.max(0.0).min(1.0));
    }
    fn set_alpha_clamped_f32(&mut self, value: f32) {
        self.set_alpha_f32(value.max(0.0).min(1.0));
    }
}

impl RgbaF32Pixel for Rgba<u8> {
    fn red_f32(&self) -> f32 {
        self.0[0] as f32 / 255.0
    }

    fn green_f32(&self) -> f32 {
        self.0[1] as f32 / 255.0
    }

    fn blue_f32(&self) -> f32 {
        self.0[2] as f32 / 255.0
    }

    fn alpha_f32(&self) -> f32 {
        self.0[3] as f32 / 255.0
    }

    fn set_red_f32(&mut self, value: f32) {
        self.0[0] = (value * 255.0) as u8;
    }

    fn set_green_f32(&mut self, value: f32) {
        self.0[1] = (value * 255.0) as u8;
    }

    fn set_blue_f32(&mut self, value: f32) {
        self.0[2] = (value * 255.0) as u8;
    }

    fn set_alpha_f32(&mut self, value: f32) {
        self.0[3] = (value * 255.0) as u8;
    }
}