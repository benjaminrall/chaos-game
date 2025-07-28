use image::Rgb;
use crate::types::{FractalSettings, Vec2};

/// Struct to represent a single vertex of a polygon.
#[derive(Clone, Debug)]
pub struct Vertex {
    pub index: usize,
    pub pos: Vec2,
    pub sides: usize,
    pub colour: Rgb<u8>,
}

impl Vertex {
    /// Constructs a new vertex instance.
    pub fn new(index: usize, pos: Vec2, angle: f64, settings: &FractalSettings) -> Vertex {
        Vertex {
            index,
            pos,
            sides: settings.sides,
            colour: settings.colour_type.get_colour(angle)
        }
    }
}
