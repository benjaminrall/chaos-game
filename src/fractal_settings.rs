use crate::colour_type::ColourType;
use crate::polygon_point::PolygonPoint;

pub struct FractalSettings {
    pub sides: u32,
    pub rotation_offset: f64,
    pub ratio: f64,
    pub colour_type: ColourType,
    pub special_condition: fn(&Vec<&PolygonPoint>, &PolygonPoint) -> bool,
}

impl FractalSettings {
    pub fn basic(
        sides: u32, rotation_offset: f64, ratio: f64, colour_type: ColourType
    ) -> FractalSettings {
        FractalSettings {
            sides, rotation_offset, ratio, colour_type, special_condition: |_, _| true
        }
    }

    pub fn no_repeat(
        sides: u32, rotation_offset: f64, ratio: f64, colour_type: ColourType
    ) -> FractalSettings {
        FractalSettings {
            sides, rotation_offset, ratio, colour_type,
            special_condition: |a, b|
                if a.len() > 1 { a[0].pos.x != b.pos.x || a[0].pos.y != b.pos.y } else { true }
        }
    }

    pub fn special(
        sides: u32, rotation_offset: f64, ratio: f64, colour_type: ColourType,
        special_condition: fn(&Vec<&PolygonPoint>, &PolygonPoint) -> bool
    ) -> FractalSettings {
        FractalSettings {
            sides, rotation_offset, ratio, colour_type, special_condition
        }
    }
}