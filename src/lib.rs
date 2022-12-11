use image::Rgb;

pub const PI: f64 = std::f64::consts::PI;

/// Converts an angle in degrees into radians
pub fn degrees_to_radian(theta: f64) -> f64 {
    theta * PI / 180.
}

/// Generates a random double between 0 and 1
pub fn random_double() -> f64 {
    rand::random::<f64>()
}

/// Generates a random double in a given range
pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

/// Generates a random integer in a given range
pub fn random_int(min: i32, max: i32) -> i32 {
    random_range(min as f64, (max + 1) as f64) as i32
}

/// Clamps a given value between a minimum and maximum
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Rgb<u8> {
    if s == 0. {
        return Rgb([(v * 255.) as u8 , (v * 255.) as u8, (v * 255.) as u8])
    }

    let mut var_h = h * 6.;
    if var_h >= 6. { var_h = 0. }
    let var_i = var_h.floor();
    let var_1 = v * (1. - s);
    let var_2 = v * (1. - s * (var_h - var_i));
    let var_3 = v * (1. - s * (1. - (var_h - var_i )));

    return match var_i as i32 {
        0 => Rgb([(v * 255.) as u8, (var_3 * 255.) as u8, (var_1 * 255.) as u8]),
        1 => Rgb([(var_2 * 255.) as u8, (v * 255.) as u8, (var_1 * 255.) as u8]),
        2 => Rgb([(var_1 * 255.) as u8, (v * 255.) as u8, (var_3 * 255.) as u8]),
        3 => Rgb([(var_1 * 255.) as u8, (var_2 * 255.) as u8, (v * 255.) as u8]),
        4 => Rgb([(var_3 * 255.) as u8, (var_1 * 255.) as u8, (v * 255.) as u8]),
        _ => Rgb([(v * 255.) as u8, (var_1 * 255.) as u8, (var_2 * 255.) as u8]),
    };
}