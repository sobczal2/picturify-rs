use crate::conversions::image_palette_bridge::{rgba_to_srgba, srgba_to_rgba};
use crate::core::apply_fn_to_pixels::{ApplyFnToImagePixels, ApplyFnToPalettePixels, Offset};
use crate::core::io::{ReadFromFile, WriteToFile};
use crate::core::read_pixels::ReadPixels;
use crate::error::PicturifyResult;
use crate::geometry::coord::Coord;
use crate::geometry::size::Size;
use crate::threading::progress::{Progress, ProgressIteratorExt};
use image::buffer::{EnumeratePixels, Pixels, PixelsMut, Rows, RowsMut};
use image::io::Reader;
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use palette::{LinSrgba, Srgba};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct FastImage {
    inner: RgbaImage,
}

impl FastImage {
    pub fn empty(size: Size) -> FastImage {
        let (width, height) = size.into();
        FastImage {
            inner: RgbaImage::new(width, height),
        }
    }

    pub fn from_rgba_vec(size: Size, rgba_vec: Vec<u8>) -> FastImage {
        let (width, height) = size.into();
        FastImage {
            inner: RgbaImage::from_raw(width, height, rgba_vec).unwrap(),
        }
    }

    pub fn to_rgba_vec(&self) -> Vec<u8> {
        self.inner.clone().into_raw()
    }
}

impl FastImage {
    #[inline(always)]
    pub fn size(&self) -> Size {
        (self.inner.width(), self.inner.height()).into()
    }

    #[inline(always)]
    pub fn get_image_pixel(&self, coord: Coord) -> Rgba<u8> {
        let (x, y) = coord.into();
        *self.inner.get_pixel(x, y)
    }

    #[inline(always)]
    pub fn get_srgba_pixel(&self, coord: Coord) -> Srgba {
        let (x, y) = coord.into();
        let pixel = self.inner.get_pixel(x, y);
        rgba_to_srgba(*pixel)
    }

    #[inline(always)]
    pub fn get_lin_srgba_pixel(&self, coord: Coord) -> LinSrgba {
        let pixel = self.get_srgba_pixel(coord);
        pixel.into_linear()
    }

    #[inline(always)]
    pub fn set_image_pixel(&mut self, coord: Coord, pixel: Rgba<u8>) {
        let (x, y) = coord.into();
        self.inner.put_pixel(x, y, pixel);
    }

    #[inline(always)]
    pub fn set_srgba_pixel(&mut self, coord: Coord, pixel: Srgba) {
        let rgba = srgba_to_rgba(pixel);
        self.set_image_pixel(coord, rgba);
    }

    #[inline(always)]
    pub fn set_lin_srgba_pixel(&mut self, coord: Coord, pixel: LinSrgba) {
        let srgba: Srgba = pixel.into();
        self.set_srgba_pixel(coord, srgba);
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
    fn apply_fn_to_srgba<F>(&mut self, f: F, progress: Option<Progress>)
    where
        F: Fn(Srgba, Coord) -> Srgba,
    {
        if let Some(mut progress) = progress {
            let (_, height) = self.size().into();
            progress.setup(height);
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .progress(progress)
                .for_each(|(_, row)| {
                    row.into_iter().for_each(|(x, y, pixel)| {
                        run_on_srgba_pixel(pixel, (x, y).into(), &f);
                    });
                });
        } else {
            let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
                row.into_iter().for_each(|(x, y, pixel)| {
                    run_on_srgba_pixel(pixel, (x, y).into(), &f);
                });
            });
        }
    }

    fn par_apply_fn_to_srgba<F>(&mut self, f: F, progress: Option<Progress>)
    where
        F: Fn(Srgba, Coord) -> Srgba + Send + Sync,
    {
        if let Some(mut progress) = progress {
            let (_, height) = self.size().into();
            progress.setup(height);

            self.inner
                .enumerate_rows_mut()
                .progress(progress)
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter().for_each(|(x, y, pixel)| {
                        run_on_srgba_pixel(pixel, (x, y).into(), &f);
                    });
                });
        } else {
            self.inner
                .enumerate_rows_mut()
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter().for_each(|(x, y, pixel)| {
                        run_on_srgba_pixel(pixel, (x, y).into(), &f);
                    });
                });
        }
    }

    fn apply_fn_to_srgba_with_offset<F>(&mut self, f: F, progress: Option<Progress>, offset: Offset)
    where
        F: Fn(Srgba, Coord) -> Srgba,
    {
        if let Some(mut progress) = progress {
            let max_value = offset.take_rows;
            progress.setup(max_value);
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .progress(progress)
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            run_on_srgba_pixel(pixel, (x, y).into(), &f);
                        });
                });
        } else {
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            run_on_srgba_pixel(pixel, (x, y).into(), &f);
                        });
                });
        }
    }

    fn par_apply_fn_to_srgba_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(Srgba, Coord) -> Srgba + Send + Sync,
    {
        if let Some(mut progress) = progress {
            let max_value = offset.take_rows;
            progress.setup(max_value);

            self.inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .progress(progress)
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            run_on_srgba_pixel(pixel, (x, y).into(), &f);
                        });
                });
        } else {
            self.inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            run_on_srgba_pixel(pixel, (x, y).into(), &f);
                        });
                });
        }
    }
}

