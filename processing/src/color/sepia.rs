use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;
use picturify_core::images::virtual_image::{VirtualImage, VirtualRGBImage};
use crate::core::processor::{Processor, ProcessorRunner, RayonCpuOptions};

pub struct SepiaFilter {
    options: SepiaFilterOptions,
}

impl SepiaFilter {
    pub fn new(options: SepiaFilterOptions) -> Self {
        SepiaFilter {
            options,
        }
    }
}

impl Default for SepiaFilter {
    fn default() -> Self {
        SepiaFilter {
            options: SepiaFilterOptions::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct SepiaFilterOptions {}

impl Default for SepiaFilterOptions {
    fn default() -> Self {
        SepiaFilterOptions {}
    }
}

impl Processor for SepiaFilter {
    fn process(&self, image: DynamicImage, processor_runner: &ProcessorRunner) -> DynamicImage {
        match processor_runner {
            ProcessorRunner::RayonCpu(rayon_cpu_options) => process_rayon_cpu(image, rayon_cpu_options, self.options),
            ProcessorRunner::CudaGpu => unimplemented!(),
        }
    }
}

fn process_rayon_cpu(image: DynamicImage, rayon_cpu_options: &RayonCpuOptions, sepia_filter_options: SepiaFilterOptions) -> DynamicImage {
    let (image_width, image_height) = image.dimensions();
    let mut new_image_data: Vec<u8> = vec![0; (image_width * image_height * 4) as usize];

    rayon::ThreadPoolBuilder::new().num_threads(rayon_cpu_options.threads).build().unwrap().install(|| {
        new_image_data.par_chunks_mut((image_width * 4) as usize).enumerate().for_each(|(y, row)| for x in 0..image_width as usize {
            let pixel = image.get_rgb(x as u32, y as u32);
            let r = pixel.0;
            let g = pixel.1;
            let b = pixel.2;

            let new_r = (r as f32 * 0.393 + g as f32 * 0.769 + b as f32 * 0.189).min(255.0) as u8;
            let new_g = (r as f32 * 0.349 + g as f32 * 0.686 + b as f32 * 0.168).min(255.0) as u8;
            let new_b = (r as f32 * 0.272 + g as f32 * 0.534 + b as f32 * 0.131).min(255.0) as u8;

            let new_pixel = [new_r, new_g, new_b, image.get_alpha(x as u32, y as u32)];
            row[x * 4..(x + 1) * 4].copy_from_slice(&new_pixel);
        });
    });

    DynamicImage::ImageRgba8(image::ImageBuffer::from_vec(image_width, image_height, new_image_data).unwrap())
}