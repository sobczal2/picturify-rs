pub fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (u8, u8, u8) {
    debug_assert!(hue >= 0.0 && hue <= 360.0);
    debug_assert!(saturation >= 0.0 && saturation <= 1.0);
    debug_assert!(lightness >= 0.0 && lightness <= 1.0);

    let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let h_prime = if hue == 360.0 {
        0.0
    } else {
        hue / 60.0
    };
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

    let (r_norm, g_norm, b_norm) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = lightness - c / 2.0;

    let red = ((r_norm + m) * 255.0).round() as u8;
    let green = ((g_norm + m) * 255.0).round() as u8;
    let blue = ((b_norm + m) * 255.0).round() as u8;

    (red, green, blue)
}

pub fn rgb_to_hsl(red: u8, green: u8, blue: u8) -> (f32, f32, f32) {
    let r_norm = red as f32 / 255.0;
    let g_norm = green as f32 / 255.0;
    let b_norm = blue as f32 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);

    let delta = max - min;
    
    let lightness = (max + min) / 2.0;
    let saturation = if max == min {
        0.0
    } else {
        let saturation = if lightness > 0.5 {
            delta / (2.0 - max - min)
        } else {
            delta / (max + min)
        };
        saturation
    };

    let hue = if max == min {
        0.0
    } else {
        let h_prime = if max == r_norm {
            (g_norm - b_norm) / (max - min)
        } else if max == g_norm {
            2.0 + (b_norm - r_norm) / (max - min)
        } else {
            4.0 + (r_norm - g_norm) / (max - min)
        };
        let hue = h_prime * 60.0;
        
        if hue < 0.0 {
            hue + 360.0
        } else {
            hue
        }
    };

    debug_assert!(hue >= 0.0 && hue <= 360.0);
    debug_assert!(saturation >= 0.0 && saturation <= 1.0);
    debug_assert!(lightness >= 0.0 && lightness <= 1.0);

    (hue, saturation, lightness)
}
