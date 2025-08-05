//! A simple command-line application for generating fractals using the 'Chaos Game' algorithm.

pub mod types;
mod fractal_generator;
pub mod utils;
mod cli;
mod rules;

pub use cli::Cli;
pub use fractal_generator::FractalGenerator;
pub use rules::get_rule_by_name;