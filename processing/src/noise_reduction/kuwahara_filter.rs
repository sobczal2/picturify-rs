use rayon::prelude::*;
use picturify_core::image::image::Image;
use picturify_core::pixel::color::{ColorChannel, ColorSpace};
use picturify_core::pixel::pixel::{HSLAPixelValue, HSVAPixelValue, Pixel, PixelValue};

use crate::core::processor::Processor;

pub struct KuwaharaFilter {
    options: KuwaharaFilterOptions,
}

impl KuwaharaFilter {
    pub fn new(options: KuwaharaFilterOptions) -> Self {
        KuwaharaFilter {
            options,
        }
    }
}

impl Default for KuwaharaFilter {
    fn default() -> Self {
        KuwaharaFilter {
            options: KuwaharaFilterOptions::default(),
        }
    }
}

pub struct KuwaharaFilterOptions {
    pub window_size: u32,
}

impl Default for KuwaharaFilterOptions {
    fn default() -> Self {
        KuwaharaFilterOptions {
            window_size: 5,
        }
    }
}

impl Processor for KuwaharaFilter {
    fn process(&self, image: Image) -> Image {
        let mut new_image_data: Vec<Pixel> = vec![Pixel::HSLA(HSLAPixelValue::default()); (image.size().width * image.size().height) as usize];
        let quadrant_size = (self.options.window_size as f32 / 2.0).ceil() as u32;
        let image_width = image.size().width as u32;
        let image_height = image.size().height as u32;

        new_image_data.par_chunks_mut(image_width as usize).enumerate().for_each(|(y, row)| {
            for x in 0..image_width as usize {
                let tl_x = x as i32 - (self.options.window_size as i32 / 2);
                let tl_y = y as i32 - (self.options.window_size as i32 / 2);

                let clamp_x = |v: i32| -> usize { v.max(0).min(image_width as i32 - 1) as usize };
                let clamp_y = |v: i32| -> usize { v.max(0).min(image_height as i32 - 1) as usize };

                let quadrant_a = (clamp_x(tl_x)..clamp_x(tl_x + quadrant_size as i32), clamp_y(tl_y)..clamp_y(tl_y + quadrant_size as i32));
                let quadrant_b = (clamp_x(tl_x)..clamp_x(tl_x + quadrant_size as i32), clamp_y(tl_y + quadrant_size as i32)..clamp_y(tl_y + self.options.window_size as i32));
                let quadrant_c = (clamp_x(tl_x + quadrant_size as i32)..clamp_x(tl_x + self.options.window_size as i32), clamp_y(tl_y)..clamp_y(tl_y + quadrant_size as i32));
                let quadrant_d = (clamp_x(tl_x + quadrant_size as i32)..clamp_x(tl_x + self.options.window_size as i32), clamp_y(tl_y + quadrant_size as i32)..clamp_y(tl_y + self.options.window_size as i32));

                let quadrant_a_std_dev = calculate_std_dev(&image, quadrant_a.clone());
                let quadrant_b_std_dev = calculate_std_dev(&image, quadrant_b.clone());
                let quadrant_c_std_dev = calculate_std_dev(&image, quadrant_c.clone());
                let quadrant_d_std_dev = calculate_std_dev(&image, quadrant_d.clone());

                let mut quadrants = vec![
                    (quadrant_a, quadrant_a_std_dev),
                    (quadrant_b, quadrant_b_std_dev),
                    (quadrant_c, quadrant_c_std_dev),
                    (quadrant_d, quadrant_d_std_dev),
                ];

                quadrants = quadrants.into_iter().filter(|(_, std_dev)| *std_dev != f32::MAX).collect();

                let selected_quadrant = quadrants.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0.clone();

                let mut brightness_sum = 0.0;
                let mut count = 0;

                for y in selected_quadrant.1.clone() {
                    for x in selected_quadrant.0.clone() {
                        let pixel = image.get_pixel(x, y);
                        brightness_sum += pixel.get(ColorChannel::Lightness);
                        count += 1;
                    }
                }

                let brightness = brightness_sum / count as f32;
                let pixel = image.get_pixel(x, y);
                let mut new_pixel = pixel.to_color_space(ColorSpace::HSLA);
                new_pixel.set(ColorChannel::Lightness, brightness).unwrap();
                row[x] = new_pixel;
            }
        });

        Image::from_1d_vec(new_image_data, image.size())
    }
}

fn calculate_std_dev(image: &Image, quadrant: (std::ops::Range<usize>, std::ops::Range<usize>)) -> f32 {
    let mut sum = 0.0;
    let mut sum_squared = 0.0;
    let mut count = 0;

    for y in quadrant.1.clone() {
        for x in quadrant.0.clone() {
            let pixel = image.get_pixel(x, y);
            let value = pixel.get(ColorChannel::Lightness);
            sum += value;
            sum_squared += value * value;
            count += 1;
        }
    }

    if count == 0 {
        return f32::MAX;
    }

    let mean = sum / count as f32;
    let variance = (sum_squared - mean * mean) / count as f32;
    variance.sqrt()
}