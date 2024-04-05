use pixel::color::{hsl_to_hsv, hsv_to_hsl, hsv_to_rgb, rgb_to_hsv};
use crate::pixel::color::{ColorChannel, ColorSpace, hsl_to_rgb, rgb_to_hsl};

#[derive(Copy, Clone)]
pub enum Pixel {
    RGB(RGBPixelValue),
    RGBA(RGBAPixelValue),
    HSL(HSLPixelValue),
    HSLA(HSLAPixelValue),
    HSV(HSVPixelValue),
    HSVA(HSVAPixelValue),
    Luma(LumaPixelValue),
}

impl Pixel {
    pub fn to_color_space(&self, color_space: ColorSpace) -> Pixel {
        match color_space {
            ColorSpace::RGB => match self {
                Pixel::RGB(value) => Pixel::RGB(RGBPixelValue {
                    r: value.r,
                    g: value.g,
                    b: value.b,
                }),
                Pixel::RGBA(pixel_value) => Pixel::RGB(RGBPixelValue {
                    r: pixel_value.r,
                    g: pixel_value.g,
                    b: pixel_value.b,
                }),
                Pixel::HSL(pixel_value) => {
                    let (r, g, b) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::RGB(RGBPixelValue {
                        r,
                        g,
                        b,
                    })
                }
                Pixel::HSLA(pixel_value) => {
                    let (r, g, b) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::RGB(RGBPixelValue {
                        r,
                        g,
                        b,
                    })
                }
                Pixel::HSV(pixel_value) => {
                    let (r, g, b) = hsv_to_rgb(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::RGB(RGBPixelValue {
                        r,
                        g,
                        b,
                    })
                }
                Pixel::HSVA(pixel_value) => {
                    let (r, g, b) = hsv_to_rgb(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::RGB(RGBPixelValue {
                        r,
                        g,
                        b,
                    })
                }
                Pixel::Luma(pixel_value) => {
                    Pixel::RGB(RGBPixelValue {
                        r: pixel_value.l,
                        g: pixel_value.l,
                        b: pixel_value.l,
                    })
                }
            },
            ColorSpace::RGBA => match self {
                Pixel::RGB(pixel_value) => Pixel::RGBA(RGBAPixelValue {
                    r: pixel_value.r,
                    g: pixel_value.g,
                    b: pixel_value.b,
                    a: 1.,
                }),
                Pixel::RGBA(pixel_value) => Pixel::RGBA(RGBAPixelValue {
                    r: pixel_value.r,
                    g: pixel_value.g,
                    b: pixel_value.b,
                    a: pixel_value.a,
                }),
                Pixel::HSL(pixel_value) => {
                    let (r, g, b) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::RGBA(RGBAPixelValue {
                        r,
                        g,
                        b,
                        a: 1.,
                    })
                }
                Pixel::HSLA(pixel_value) => {
                    let (r, g, b) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::RGBA(RGBAPixelValue {
                        r,
                        g,
                        b,
                        a: pixel_value.a,
                    })
                }
                Pixel::HSV(pixel_value) => {
                    let (r, g, b) = hsv_to_rgb(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::RGBA(RGBAPixelValue {
                        r,
                        g,
                        b,
                        a: 1.,
                    })
                }
                Pixel::HSVA(pixel_value) => {
                    let (r, g, b) = hsv_to_rgb(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::RGBA(RGBAPixelValue {
                        r,
                        g,
                        b,
                        a: pixel_value.a,
                    })
                }
                Pixel::Luma(pixel_value) => {
                    Pixel::RGBA(RGBAPixelValue {
                        r: pixel_value.l,
                        g: pixel_value.l,
                        b: pixel_value.l,
                        a: 1.,
                    })
                }
            },
            ColorSpace::HSL => match self {
                Pixel::RGB(pixel_value) => {
                    let (h, s, l) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSL(HSLPixelValue {
                        h,
                        s,
                        l,
                    })
                }
                Pixel::RGBA(pixel_value) => {
                    let (h, s, l) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSL(HSLPixelValue {
                        h,
                        s,
                        l,
                    })
                }
                Pixel::HSL(pixel_value) => Pixel::HSL(HSLPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    l: pixel_value.l,
                }),
                Pixel::HSLA(pixel_value) => Pixel::HSL(HSLPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    l: pixel_value.l,
                }),
                Pixel::HSV(pixel_value) => {
                    let (h, s, l) = hsv_to_hsl(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::HSL(HSLPixelValue {
                        h,
                        s,
                        l,
                    })
                }
                Pixel::HSVA(pixel_value) => {
                    let (h, s, l) = hsv_to_hsl(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::HSL(HSLPixelValue {
                        h,
                        s,
                        l,
                    })
                }
                Pixel::Luma(pixel_value) => {
                    let (h, s, l) = rgb_to_hsl(pixel_value.l, pixel_value.l, pixel_value.l);
                    Pixel::HSL(HSLPixelValue {
                        h,
                        s,
                        l,
                    })
                }
            },
            ColorSpace::HSLA => match self {
                Pixel::RGB(pixel_value) => {
                    let (h, s, l) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSLA(HSLAPixelValue {
                        h,
                        s,
                        l,
                        a: 1.,
                    })
                }
                Pixel::RGBA(pixel_value) => {
                    let (h, s, l) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSLA(HSLAPixelValue {
                        h,
                        s,
                        l,
                        a: pixel_value.a,
                    })
                }
                Pixel::HSL(pixel_value) => Pixel::HSLA(HSLAPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    l: pixel_value.l,
                    a: 1.,
                }),
                Pixel::HSLA(pixel_value) => Pixel::HSLA(HSLAPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    l: pixel_value.l,
                    a: pixel_value.a,
                }),
                Pixel::HSV(pixel_value) => {
                    let (h, s, l) = hsv_to_hsl(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::HSLA(HSLAPixelValue {
                        h,
                        s,
                        l,
                        a: 1.,
                    })
                }
                Pixel::HSVA(pixel_value) => {
                    let (h, s, l) = hsv_to_hsl(pixel_value.h, pixel_value.s, pixel_value.v);
                    Pixel::HSLA(HSLAPixelValue {
                        h,
                        s,
                        l,
                        a: pixel_value.a,
                    })
                }
                Pixel::Luma(pixel_value) => {
                    let (h, s, l) = rgb_to_hsl(pixel_value.l, pixel_value.l, pixel_value.l);
                    Pixel::HSLA(HSLAPixelValue {
                        h,
                        s,
                        l,
                        a: 1.,
                    })
                }
            },
            ColorSpace::HSV => match self {
                Pixel::RGB(pixel_value) => {
                    let (h, s, v) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSV(HSVPixelValue {
                        h,
                        s,
                        v,
                    })
                }
                Pixel::RGBA(pixel_value) => {
                    let (h, s, v) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSV(HSVPixelValue {
                        h,
                        s,
                        v,
                    })
                }
                Pixel::HSL(pixel_value) => {
                    let (h, s, v) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::HSV(HSVPixelValue {
                        h,
                        s,
                        v,
                    })
                }
                Pixel::HSLA(pixel_value) => {
                    let (h, s, v) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::HSV(HSVPixelValue {
                        h,
                        s,
                        v,
                    })
                }
                Pixel::HSV(pixel_value) => Pixel::HSV(HSVPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    v: pixel_value.v,
                }),
                Pixel::HSVA(pixel_value) => Pixel::HSV(HSVPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    v: pixel_value.v,
                }),
                Pixel::Luma(pixel_value) => {
                    let (h, s, v) = rgb_to_hsv(pixel_value.l, pixel_value.l, pixel_value.l);
                    Pixel::HSV(HSVPixelValue {
                        h,
                        s,
                        v,
                    })
                }
            },
            ColorSpace::HSVA => match self {
                Pixel::RGB(pixel_value) => {
                    let (h, s, v) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSVA(HSVAPixelValue {
                        h,
                        s,
                        v,
                        a: 1.,
                    })
                }
                Pixel::RGBA(pixel_value) => {
                    let (h, s, v) = rgb_to_hsl(pixel_value.r, pixel_value.g, pixel_value.b);
                    Pixel::HSVA(HSVAPixelValue {
                        h,
                        s,
                        v,
                        a: pixel_value.a,
                    })
                }
                Pixel::HSL(pixel_value) => {
                    let (h, s, v) = hsl_to_hsv(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::HSVA(HSVAPixelValue {
                        h,
                        s,
                        v,
                        a: 1.,
                    })
                }
                Pixel::HSLA(pixel_value) => {
                    let (h, s, v) = hsl_to_hsv(pixel_value.h, pixel_value.s, pixel_value.l);
                    Pixel::HSVA(HSVAPixelValue {
                        h,
                        s,
                        v,
                        a: pixel_value.a,
                    })
                }
                Pixel::HSV(pixel_value) => Pixel::HSVA(HSVAPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    v: pixel_value.v,
                    a: 1.,
                }),
                Pixel::HSVA(pixel_value) => Pixel::HSVA(HSVAPixelValue {
                    h: pixel_value.h,
                    s: pixel_value.s,
                    v: pixel_value.v,
                    a: pixel_value.a,
                }),
                Pixel::Luma(pixel_value) => {
                    let (h, s, v) = rgb_to_hsv(pixel_value.l, pixel_value.l, pixel_value.l);
                    Pixel::HSVA(HSVAPixelValue {
                        h,
                        s,
                        v,
                        a: 1.,
                    })
                }
            },
            ColorSpace::Luma => {
                match self {
                    Pixel::RGB(pixel_value) => {
                        let l = 0.2126 * pixel_value.r + 0.7152 * pixel_value.g + 0.0722 * pixel_value.b;
                        Pixel::Luma(LumaPixelValue {
                            l,
                        })
                    }
                    Pixel::RGBA(pixel_value) => {
                        let l = 0.2126 * pixel_value.r + 0.7152 * pixel_value.g + 0.0722 * pixel_value.b;
                        Pixel::Luma(LumaPixelValue {
                            l,
                        })
                    }
                    Pixel::HSL(pixel_value) => {
                        let (r, g, b) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                        let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                        Pixel::Luma(LumaPixelValue {
                            l,
                        })
                    }
                    Pixel::HSLA(pixel_value) => {
                        let (r, g, b) = hsl_to_rgb(pixel_value.h, pixel_value.s, pixel_value.l);
                        let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                        Pixel::Luma(LumaPixelValue {
                            l,
                        })
                    }
                    Pixel::HSV(pixel_value) => {
                        let (r, g, b) = hsv_to_rgb(pixel_value.h, pixel_value.s, pixel_value.v);
                        let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                        Pixel::Luma(LumaPixelValue {
                            l,
                        })
                    }
                    Pixel::HSVA(pixel_value) => {
                        let (r, g, b) = hsv_to_rgb(pixel_value.h, pixel_value.s, pixel_value.v);
                        let l = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                        Pixel::Luma(LumaPixelValue::new(l).unwrap())
                    }
                    Pixel::Luma(pixel_value) => Pixel::Luma(LumaPixelValue {
                        l: pixel_value.l,
                    }),
                }
            }
        }
    }

    pub fn get(&self, color_channel: ColorChannel) -> f32 {
        match self {
            Pixel::RGB(pixel_value) => pixel_value.get(color_channel),
            Pixel::RGBA(pixel_value) => pixel_value.get(color_channel),
            Pixel::HSL(pixel_value) => pixel_value.get(color_channel),
            Pixel::HSLA(pixel_value) => pixel_value.get(color_channel),
            Pixel::HSV(pixel_value) => pixel_value.get(color_channel),
            Pixel::HSVA(pixel_value) => pixel_value.get(color_channel),
            Pixel::Luma(pixel_value) => pixel_value.get(color_channel),
        }
    }

    pub fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match self {
            Pixel::RGB(pixel_value) => pixel_value.set(color_channel, value),
            Pixel::RGBA(pixel_value) => pixel_value.set(color_channel, value),
            Pixel::HSL(pixel_value) => pixel_value.set(color_channel, value),
            Pixel::HSLA(pixel_value) => pixel_value.set(color_channel, value),
            Pixel::HSV(pixel_value) => pixel_value.set(color_channel, value),
            Pixel::HSVA(pixel_value) => pixel_value.set(color_channel, value),
            Pixel::Luma(pixel_value) => pixel_value.set(color_channel, value),
        }
    }
}

