use image::Rgb;

/// Converts a colour from HSV to RGB color space.
/// `h`, `s`, and `v` are expected to be in the range [0.0, 1.0].
pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Rgb<u8> {
    if s == 0.0 {
        let val = (v * 255.0) as u8;
        return Rgb([val, val, val]);
    }

    let hue_sector = h * 6.0;
    let sector_index = hue_sector.floor();
    let fractional_part = hue_sector - sector_index;

    let p = v * (1.0 - s);
    let q = v * (1.0 - s * fractional_part);
    let t = v * (1.0 - s * (1.0 - fractional_part));

    let (r, g, b) = match sector_index as i32 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    Rgb([
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    ])
}