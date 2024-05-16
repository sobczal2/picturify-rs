use std::sync::{Arc, Mutex, RwLock};
use palette::convert::FromColorUnclamped;
use palette::rgb::Rgb;
use palette::{Alpha, Clamp, IntoColor, LinSrgba, Srgba, WithAlpha};
use crate::threading::progress::Progress;

pub trait ReadPixels {
    fn read_srgba_pixel<F>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(Srgba, usize, usize);
    fn read_linsrgba_pixel<F>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(LinSrgba, usize, usize),
    {
        self.read_srgba_pixel(
            |pixel, x, y| {
                let linsrgba = pixel.into_linear();
                f(linsrgba, x, y);
            },
            progress,
        );
    }

    fn read_pixel<F, P>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(P, usize, usize),
            P: FromColorUnclamped<Alpha<Rgb, f32>> + Clamp + WithAlpha<f32>,
            Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.read_srgba_pixel(
            |pixel, x, y| {
                let color: P = pixel.into_color();
                f(color, x, y);
            },
            progress,
        );
    }

    fn par_read_srgba_pixel<F>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(Srgba, usize, usize) + Send + Sync;

    fn par_read_linsrgba_pixel<F>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(LinSrgba, usize, usize) + Send + Sync,
    {
        self.par_read_srgba_pixel(
            |pixel, x, y| {
                let linsrgba = pixel.into_linear();
                f(linsrgba, x, y);
            },
            progress,
        );
    }

    fn par_read_pixel<F, P>(&self, f: F, progress: Option<Arc<RwLock<Progress>>>)
        where
            F: Fn(P, usize, usize) + Send + Sync,
            P: FromColorUnclamped<Alpha<Rgb, f32>> + Clamp + WithAlpha<f32>,
            Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.par_read_srgba_pixel(
            |pixel, x, y| {
                let color: P = pixel.into_color();
                f(color, x, y);
            },
            progress,
        );
    }
}
