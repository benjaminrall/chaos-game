//! A simple command-line application for generating fractals using the 'Chaos Game' algorithm.

pub mod types;
pub mod fractal_generator;
pub mod utils;
pub mod cli;
pub mod rules;

pub use fractal_generator::FractalGenerator;