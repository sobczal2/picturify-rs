use crate::geometry::coord::Coord;
use palette::convert::FromColorUnclamped;
use palette::rgb::Rgb;
use palette::{Alpha, Clamp, IntoColor, LinSrgba, Srgba, WithAlpha};

use crate::threading::progress::Progress;

#[allow(dead_code)]
pub trait ReadPixels {
    fn read_srgba_pixel<F>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(Srgba, Coord);
    fn read_linsrgba_pixel<F>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(LinSrgba, Coord),
    {
        self.read_srgba_pixel(
            |pixel, coord| {
                let linsrgba = pixel.into_linear();
                f(linsrgba, coord);
            },
            progress,
        );
    }

    fn read_pixel<F, P>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(P, Coord),
        P: FromColorUnclamped<Alpha<Rgb, f32>> + Clamp + WithAlpha<f32>,
        Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.read_srgba_pixel(
            |pixel, coord| {
                let color: P = pixel.into_color();
                f(color, coord);
            },
            progress,
        );
    }

    fn par_read_srgba_pixel<F>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(Srgba, Coord) + Send + Sync;

    fn par_read_linsrgba_pixel<F>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(LinSrgba, Coord) + Send + Sync,
    {
        self.par_read_srgba_pixel(
            |pixel, coord| {
                let linsrgba = pixel.into_linear();
                f(linsrgba, coord);
            },
            progress,
        );
    }

    fn par_read_pixel<F, P>(&self, f: F, progress: Option<Progress>)
    where
        F: Fn(P, Coord) + Send + Sync,
        P: FromColorUnclamped<Alpha<Rgb, f32>> + Clamp + WithAlpha<f32>,
        Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.par_read_srgba_pixel(
            |pixel, coord| {
                let color: P = pixel.into_color();
                f(color, coord);
            },
            progress,
        );
    }
}
