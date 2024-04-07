use rayon::prelude::*;
use rayon::slice::ChunksExactMut;

use crate::error::PicturifyResult;
use crate::image::pixel::{HslaPixel, HsvaPixel, RgbaPixel};

pub struct Layer<T: Sized + Clone + Copy + Send + Sync> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Sized + Clone + Copy + Send + Sync> Layer<T> {
    pub fn new(width: usize, height: usize, default: T) -> Layer<T> {
        Layer {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.data[y * self.width + x] = value;
    }
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }
    
    pub fn chunk_exact_mut(&mut self, chunk_size: usize) -> core::slice::ChunksExactMut<T> {
        self.data.chunks_exact_mut(chunk_size)
    }
    pub fn par_chunk_exact_mut(&mut self, chunk_size: usize) -> rayon::slice::ChunksExactMut<T> {
        self.data.par_chunks_exact_mut(chunk_size)
    }
}

pub type LayerU8 = Layer<u8>;
pub type LayerF32 = Layer<f32>;

pub type RedLayer = LayerU8;
pub type GreenLayer = LayerU8;
pub type BlueLayer = LayerU8;
pub type AlphaLayer = LayerU8;
pub type HueLayer = LayerF32;
pub type SaturationLayer = LayerF32;
pub type ValueLayer = LayerF32;
pub type LightnessLayer = LayerF32;

pub trait RgbaLayered {
    fn get_rgba_layers(&self) -> RgbaLayers;
    fn from_rgba_layers(rgba_layers: RgbaLayers) -> Self;
}

pub trait HsvaLayered {
    fn get_hsva_layers(&self) -> HsvaLayers;
    fn from_hsva_layers(hsva_layers: HsvaLayers) -> Self;
}

pub trait HslaLayered {
    fn get_hsla_layers(&self) -> HslaLayers;
    fn from_hsla_layers(hsla_layers: HslaLayers) -> Self;
}

pub trait LaLayered {
    fn get_la_layers(&self) -> PicturifyResult<LaLayers>;
    fn from_la_layers(la_layers: LaLayers) -> Self;
}

pub struct RgbaLayers {
    red: RedLayer,
    green: GreenLayer,
    blue: BlueLayer,
    alpha: AlphaLayer,
}

impl RgbaLayers {
    pub fn new(width: usize, height: usize) -> RgbaLayers {
        RgbaLayers {
            red: RedLayer::new(width, height, 0),
            green: GreenLayer::new(width, height, 0),
            blue: BlueLayer::new(width, height, 0),
            alpha: AlphaLayer::new(width, height, 255),
        }
    }

