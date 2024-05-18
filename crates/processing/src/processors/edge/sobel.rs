use crate::common::execution::{Processor, WithOptions};
use picturify_core::fast_image::apply_fn_to_pixels::{
    ApplyFnToImagePixels, ApplyFnToPalettePixels,
};
use picturify_core::fast_image::FastImage;
use picturify_core::rayon::prelude::*;
use picturify_core::threading::progress::Progress;
use std::sync::{Arc, Mutex, RwLock};

pub struct SobelProcessorOptions {
    pub use_fast_approximation: bool,
}

impl Default for SobelProcessorOptions {
    fn default() -> Self {
        SobelProcessorOptions {
            use_fast_approximation: false,
        }
    }
}

pub struct SobelProcessor {
    options: SobelProcessorOptions,
}

const SOBEL_KERNEL_X: [[f32; 3]; 3] = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];

const SOBEL_KERNEL_Y: [[f32; 3]; 3] = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

impl SobelProcessor {
    pub fn new() -> Self {
        SobelProcessor {
            options: Default::default(),
        }
    }
}

impl WithOptions<SobelProcessorOptions> for SobelProcessor {
    fn with_options(self, options: SobelProcessorOptions) -> Self {
        SobelProcessor { options }
    }
}

impl Processor for SobelProcessor {
    fn process(&self, mut fast_image: FastImage, progress: Arc<RwLock<Progress>>) -> FastImage {
        let width = fast_image.get_width();
        let height = fast_image.get_height();

        let mut magnitude_vec = vec![vec![0.0; width - 2]; height - 2];
        let min_magnitude = Arc::new(Mutex::new(f32::MAX));
        let max_magnitude = Arc::new(Mutex::new(f32::MIN));

        progress
            .write()
            .expect("Failed to lock progress")
            .setup((height * 2) as u32);
        magnitude_vec
            .iter_mut()
            .enumerate()
            .par_bridge()
            .for_each(|(y_mag, row)| {
                progress
                    .read()
                    .expect("Failed to lock progress")
                    .increment();
                let mut row_min_magnitude = f32::MAX;
                let mut row_max_magnitude = f32::MIN;
                row.iter_mut().enumerate().for_each(|(x_mag, magnitude)| {
                    let x = x_mag + 1;
                    let y = y_mag + 1;

                    let mut magnitude_x = 0.0;
                    let mut magnitude_y = 0.0;

                    for i in 0..3 {
                        for j in 0..3 {
                            let red: f32;
                            let green: f32;
                            let blue: f32;
                            if self.options.use_fast_approximation {
                                let pixel = fast_image.get_image_pixel(x + i - 1, y + j - 1);
                                red = pixel[0] as f32 / 255.0;
                                green = pixel[1] as f32 / 255.0;
                                blue = pixel[2] as f32 / 255.0;
                            } else {
                                let pixel = fast_image.get_lin_srgba_pixel(x + i - 1, y + j - 1);
                                red = pixel.red;
                                green = pixel.green;
                                blue = pixel.blue;
                            }

                            magnitude_x += SOBEL_KERNEL_X[j][i] * (red + green + blue) / 3.0;
                            magnitude_y += SOBEL_KERNEL_Y[j][i] * (red + green + blue) / 3.0;
                        }
                    }

                    let actual_magnitude = (magnitude_x.powi(2) + magnitude_y.powi(2)).sqrt();
                    *magnitude = actual_magnitude;

                    if actual_magnitude < row_min_magnitude {
                        row_min_magnitude = actual_magnitude;
                    }

                    if actual_magnitude > row_max_magnitude {
                        row_max_magnitude = actual_magnitude;
                    }
                });

                if row_min_magnitude < *min_magnitude.lock().unwrap() {
                    *min_magnitude.lock().unwrap() = row_min_magnitude;
                }

                if row_max_magnitude > *max_magnitude.lock().unwrap() {
                    *max_magnitude.lock().unwrap() = row_max_magnitude;
                }
            });

        let min_magnitude = *min_magnitude.lock().unwrap();
        let max_magnitude = *max_magnitude.lock().unwrap();
        
        let inner_progress = Arc::new(RwLock::new(Progress::new()));
        inner_progress.write().unwrap().set_on_increment(move || {
            progress.write().unwrap().increment();
        });

        if self.options.use_fast_approximation {
            fast_image.par_apply_fn_to_image_pixel(
                |pixel, x, y| {
                    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                        return;
                    }
                    let magnitude = magnitude_vec[y - 1][x - 1];
                    let magnitude =
                        ((magnitude - min_magnitude) / (max_magnitude - min_magnitude)) * 255.0;
                    let magnitude = magnitude as u8;

                    pixel[0] = magnitude;
                    pixel[1] = magnitude;
                    pixel[2] = magnitude;
                },
                Some(inner_progress),
            );
        } else {
            fast_image.par_apply_fn_to_lin_srgba(
                |mut pixel, x, y| {
                    if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                        return pixel;
                    }
                    let magnitude = magnitude_vec[y - 1][x - 1];
                    let magnitude = (magnitude - min_magnitude) / (max_magnitude - min_magnitude);

                    pixel.red = magnitude;
                    pixel.green = magnitude;
                    pixel.blue = magnitude;

                    pixel
                },
                Some(inner_progress),
            );
        }

        fast_image
    }
}
