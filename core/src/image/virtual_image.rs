use crate::image::pixel::{HslaPixel, HsvaPixel, RgbaPixel};

pub trait VirtualImage {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub trait VirtualRgbaImage: VirtualImage {
    fn get_red(&self, x: usize, y: usize) -> u8 {
        let pixel = self.get_rgba(x, y);
        pixel.red
    }
    fn get_green(&self, x: usize, y: usize) -> u8 {
        let pixel = self.get_rgba(x, y);
        pixel.green
    }
    fn get_blue(&self, x: usize, y: usize) -> u8 {
        let pixel = self.get_rgba(x, y);
        pixel.blue
    }
    fn get_alpha(&self, x: usize, y: usize) -> u8 {
        let pixel = self.get_rgba(x, y);
        pixel.alpha
    }
    fn set_red(&mut self, x: usize, y: usize, red: u8) {
        let pixel = self.get_rgba(x, y);
        self.set_rgba(x, y, RgbaPixel { red, ..pixel });
    }
    fn set_green(&mut self, x: usize, y: usize, green: u8) {
        let pixel = self.get_rgba(x, y);
        self.set_rgba(x, y, RgbaPixel { green, ..pixel });
    }
    fn set_blue(&mut self, x: usize, y: usize, blue: u8) {
        let pixel = self.get_rgba(x, y);
        self.set_rgba(x, y, RgbaPixel { blue, ..pixel });
    }

    fn set_alpha(&mut self, x: usize, y: usize, alpha: u8) {
        let pixel = self.get_rgba(x, y);
        self.set_rgba(x, y, RgbaPixel { alpha, ..pixel });
    }
    fn get_rgba(&self, x: usize, y: usize) -> RgbaPixel;
    fn set_rgba(&mut self, x: usize, y: usize, pixel: RgbaPixel);

    fn iterate_rgba<F>(&mut self, f: F)
    where
        F: Fn(&mut RgbaPixel, usize, usize);
    fn iterate_par_rgba<F>(&mut self, f: F)
    where
        F: Fn(&mut RgbaPixel, usize, usize) + Sync + Send;
}

pub trait VirtualHsvaImage: VirtualImage {
    fn get_hue(&self, x: usize, y: usize) -> f32 {
        let pixel = self.get_hsva(x, y);
        pixel.hue
    }
    fn get_saturation(&self, x: usize, y: usize) -> f32 {
        let pixel = self.get_hsva(x, y);
        pixel.saturation
    }
    fn get_value(&self, x: usize, y: usize) -> f32 {
        let pixel = self.get_hsva(x, y);
        pixel.value
    }
    fn get_alpha_from_hsva(&self, x: usize, y: usize) -> u8 {
        let pixel = self.get_hsva(x, y);
        pixel.alpha
    }
    fn set_hue(&mut self, x: usize, y: usize, hue: f32) {
        let pixel = self.get_hsva(x, y);
        self.set_hsva(x, y, HsvaPixel { hue, ..pixel });
    }
    fn set_saturation(&mut self, x: usize, y: usize, saturation: f32) {
        let pixel = self.get_hsva(x, y);
        self.set_hsva(
            x,
            y,
            HsvaPixel {
                saturation,
                ..pixel
            },
        );
    }
    fn set_value(&mut self, x: usize, y: usize, value: f32) {
        let pixel = self.get_hsva(x, y);
        self.set_hsva(x, y, HsvaPixel { value, ..pixel });
    }
    fn set_alpha_for_hsva(&mut self, x: usize, y: usize, alpha: u8) {
        let pixel = self.get_hsva(x, y);
        self.set_hsva(x, y, HsvaPixel { alpha, ..pixel });
    }
    fn get_hsva(&self, x: usize, y: usize) -> HsvaPixel;
    fn set_hsva(&mut self, x: usize, y: usize, pixel: HsvaPixel);

    fn iterate_hsva<F>(&mut self, f: F)
    where
        F: Fn(&mut HsvaPixel, usize, usize);

    fn iterate_par_hsva<F>(&mut self, f: F)
    where
        F: Fn(&mut HsvaPixel, usize, usize) + Sync + Send;
}

pub trait VirtualHslImage: VirtualImage {
    fn get_hue(&self, x: usize, y: usize) -> f32 {
        let pixel = self.get_hsla(x, y);
        pixel.hue
    }
    fn get_saturation(&self, x: usize, y: usize) -> f32 {
        let pixel = self.get_hsla(x, y);
        pixel.saturation
    }
    fn get_lightness(&self, x: usize, y: usize) -> f32 {
        let pixel = self.get_hsla(x, y);
        pixel.lightness
    }
    fn get_alpha_from_hsla(&self, x: usize, y: usize) -> u8 {
        let pixel = self.get_hsla(x, y);
        pixel.alpha
    }
    fn set_hue(&mut self, x: usize, y: usize, hue: f32) {
        let pixel = self.get_hsla(x, y);
        self.set_hsla(x, y, HslaPixel { hue, ..pixel });
    }
    fn set_saturation(&mut self, x: usize, y: usize, saturation: f32) {
        let pixel = self.get_hsla(x, y);
        self.set_hsla(
            x,
            y,
            HslaPixel {
                saturation,
                ..pixel
            },
        );
    }
    fn set_lightness(&mut self, x: usize, y: usize, lightness: f32) {
        let pixel = self.get_hsla(x, y);
        self.set_hsla(x, y, HslaPixel { lightness, ..pixel });
    }
    fn set_alpha_for_hsla(&mut self, x: usize, y: usize, alpha: u8) {
        let pixel = self.get_hsla(x, y);
        self.set_hsla(x, y, HslaPixel { alpha, ..pixel });
    }
    fn get_hsla(&self, x: usize, y: usize) -> HslaPixel;
    fn set_hsla(&mut self, x: usize, y: usize, pixel: HslaPixel);

    fn iterate_hsla<F>(&mut self, f: F)
    where
        F: Fn(&mut HslaPixel, usize, usize);
    fn iterate_par_hsla<F>(&mut self, f: F)
    where
        F: Fn(&mut HslaPixel, usize, usize) + Sync + Send;
}
