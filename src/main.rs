use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rand::rng;
use chaos_game::cli::Cli;
use chaos_game::FractalGenerator;
use chaos_game::rules::get_rule_by_name;
use chaos_game::types::{ColourType, FractalSettings};

fn main() {
    // Parses command line arguments
    let args = Cli::parse();

    // Sets the colour type based on the coloured flag
    let colour_type = if args.coloured {
        ColourType::Coloured
    } else {
        ColourType::White
    };

    // Gets the Chaos Game rule to use from the specified rule name
    let rule = get_rule_by_name(&args.rule).unwrap_or_else(|| {
        eprintln!("Could not find the '{}' rule. Now running using the 'default' rule.", args.rule);
        get_rule_by_name("default").unwrap()
    });

    // Creates the fractal's settings object
    let settings = FractalSettings::new(
        args.sides,  args.ratio, colour_type, args.rotation_offset, rule.history, rule.function);

    // Creates the generator for the fractal
    let generator = FractalGenerator::new(settings, args.image_size, args.iterations, args.colour_scale);

    // Initialises progress bar
    let progress_bar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .progress_chars("#>-");
    let progress_bar = ProgressBar::new(args.iterations).with_style(progress_bar_style);

    // Generates and saves the fractal image
    let buffer = generator.generate(&mut rng(), |i| progress_bar.set_position(i));
    match buffer.save(args.output) {
        Err(e) => eprintln!("Error writing to file: {}", e),
        Ok(()) => eprintln!("Image saved successfully.")
    }
}