pub trait PixelValue {
    fn color_space() -> ColorSpace;
    fn get(&self, color_channel: ColorChannel) -> f32;
    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError>;
}

#[derive(Debug)]
pub enum PixelValueError {
    UnableToSetColorChannel,
    InvalidRedValue,
    InvalidGreenValue,
    InvalidBlueValue,
    InvalidAlphaValue,
    InvalidHueValue,
    InvalidSaturationValue,
    InvalidLightnessValue,
    InvalidValueValue,
}

#[derive(Copy, Clone)]
pub struct LumaPixelValue {
    l: f32,
}

impl LumaPixelValue {
    pub fn new(l: f32) -> Result<LumaPixelValue, PixelValueError> {
        if l < 0. || l > 1. {
            return Err(PixelValueError::InvalidLightnessValue);
        }
        Ok(LumaPixelValue {
            l,
        })
    }
}

impl PixelValue for LumaPixelValue {
    fn color_space() -> ColorSpace {
        ColorSpace::Luma
    }

    fn get(&self, color_channel: ColorChannel) -> f32 {
        match color_channel {
            ColorChannel::Red => {
                self.l
            }
            ColorChannel::Green => {
                self.l
            }
            ColorChannel::Blue => {
                self.l
            }
            ColorChannel::Alpha => {
                1.
            }
            ColorChannel::Hue => {
                0.
            }
            ColorChannel::Saturation => {
                0.
            }
            ColorChannel::Lightness => {
                self.l
            }
            ColorChannel::Value => {
                0.
            }
        }
    }

    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match color_channel {
            ColorChannel::Lightness => {
                self.l = value;
                Ok(())
            }
            _ => Err(PixelValueError::UnableToSetColorChannel),
        }
    }
}

