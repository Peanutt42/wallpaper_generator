use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

use image::RgbImage;

struct Color {
	r: f64,
	g: f64,
	b: f64,
}

impl std::ops::Add<Color> for Color {
	type Output = Color;
	fn add(self, rhs: Color) -> Self::Output {
		Color {
			r: self.r + rhs.r,
			g: self.g + rhs.g,
			b: self.b + rhs.b,
		}
	}
}
impl std::ops::Mul<f64> for Color {
	type Output = Color;

	fn mul(self, rhs: f64) -> Color {
		Color {
			r: self.r * rhs,
			g: self.g * rhs,
			b: self.b * rhs,
		}
	}
}
impl std::ops::Mul<Color> for f64 {
	type Output = Color;

	fn mul(self, rhs: Color) -> Self::Output {
		Color {
			r: self * rhs.r,
			g: self * rhs.g,
			b: self * rhs.b,
		}
	}
}

fn noise_to_color(noise: f64, color1: Color, color2: Color) -> Color {
	if (noise.abs() % 0.05) < 0.005 { color1 } else { color2 }
	//let t = ((noise % 0.25).abs() / 0.1).powf(0.1);
	//(1.0 - t) * color1 + t * color2
}

fn main() {
	let fbm = Fbm::<Perlin>::new(0);

	const WIDTH: usize = 2560;
	const HEIGHT: usize = 1440;
	const ASPECT: f64 = WIDTH as f64 / HEIGHT as f64;

	let noise = PlaneMapBuilder::<_, 2>::new(&fbm)
		.set_size(WIDTH, HEIGHT)
		.set_x_bounds(-0.25 * ASPECT, 0.25)
		.set_y_bounds(-0.25 * ASPECT, 0.25)
		.build();
	
	let mut output = RgbImage::new(WIDTH as u32, HEIGHT as u32);
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			let value = noise.get_value(x, y);
			let color = noise_to_color(
				value,
				Color{r: 115.0 / 255.0, g: 0.0, b: 1.0},
				Color{ r:0.0, g:0.0, b: 0.0}
			);
			output.put_pixel(x as u32, y as u32, image::Rgb([(color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8]));
		}
	}
	output.save("output.png").expect("Failed to save output.png");
}