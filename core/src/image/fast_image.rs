use image::{DynamicImage, GenericImage, GenericImageView, Rgba, RgbaImage};
use rayon::prelude::*;

use crate::color::hsl_conversions::{hsl_to_rgb, rgb_to_hsl};
use crate::color::hsv_conversions::{hsv_to_rgb, rgb_to_hsv};
use crate::error::{PicturifyError, PicturifyResult};
use crate::image::io::{ReadFromFile, WriteToFile};
use crate::image::layer::{
    HslaLayered, HslaLayers, HsvaLayered, HsvaLayers, LaLayered, LaLayers, LightnessLayer,
    RgbaLayered, RgbaLayers,
};
use crate::image::pixel::{HslaPixel, HsvaPixel, RgbaPixel};
use crate::image::virtual_image::{
    VirtualHslImage, VirtualHsvaImage, VirtualImage, VirtualRgbaImage,
};

#[derive(Debug)]
pub struct FastImage {
    dynamic_image: DynamicImage,
}

impl FastImage {
    pub fn to_grayscale(&mut self) {
        match &self.dynamic_image {
            DynamicImage::ImageLuma8(_) => {}
            _ => self.dynamic_image = DynamicImage::ImageLuma8(self.dynamic_image.to_luma8()),
        }
    }

    pub fn to_color(&mut self) {
        match &self.dynamic_image {
            DynamicImage::ImageRgba8(_) => {}
            _ => self.dynamic_image = DynamicImage::ImageRgba8(self.dynamic_image.to_rgba8()),
        }
    }
}

impl VirtualImage for FastImage {
    fn get_width(&self) -> usize {
        self.dynamic_image.width() as usize
    }

    fn get_height(&self) -> usize {
        self.dynamic_image.height() as usize
    }
}

impl VirtualRgbaImage for FastImage {
    fn get_rgba(&self, x: usize, y: usize) -> RgbaPixel {
        let pixel = self.dynamic_image.get_pixel(x as u32, y as u32);
        RgbaPixel {
            red: pixel.0[0],
            green: pixel.0[1],
            blue: pixel.0[2],
            alpha: pixel.0[3],
        }
    }

    fn set_rgba(&mut self, x: usize, y: usize, pixel: RgbaPixel) {
        let mut existing_pixel = self.dynamic_image.get_pixel(x as u32, y as u32);
        existing_pixel.0[0] = pixel.red;
        existing_pixel.0[1] = pixel.green;
        existing_pixel.0[2] = pixel.blue;
        self.dynamic_image
            .put_pixel(x as u32, y as u32, existing_pixel);
    }

    fn iterate_rgba<F>(&mut self, f: F)
    where
        F: Fn(&mut RgbaPixel, usize, usize),
    {
        self.to_color();
        self.dynamic_image
            .as_mut_rgba8()
            .unwrap()
            .enumerate_rows_mut()
            .for_each(|(_, row)| {
                for (_, (x, y, mut rgba)) in row.enumerate() {
                    Self::invoke_func_on_rgba_pixel(x as usize, y as usize, &mut rgba, &f);
                }
            });
    }

    fn iterate_par_rgba<F>(&mut self, f: F)
    where
        F: Fn(&mut RgbaPixel, usize, usize) + Sync + Send,
    {
        self.to_color();
        self.dynamic_image
            .as_mut_rgba8()
            .unwrap()
            .enumerate_rows_mut()
            .par_bridge()
            .for_each(|(_, row)| {
                for (_, (x, y, mut rgba)) in row.enumerate() {
                    Self::invoke_func_on_rgba_pixel(x as usize, y as usize, &mut rgba, &f);
                }
            });
    }
}

impl FastImage {
    #[inline(always)]
    fn invoke_func_on_rgba_pixel<F>(x: usize, y: usize, rgba: &mut Rgba<u8>, f: &F)
    where
        F: Fn(&mut RgbaPixel, usize, usize),
    {
        let mut rgba_pixel = RgbaPixel {
            red: rgba[0],
            green: rgba[1],
            blue: rgba[2],
            alpha: rgba[3],
        };
        f(&mut rgba_pixel, x, y);
        rgba[0] = rgba_pixel.red;
        rgba[1] = rgba_pixel.green;
        rgba[2] = rgba_pixel.blue;
        rgba[3] = rgba_pixel.alpha;
    }
}

impl VirtualHsvaImage for FastImage {
    fn get_hsva(&self, x: usize, y: usize) -> HsvaPixel {
        let pixel = self.get_rgba(x, y);
        let (hue, saturation, value) = rgb_to_hsv(pixel.red, pixel.green, pixel.blue);
        HsvaPixel {
            hue,
            saturation,
            value,
            alpha: pixel.alpha,
        }
    }

    fn set_hsva(&mut self, x: usize, y: usize, pixel: HsvaPixel) {
        let (red, green, blue) = hsv_to_rgb(pixel.hue, pixel.saturation, pixel.value);
        self.set_rgba(
            x,
            y,
            RgbaPixel {
                red,
                green,
                blue,
                alpha: pixel.alpha,
            },
        );
    }

