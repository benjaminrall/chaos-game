use image::Rgb;
use crate::colour_type::ColourType;
use crate::vec2::Vec2;

pub struct PolygonPoint {
    pub index: u32,
    pub sides: u32,
    pub pos: Vec2,
    pub angle: f64,
    pub colour: Rgb<u8>,
}

impl PolygonPoint {
    pub fn new(index: u32, sides: u32, pos: Vec2, angle: f64, colour_type: &ColourType) -> PolygonPoint {
        let mut p = PolygonPoint { index, sides, pos, angle, colour: Rgb([0, 0, 0]) };
        p.colour = colour_type.get_colour(&p);
        p
    }
}