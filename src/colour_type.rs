use image::Rgb;
use chaos_game::hsv_to_rgb;
use crate::polygon_point::PolygonPoint;

pub enum ColourType {
    White,
    Coloured,
}

impl ColourType {
    pub fn get_colour(&self, point: &PolygonPoint) -> Rgb<u8> {
        return match self {
            ColourType::White => Rgb([255, 255, 255]),
            ColourType::Coloured => hsv_to_rgb(point.angle / 360., 1., 1.),
        }
    }
}