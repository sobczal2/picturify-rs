use image::{DynamicImage, GenericImageView, RgbaImage};
use rayon::prelude::*;
use picturify_core::colors::hsl_conversions::hsl_to_rgb;
use picturify_core::images::virtual_image::{VirtualHSLImage, VirtualImage};

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
    fn process(&self, image: DynamicImage) -> DynamicImage {
        let (image_width, image_height) = image.dimensions();
        let mut new_image_data: Vec<u8> = vec![0; (image_width * image_height * 4) as usize];
        let quadrant_size = (self.options.window_size as f32 / 2.0).ceil() as u32;

        new_image_data.par_chunks_mut((image_width * 4) as usize).enumerate().for_each(|(y, row)| {
            for x in 0..image_width as usize {
                let tl_x = x as i32 - (self.options.window_size as i32 / 2);
                let tl_y = y as i32 - (self.options.window_size as i32 / 2);

                let clamp_x = |v: i32| -> usize { v.max(0).min(image_width as i32 - 1) as usize };
                let clamp_y = |v: i32| -> usize { v.max(0).min(image_height as i32 - 1) as usize };

                let quadrant_a = (clamp_x(tl_x)..clamp_x(tl_x + quadrant_size as i32), clamp_y(tl_y)..clamp_y(tl_y + quadrant_size as i32));
                let quadrant_b = (clamp_x(tl_x)..clamp_x(tl_x + quadrant_size as i32), clamp_y(tl_y + quadrant_size as i32)..clamp_y(tl_y + self.options.window_size as i32));
                let quadrant_c = (clamp_x(tl_x + quadrant_size as i32)..clamp_x(tl_x + self.options.window_size as i32), clamp_y(tl_y)..clamp_y(tl_y + quadrant_size as i32));
                let quadrant_d = (clamp_x(tl_x + quadrant_size as i32)..clamp_x(tl_x + self.options.window_size as i32), clamp_y(tl_y + quadrant_size as i32)..clamp_y(tl_y + self.options.window_size as i32));

                let quadrant_a_variance = calculate_variance(&image, quadrant_a.clone());
                let quadrant_b_quadrant_a_variance = calculate_variance(&image, quadrant_b.clone());
                let quadrant_c_quadrant_a_variance = calculate_variance(&image, quadrant_c.clone());
                let quadrant_d_quadrant_a_variance = calculate_variance(&image, quadrant_d.clone());

                let mut quadrants = vec![
                    (quadrant_a, quadrant_a_variance),
                    (quadrant_b, quadrant_b_quadrant_a_variance),
                    (quadrant_c, quadrant_c_quadrant_a_variance),
                    (quadrant_d, quadrant_d_quadrant_a_variance),
                ];

                let selected_quadrant = quadrants.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0.clone();

                let mut lightness_sum = 0.0;
                let mut count = 0;

                for y in selected_quadrant.1.clone() {
                    for x in selected_quadrant.0.clone() {
                        let pixel = image.get_hsl(x as u32, y as u32);
                        lightness_sum += pixel.2;
                        count += 1;
                    }
                }

                let brightness = lightness_sum / count as f32;
                let pixel = image.get_hsl(x as u32, y as u32);
                let new_pixel = hsl_to_rgb(pixel.0, pixel.1, brightness);
                row[x * 4] = new_pixel.0;
                row[x * 4 + 1] = new_pixel.1;
                row[x * 4 + 2] = new_pixel.2;
                row[x * 4 + 3] = image.get_alpha(x as u32, y as u32);
            }
        });

        DynamicImage::ImageRgba8(RgbaImage::from_raw(image_width, image_height, new_image_data).unwrap())
    }
}

fn calculate_variance(image: &DynamicImage, quadrant: (std::ops::Range<usize>, std::ops::Range<usize>)) -> f32 {
    let mut sum = 0.0;
    let mut sum_squared = 0.0;
    let mut count = 0;

    for y in quadrant.1.clone() {
        for x in quadrant.0.clone() {
            let pixel = image.get_hsl(x as u32, y as u32);
            sum += pixel.2;
            sum_squared += pixel.2 * pixel.2;
            count += 1;
        }
    }

    if count == 0 {
        return f32::MAX;
    }

    let mean = sum / count as f32;
    (sum_squared - mean * mean) / count as f32
}