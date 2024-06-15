use picturify_core::core::fast_image::FastImage;
use picturify_core::error::processing::{ProcessingError, ProcessingResult};
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::image::Rgba;
use picturify_core::palette::LinSrgba;
use crate::common::functions::gaussian_2d;

#[derive(Clone, Debug)]
pub struct ConvolutionKernel {
    values: Vec<f32>,
    width: usize,
    height: usize,
}

impl ConvolutionKernel {
    pub fn new(values: Vec<Vec<f32>>) -> ProcessingResult<Self> {
        let width = values[0].len();
        let height = values.len();

        let kernel = ConvolutionKernel {
            values: values.into_iter().flatten().collect(),
            width,
            height,
        };

        if !kernel.validate() {
            return Err(ProcessingError::InvalidKernel);
        }

        Ok(kernel)
    }

    pub fn validate(&self) -> bool {
        self.values.len() == self.width * self.height
    }

    pub fn new_mean(radius: usize) -> Self {
        let value = 1.0 / ((2 * radius + 1) * (2 * radius + 1)) as f32;
        let values = vec![vec![value; 2 * radius + 1]; 2 * radius + 1];
        ConvolutionKernel::new(values).unwrap()
    }

    pub fn new_sharpen() -> Self {
        ConvolutionKernel::new(
            vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 5.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
        )
            .unwrap()
    }

    pub fn new_gaussian(radius: usize, sigma: f32) -> Self {
        let mut values = vec![vec![0.0; 2 * radius + 1]; 2 * radius + 1];

        let sigma_squared = sigma * sigma;
        let two_sigma_squared = 2.0 * sigma_squared;

        let mut sum = 0.0;

        values.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, value)| {
                let x = i as f32 - radius as f32;
                let y = j as f32 - radius as f32;

                *value = gaussian_2d(x, y, two_sigma_squared);
                sum += *value;
            });
        });

        values
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|value| *value /= sum));

        ConvolutionKernel::new(values).unwrap()
    }

    #[inline(always)]
    pub fn size(&self) -> Size {
        (self.width, self.height).into()
    }

    #[inline(always)]
    pub fn radius(&self) -> usize {
        let (width, _): (usize, usize) = self.size().into();
        width / 2
    }

    #[inline(always)]
    pub fn get(&self, coord: Coord) -> f32 {
        self.values[coord.array_index(self.width)]
    }

    pub fn convolve_rgb_fast(&self, image: &FastImage, coord: Coord) -> Rgba<u8> {
        let mut result_red_f32 = 0f32;
        let mut result_green_f32 = 0f32;
        let mut result_blue_f32 = 0f32;

        let (width, height) = self.size().into();

        for i in 0..width {
            for j in 0..height {
                let kernel_coord = (i, j).into();
                let kernel_value = self.get(kernel_coord);

                if kernel_value == 0.0 {
                    continue;
                }

                let (x, y): (usize, usize) = coord.into();
                let inner_coord = (x + i - width / 2, y + j - height / 2).into();
                let image_pixel = image.get_image_pixel(inner_coord);
                result_red_f32 += image_pixel.0[0] as f32 * kernel_value;
                result_green_f32 += image_pixel.0[1] as f32 * kernel_value;
                result_blue_f32 += image_pixel.0[2] as f32 * kernel_value;
            }
        }

        let result_alpha = image.get_image_pixel(coord).0[3];

        Rgba([
            result_red_f32.clamp(0.0, 255.0) as u8,
            result_green_f32.clamp(0.0, 255.0) as u8,
            result_blue_f32.clamp(0.0, 255.0) as u8,
            result_alpha,
        ])
    }

    pub fn convolve_rgb_slow(&self, image: &FastImage, coord: Coord) -> LinSrgba {
        let mut result_red_f32 = 0f32;
        let mut result_green_f32 = 0f32;
        let mut result_blue_f32 = 0f32;

        let (width, height) = self.size().into();

        for i in 0..width {
            for j in 0..height {
                let kernel_coord = (i, j).into();
                let kernel_value = self.get(kernel_coord);

                if kernel_value == 0.0 {
                    continue;
                }

                let (x, y): (usize, usize) = coord.into();
                let image_pixel_coord = (x + i - width / 2, y + j - height / 2).into();
                let image_pixel = image.get_lin_srgba_pixel(image_pixel_coord);

                result_red_f32 += image_pixel.red * kernel_value;
                result_green_f32 += image_pixel.green * kernel_value;
                result_blue_f32 += image_pixel.blue * kernel_value;
            }
        }

        let result_alpha = image.get_lin_srgba_pixel(coord).alpha;

        LinSrgba::new(
            result_red_f32.clamp(0.0, 1.0),
            result_green_f32.clamp(0.0, 1.0),
            result_blue_f32.clamp(0.0, 1.0),
            result_alpha,
        )
    }
}
