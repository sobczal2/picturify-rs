use picturify_core::error::PicturifyResult;
use std::marker::PhantomData;

use picturify_core::image::fast_image::FastImage;
use picturify_core::image::layer::{
    AlphaLayer, BlueLayer, GreenLayer, HslaLayered, HslaLayers, HsvaLayered, HsvaLayers, HueLayer,
    LaLayered, LaLayers, LightnessLayer, RedLayer, RgbaLayered, RgbaLayers, SaturationLayer,
    ValueLayer,
};

use crate::common::channel::ChannelSelector;
use crate::common::execution::ExecutionPlan;

pub trait Processor {
    fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) -> PicturifyResult<()>;
    fn set_channel_selector(&mut self, channel_selector: ChannelSelector) -> PicturifyResult<()>;
    fn process(&self, fast_image: FastImage) -> FastImage;
}

struct NotSelected;
struct LayersPrepared;
struct FinalImagePrepared;

pub struct LayerPipe<T> {
    _state: PhantomData<T>,
    channel_selector: Option<ChannelSelector>,
    red_layer: Option<RedLayer>,
    green_layer: Option<GreenLayer>,
    blue_layer: Option<BlueLayer>,
    alpha_layer: Option<AlphaLayer>,
    hue_layer: Option<HueLayer>,
    saturation_layer: Option<SaturationLayer>,
    lightness_layer: Option<LightnessLayer>,
    value_layer: Option<ValueLayer>,
    final_image: Option<FastImage>,
}

impl LayerPipe<NotSelected> {
    pub fn new() -> LayerPipe<NotSelected> {
        LayerPipe {
            _state: PhantomData,
            channel_selector: None,
            red_layer: None,
            green_layer: None,
            blue_layer: None,
            alpha_layer: None,
            hue_layer: None,
            saturation_layer: None,
            lightness_layer: None,
            value_layer: None,
            final_image: None,
        }
    }

    pub fn prepare_layers(
        mut self,
        fast_image: FastImage,
        channel_selector: ChannelSelector,
    ) -> LayerPipe<LayersPrepared> {
        match channel_selector {
            ChannelSelector::Rgba(_) => {
                self.prepare_rgba_layers(fast_image);
            }
            ChannelSelector::Hsva(_) => {
                self.prepare_hsva_layers(fast_image);
            }
            ChannelSelector::Hsla(_) => {
                self.prepare_hsla_layers(fast_image);
            }
            ChannelSelector::La(_) => {
                self.prepare_la_layers(fast_image);
            }
        }

        self.channel_selector = Some(channel_selector);

        LayerPipe {
            _state: PhantomData,
            channel_selector: self.channel_selector,
            red_layer: self.red_layer,
            green_layer: self.green_layer,
            blue_layer: self.blue_layer,
            alpha_layer: self.alpha_layer,
            hue_layer: self.hue_layer,
            saturation_layer: self.saturation_layer,
            lightness_layer: self.lightness_layer,
            value_layer: self.value_layer,
            final_image: None,
        }
    }

    fn prepare_rgba_layers(&mut self, fast_image: FastImage) {
        let rgba_layers = fast_image.get_rgba_layers();
        let (red, green, blue, alpha) = rgba_layers.get_all();
        self.red_layer = Some(red);
        self.green_layer = Some(green);
        self.blue_layer = Some(blue);
        self.alpha_layer = Some(alpha);
    }

    fn prepare_hsva_layers(&mut self, fast_image: FastImage) {
        let hsva_layers = fast_image.get_hsva_layers();
        let (hue, saturation, value, alpha) = hsva_layers.get_all();
        self.hue_layer = Some(hue);
        self.saturation_layer = Some(saturation);
        self.value_layer = Some(value);
        self.alpha_layer = Some(alpha);
    }

