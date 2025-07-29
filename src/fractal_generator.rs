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
    pub fn new(settings: FractalSettings, image_size: usize, iterations: u64, colour_scale: f64) -> FractalGenerator {
        // Calculates the pixel scale factor based on the image size, number of iterations, and additional colour scaling factor
        let iteration_scale = iterations as f64 / (image_size * image_size) as f64;
        let colour_scale = iteration_scale * colour_scale;

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
    /// * `rng`: An RNG instance used when selecting random vertices.
    /// * `on_progress`: A closure that is called each iteration with the current progress.
    ///
    /// returns: An ImageBuffer containing the final rendered fractal.
    pub fn generate<R, F>(&self, rng: &mut R, mut on_progress: F) -> ImageBuffer<Rgb<u8>, Vec<u8>>
    where
        R: Rng,
        F: FnMut(u64) -> (),
    {
        // Creates a list of all pixels in the image
        let mut pixels = vec![Pixel::default(); self.image_size * self.image_size];

        // Sets the initial point, and creates the history points buffer
        let mut current_point = Vec2::default();
        let mut history_points = VecDeque::with_capacity(self.settings.history);

        for i in 0..self.iterations {
            // Gets a random vertex that follows the fractal's rules
            let vertex = self.select_vertex(rng, &history_points);

            // Calculates the next point's position using the fractal's ratio
            current_point = current_point + (vertex.pos - current_point) * self.settings.ratio;

            // Plots the new point onto the pixel buffer
            self.plot_point(&mut pixels, current_point, vertex);

            // Shifts and updates the history points buffer
            if self.settings.history > 0 {
                if history_points.len() == self.settings.history {
                    history_points.pop_back();
                }
                history_points.push_front(vertex);
            }

            // Reports the current progress using the callback function
            on_progress(i);
        }

        // Writes the final pixel data to an image buffer and returns it
        self.pixels_to_buffer(&pixels)
    }

    /// Selects a random vertex from the polygon that satisfies the current rule.
    fn select_vertex<'a, R: Rng>(&'a self, rng: &mut R, history_points: &VecDeque<&Vertex>) -> &'a Vertex {
        loop {
            let random_vertex = &self.polygon[rng.random_range(0..self.settings.sides)];
            if (self.settings.rule)(history_points, random_vertex) {
                return random_vertex;
            }
        }
    }

    /// Plots a single point onto the pixel buffer using bilinear interpolation.
    fn plot_point(&self, pixels: &mut [Pixel], point: Vec2, vertex: &Vertex) {
        // Transforms the world coordinates (in the range [-1, 1]) into pixel coordinates
        let pixel_pos = 0.5 * (self.image_size as f64 * point + Vec2::new(self.image_size as f64, self.image_size as f64));

        // Gets the integer and fractional parts of the coordinates
        let x = pixel_pos.x.floor();
        let y = pixel_pos.y.floor();
        let dx = pixel_pos.x - x;
        let dy = pixel_pos.y - y;

        // Calculates weightings for the 4 surrounding pixels
        let w_tl = (1.0 - dx) * (1.0 - dy);
        let w_tr = dx * (1.0 - dy);
        let w_bl = (1.0 - dx) * dy;
        let w_br = dx * dy;

        // Distributes the point's color to the 4 surrounding pixels based on their weights
        self.update_pixel_with_weight(pixels, x as usize, y as usize, w_tl, vertex);
        self.update_pixel_with_weight(pixels, x as usize + 1, y as usize, w_tr, vertex);
        self.update_pixel_with_weight(pixels, x as usize, y as usize + 1, w_bl, vertex);
        self.update_pixel_with_weight(pixels, x as usize + 1, y as usize + 1, w_br, vertex);
    }

    /// Writes a list of floating-point pixel values to an ImageBuffer object
    fn pixels_to_buffer(&self, pixels: &[Pixel]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut buffer = ImageBuffer::new(self.image_size as u32, self.image_size as u32);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let p = &pixels[(y * self.image_size as u32 + x) as usize];
            *pixel = Rgb([p.r as u8, p.g as u8, p.b as u8])
        }
        buffer
    }

    /// Updates a pixel's colour, scaled by an interpolation weight.
    fn update_pixel_with_weight(&self, pixels: &mut [Pixel], x: usize, y: usize, weight: f64, vertex: &Vertex) {
        if x < self.image_size && y < self.image_size {
            let index = y * self.image_size + x;
            if let Some(pixel) = pixels.get_mut(index) {
                let r_contribution = vertex.colour.0[0] as f64 * weight / self.colour_scale;
                let g_contribution = vertex.colour.0[1] as f64 * weight / self.colour_scale;
                let b_contribution = vertex.colour.0[2] as f64 * weight / self.colour_scale;

                pixel.r = (pixel.r + r_contribution).min(255.);
                pixel.g = (pixel.g + g_contribution).min(255.);
                pixel.b = (pixel.b + b_contribution).min(255.);
            }
        }
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