use image::RgbImage;
use indicatif::ParallelProgressIterator;
use noise::{Fbm, NoiseFn, Perlin};
use rand::Rng;
use rayon::prelude::*;

mod color;
use color::Color;

fn noise_to_color(noise: f64, color1: Color, color2: Color, bands: f64, band_width: f64) -> Color {
	if (noise.abs() % (1.0 / bands)) < band_width {
		color1
	}
	else {
		color2
	}
}

// returns percentage (150% - 50%) for how much a sample point should add to the final color
// avarage between offset = -0.5 and offset = 0.5 is 100%
fn get_sample_importance(x_offset: f64, y_offset: f64) -> f64 {
	let offset = (x_offset * x_offset + y_offset * y_offset).sqrt();
	-2.0 * offset + 1.5
}

fn generate(filepath: &str, width: usize, height: usize, samples: usize, scale: f64, seed: u32) {
	let fbm = Fbm::<Perlin>::new(seed);

	let mut image = vec![Color::default(); width * height];
	image
		.par_chunks_exact_mut(width)
		.enumerate()
		.progress()
		.for_each(|(y, row)| {
			let mut rng = rand::thread_rng();
			for (x, output) in row.iter_mut().enumerate() {
				for _ in 0..samples {
					let x = x as f64 * scale;
					let y = y as f64 * scale;
					let x_offset = rng.gen_range(-0.5..0.5);
					let y_offset = rng.gen_range(-0.5..0.5);
					let sample_importance = get_sample_importance(x_offset, y_offset);
					let value = fbm.get([
						(x + x_offset) / width as f64,
						(y + y_offset) / height as f64,
					]);
					let color = noise_to_color(
						value,
						Color { r: 1.0, g: 1.0, b: 1.0 },
						Color { r: 0.0, g: 0.0, b: 0.0 },
						100.0,
						0.0018,
					);
					*output += color * sample_importance * (1.0 / samples as f64);
				}
			}
		});

	let mut output = RgbImage::new(width as u32, height as u32);
	for y in 0..height {
		for x in 0..width {
			let color = image[y * width + x];
			output.put_pixel(
				x as u32,
				y as u32,
				image::Rgb([
					(color.r * 255.0) as u8,
					(color.g * 255.0) as u8,
					(color.b * 255.0) as u8,
				]),
			);
		}
	}
	output.save(filepath).expect("Failed to save output.png");
}

fn main() {
	generate("output.png", 2560, 1440, 200, 0.75, 0);
}

#[test]
fn generate_preview() {
	generate("preview.png", 800, 500, 25, 0.75, 0);
}
