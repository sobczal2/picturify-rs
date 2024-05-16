pub mod apply_fn_to_pixels;
pub mod io;
mod read_pixels;
pub mod util;

use std::sync::{Arc, RwLock};
use image::buffer::{EnumeratePixels, Pixels, PixelsMut, Rows, RowsMut};
use image::io::Reader;
use image::{Rgba, RgbaImage};
use palette::{LinSrgba, Srgba};
use rayon::prelude::*;

use crate::error::PicturifyResult;
use crate::fast_image::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels};
use crate::fast_image::io::{ReadFromFile, WriteToFile};
use crate::fast_image::read_pixels::ReadPixels;
use crate::threading::progress::{Progress};

#[derive(Debug, Clone)]
pub struct FastImage {
    inner: RgbaImage,
}

impl FastImage {
    pub fn empty(width: usize, height: usize) -> FastImage {
        FastImage {
            inner: RgbaImage::new(width as u32, height as u32),
        }
    }

    pub fn from_rgba_vec(width: usize, height: usize, rgba_vec: Vec<u8>) -> FastImage {
        FastImage {
            inner: RgbaImage::from_raw(width as u32, height as u32, rgba_vec).unwrap(),
        }
    }

    pub fn to_rgba_vec(&self) -> Vec<u8> {
        self.inner.clone().into_raw()
    }
}

impl FastImage {
    pub fn get_width(&self) -> usize {
        self.inner.width() as usize
    }

    pub fn get_height(&self) -> usize {
        self.inner.height() as usize
    }

    pub fn get_image_pixel(&self, x: usize, y: usize) -> Rgba<u8> {
        *self.inner.get_pixel(x as u32, y as u32)
    }

    pub fn get_srgba_pixel(&self, x: usize, y: usize) -> Srgba {
        let pixel = self.inner.get_pixel(x as u32, y as u32);
        Srgba::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
            pixel[3] as f32 / 255.0,
        )
    }

    pub fn get_lin_srgba_pixel(&self, x: usize, y: usize) -> LinSrgba {
        let pixel = self.get_srgba_pixel(x, y);
        pixel.into_linear()
    }

    pub fn set_image_pixel(&mut self, x: usize, y: usize, pixel: Rgba<u8>) {
        self.inner.put_pixel(x as u32, y as u32, pixel);
    }

    pub fn set_srgba_pixel(&mut self, x: usize, y: usize, pixel: Srgba) {
        let r = (pixel.red * 255.0).round() as u8;
        let g = (pixel.green * 255.0).round() as u8;
        let b = (pixel.blue * 255.0).round() as u8;
        let a = (pixel.alpha * 255.0).round() as u8;
        self.inner.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
    }

    pub fn set_lin_srgba_pixel(&mut self, x: usize, y: usize, pixel: LinSrgba) {
        let srgba: Srgba = pixel.into();
        self.set_srgba_pixel(x, y, srgba);
    }

    pub fn pixels(&self) -> Pixels<Rgba<u8>> {
        self.inner.pixels()
    }

    pub fn pixels_mut(&mut self) -> PixelsMut<Rgba<u8>> {
        self.inner.pixels_mut()
    }

    pub fn rows(&self) -> Rows<Rgba<u8>> {
        self.inner.rows()
    }

    pub fn rows_mut(&mut self) -> RowsMut<Rgba<u8>> {
        self.inner.rows_mut()
    }
}

// Slower implementation, use if you need to work with the pixel's color space
impl ApplyFnToPalettePixels for FastImage {
    fn apply_fn_to_srgba<F>(&mut self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(Srgba, usize, usize) -> Srgba,
    {
        if let Some(progress) = progress {
            let max_value = self.get_height() as u32;
            progress.write().expect("Failed to lock progress").setup(max_value);
            let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
                progress.read().expect("Failed to lock progress").increment();
                row.into_iter().for_each(|(x, y, pixel)| {
                    run_on_srgba_pixel(pixel, x as usize, y as usize, &f);
                });
            });
        } else {
            let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
                row.into_iter().for_each(|(x, y, pixel)| {
                    run_on_srgba_pixel(pixel, x as usize, y as usize, &f);
                });
            });
        }
    }

    fn par_apply_fn_to_srgba<F>(&mut self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(Srgba, usize, usize) -> Srgba + Send + Sync,
    {
        if let Some(progress) = progress {
            let max_value = self.get_height() as u32;
            progress.write().expect("Failed to lock progress").setup(max_value);

            let _ = self
                .inner
                .enumerate_rows_mut()
                .par_bridge()
                .for_each(|(_, row)| {
                    progress.read().expect("Failed to lock progress").increment();
                    row.into_iter().for_each(|(x, y, pixel)| {
                        run_on_srgba_pixel(pixel, x as usize, y as usize, &f);
                    });
                });
        } else {
            let _ = self
                .inner
                .enumerate_rows_mut()
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter().for_each(|(x, y, pixel)| {
                        run_on_srgba_pixel(pixel, x as usize, y as usize, &f);
                    });
                });
        }
    }
}

