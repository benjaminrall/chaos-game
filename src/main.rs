use crate::vec2::Vec2;
use image::{ ImageBuffer, Rgb };
use indicatif::{ ProgressBar, ProgressStyle };
use chaos_game::*;
use crate::colour_type::ColourType;
use crate::fractal_settings::FractalSettings;
use crate::polygon_point::PolygonPoint;


mod vec2;
mod colour_type;
mod polygon_point;
mod fractal_settings;

const IMAGE_SIZE: usize = 10000;
const SCALE: usize = 10;
const COLOUR_SCALE: f64 = SCALE as f64 * 10./3.;
const ITERATIONS: u64 = (IMAGE_SIZE * IMAGE_SIZE * SCALE) as u64;
const PREVIOUS_POINTS: usize = 3;
const FRACTAL_PRESET: i32 = 3;

fn main() {

    // Creates fractal settings from a preset
    let fractal_settings = match FRACTAL_PRESET {
        1 => FractalSettings::basic(5, 0., 0.5, ColourType::Coloured),
        2 => FractalSettings::no_repeat(5, 0., 0.5, ColourType::Coloured),
        3 => FractalSettings::special(5, 0., 0.5, ColourType::Coloured,
            |a, b| if a.len() < 2 { true } else {
                if a[0].pos.x == a[1].pos.x && a[0].pos.y == a[1].pos.y {
                    b.index != (a[0].index + 1) % b.sides && b.index != (a[0].index + b.sides - 1) % b.sides
                } else {
                    true
                }
            }
        ),
        _ => FractalSettings::basic(3, 0., 0.5, ColourType::White)
    };

    // Sets up progress bar
    let progress_bar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .progress_chars("#>-");
    let progress_bar = ProgressBar::new(ITERATIONS);
    progress_bar.set_style(progress_bar_style.clone());

    // Creates polygon and sets up pixel array
    let polygon = create_polygon(&fractal_settings);
    let mut current_point = Vec2::zero();
    let mut pixels = vec![Rgb([0 as u8, 0 as u8, 0 as u8]); IMAGE_SIZE * IMAGE_SIZE];
    let mut pixels = vec![vec![0., 0., 0.]; IMAGE_SIZE * IMAGE_SIZE];

    // Performs iterations
    let mut previous_points = vec![];
    for _ in 0..ITERATIONS {
        // Gets a random polygon and ensures it is valid using the fractal settings' condition
        let mut polygon_point = &polygon[
            random_int(0, (polygon.len() - 1) as i32) as usize
        ];
        while !(fractal_settings.special_condition)(&previous_points, polygon_point) {
            polygon_point = &polygon[
                random_int(0, (fractal_settings.sides - 1) as i32) as usize
            ];
        }

        // Interpolates the next point using the fractal settings' ratio
        let position = polygon_point.pos;
        let next_point = current_point + (position - current_point) * fractal_settings.ratio;

        // Finds the screen point and uses it to update that pixel's colour
        let screen_point = get_screen_point(next_point);
        let index = screen_point.x as usize + screen_point.y as usize * IMAGE_SIZE;
        pixels[index] = mark_pixel(&pixels[index], &polygon_point.colour);

        // Shifts previous points array
        if previous_points.len() < PREVIOUS_POINTS {
            previous_points.insert(0, polygon_point);
        } else {
            for i in 0..(PREVIOUS_POINTS - 1) {
                previous_points[i + 1] = previous_points[i];
            }
            previous_points[0] = polygon_point;
        }

        current_point = next_point;

        progress_bar.inc(1);
    }

    // Draws pixels to image
    let mut buffer = ImageBuffer::new(IMAGE_SIZE as u32, IMAGE_SIZE as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        *pixel = vec_to_rgb(&pixels[(y * IMAGE_SIZE as u32 + x) as usize]);
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing to file: {}", e),
        Ok(()) => eprintln!("Image saved successfully.")
    }
}

fn vec_to_rgb(pixel: &Vec<f64>) -> Rgb<u8> {
    Rgb([
        pixel[0] as u8,
        pixel[1] as u8,
        pixel[2] as u8
    ])
}

fn mark_pixel(pixel: &Vec<f64>, colour_info: &Rgb<u8>) -> Vec<f64> {
    vec![
        f64::min(pixel[0] + colour_info.0[0] as f64 / COLOUR_SCALE, 255.),
        f64::min(pixel[1] + colour_info.0[1] as f64 / COLOUR_SCALE, 255.),
        f64::min(pixel[2] + colour_info.0[2] as f64 / COLOUR_SCALE, 255.),
    ]
}

fn get_screen_point(world_point: Vec2) -> Vec2 {
    0.5 * (IMAGE_SIZE as f64 * world_point + Vec2::new(IMAGE_SIZE as f64, IMAGE_SIZE as f64))
}

fn create_polygon(fractal_settings: &FractalSettings) -> Vec<PolygonPoint> {
    let mut points = vec![];

    let delta = 360. / fractal_settings.sides as f64;
    for i in 0..fractal_settings.sides {
        let angle = i as f64 * delta + fractal_settings.rotation_offset;
        let radians = degrees_to_radian(angle);
        let y = radians.cos();
        let x = radians.sin();
        points.push(PolygonPoint::new(
            i, fractal_settings.sides,Vec2::new(x, -y), angle, &fractal_settings.colour_type
        ))
    }

    points
}
