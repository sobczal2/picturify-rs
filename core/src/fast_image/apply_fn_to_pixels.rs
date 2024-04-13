use image::Rgba;
use palette::convert::FromColorUnclamped;
use palette::rgb::Rgb;
use palette::{Clamp, IntoColor, LinSrgba, Srgba, WithAlpha};

pub trait ApplyFnToPalettePixels {
    fn apply_fn_to_srgba<F>(&mut self, f: F)
    where
        F: Fn(Srgba, usize, usize) -> Srgba;
    fn apply_fn_to_linsrgba<F>(&mut self, f: F)
    where
        F: Fn(LinSrgba, usize, usize) -> LinSrgba,
    {
        self.apply_fn_to_srgba(|pixel, x, y| {
            let linsrgba = pixel.into_linear();
            let new_linsrgba = f(linsrgba, x, y);
            new_linsrgba.into()
        });
    }

    fn apply_fn_to_pixel<F, P>(&mut self, f: F)
    where
        F: Fn(P, usize, usize) -> P + Send + Sync,
        P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
        Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.apply_fn_to_srgba(|pixel, x, y| run_on_srgba_pixel(pixel, x, y, &f));
    }
    fn par_apply_fn_to_srgba<F>(&mut self, f: F)
    where
        F: Fn(Srgba, usize, usize) -> Srgba + Send + Sync;
    fn par_apply_fn_to_linsrgba<F>(&mut self, f: F)
    where
        F: Fn(LinSrgba, usize, usize) -> LinSrgba + Send + Sync,
    {
        self.par_apply_fn_to_srgba(|pixel, x, y| {
            let linsrgba: LinSrgba = pixel.into_linear();
            let new_linsrgba = f(linsrgba, x, y);
            new_linsrgba.into()
        });
    }

    fn par_apply_fn_to_pixel<F, P>(&mut self, f: F)
    where
        F: Fn(P, usize, usize) -> P + Send + Sync,
        P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
        Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
    {
        self.par_apply_fn_to_srgba(|pixel, x, y| run_on_srgba_pixel(pixel, x, y, &f));
    }
}

fn run_on_srgba_pixel<F, P>(pixel: Srgba, x: usize, y: usize, f: F) -> Srgba
where
    F: Fn(P, usize, usize) -> P,
    P: FromColorUnclamped<Rgb> + Clamp + WithAlpha<f32>,
    Rgb: FromColorUnclamped<<P as WithAlpha<f32>>::Color>,
{
    let color: P = pixel.color.into_color();
    let result = f(color, x, y);
    
    result.into_color()
}

pub trait ApplyFnToImagePixels {
    fn apply_fn_to_image_pixel<F>(&mut self, f: F)
    where
        F: Fn(&mut Rgba<u8>, usize, usize);

    fn par_apply_fn_to_image_pixel<F>(&mut self, f: F)
    where
        F: Fn(&mut Rgba<u8>, usize, usize) + Send + Sync;
}