fn run_on_srgba_pixel<F>(pixel: &mut Rgba<u8>, x: usize, y: usize, f: F)
    where
        F: Fn(Srgba, usize, usize) -> Srgba,
{
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;
    let a = pixel[3] as f32 / 255.0;

    let srgba = Srgba::new(r, g, b, a);
    let new_srgba = f(srgba, x, y);

    let r = (new_srgba.red * 255.0).round() as u8;
    let g = (new_srgba.green * 255.0).round() as u8;
    let b = (new_srgba.blue * 255.0).round() as u8;
    let a = (new_srgba.alpha * 255.0).round() as u8;

    let new_pixel = Rgba::<u8>::from([r, g, b, a]);
    *pixel = new_pixel;
}

// Speedy implementation, use if you don't need to work with the pixel's color space
impl ApplyFnToImagePixels for FastImage {
    fn apply_fn_to_image_pixel<F>(&mut self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(&mut Rgba<u8>, usize, usize),
    {
        if let Some(progress) = progress {
            let max_value = self.get_height() as u32;
            progress.write().expect("Failed to lock progress").setup(max_value);
            let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
                progress.read().expect("Failed to lock progress").increment();
                row.into_iter().for_each(|(x, y, pixel)| {
                    f(pixel, x as usize, y as usize);
                });
            });
        } else {
            let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
                row.into_iter().for_each(|(x, y, pixel)| {
                    f(pixel, x as usize, y as usize);
                });
            });
        }
    }

    fn par_apply_fn_to_image_pixel<F>(&mut self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(&mut Rgba<u8>, usize, usize) + Send + Sync,
    {
        if let Some(progress) = progress {
            let max_value = self.get_height() as u32;
            progress.write().expect("Failed to lock progress").setup(max_value);
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .par_bridge()
                .for_each(|(_, row)| {
                    progress.read().expect("Failed to lock progress").increment();
                    row.into_iter().for_each(|(x, y, pixel)| {
                        f(pixel, x as usize, y as usize);
                    });
                });
        } else {
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
}

impl ReadPixels for FastImage {
    fn read_srgba_pixel<F>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(Srgba, usize, usize),
    {
        if let Some(progress) = progress {
            let max_value = self.get_height() as u32;
            progress.write().expect("Failed to lock progress").setup(max_value);
            let _ = &self.inner.enumerate_rows().for_each(|(_, row)| {
                progress.read().expect("Failed to lock progress").increment();
                read_srgba_pixel_process_row(&f, row);
            });
        } else {
            let _ = &self.inner.enumerate_rows().for_each(|(_, row)| {
                read_srgba_pixel_process_row(&f, row);
            });
        }
    }

    fn par_read_srgba_pixel<F>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(Srgba, usize, usize) + Send + Sync,
    {
        if let Some(progress) = progress {
            let max_value = self.get_height() as u32;
            progress.write().expect("Failed to lock progress").setup(max_value);
            let _ = &self
                .inner
                .enumerate_rows()
                .par_bridge()
                .for_each(|(_, row)| {
                    progress.read().expect("Failed to lock progress").increment();
                    read_srgba_pixel_process_row(&f, row);
                });
        } else {
            let _ = &self
                .inner
                .enumerate_rows()
                .par_bridge()
                .for_each(|(_, row)| {
                    read_srgba_pixel_process_row(&f, row);
                });
        }
    }
}

fn read_srgba_pixel_process_row<F>(f: &F, row: EnumeratePixels<Rgba<u8>>) where F: Fn(Srgba, usize, usize) {
    row.into_iter().for_each(|(x, y, pixel)| {
        let r = pixel[0] as f32 / 255.0;
        let g = pixel[1] as f32 / 255.0;
        let b = pixel[2] as f32 / 255.0;
        let a = pixel[3] as f32 / 255.0;

        let srgba = Srgba::new(r, g, b, a);
        f(srgba, x as usize, y as usize);
    });
}

impl ReadFromFile for FastImage {
    fn read_from_file(path: &str) -> PicturifyResult<Box<Self>> {
        let dynamic_image = Reader::open(path)?.decode()?;
        Ok(Box::new(FastImage {
            inner: dynamic_image.into_rgba8(),
        }))
    }
}

impl WriteToFile for FastImage {
    fn write_to_file(&self, path: &str) -> PicturifyResult<()> {
        self.inner.save(path)?;
        Ok(())
    }
}