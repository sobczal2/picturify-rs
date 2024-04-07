#[derive(Debug, Clone, Copy)]
pub struct RgbaPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct HsvaPixel {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub alpha: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct HslaPixel {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: u8,
}