    fn iterate_hsva<F>(&mut self, f: F)
    where
        F: Fn(&mut HsvaPixel, usize, usize),
    {
        self.to_color();
        self.dynamic_image
            .as_mut_rgba8()
            .unwrap()
            .enumerate_rows_mut()
            .for_each(|(_, row)| {
                for (_, (x, y, mut rgba)) in row.enumerate() {
                    Self::invoke_func_on_hsva_pixel(x as usize, y as usize, &mut rgba, &f);
                }
            });
    }

    fn iterate_par_hsva<F>(&mut self, f: F)
    where
        F: Fn(&mut HsvaPixel, usize, usize) + Sync + Send,
    {
        self.to_color();
        self.dynamic_image
            .as_mut_rgba8()
            .unwrap()
            .enumerate_rows_mut()
            .par_bridge()
            .for_each(|(_, row)| {
                for (_, (x, y, mut rgba)) in row.enumerate() {
                    Self::invoke_func_on_hsva_pixel(x as usize, y as usize, &mut rgba, &f);
                }
            });
    }
}

impl FastImage {
    #[inline(always)]
    fn invoke_func_on_hsva_pixel<F>(x: usize, y: usize, rgba: &mut Rgba<u8>, f: &F)
    where
        F: Fn(&mut HsvaPixel, usize, usize),
    {
        let mut hsva_pixel = HsvaPixel {
            hue: 0.0,
            saturation: 0.0,
            value: 0.0,
            alpha: rgba[3],
        };
        let (red, green, blue) = (rgba[0], rgba[1], rgba[2]);
        let (hue, saturation, value) = rgb_to_hsv(red, green, blue);
        hsva_pixel.hue = hue;
        hsva_pixel.saturation = saturation;
        hsva_pixel.value = value;
        f(&mut hsva_pixel, x, y);
        let (red, green, blue) =
            hsv_to_rgb(hsva_pixel.hue, hsva_pixel.saturation, hsva_pixel.value);
        rgba[0] = red;
        rgba[1] = green;
        rgba[2] = blue;
        rgba[3] = hsva_pixel.alpha;
    }
}

impl VirtualHslImage for FastImage {
    fn get_hsla(&self, x: usize, y: usize) -> HslaPixel {
        let pixel = self.get_rgba(x, y);
        let (hue, saturation, lightness) = rgb_to_hsl(pixel.red, pixel.green, pixel.blue);
        HslaPixel {
            hue,
            saturation,
            lightness,
            alpha: pixel.alpha,
        }
    }

    fn set_hsla(&mut self, x: usize, y: usize, pixel: HslaPixel) {
        let (red, green, blue) = hsl_to_rgb(pixel.hue, pixel.saturation, pixel.lightness);
        self.set_rgba(
            x,
            y,
            RgbaPixel {
                red,
                green,
                blue,
                alpha: pixel.alpha,
            },
        );
    }

    fn iterate_hsla<F>(&mut self, f: F)
    where
        F: Fn(&mut HslaPixel, usize, usize),
    {
        self.to_color();
        self.dynamic_image
            .as_mut_rgba8()
            .unwrap()
            .enumerate_rows_mut()
            .for_each(|(_, row)| {
                for (_, (x, y, mut rgba)) in row.enumerate() {
                    Self::invoke_func_on_hsla_pixel(x as usize, y as usize, &mut rgba, &f);
                }
            });
    }

    fn iterate_par_hsla<F>(&mut self, f: F)
    where
        F: Fn(&mut HslaPixel, usize, usize) + Sync + Send,
    {
        self.to_color();
        self.dynamic_image
            .as_mut_rgba8()
            .unwrap()
            .enumerate_rows_mut()
            .par_bridge()
            .for_each(|(_, row)| {
                for (_, (x, y, mut rgba)) in row.enumerate() {
                    Self::invoke_func_on_hsla_pixel(x as usize, y as usize, &mut rgba, &f);
                }
            });
    }
}

impl FastImage {
    #[inline(always)]
    fn invoke_func_on_hsla_pixel<F>(x: usize, y: usize, rgba: &mut Rgba<u8>, f: &F)
    where
        F: Fn(&mut HslaPixel, usize, usize),
    {
        let mut hsla_pixel = HslaPixel {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.0,
            alpha: rgba[3],
        };
        let (red, green, blue) = (rgba[0], rgba[1], rgba[2]);
        let (hue, saturation, lightness) = rgb_to_hsl(red, green, blue);
        hsla_pixel.hue = hue;
        hsla_pixel.saturation = saturation;
        hsla_pixel.lightness = lightness;
        f(&mut hsla_pixel, x, y);
        let (red, green, blue) =
            hsl_to_rgb(hsla_pixel.hue, hsla_pixel.saturation, hsla_pixel.lightness);
        rgba[0] = red;
        rgba[1] = green;
        rgba[2] = blue;
        rgba[3] = hsla_pixel.alpha;
    }
}

