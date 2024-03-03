#[derive(Debug, Copy, Clone)]
pub struct Color {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl Color {
	pub fn new(r: f64, g: f64, b: f64) -> Self {
		Self { r, g, b }
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::new(0.0, 0.0, 0.0)
	}
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
impl std::ops::AddAssign<Color> for Color {
	fn add_assign(&mut self, rhs: Color) {
		*self = *self + rhs
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