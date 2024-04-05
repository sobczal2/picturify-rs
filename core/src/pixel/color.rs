#[derive(Clone, Copy)]
pub enum ColorSpace {
    Luma,
    RGB,
    RGBA,
    HSL,
    HSLA,
    HSV,
    HSVA,
}

pub enum ColorChannel {
    Red,
    Green,
    Blue,
    Alpha,
    Hue,
    Saturation,
    Lightness,
    Value,
}

fn normalize_hue(h: f32) -> f32 {
    ((h % 360.0) + 360.0) % 360.0
}

fn normalize_value(v: f32) -> f32 {
    v.max(0.0).min(1.0)
}

pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    assert!((0.0..=1.0).contains(&r), "R value out of range");
    assert!((0.0..=1.0).contains(&g), "G value out of range");
    assert!((0.0..=1.0).contains(&b), "B value out of range");

    let min = r.min(g.min(b));
    let max = r.max(g.max(b));
    let delta = max - min;
    let l = (max + min) / 2.0;
    let mut h = 0.0;
    let mut s = 0.0;

    if max != min {
        if l < 0.5 {
            s = delta / (max + min);
        } else {
            s = delta / (2.0 - max - min);
        }

        if r == max {
            h = (g - b) / delta + (if g < b { 6.0 } else { 0.0 } );
        } else if g == max {
            h = (b - r) / delta + 2.0;
        } else {
            h = (r - g) / delta + 4.0;
        }

        h /= 6.0;
    }

    (normalize_hue(h * 360.0), normalize_value(s), normalize_value(l))
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    assert!((0.0..=360.0).contains(&h), "H value out of range");
    assert!((0.0..=1.0).contains(&s), "S value out of range");
    assert!((0.0..=1.0).contains(&l), "L value out of range");

    let c = (1.0 - ((2.0 * l) - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (rp, gp, bp) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (normalize_value(rp + m), normalize_value(gp + m), normalize_value(bp + m))
}

pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    assert!((0.0..=1.0).contains(&r), "R value out of range");
    assert!((0.0..=1.0).contains(&g), "G value out of range");
    assert!((0.0..=1.0).contains(&b), "B value out of range");

    let min = r.min(g.min(b));
    let max = r.max(g.max(b));
    let delta = max - min;
    let mut h = 0.0;

    if r == max {
        h = (g - b) / delta + (if g < b { 6.0 } else { 0.0 });
    } else if g == max {
        h = (b - r) / delta + 2.0;
    } else {
        h = (r - g) / delta + 4.0;
    }

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    h /= 6.0;

    (normalize_hue(h * 360.0), normalize_value(s), normalize_value(v))
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    assert!((0.0..=360.0).contains(&h), "H value out of range");
    assert!((0.0..=1.0).contains(&s), "S value out of range");
    assert!((0.0..=1.0).contains(&v), "V value out of range");

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (rp, gp, bp) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (normalize_value(rp + m), normalize_value(gp + m), normalize_value(bp + m))
}

pub fn hsv_to_hsl(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    assert!((0.0..=360.0).contains(&h), "H value out of range");
    assert!((0.0..=1.0).contains(&s), "S value out of range");
    assert!((0.0..=1.0).contains(&v), "V value out of range");

    let l = v * (1.0 - s / 2.0);
    let mut s = s * v / (if l < 0.5 {l * 2.0} else {2.0 - l * 2.0});
    if l == 0.0 || l == 1.0 {
        s = 0.0;
    }

    (h, s, l)
}

pub fn hsl_to_hsv(h: f32, s: f32, l: f32) -> (f32, f32, f32){
    assert!((0.0..=360.0).contains(&h), "H value out of range");
    assert!((0.0..=1.0).contains(&s), "S value out of range");
    assert!((0.0..=1.0).contains(&l), "L value out of range");

    let v = l + s * if l < 0.5 { l } else { 1.0 - l };
    let mut s = 2.0 - 2.0 * l / v;

    if v == 0.0 || v == 1.0 {
        s = 0.0;
    }

    (h, s, v)
}