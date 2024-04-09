use crate::error::PicturifyResult;
use crate::image::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels};
use crate::image::io::{ReadFromFile, WriteToFile};
use image::io::Reader;
use image::{DynamicImage, Rgba, Rgba32FImage};
use palette::{LinSrgba, Srgba};
use rayon::prelude::*;

#[derive(Debug)]
pub struct FastImage {
    inner: Rgba32FImage,
}

impl FastImage {
    pub fn empty(width: usize, height: usize) -> FastImage {
        FastImage {
            inner: Rgba32FImage::new(width as u32, height as u32),
        }
    }
}

impl FastImage {
    pub fn get_width(&self) -> usize {
        self.inner.width() as usize
    }

    pub fn get_height(&self) -> usize {
        self.inner.height() as usize
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Srgba {
        let pixel = self.inner.get_pixel(x as u32, y as u32);
        Srgba::new(pixel[0], pixel[1], pixel[2], pixel[3])
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Srgba) {
        let array: [f32; 4] = pixel.into_format().into();
        let pixel = Rgba::<f32>::from(array);
        self.inner.put_pixel(x as u32, y as u32, pixel);
    }
}

// Slower implementation, use if you need to work with the pixel's color space
impl ApplyFnToPalettePixels for FastImage {
    fn apply_fn_to_linsrgba<F>(&mut self, f: F)
    where
        F: Fn(LinSrgba, usize, usize) -> LinSrgba,
    {
        let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
            row.into_iter().for_each(|(x, y, pixel)| {
                run_on_linsrgba_pixel(pixel, x as usize, y as usize, &f);
            });
        });
    }
    fn par_apply_fn_to_linsrgba<F>(&mut self, f: F)
    where
        F: Fn(LinSrgba, usize, usize) -> LinSrgba + Send + Sync,
    {
        let _ = &self
            .inner
            .enumerate_rows_mut()
            .par_bridge()
            .for_each(|(_, row)| {
                row.into_iter().for_each(|(x, y, pixel)| {
                    run_on_linsrgba_pixel(pixel, x as usize, y as usize, &f);
                });
            });
    }
}

fn run_on_linsrgba_pixel<F>(pixel: &mut Rgba<f32>, x: usize, y: usize, f: F)
where
    F: Fn(LinSrgba, usize, usize) -> LinSrgba,
{
    let srgba = Srgba::new(pixel[0], pixel[1], pixel[2], pixel[3]);
    let linsrgba = srgba.into_linear();
    let new_linsrgba = f(linsrgba, x, y);
    let new_srgba: Srgba = new_linsrgba.into();
    let array: [f32; 4] = new_srgba.into();
    let new_pixel = Rgba::<f32>::from(array);
    *pixel = new_pixel;
}

// Speedy implementation, use if you don't need to work with the pixel's color space
impl ApplyFnToImagePixels for FastImage {
    fn apply_fn_to_pixel<F>(&mut self, f: F)
    where
        F: Fn(&mut Rgba<f32>, usize, usize),
    {
        let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
            row.into_iter().for_each(|(x, y, pixel)| {
                f(pixel, x as usize, y as usize);
            });
        });
    }

    fn par_apply_fn_to_pixel<F>(&mut self, f: F)
    where
        F: Fn(&mut Rgba<f32>, usize, usize) + Send + Sync,
    {
        let _ = &self
            .inner
            .enumerate_rows_mut()
            .par_bridge()
            .for_each(|(_, row)| {
                row.into_iter().for_each(|(x, y, pixel)| {
                    f(pixel, x as usize, y as usize);
                });
            });
    }
}

impl ReadFromFile for FastImage {
    fn read_from_file(path: &str) -> PicturifyResult<Box<Self>> {
        let dynamic_image = Reader::open(path)?.decode()?;
        Ok(Box::new(FastImage {
            inner: dynamic_image.into_rgba32f(),
        }))
    }
}

impl WriteToFile for FastImage {
    fn write_to_file(&self, path: &str) -> PicturifyResult<()> {
        let dynamic_image = DynamicImage::from(self.inner.clone());
        dynamic_image.to_rgba8().save(path)?;
        Ok(())
    }
}