    pub fn from_layers(
        red: RedLayer,
        green: GreenLayer,
        blue: BlueLayer,
        alpha: AlphaLayer,
    ) -> RgbaLayers {
        RgbaLayers {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> RgbaPixel {
        RgbaPixel {
            red: *self.red.get(x, y),
            green: *self.green.get(x, y),
            blue: *self.blue.get(x, y),
            alpha: *self.alpha.get(x, y),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: RgbaPixel, alpha: u8) {
        self.red.set(x, y, pixel.red);
        self.green.set(x, y, pixel.green);
        self.blue.set(x, y, pixel.blue);
        self.alpha.set(x, y, alpha);
    }

    pub fn get_height(&self) -> usize {
        self.red.height
    }

    pub fn get_width(&self) -> usize {
        self.red.width
    }

    pub fn get_red(&self) -> &RedLayer {
        &self.red
    }

    pub fn get_green(&self) -> &GreenLayer {
        &self.green
    }

    pub fn get_blue(&self) -> &BlueLayer {
        &self.blue
    }

    pub fn get_alpha(&self) -> &AlphaLayer {
        &self.alpha
    }

    pub fn get_red_mut(&mut self) -> &mut RedLayer {
        &mut self.red
    }

    pub fn get_green_mut(&mut self) -> &mut GreenLayer {
        &mut self.green
    }

    pub fn get_blue_mut(&mut self) -> &mut BlueLayer {
        &mut self.blue
    }

    pub fn get_alpha_mut(&mut self) -> &mut AlphaLayer {
        &mut self.alpha
    }

    pub fn get_all(self) -> (RedLayer, GreenLayer, BlueLayer, AlphaLayer) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

pub struct HsvaLayers {
    hue: HueLayer,
    saturation: SaturationLayer,
    value: ValueLayer,
    alpha: AlphaLayer,
}

impl HsvaLayers {
    pub fn new(width: usize, height: usize) -> HsvaLayers {
        HsvaLayers {
            hue: HueLayer::new(width, height, 0.0),
            saturation: SaturationLayer::new(width, height, 0.0),
            value: ValueLayer::new(width, height, 0.0),
            alpha: AlphaLayer::new(width, height, 255),
        }
    }

    pub fn from_layers(
        hue: HueLayer,
        saturation: SaturationLayer,
        value: ValueLayer,
        alpha: AlphaLayer,
    ) -> HsvaLayers {
        HsvaLayers {
            hue,
            saturation,
            value,
            alpha,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> HsvaPixel {
        HsvaPixel {
            hue: *self.hue.get(x, y),
            saturation: *self.saturation.get(x, y),
            value: *self.value.get(x, y),
            alpha: *self.alpha.get(x, y),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: HsvaPixel, alpha: u8) {
        self.hue.set(x, y, pixel.hue);
        self.saturation.set(x, y, pixel.saturation);
        self.value.set(x, y, pixel.value);
        self.alpha.set(x, y, alpha);
    }

    pub fn get_height(&self) -> usize {
        self.hue.height
    }

    pub fn get_width(&self) -> usize {
        self.hue.width
    }

    pub fn get_hue(&self) -> &HueLayer {
        &self.hue
    }

    pub fn get_saturation(&self) -> &SaturationLayer {
        &self.saturation
    }

    pub fn get_value(&self) -> &ValueLayer {
        &self.value
    }

    pub fn get_alpha(&self) -> &AlphaLayer {
        &self.alpha
    }

    pub fn get_hue_mut(&mut self) -> &mut HueLayer {
        &mut self.hue
    }

    pub fn get_saturation_mut(&mut self) -> &mut SaturationLayer {
        &mut self.saturation
    }

    pub fn get_value_mut(&mut self) -> &mut ValueLayer {
        &mut self.value
    }

    pub fn get_alpha_mut(&mut self) -> &mut AlphaLayer {
        &mut self.alpha
    }
    pub fn get_all(self) -> (HueLayer, SaturationLayer, ValueLayer, AlphaLayer) {
        (self.hue, self.saturation, self.value, self.alpha)
    }
}

pub struct HslaLayers {
    hue: HueLayer,
    saturation: SaturationLayer,
    lightness: LightnessLayer,
    alpha: AlphaLayer,
}

impl HslaLayers {
    pub fn new(width: usize, height: usize) -> HslaLayers {
        HslaLayers {
            hue: HueLayer::new(width, height, 0.0),
            saturation: SaturationLayer::new(width, height, 0.0),
            lightness: LightnessLayer::new(width, height, 0.0),
            alpha: AlphaLayer::new(width, height, 255),
        }
    }

    pub fn from_layers(
        hue: HueLayer,
        saturation: SaturationLayer,
        lightness: LightnessLayer,
        alpha: AlphaLayer,
    ) -> HslaLayers {
        HslaLayers {
            hue,
            saturation,
            lightness,
            alpha,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> HslaPixel {
        HslaPixel {
            hue: *self.hue.get(x, y),
            saturation: *self.saturation.get(x, y),
            lightness: *self.lightness.get(x, y),
            alpha: *self.alpha.get(x, y),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, pixel: HslaPixel, alpha: u8) {
        self.hue.set(x, y, pixel.hue);
        self.saturation.set(x, y, pixel.saturation);
        self.lightness.set(x, y, pixel.lightness);
        self.alpha.set(x, y, alpha);
    }

    pub fn get_height(&self) -> usize {
        self.hue.height
    }

    pub fn get_width(&self) -> usize {
        self.hue.width
    }

    pub fn get_hue(&self) -> &HueLayer {
        &self.hue
    }

    pub fn get_saturation(&self) -> &SaturationLayer {
        &self.saturation
    }

    pub fn get_lightness(&self) -> &LightnessLayer {
        &self.lightness
    }

    pub fn get_alpha(&self) -> &AlphaLayer {
        &self.alpha
    }

    pub fn get_hue_mut(&mut self) -> &mut HueLayer {
        &mut self.hue
    }

    pub fn get_saturation_mut(&mut self) -> &mut SaturationLayer {
        &mut self.saturation
    }

    pub fn get_lightness_mut(&mut self) -> &mut LightnessLayer {
        &mut self.lightness
    }

    pub fn get_alpha_mut(&mut self) -> &mut AlphaLayer {
        &mut self.alpha
    }
    pub fn get_all(self) -> (HueLayer, SaturationLayer, LightnessLayer, AlphaLayer) {
        (self.hue, self.saturation, self.lightness, self.alpha)
    }
}

pub struct LaLayers {
    lightness: LightnessLayer,
    alpha: AlphaLayer,
}

impl LaLayers {
    pub fn new(width: usize, height: usize) -> LaLayers {
        LaLayers {
            lightness: LightnessLayer::new(width, height, 0.0),
            alpha: AlphaLayer::new(width, height, 255),
        }
    }

    pub fn from_layers(lightness: LightnessLayer, alpha: AlphaLayer) -> LaLayers {
        LaLayers { lightness, alpha }
    }

    pub fn get(&self, x: usize, y: usize) -> (f32, u8) {
        (*self.lightness.get(x, y), *self.alpha.get(x, y))
    }

    pub fn set(&mut self, x: usize, y: usize, lightness: f32, alpha: u8) {
        self.lightness.set(x, y, lightness);
        self.alpha.set(x, y, alpha);
    }

    pub fn get_height(&self) -> usize {
        self.lightness.height
    }

    pub fn get_width(&self) -> usize {
        self.lightness.width
    }

    pub fn get_lightness(&self) -> &LightnessLayer {
        &self.lightness
    }

    pub fn get_alpha(&self) -> &AlphaLayer {
        &self.alpha
    }

    pub fn get_lightness_mut(&mut self) -> &mut LightnessLayer {
        &mut self.lightness
    }

    pub fn get_alpha_mut(&mut self) -> &mut AlphaLayer {
        &mut self.alpha
    }
    pub fn get_all(self) -> (LightnessLayer, AlphaLayer) {
        (self.lightness, self.alpha)
    }
}
