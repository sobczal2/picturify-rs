use crate::threading::progress::Progress;
use image::Rgba;
use palette::convert::FromColorUnclamped;
use palette::rgb::Rgb;
use palette::{Clamp, IntoColor, LinSrgba, Srgba, WithAlpha};
use crate::geometry::coord::Coord;

pub struct Offset {
    pub skip_rows: usize,
    pub take_rows: usize,
    pub skip_columns: usize,
    pub take_columns: usize,
}

pub trait ApplyFnToPalettePixels {
    fn apply_fn_to_srgba<F>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(Srgba, Coord) -> Srgba;

    fn apply_fn_to_linsrgba<F>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(LinSrgba, Coord) -> LinSrgba,
    {
        self.apply_fn_to_srgba(
            |pixel, coord| {
                let linsrgba = pixel.into_linear();
                let new_linsrgba = f(linsrgba, coord);
                new_linsrgba.into()
            },
            progress,
        );
    }

    fn apply_fn_to_pixel<F, P>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(P, Coord) -> P + Send + Sync,
            P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
            Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.apply_fn_to_srgba(|pixel, coord| run_on_srgba_pixel(pixel, coord, &f), progress);
    }

    fn par_apply_fn_to_srgba<F>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(Srgba, Coord) -> Srgba + Send + Sync;

    fn par_apply_fn_to_lin_srgba<F>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(LinSrgba, Coord) -> LinSrgba + Send + Sync,
    {
        self.par_apply_fn_to_srgba(
            |pixel, coord| {
                let linsrgba: LinSrgba = pixel.into_linear();
                let new_linsrgba = f(linsrgba, coord);
                new_linsrgba.into()
            },
            progress,
        );
    }

    fn par_apply_fn_to_pixel<F, P>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(P, Coord) -> P + Send + Sync,
            P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
            Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.par_apply_fn_to_srgba(|pixel, coord| run_on_srgba_pixel(pixel, coord, &f), progress);
    }

    fn apply_fn_to_srgba_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(Srgba, Coord) -> Srgba;

    fn apply_fn_to_linsrgba_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(LinSrgba, Coord) -> LinSrgba,
    {
        self.apply_fn_to_srgba_with_offset(
            |pixel, coord| {
                let linsrgba = pixel.into_linear();
                let new_linsrgba = f(linsrgba, coord);
                new_linsrgba.into()
            },
            progress,
            offset,
        );
    }

    fn apply_fn_to_pixel_with_offset<F, P>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(P, Coord) -> P + Send + Sync,
        P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
        Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.apply_fn_to_srgba_with_offset(
            |pixel, coord| run_on_srgba_pixel(pixel, coord, &f),
            progress,
            offset,
        );
    }

    fn par_apply_fn_to_srgba_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(Srgba, Coord) -> Srgba + Send + Sync;

    fn par_apply_fn_to_lin_srgba_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(LinSrgba, Coord) -> LinSrgba + Send + Sync,
    {
        self.par_apply_fn_to_srgba_with_offset(
            |pixel, coord| {
                let linsrgba: LinSrgba = pixel.into_linear();
                let new_linsrgba = f(linsrgba, coord);
                new_linsrgba.into()
            },
            progress,
            offset,
        );
    }

    fn par_apply_fn_to_pixel_with_offset<F, P>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(P, Coord) -> P + Send + Sync,
        P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
        Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.par_apply_fn_to_srgba_with_offset(
            |pixel, coord| run_on_srgba_pixel(pixel, coord, &f),
            progress,
            offset,
        );
    }
}

#[inline(always)]
fn run_on_srgba_pixel<F, P>(pixel: Srgba, coord: Coord, f: F) -> Srgba
    where
        F: Fn(P, Coord) -> P,
        P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
        Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
{
    let color: P = pixel.color.into_color();
    let result = f(color, coord);

    result.into_color()
}

pub trait ApplyFnToImagePixels {
    fn apply_fn_to_image_pixel<F>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(&mut Rgba<u8>, Coord);

    fn par_apply_fn_to_image_pixel<F>(&mut self, f: F, progress: Option<Progress>)
        where
            F: Fn(&mut Rgba<u8>, Coord) + Send + Sync;

    fn apply_fn_to_image_pixel_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(&mut Rgba<u8>, Coord);

    fn par_apply_fn_to_image_pixel_with_offset<F>(
        &mut self,
        f: F,
        progress: Option<Progress>,
        offset: Offset,
    ) where
        F: Fn(&mut Rgba<u8>, Coord) + Send + Sync;
}