impl Default for LumaPixelValue {
    fn default() -> Self {
        LumaPixelValue {
            l: 0.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RGBPixelValue {
    r: f32,
    g: f32,
    b: f32,
}

impl RGBPixelValue {
    pub fn new(r: f32, g: f32, b: f32) -> Result<RGBPixelValue, PixelValueError> {
        if r < 0. || r > 1. {
            return Err(PixelValueError::InvalidRedValue);
        }
        if g < 0. || g > 1. {
            return Err(PixelValueError::InvalidGreenValue);
        }
        if b < 0. || b > 1. {
            return Err(PixelValueError::InvalidBlueValue);
        }
        Ok(RGBPixelValue {
            r,
            g,
            b,
        })
    }
}

impl PixelValue for RGBPixelValue {
    fn color_space() -> ColorSpace {
        ColorSpace::RGB
    }

    fn get(&self, color_channel: ColorChannel) -> f32 {
        match color_channel {
            ColorChannel::Red => {
                self.r
            }
            ColorChannel::Green => {
                self.g
            }
            ColorChannel::Blue => {
                self.b
            }
            ColorChannel::Alpha => {
                1.
            }
            ColorChannel::Hue => {
                rgb_to_hsl(self.r, self.g, self.b).0
            }
            ColorChannel::Saturation => {
                rgb_to_hsl(self.r, self.g, self.b).1
            }
            ColorChannel::Lightness => {
                rgb_to_hsl(self.r, self.g, self.b).2
            }
            ColorChannel::Value => { 
                rgb_to_hsv(self.r, self.g, self.b).2
            }
        }
    }

    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match color_channel {
            ColorChannel::Red => {
                self.r = value;
                Ok(())
            }
            ColorChannel::Green => {
                self.g = value;
                Ok(())
            }
            ColorChannel::Blue => {
                self.b = value;
                Ok(())
            }
            _ => Err(PixelValueError::UnableToSetColorChannel),
        }
    }
}

impl Default for RGBPixelValue {
    fn default() -> Self {
        RGBPixelValue {
            r: 0.,
            g: 0.,
            b: 0.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RGBAPixelValue {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl RGBAPixelValue {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Result<RGBAPixelValue, PixelValueError> {
        if r < 0. || r > 1. {
            return Err(PixelValueError::InvalidRedValue);
        }
        if g < 0. || g > 1. {
            return Err(PixelValueError::InvalidGreenValue);
        }
        if b < 0. || b > 1. {
            return Err(PixelValueError::InvalidBlueValue);
        }
        if a < 0. || a > 1. {
            return Err(PixelValueError::InvalidAlphaValue);
        }
        Ok(RGBAPixelValue {
            r,
            g,
            b,
            a,
        })
    }
}

impl PixelValue for RGBAPixelValue {
    fn color_space() -> ColorSpace {
        ColorSpace::RGB
    }

    fn get(&self, color_channel: ColorChannel) -> f32 {
        match color_channel {
            ColorChannel::Red => {
                self.r
            }
            ColorChannel::Green => {
                self.g
            }
            ColorChannel::Blue => {
                self.b
            }
            ColorChannel::Alpha => {
                self.a
            }
            ColorChannel::Hue => {
                rgb_to_hsl(self.r, self.g, self.b).0
            }
            ColorChannel::Saturation => {
                rgb_to_hsl(self.r, self.g, self.b).1
            }
            ColorChannel::Lightness => {
                rgb_to_hsl(self.r, self.g, self.b).2
            }
            ColorChannel::Value => { 
                rgb_to_hsv(self.r, self.g, self.b).2
            }
        }
    }

    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match color_channel {
            ColorChannel::Red => {
                self.r = value;
                Ok(())
            }
            ColorChannel::Green => {
                self.g = value;
                Ok(())
            }
            ColorChannel::Blue => {
                self.b = value;
                Ok(())
            }
            ColorChannel::Alpha => {
                self.a = value;
                Ok(())
            }
            _ => Err(PixelValueError::UnableToSetColorChannel),
        }
    }
}

impl Default for RGBAPixelValue {
    fn default() -> Self {
        RGBAPixelValue {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HSLPixelValue {
    h: f32,
    s: f32,
    l: f32,
}

impl HSLPixelValue {
    pub fn new(h: f32, s: f32, l: f32) -> Result<HSLPixelValue, PixelValueError> {
        if h < 0. || h > 1. {
            return Err(PixelValueError::InvalidHueValue);
        }
        if s < 0. || s > 1. {
            return Err(PixelValueError::InvalidSaturationValue);
        }
        if l < 0. || l > 1. {
            return Err(PixelValueError::InvalidLightnessValue);
        }
        Ok(HSLPixelValue {
            h,
            s,
            l,
        })
    }
}

impl PixelValue for HSLPixelValue {
    fn color_space() -> ColorSpace {
        ColorSpace::RGB
    }

    fn get(&self, color_channel: ColorChannel) -> f32 {
        match color_channel {
            ColorChannel::Red => {
                hsl_to_rgb(self.h, self.s, self.l).0
            }
            ColorChannel::Green => {
                hsl_to_rgb(self.h, self.s, self.l).1
            }
            ColorChannel::Blue => {
                hsl_to_rgb(self.h, self.s, self.l).2
            }
            ColorChannel::Alpha => {
                1.
            }
            ColorChannel::Hue => {
                self.h
            }
            ColorChannel::Saturation => {
                self.s
            }
            ColorChannel::Lightness => {
                self.l
            }
            ColorChannel::Value => { 
                hsl_to_hsv(self.h, self.s, self.l).2
            }
        }
    }

    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match color_channel {
            ColorChannel::Hue => {
                self.h = value;
                Ok(())
            }
            ColorChannel::Saturation => {
                self.s = value;
                Ok(())
            }
            ColorChannel::Lightness => {
                self.l = value;
                Ok(())
            }
            _ => Err(PixelValueError::UnableToSetColorChannel),
        }
    }
}

impl Default for HSLPixelValue {
    fn default() -> Self {
        HSLPixelValue {
            h: 0.,
            s: 0.,
            l: 0.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HSLAPixelValue {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

impl HSLAPixelValue {
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Result<HSLAPixelValue, PixelValueError> {
        if h < 0. || h > 1. {
            return Err(PixelValueError::InvalidHueValue);
        }
        if s < 0. || s > 1. {
            return Err(PixelValueError::InvalidSaturationValue);
        }
        if l < 0. || l > 1. {
            return Err(PixelValueError::InvalidLightnessValue);
        }
        if a < 0. || a > 1. {
            return Err(PixelValueError::InvalidAlphaValue);
        }
        Ok(HSLAPixelValue {
            h,
            s,
            l,
            a,
        })
    }
}

impl PixelValue for HSLAPixelValue {
    fn color_space() -> ColorSpace {
        ColorSpace::RGB
    }

    fn get(&self, color_channel: ColorChannel) -> f32 {
        match color_channel {
            ColorChannel::Red => {
                hsl_to_rgb(self.h, self.s, self.l).0
            }
            ColorChannel::Green => {
                hsl_to_rgb(self.h, self.s, self.l).1
            }
            ColorChannel::Blue => {
                hsl_to_rgb(self.h, self.s, self.l).2
            }
            ColorChannel::Alpha => {
                self.a
            }
            ColorChannel::Hue => {
                self.h
            }
            ColorChannel::Saturation => {
                self.s
            }
            ColorChannel::Lightness => {
                self.l
            }
            ColorChannel::Value => { 
                hsl_to_hsv(self.h, self.s, self.l).2
            }
        }
    }

    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match color_channel {
            ColorChannel::Hue => {
                self.h = value;
                Ok(())
            }
            ColorChannel::Saturation => {
                self.s = value;
                Ok(())
            }
            ColorChannel::Lightness => {
                self.l = value;
                Ok(())
            }
            ColorChannel::Alpha => {
                self.a = value;
                Ok(())
            }
            _ => Err(PixelValueError::UnableToSetColorChannel),
        }
    }
}

impl Default for HSLAPixelValue {
    fn default() -> Self {
        HSLAPixelValue {
            h: 0.,
            s: 0.,
            l: 0.,
            a: 1.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HSVPixelValue {
    h: f32,
    s: f32,
    v: f32,
}

impl HSVPixelValue {
    pub fn new(h: f32, s: f32, v: f32) -> Result<HSVPixelValue, PixelValueError> {
        if h < 0. || h > 1. {
            return Err(PixelValueError::InvalidHueValue);
        }
        if s < 0. || s > 1. {
            return Err(PixelValueError::InvalidSaturationValue);
        }
        if v < 0. || v > 1. {
            return Err(PixelValueError::InvalidValueValue);
        }
        Ok(HSVPixelValue {
            h,
            s,
            v,
        })
    }
}

impl PixelValue for HSVPixelValue {
    fn color_space() -> ColorSpace {
        ColorSpace::RGB
    }

    fn get(&self, color_channel: ColorChannel) -> f32 {
        match color_channel {
            ColorChannel::Red => {
                hsl_to_rgb(self.h, self.s, self.v).0
            }
            ColorChannel::Green => {
                hsl_to_rgb(self.h, self.s, self.v).1
            }
            ColorChannel::Blue => {
                hsl_to_rgb(self.h, self.s, self.v).2
            }
            ColorChannel::Alpha => {
                1.
            }
            ColorChannel::Hue => {
                self.h
            }
            ColorChannel::Saturation => {
                self.s
            }
            ColorChannel::Lightness => {
                hsv_to_hsl(self.h, self.s, self.v).2
            }
            ColorChannel::Value => {
                self.v
            }
        }
    }

    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match color_channel {
            ColorChannel::Hue => {
                self.h = value;
                Ok(())
            }
            ColorChannel::Saturation => {
                self.s = value;
                Ok(())
            }
            ColorChannel::Value => {
                self.v = value;
                Ok(())
            }
            _ => Err(PixelValueError::UnableToSetColorChannel),
        }
    }
}

impl Default for HSVPixelValue {
    fn default() -> Self {
        HSVPixelValue {
            h: 0.,
            s: 0.,
            v: 0.,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HSVAPixelValue {
    h: f32,
    s: f32,
    v: f32,
    a: f32,
}

impl HSVAPixelValue {
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Result<HSVAPixelValue, PixelValueError> {
        if h < 0. || h > 1. {
            return Err(PixelValueError::InvalidHueValue);
        }
        if s < 0. || s > 1. {
            return Err(PixelValueError::InvalidSaturationValue);
        }
        if v < 0. || v > 1. {
            return Err(PixelValueError::InvalidValueValue);
        }
        if a < 0. || a > 1. {
            return Err(PixelValueError::InvalidAlphaValue);
        }
        Ok(HSVAPixelValue {
            h,
            s,
            v,
            a,
        })
    }
}

impl PixelValue for HSVAPixelValue {
    fn color_space() -> ColorSpace {
        ColorSpace::RGB
    }

    fn get(&self, color_channel: ColorChannel) -> f32 {
        match color_channel {
            ColorChannel::Red => {
                hsl_to_rgb(self.h, self.s, self.v).0
            }
            ColorChannel::Green => {
                hsl_to_rgb(self.h, self.s, self.v).1
            }
            ColorChannel::Blue => {
                hsl_to_rgb(self.h, self.s, self.v).2
            }
            ColorChannel::Alpha => {
                self.a
            }
            ColorChannel::Hue => {
                self.h
            }
            ColorChannel::Saturation => {
                self.s
            }
            ColorChannel::Lightness => {
                hsv_to_hsl(self.h, self.s, self.v).2
            }
            ColorChannel::Value => {
                self.v
            }
        }
    }

    fn set(&mut self, color_channel: ColorChannel, value: f32) -> Result<(), PixelValueError> {
        match color_channel {
            ColorChannel::Hue => {
                self.h = value;
                Ok(())
            }
            ColorChannel::Saturation => {
                self.s = value;
                Ok(())
            }
            ColorChannel::Value => {
                self.v = value;
                Ok(())
            }
            ColorChannel::Alpha => {
                self.a = value;
                Ok(())
            }
            _ => Err(PixelValueError::UnableToSetColorChannel),
        }
    }
}

impl Default for HSVAPixelValue {
    fn default() -> Self {
        HSVAPixelValue {
            h: 0.,
            s: 0.,
            v: 0.,
            a: 1.,
        }
    }
}