#[inline(always)]
fn run_on_srgba_pixel<F>(pixel: &mut Rgba<u8>, coord: Coord, f: F)
where
    F: Fn(Srgba, Coord) -> Srgba,
{
    let srgba = rgba_to_srgba(*pixel);
    let new_srgba = f(srgba, coord);
    *pixel = srgba_to_rgba(new_srgba);
}

// Speedy implementation, use if you don't need to work with the pixel's color space
impl ApplyFnToImagePixels for FastImage {
    fn apply_fn_to_image_pixel<F>(&mut self, f: F, progress: Option<Progress>)
    where
        F: Fn(&mut Rgba<u8>, Coord),
    {
        if let Some(mut progress) = progress {
            let (_, height) = self.size().into();
            progress.setup(height);
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .progress(progress)
                .for_each(|(_, row)| {
                    row.into_iter().for_each(|(x, y, pixel)| {
                        f(pixel, (x, y).into());
                    });
                });
        } else {
            let _ = &self.inner.enumerate_rows_mut().for_each(|(_, row)| {
                row.into_iter().for_each(|(x, y, pixel)| {
                    f(pixel, (x, y).into());
                });
            });
        }
    }

    fn par_apply_fn_to_image_pixel<F>(&mut self, f: F, progress: Option<Progress>)
    where
        F: Fn(&mut Rgba<u8>, Coord) + Send + Sync,
    {
        if let Some(mut progress) = progress {
            let (_, height) = self.size().into();
            progress.setup(height);
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .progress(progress)
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter().for_each(|(x, y, pixel)| {
                        f(pixel, (x, y).into());
                    });
                });
        } else {
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter().for_each(|(x, y, pixel)| {
                        f(pixel, (x, y).into());
                    });
                });
        }
    }

    fn apply_fn_to_image_pixel_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(&mut Rgba<u8>, Coord),
    {
        if let Some(mut progress) = progress {
            let max_value = offset.take_rows;
            progress.setup(max_value);
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .progress(progress)
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            f(pixel, (x, y).into());
                        });
                });
        } else {
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            f(pixel, (x, y).into());
                        });
                });
        }
    }

    fn par_apply_fn_to_image_pixel_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(&mut Rgba<u8>, Coord) + Send + Sync,
    {
        if let Some(mut progress) = progress {
            let max_value = offset.take_rows;
            progress.setup(max_value);
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .progress(progress)
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            f(pixel, (x, y).into());
                        });
                });
        } else {
            let _ = &self
                .inner
                .enumerate_rows_mut()
                .skip(offset.skip_rows)
                .take(offset.take_rows)
                .par_bridge()
                .for_each(|(_, row)| {
                    row.into_iter()
                        .skip(offset.skip_columns)
                        .take(offset.take_columns)
                        .for_each(|(x, y, pixel)| {
                            f(pixel, (x, y).into());
                        });
                });
        }
    }
}

impl ReadPixels for FastImage {
    fn read_srgba_pixel<F>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(Srgba, Coord),
    {
        if let Some(mut progress) = progress {
            let (_, height) = self.size().into();
            progress.setup(height);
            let _ = &self
                .inner
                .enumerate_rows()
                .progress(progress)
                .for_each(|(_, row)| {
                    read_srgba_pixel_process_row(&f, row);
                });
        } else {
            let _ = &self.inner.enumerate_rows().for_each(|(_, row)| {
                read_srgba_pixel_process_row(&f, row);
            });
        }
    }

    fn par_read_srgba_pixel<F>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(Srgba, Coord) + Send + Sync,
    {
        if let Some(mut progress) = progress {
            let (_, height) = self.size().into();
            progress.setup(height);
            let _ = &self
                .inner
                .enumerate_rows()
                .progress(progress)
                .par_bridge()
                .for_each(|(_, row)| {
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

#[inline(always)]
fn read_srgba_pixel_process_row<F>(f: &F, row: EnumeratePixels<Rgba<u8>>)
where
    F: Fn(Srgba, Coord),
{
    row.into_iter().for_each(|(x, y, pixel)| {
        let srgba = rgba_to_srgba(*pixel);
        f(srgba, (x, y).into());
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
        let format = ImageFormat::from_path(path)?;
        let supports_alpha = match format {
            ImageFormat::Png => true,
            ImageFormat::Jpeg => false,
            ImageFormat::Gif => true,
            ImageFormat::WebP => true,
            ImageFormat::Pnm => false,
            ImageFormat::Tiff => true,
            ImageFormat::Tga => true,
            ImageFormat::Dds => true,
            ImageFormat::Bmp => true,
            ImageFormat::Ico => true,
            ImageFormat::Hdr => true,
            ImageFormat::OpenExr => true,
            ImageFormat::Farbfeld => true,
            ImageFormat::Avif => true,
            ImageFormat::Qoi => true,
            _ => unreachable!("Unsupported image format")
        };
        
        let image = if supports_alpha {
            self.inner.save(path)?
        } else {
            DynamicImage::ImageRgba8(self.inner.clone()).to_rgb8().save(path)?
        };
        
        Ok(image)
    }
}
