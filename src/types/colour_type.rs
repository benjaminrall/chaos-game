use crate::utils::hsv_to_rgb;
use image::Rgb;


/// Enum to define the colouring strategy for the fractal
pub enum ColourType {
    White,
    Coloured,
}

impl ColourType {
    /// Calculates the RGB colour of a point based on the colour type and angle.
    pub fn get_colour(&self, angle: f64) -> Rgb<u8> {
        match self {
            ColourType::White => Rgb([255, 255, 255]),
            ColourType::Coloured => hsv_to_rgb(angle / 360., 1., 1.),
        }
    }
}