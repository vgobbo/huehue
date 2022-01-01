use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
	pub x: f32,
	pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gamut {
	pub red: Component,
	pub green: Component,
	pub blue: Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
	pub gamut: Gamut,
	pub gamut_type: String,
	pub xy: Component,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirekSchema {
	pub mirek_maximum: u32,
	pub mirek_minimum: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temperature {
	pub mirek: Option<u32>,
	pub mirek_schema: MirekSchema,
	pub mirek_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RGB8 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl RGB8 {
	pub fn new(r: u8, g: u8, b: u8) -> RGB8 {
		RGB8 { r, g, b }
	}
}

impl Component {
	pub fn new(x: f32, y: f32) -> Option<Component> {
		if x >= 0f32 && y >= 0f32 {
			Some(Component { x, y })
		} else {
			None
		}
	}

	pub fn unchecked(x: f32, y: f32) -> Component {
		Self::new(x, y).expect(format!("Values ({}, {}) invalid.", x, y).as_str())
	}

	pub fn distance2(&self, p: &Component) -> f32 {
		(self.x - p.x).powf(2.0) + (self.y - p.y).powf(2.0)
	}
}

impl Gamut {
	pub fn new(red: Component, green: Component, blue: Component) -> Gamut {
		Gamut { red, green, blue }
	}

	pub fn contains(&self, xy: &Component) -> bool {
		let s = (self.red.x - self.blue.x) * (xy.y - self.blue.y) - (self.red.y - self.blue.y) * (xy.x - self.blue.x);
		let t = (self.green.x - self.red.x) * (xy.y - self.red.y) - (self.green.y - self.red.y) * (xy.x - self.red.x);

		if s != 0f32 && t != 0f32 {
			if (s < 0f32 && t >= 0f32) || (s >= 0f32 && t < 0f32) {
				return false;
			}
		}

		let d =
			(self.blue.x - self.green.x) * (xy.y - self.green.y) - (self.blue.y - self.green.y) * (xy.x - self.green.x);
		return (d == 0f32) || (d < 0f32 && s + t <= 0f32) || (d >= 0f32 && s + t > 0f32);
	}

	pub fn restrain(&self, xy: &Component) -> Component {
		if self.contains(&xy) {
			xy.clone()
		} else {
			let rg = Self::restrain_point_in_segment(&xy, &self.red, &self.green);
			let gb = Self::restrain_point_in_segment(&xy, &self.green, &self.blue);
			let br = Self::restrain_point_in_segment(&xy, &self.blue, &self.red);

			let drg = rg.distance2(&xy);
			let dgb = gb.distance2(&xy);
			let dbr = br.distance2(&xy);

			if drg <= dgb && drg <= dbr {
				rg
			} else if dgb <= dbr && dgb <= drg {
				gb
			} else {
				br
			}
		}
	}

	fn restrain_point_in_segment(p: &Component, a: &Component, b: &Component) -> Component {
		let d2 = (a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0);
		let t = (0.0f32).max((1.0f32).min(((p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y)) / d2));
		Component::unchecked(a.x + t * (b.x - a.x), a.y + t * (b.y - a.y))
	}

	pub fn xy_from_rgb8(&self, rgb: RGB8) -> Component {
		let r = Self::gamma_correct(rgb.r as f32 / 255f32);
		let g = Self::gamma_correct(rgb.g as f32 / 255f32);
		let b = Self::gamma_correct(rgb.b as f32 / 255f32);

		let x_ = 0.4124564 * r + 0.3575761 * g + 0.1804375 * b;
		let y_ = 0.2126729 * r + 0.7151522 * g + 0.0721750 * b;
		let z_ = 0.0193339 * r + 0.1191920 * g + 0.9503041 * b;

		let x = x_ / (x_ + y_ + z_);
		let y = y_ / (x_ + y_ + z_);
		// we could calculate z the same way, but we don't need it.
		// we could also use y at its current value as brightness.

		self.restrain(&Component::unchecked(x, y))
	}

	pub fn xy_to_rgb8(&self, xy: &Component) -> RGB8 {
		let gxy = self.restrain(&xy);

		let x = gxy.x;
		let y = gxy.y;
		let z = 1.0 - gxy.x - gxy.y;

		// if brightness is supplied:
		// let x = (brightness / xy.y) * xy.x;
		// let y = brightness;
		// let z = (brightness / xy.y) * xy.z;

		let r = Self::gamma_inverse(3.2404542 * x + -1.5371385 * y + -0.4985314 * z);
		let g = Self::gamma_inverse(-0.9692660 * x + 1.8760108 * y + 0.0415560 * z);
		let b = Self::gamma_inverse(0.0556434 * x + -0.2040259 * y + 1.0572252 * z);

		RGB8::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
	}

	fn gamma_correct(c: f32) -> f32 {
		if c > 0.04045 {
			((c + 0.055) / (1.0 + 0.055)).powf(2.4)
		} else {
			c / 12.92
		}
	}

	fn gamma_inverse(c: f32) -> f32 {
		if c > 0.0031308 {
			(1.0 + 0.055) * c.powf(1.0 / 2.4) - 0.055
		} else {
			c * 12.92
		}
	}
}

impl Color {
	pub fn new(xy: Component, gamut: Gamut) -> Option<Color> {
		match gamut.contains(&xy) {
			true => {
				Some(Color {
					xy,
					gamut,
					gamut_type: "C".to_owned(),
				})
			},
			false => None,
		}
	}
}

#[macro_export]
macro_rules! assert_component_eq {
	($a:expr, $b:expr, $d:expr) => {
		assert!(
			!(($a.x - $b.x > $d) || ($b.x - $a.x > $d)),
			"x component {} and {} outside of range {} ({})",
			$a.x,
			$b.x,
			$d,
			($a.x - $b.x).abs()
		);
		assert!(
			!(($a.y - $b.y > $d) || ($b.y - $a.y > $d)),
			"y component {} and {} outside of range {} ({})",
			$a.y,
			$b.y,
			$d,
			($a.y - $b.y).abs()
		);
	};
}

#[macro_export]
macro_rules! assert_rgb_eq {
	($a:expr, $b:expr, $d:expr) => {
		assert!(
			!(($a.r as i16 - $b.r as i16).abs() > $d),
			"r component {} and {} outside of range {} ({})",
			$a.r,
			$b.r,
			$d,
			($a.r as i16 - $b.r as i16).abs()
		);
		assert!(
			!(($a.g as i16 - $b.g as i16).abs() > $d),
			"g component {} and {} outside of range {} ({})",
			$a.g,
			$b.g,
			$d,
			($a.g as i16 - $b.g as i16).abs()
		);
		assert!(
			!(($a.b as i16 - $b.b as i16).abs() > $d),
			"b component {} and {} outside of range {} ({})",
			$a.b,
			$b.b,
			$d,
			($a.b as i16 - $b.b as i16).abs()
		);
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn gamut_contains() {
		let gamut = Gamut::new(
			Component::unchecked(0.6915f32, 0.3083f32),
			Component::unchecked(0.17f32, 0.7f32),
			Component::unchecked(0.1532f32, 0.0475f32),
		);
		assert!(gamut.contains(&Component::unchecked(0.2986f32, 0.3425f32)));
		assert!(gamut.contains(&Component::unchecked(0.675f32, 0.308f32)));
		assert!(gamut.contains(&Component::unchecked(0.4f32, 0.4f32)));
		assert!(!gamut.contains(&Component::unchecked(0.5f32, 0.5f32)));
		assert!(!gamut.contains(&Component::unchecked(0.01f32, 0.01f32)));
	}

	#[test]
	fn gamut_xy_from_rgb_on_edge() {
		let gamut = Gamut::new(
			Component::unchecked(0.6915f32, 0.3083f32),
			Component::unchecked(0.17f32, 0.7f32),
			Component::unchecked(0.1532f32, 0.0475f32),
		);

		assert_component_eq!(
			Component::unchecked(0.6399f32, 0.3300f32),
			gamut.xy_from_rgb8(RGB8::new(255, 0, 0)),
			0.0001
		);
		assert_component_eq!(
			Component::unchecked(0.3f32, 0.6f32),
			gamut.xy_from_rgb8(RGB8::new(0, 255, 0)),
			0.0001
		);
		assert_component_eq!(
			Component::unchecked(0.1535f32, 0.0599f32),
			gamut.xy_from_rgb8(RGB8::new(0, 0, 255)),
			0.0001
		);
	}

	#[test]
	fn gamut_xy_from_rgb_inside() {
		let gamut = Gamut::new(
			Component::unchecked(0.6915f32, 0.3083f32),
			Component::unchecked(0.17f32, 0.7f32),
			Component::unchecked(0.1532f32, 0.0475f32),
		);

		assert_component_eq!(
			Component::unchecked(0.3127301, 0.32901987),
			gamut.xy_from_rgb8(RGB8::new(128, 128, 128)),
			0.0001
		);
	}

	#[test]
	fn gamut_xy_to_rgb_on_edge() {
		let gamut = Gamut::new(
			Component::unchecked(0.6915f32, 0.3083f32),
			Component::unchecked(0.17f32, 0.7f32),
			Component::unchecked(0.1532f32, 0.0475f32),
		);

		assert_rgb_eq!(
			RGB8::new(255, 0, 0),
			gamut.xy_to_rgb8(&Component::unchecked(0.6399f32, 0.3300f32)),
			1
		);
		assert_rgb_eq!(
			RGB8::new(0, 236, 0),
			gamut.xy_to_rgb8(&Component::unchecked(0.3f32, 0.6f32)),
			1
		);
		assert_rgb_eq!(
			RGB8::new(30, 0, 234),
			gamut.xy_to_rgb8(&Component::unchecked(0.1535f32, 0.0599f32)),
			1
		);
	}

	#[test]
	fn gamut_xy_to_rgb_inside() {
		let gamut = Gamut::new(
			Component::unchecked(0.6915f32, 0.3083f32),
			Component::unchecked(0.17f32, 0.7f32),
			Component::unchecked(0.1532f32, 0.0475f32),
		);

		// we don't have brightness component, so it is not RGB(128, 128, 128) as in gamut_xy_from_rgb_inside.
		assert_rgb_eq!(
			RGB8::new(155, 155, 155),
			gamut.xy_to_rgb8(&Component::unchecked(0.3127301, 0.32901987)),
			1
		);
	}
}
