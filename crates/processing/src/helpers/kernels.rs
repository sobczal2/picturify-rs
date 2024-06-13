use crate::helpers::functions::gaussian_2d;
use picturify_core::fast_image::FastImage;
use picturify_core::geometry::coord::Coord;
use picturify_core::geometry::size::Size;
use picturify_core::image::Rgba;
use picturify_core::palette::LinSrgba;

pub fn create_sobel_kernel_x() -> Vec<Vec<f32>> {
    vec![
        vec![-1.0, 0.0, 1.0],
        vec![-2.0, 0.0, 2.0],
        vec![-1.0, 0.0, 1.0],
    ]
}

pub fn create_sobel_kernel_y() -> Vec<Vec<f32>> {
    vec![
        vec![-1.0, -2.0, -1.0],
        vec![0.0, 0.0, 0.0],
        vec![1.0, 2.0, 1.0],
    ]
}

pub struct ConvolutionKernel {
    pub values: Vec<Vec<f32>>,
}

impl ConvolutionKernel {
    pub fn new(values: Vec<Vec<f32>>) -> Self {
        ConvolutionKernel { values }
    }

    pub fn new_mean(radius: usize) -> Self {
        let value = 1.0 / ((2 * radius + 1) * (2 * radius + 1)) as f32;
        let values = vec![vec![value; 2 * radius + 1]; 2 * radius + 1];
        ConvolutionKernel { values }
    }

    pub fn new_sharpen() -> Self {
        ConvolutionKernel {
            values: vec![
                vec![0.0, -1.0, 0.0],
                vec![-1.0, 5.0, -1.0],
                vec![0.0, -1.0, 0.0],
            ],
        }
    }

    pub fn new_gaussian(radius: usize, sigma: f32) -> Self {
        let mut values = vec![vec![0.0; 2 * radius + 1]; 2 * radius + 1];

        let sigma_squared = sigma * sigma;
        let two_sigma_squared = 2.0 * sigma_squared;

        let mut sum = 0.0;

        for i in 0..2 * radius + 1 {
            for j in 0..2 * radius + 1 {
                let x = i as f32 - radius as f32;
                let y = j as f32 - radius as f32;

                let value = gaussian_2d(x, y, two_sigma_squared);
                values[i][j] = value;
                sum += value;
            }
        }

        for i in 0..2 * radius + 1 {
            for j in 0..2 * radius + 1 {
                values[i][j] /= sum;
            }
        }

        ConvolutionKernel { values }
    }

    #[inline(always)]
    pub fn size(&self) -> Size {
        (self.values[0].len(), self.values.len()).into()
    }

    #[inline(always)]
    pub fn get(&self, coord: Coord) -> f32 {
        let (x, y): (usize, usize) = coord.into();
        self.values[y][x]
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
