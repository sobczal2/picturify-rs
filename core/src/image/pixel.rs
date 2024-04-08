use crate::color::hsv_conversions::{hsv_to_rgb, rgb_to_hsv};

#[derive(Debug, Clone, Copy)]
pub struct RgbaPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl RgbaPixel {
    pub fn copy_from_hsva(&mut self, hsva: HsvaPixel) {
        let (red, green, blue) = hsv_to_rgb(hsva.hue, hsva.saturation, hsva.value);
        self.red = red;
        self.green = green;
        self.blue = blue;
    }

    pub fn copy_from_hsla(&mut self, hsla: HslaPixel) {
        let (red, green, blue) = hsv_to_rgb(hsla.hue, hsla.saturation, hsla.lightness);
        self.red = red;
        self.green = green;
        self.blue = blue;
    }
}

impl From<HsvaPixel> for RgbaPixel {
    fn from(hsva: HsvaPixel) -> Self {
        let (red, green, blue) = hsv_to_rgb(hsva.hue, hsva.saturation, hsva.value);
        RgbaPixel {
            red,
            green,
            blue,
            alpha: hsva.alpha,
        }
    }
}

impl From<HslaPixel> for RgbaPixel {
    fn from(hsla: HslaPixel) -> Self {
        let (red, green, blue) = hsv_to_rgb(hsla.hue, hsla.saturation, hsla.lightness);
        RgbaPixel {
            red,
            green,
            blue,
            alpha: hsla.alpha,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HsvaPixel {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub alpha: u8,
}

impl HsvaPixel {
    pub fn copy_from_rgba(&mut self, rgba: RgbaPixel) {
        let (hue, saturation, value) = rgb_to_hsv(rgba.red, rgba.green, rgba.blue);
        self.hue = hue;
        self.saturation = saturation;
        self.value = value;
    }
}

impl From<RgbaPixel> for HsvaPixel {
    fn from(rgba: RgbaPixel) -> Self {
        let (hue, saturation, value) = rgb_to_hsv(rgba.red, rgba.green, rgba.blue);
        HsvaPixel {
            hue,
            saturation,
            value,
            alpha: rgba.alpha,
        }
    }
}

impl From<HslaPixel> for HsvaPixel {
    fn from(hsla: HslaPixel) -> Self {
        let (red, green, blue) = hsv_to_rgb(hsla.hue, hsla.saturation, hsla.lightness);
        let (hue, saturation, value) = rgb_to_hsv(red, green, blue);
        HsvaPixel {
            hue,
            saturation,
            value,
            alpha: hsla.alpha,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HslaPixel {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: u8,
}

impl HslaPixel {
    pub fn copy_from_rgba(&mut self, rgba: RgbaPixel) {
        let (hue, saturation, lightness) = rgb_to_hsv(rgba.red, rgba.green, rgba.blue);
        self.hue = hue;
        self.saturation = saturation;
        self.lightness = lightness;
    }
}

impl From<RgbaPixel> for HslaPixel {
    fn from(rgba: RgbaPixel) -> Self {
        let (hue, saturation, lightness) = rgb_to_hsv(rgba.red, rgba.green, rgba.blue);
        HslaPixel {
            hue,
            saturation,
            lightness,
            alpha: rgba.alpha,
        }
    }
}

impl From<HsvaPixel> for HslaPixel {
    fn from(hsva: HsvaPixel) -> Self {
        let (red, green, blue) = hsv_to_rgb(hsva.hue, hsva.saturation, hsva.value);
        let (hue, saturation, lightness) = rgb_to_hsv(red, green, blue);
        HslaPixel {
            hue,
            saturation,
            lightness,
            alpha: hsva.alpha,
        }
    }
}
