use image::Rgba;
use palette::convert::FromColorUnclamped;
use palette::encoding::{Linear, Srgb};
use palette::rgb::Rgb;
use palette::{Clamp, IntoColor, LinSrgba, WithAlpha};

pub trait ApplyFnToPalettePixels {
    fn apply_fn_to_linsrgba<F>(&mut self, f: F)
    where
        F: Fn(LinSrgba, usize, usize) -> LinSrgba;

    fn apply_fn_to_pixel<F, P>(&mut self, f: F)
    where
        F: Fn(P, usize, usize) -> P,
        P: FromColorUnclamped<Rgb<Linear<Srgb>>> + Clamp + WithAlpha<f32>,
        Rgb<Linear<Srgb>>: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.apply_fn_to_linsrgba(|pixel, x, y| run_on_linsrgba_pixel(pixel, x, y, &f));
    }
    fn par_apply_fn_to_linsrgba<F>(&mut self, f: F)
    where
        F: Fn(LinSrgba, usize, usize) -> LinSrgba + Send + Sync;
}

fn run_on_linsrgba_pixel<F, P>(pixel: LinSrgba, x: usize, y: usize, f: F) -> LinSrgba
where
    F: Fn(P, usize, usize) -> P,
    P: FromColorUnclamped<Rgb<Linear<Srgb>>> + Clamp + WithAlpha<f32>,
    Rgb<Linear<Srgb>>: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
{
    let color: P = pixel.color.into_color();
    let result = f(color, x, y);
    let linsrgba: LinSrgba = result.into_color();
    linsrgba
}

pub trait ApplyFnToImagePixels {
    fn apply_fn_to_pixel<F>(&mut self, f: F)
    where
        F: Fn(&mut Rgba<u8>, usize, usize);

    fn par_apply_fn_to_pixel<F>(&mut self, f: F)
    where
        F: Fn(&mut Rgba<u8>, usize, usize) + Send + Sync;
}
