use crate::rules::RuleFn;
use crate::types::ColourType;

/// Struct that holds all configuration parameters for generating a fractal.
pub struct FractalSettings {
    pub sides: usize,
    pub ratio: f64,
    pub rotation_offset: f64,
    pub colour_type: ColourType,
    pub history: usize,
    pub rule: RuleFn,
}

impl FractalSettings {
    /// Constructs a new FractalSettings struct.
    pub fn new(
        sides: usize, ratio: f64, colour_type: ColourType, rotation_offset: f64, history: usize, rule: RuleFn
    ) -> FractalSettings {
        FractalSettings { sides, ratio, colour_type, rotation_offset, history, rule }
    }
}