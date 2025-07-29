use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Number of sides for the polygon
    #[arg(short = 'n', long, default_value_t = 3)]
    pub sides: usize,

    /// Ratio for point interpolation (between 0.0 and 1.0)
    #[arg(short, long, default_value_t = 0.5)]
    pub ratio: f64,

    /// Number of iterations to run
    #[arg(short, long, default_value_t = 100_000_000)]
    pub iterations: u64,

    /// Output filename
    #[arg(short, long, default_value_t = String::from("output.png"))]
    pub output: String,

    /// Flag to generate a coloured fractal based on vertex angle
    #[arg(short, long)]
    pub coloured: bool,

    #[arg(long, default_value_t = 4.0)]
    pub colour_scale: f64,

    /// Image width and height in pixels
    #[arg(long, default_value_t = 1000)]
    pub image_size: usize,

    /// Rotation offset for the polygon (in degrees)
    #[arg(long, default_value_t = 0.0)]
    pub rotation_offset: f64,

    /// Name of the rule to be used
    #[arg(long, default_value_t = String::from("default"))]
    pub rule: String,
}