    fn prepare_hsla_layers(&mut self, fast_image: FastImage) {
        let hsla_layers = fast_image.get_hsla_layers();
        let (hue, saturation, lightness, alpha) = hsla_layers.get_all();
        self.hue_layer = Some(hue);
        self.saturation_layer = Some(saturation);
        self.lightness_layer = Some(lightness);
        self.alpha_layer = Some(alpha);
    }

    fn prepare_la_layers(&mut self, fast_image: FastImage) {
        let la_layers = fast_image.get_la_layers().expect("Failed to get LA layers, image is not Grayscale. Calling to_grayscale() method will help. If this is frequently a problem, consider refactoring this method to return a Result.");
        let (lightness, alpha) = la_layers.get_all();
        self.lightness_layer = Some(lightness);
        self.alpha_layer = Some(alpha);
    }
}

impl LayerPipe<LayersPrepared> {
    pub fn get_red_mut(&mut self) -> &mut RedLayer {
        self.red_layer.as_mut().expect("Red layer not prepared")
    }

    pub fn get_green_mut(&mut self) -> &mut GreenLayer {
        self.green_layer.as_mut().expect("Green layer not prepared")
    }

    pub fn get_blue_mut(&mut self) -> &mut BlueLayer {
        self.blue_layer.as_mut().expect("Blue layer not prepared")
    }

    pub fn get_alpha_mut(&mut self) -> &mut AlphaLayer {
        self.alpha_layer.as_mut().expect("Alpha layer not prepared")
    }

    pub fn get_hue_mut(&mut self) -> &mut HueLayer {
        self.hue_layer.as_mut().expect("Hue layer not prepared")
    }

    pub fn get_saturation_mut(&mut self) -> &mut SaturationLayer {
        self.saturation_layer
            .as_mut()
            .expect("Saturation layer not prepared")
    }

    pub fn get_lightness_mut(&mut self) -> &mut LightnessLayer {
        self.lightness_layer
            .as_mut()
            .expect("Lightness layer not prepared")
    }

    pub fn get_value_mut(&mut self) -> &mut ValueLayer {
        self.value_layer.as_mut().expect("Value layer not prepared")
    }

    pub fn prepare_final_image(mut self) -> LayerPipe<FinalImagePrepared> {
        let final_image = match self.channel_selector.as_ref().unwrap() {
            ChannelSelector::Rgba(_) => FastImage::from_rgba_layers(RgbaLayers::from_layers(
                self.red_layer.take().unwrap(),
                self.green_layer.take().unwrap(),
                self.blue_layer.take().unwrap(),
                self.alpha_layer.take().unwrap(),
            )),
            ChannelSelector::Hsva(_) => FastImage::from_hsva_layers(HsvaLayers::from_layers(
                self.hue_layer.take().unwrap(),
                self.saturation_layer.take().unwrap(),
                self.value_layer.take().unwrap(),
                self.alpha_layer.take().unwrap(),
            )),
            ChannelSelector::Hsla(_) => FastImage::from_hsla_layers(HslaLayers::from_layers(
                self.hue_layer.take().unwrap(),
                self.saturation_layer.take().unwrap(),
                self.lightness_layer.take().unwrap(),
                self.alpha_layer.take().unwrap(),
            )),
            ChannelSelector::La(_) => FastImage::from_la_layers(LaLayers::from_layers(
                self.lightness_layer.take().unwrap(),
                self.alpha_layer.take().unwrap(),
            )),
        };

        LayerPipe {
            _state: PhantomData,
            channel_selector: self.channel_selector,
            red_layer: self.red_layer,
            green_layer: self.green_layer,
            blue_layer: self.blue_layer,
            alpha_layer: self.alpha_layer,
            hue_layer: self.hue_layer,
            saturation_layer: self.saturation_layer,
            lightness_layer: self.lightness_layer,
            value_layer: self.value_layer,
            final_image: Some(final_image),
        }
    }
}

impl LayerPipe<FinalImagePrepared> {
    pub fn get_final_image(self) -> FastImage {
        self.final_image.expect("Final image not prepared")
    }
}