impl ReadFromFile for FastImage {
    fn read_from_file(path: &str) -> PicturifyResult<Box<Self>> {
        let dynamic_image = image::io::Reader::open(path)?.decode()?;
        Ok(Box::new(FastImage { dynamic_image }))
    }
}

impl WriteToFile for FastImage {
    fn write_to_file(&self, path: &str) -> PicturifyResult<()> {
        self.dynamic_image.save(path)?;
        Ok(())
    }
}

impl RgbaLayered for FastImage {
    fn get_rgba_layers(&self) -> RgbaLayers {
        let (width, height) = (self.get_width(), self.get_height());
        let mut rgba_layers = RgbaLayers::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_rgba(x, y);
                let alpha = self.get_alpha(x, y);
                rgba_layers.set(x, y, pixel, alpha);
            }
        }

        rgba_layers
    }

    fn from_rgba_layers(rgba_layers: RgbaLayers) -> Self {
        let mut dynamic_image = DynamicImage::new_rgba8(
            rgba_layers.get_width() as u32,
            rgba_layers.get_height() as u32,
        );

        for y in 0..rgba_layers.get_height() {
            for x in 0..rgba_layers.get_width() {
                let rgba = rgba_layers.get(x, y);
                dynamic_image.put_pixel(
                    x as u32,
                    y as u32,
                    image::Rgba([rgba.red, rgba.green, rgba.blue, rgba.alpha]),
                );
            }
        }

        FastImage { dynamic_image }
    }
}

impl HsvaLayered for FastImage {
    fn get_hsva_layers(&self) -> HsvaLayers {
        let (width, height) = (self.get_width(), self.get_height());
        let mut hsva_layers = HsvaLayers::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_hsva(x, y);
                let alpha = self.get_alpha(x, y);
                hsva_layers.set(x, y, pixel, alpha);
            }
        }

        hsva_layers
    }

    fn from_hsva_layers(hsva_layers: HsvaLayers) -> Self {
        let mut dynamic_image = DynamicImage::new_rgba8(
            hsva_layers.get_width() as u32,
            hsva_layers.get_height() as u32,
        );

        for y in 0..hsva_layers.get_height() {
            for x in 0..hsva_layers.get_width() {
                let hsva = hsva_layers.get(x, y);
                let (red, green, blue) = hsv_to_rgb(hsva.hue, hsva.saturation, hsva.value);
                dynamic_image.put_pixel(
                    x as u32,
                    y as u32,
                    image::Rgba([red, green, blue, hsva.alpha]),
                );
            }
        }

        FastImage { dynamic_image }
    }
}

impl HslaLayered for FastImage {
    fn get_hsla_layers(&self) -> HslaLayers {
        let (width, height) = (self.get_width(), self.get_height());
        let mut hsla_layers = HslaLayers::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_hsla(x, y);
                let alpha = self.get_alpha(x, y);
                hsla_layers.set(x, y, pixel, alpha);
            }
        }

        hsla_layers
    }

    fn from_hsla_layers(hsla_layers: HslaLayers) -> Self {
        let mut dynamic_image = DynamicImage::new_rgba8(
            hsla_layers.get_width() as u32,
            hsla_layers.get_height() as u32,
        );

        for y in 0..hsla_layers.get_height() {
            for x in 0..hsla_layers.get_width() {
                let hsla = hsla_layers.get(x, y);
                let (red, green, blue) = hsl_to_rgb(hsla.hue, hsla.saturation, hsla.lightness);
                dynamic_image.put_pixel(
                    x as u32,
                    y as u32,
                    image::Rgba([red, green, blue, hsla.alpha]),
                );
            }
        }

        FastImage { dynamic_image }
    }
}

impl LaLayered for FastImage {
    fn get_la_layers(&self) -> PicturifyResult<LaLayers> {
        if let DynamicImage::ImageLumaA8(image) = &self.dynamic_image {
            let (width, height) = (self.get_width(), self.get_height());
            let mut la_layers = LaLayers::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let pixel = image.get_pixel(x as u32, y as u32);
                    la_layers.set(x, y, pixel.0[0] as f32 / 255., pixel.0[1]);
                }
            }

            Ok(la_layers)
        } else {
            Err(PicturifyError::InvalidImageFormat)
        }
    }

    fn from_la_layers(la_layers: LaLayers) -> Self {
        let mut dynamic_image = DynamicImage::ImageLumaA8(image::ImageBuffer::new(
            la_layers.get_width() as u32,
            la_layers.get_height() as u32,
        ));

        for y in 0..la_layers.get_height() {
            for x in 0..la_layers.get_width() {
                let (lightness, alpha) = la_layers.get(x, y);
                let lightness = (lightness * 255.) as u8;
                dynamic_image.put_pixel(
                    x as u32,
                    y as u32,
                    image::Rgba([lightness, lightness, lightness, alpha]),
                );
            }
        }

        FastImage { dynamic_image }
    }
}
