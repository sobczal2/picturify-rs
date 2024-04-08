pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (u8, u8, u8) {
    debug_assert!(hue >= 0.0 && hue <= 360.0);
    debug_assert!(saturation >= 0.0 && saturation <= 1.0);
    debug_assert!(value >= 0.0 && value <= 1.0);
    
    if saturation == 0.0 {
        let value = (value * 255.0).round() as u8;
        return (value, value, value);
    }
    
    let h_prime = if hue == 360.0 {
        0.0
    } else {
        hue / 60.0
    };
    
    let i = h_prime.floor() as i32;
    let f = h_prime - i as f32;
    
    let c = value * saturation;
    let x = c * (1.0 - f);
    let m = value - c;
    
    let (r_norm, g_norm, b_norm) = if i == 0 {
        (c, x, 0.0)
    } else if i == 1 {
        (x, c, 0.0)
    } else if i == 2 {
        (0.0, c, x)
    } else if i == 3 {
        (0.0, x, c)
    } else if i == 4 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    let red = (r_norm + m) * 255.0;
    let green = (g_norm + m) * 255.0;
    let blue = (b_norm + m) * 255.0;
    
    debug_assert!(red >= 0.0 && red <= 255.0);
    debug_assert!(green >= 0.0 && green <= 255.0);
    debug_assert!(blue >= 0.0 && blue <= 255.0);
    
    (red.round() as u8, green.round() as u8, blue.round() as u8)
}

pub fn rgb_to_hsv(red: u8, green: u8, blue: u8) -> (f32, f32, f32) {
    let r_norm = red as f32 / 255.0;
    let g_norm = green as f32 / 255.0;
    let b_norm = blue as f32 / 255.0;
    
    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    
    let delta = max - min;
    
    let hue = if max == min {
        0.0
    } else {
        let h_prime = if max == r_norm {
            (g_norm - b_norm) / delta
        } else if max == g_norm {
            2.0 + (b_norm - r_norm) / delta
        } else {
            4.0 + (r_norm - g_norm) / delta
        };
        let hue = h_prime * 60.0;
        
        if hue < 0.0 {
            hue + 360.0
        } else {
            hue
        }
    };
    
    let saturation = if max == 0.0 {
        0.0
    } else {
        delta / max
    };
    
    let value = max;
    
    debug_assert!(hue >= 0.0 && hue <= 360.0);
    debug_assert!(saturation >= 0.0 && saturation <= 1.0);
    debug_assert!(value >= 0.0 && value <= 1.0);
    
    (hue, saturation, value)
}
