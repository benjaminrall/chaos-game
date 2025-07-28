use std::collections::VecDeque;
use image::{ImageBuffer, Rgb};
use rand::{rng, Rng};
use crate::types::{FractalSettings, Vertex, Vec2};

// A simple pixel struct to hold intermediate floating-point color data.
#[derive(Clone, Copy, Default)]
struct Pixel {
    r: f64,
    g: f64,
    b: f64,
}

/// The main fractal generator struct, which holds the configuration and state required
/// to generate a fractal image using the Chaos Game algorithm.
pub struct FractalGenerator {
    settings: FractalSettings,
    image_size: usize,
    iterations: u64,
    colour_scale: f64,
    polygon: Vec<Vertex>,
}

impl FractalGenerator {
    /// Creates a new `FractalGenerator` with the given settings.
    pub fn new(settings: FractalSettings, image_size: usize, iterations: u64) -> FractalGenerator {
        // Calculates the colour scale based on the image size and number of iterations
        let scale: f64 = iterations as f64 / (image_size * image_size) as f64;
        let colour_scale: f64 = scale * 10. / 3.;

        // Pre-computes the fractal's polygon vertices to be stored in the generator instance
        let polygon = FractalGenerator::create_polygon(&settings);

        FractalGenerator {
            settings,
            image_size,
            iterations,
            colour_scale,
            polygon,
        }
    }


    /// Generates the fractal image by running the Chaos Game algorithm.
    ///
    /// # Arguments
    ///
    /// * `on_progress`: A closure that is called each iteration with the current progress.
    ///
    /// returns: An ImageBuffer containing the final rendered fractal.
    pub fn generate<F>(&self, mut on_progress: F) -> ImageBuffer<Rgb<u8>, Vec<u8>>
    where
        F: FnMut(u64) -> (),
    {
        // Creates a list of all pixels in the image
        let mut pixels = vec![Pixel::default(); self.image_size * self.image_size];

        // Sets the initial point, and creates the previous points buffer
        let mut current_point = Vec2::default();
        let mut previous_points = VecDeque::with_capacity(self.settings.history);

        let mut rng = rng();
        for i in 0..self.iterations {
            // Gets a random vertex that follows the fractal's rules
            let vertex = loop {
                let random_vertex = &self.polygon[rng.random_range(0..self.settings.sides)];
                if (self.settings.rule)(&previous_points, random_vertex) {
                    break random_vertex;
                }
            };

            // Calculates the next point's position using the fractal's ratio
            current_point = current_point + (vertex.pos - current_point) * self.settings.ratio;

            // Finds the point's position on the screen, and updates that pixel's colour
            let screen_point = 0.5 * (self.image_size as f64 * current_point + Vec2::new(self.image_size as f64, self.image_size as f64));
            let index = screen_point.x as usize + screen_point.y as usize * self.image_size;
            if let Some(pixel) = pixels.get_mut(index) {
                pixel.r = (pixel.r + vertex.colour.0[0] as f64 / self.colour_scale).min(255.0);
                pixel.g = (pixel.g + vertex.colour.0[1] as f64 / self.colour_scale).min(255.0);
                pixel.b = (pixel.b + vertex.colour.0[2] as f64 / self.colour_scale).min(255.0);
            }

            // Shifts and updates the previous points array
            if self.settings.history > 0 {
                if previous_points.len() == self.settings.history {
                    previous_points.pop_back(); // Remove the oldest element from the end
                }
                previous_points.push_front(vertex.clone()); // Add the newest element to the front
            }

            // Reports the current progress
            on_progress(i);
        }

        // Writes the final pixel data to an image buffer
        let mut buffer = ImageBuffer::new(self.image_size as u32, self.image_size as u32);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let p = &pixels[(y * self.image_size as u32 + x) as usize];
            *pixel = Rgb([p.r as u8, p.g as u8, p.b as u8])
        }

        buffer
    }

    /// Constructs a list of the vertices of the fractal polygon.
    fn create_polygon(settings: &FractalSettings) -> Vec<Vertex> {
        let mut vertices = Vec::with_capacity(settings.sides);
        let delta = 360.0 / settings.sides as f64;

        for i in 0..settings.sides {
            let angle = i as f64 * delta + settings.rotation_offset;
            let radians = angle.to_radians();
            let pos = Vec2::new(radians.sin(), -radians.cos());
            vertices.push(Vertex::new(i, pos, angle, settings));
        }
        vertices
    }
}