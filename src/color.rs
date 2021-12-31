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

	pub fn restrain(&self, xy: Component) -> Component {
		if self.contains(&xy) {
			xy
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

	pub fn xy_from_rgb(&self, r: u8, g: u8, b: u8) -> Component {
		let r_ = Self::gamma_correct(r as f32 / 255f32);
		let g_ = Self::gamma_correct(g as f32 / 255f32);
		let b_ = Self::gamma_correct(b as f32 / 255f32);

		let x_ = 0.649926 * r_ + 0.103455 * g_ + 0.197109 * b_;
		let y_ = 0.234327 * r_ + 0.743075 * g_ + 0.022598 * b_;
		let z_ = 0.000000 * r_ + 0.053077 * g_ + 1.035763 * b_;

		let x = x_ / (x_ + y_ + z_);
		let y = y_ / (x_ + y_ + z_);
		// we could calculate z the same way, but we don't need it.

		self.restrain(Component::unchecked(x, y))
	}

	fn gamma_correct(c: f32) -> f32 {
		if c > 0.04045 {
			((c + 0.055) / (1.0 + 0.055)).powf(2.4)
		} else {
			c / 12.92
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

		assert_component_eq(
			Component::unchecked(0.6915f32, 0.3083f32),
			gamut.xy_from_rgb(255, 0, 0),
			0.0001,
		);
		assert_component_eq(
			Component::unchecked(0.17f32, 0.7f32),
			gamut.xy_from_rgb(0, 255, 0),
			0.0001,
		);
		assert_component_eq(
			Component::unchecked(0.1532f32, 0.0475f32),
			gamut.xy_from_rgb(0, 0, 255),
			0.0001,
		);
	}

	#[test]
	fn gamut_xy_from_rgb_inside() {
		let gamut = Gamut::new(
			Component::unchecked(0.6915f32, 0.3083f32),
			Component::unchecked(0.17f32, 0.7f32),
			Component::unchecked(0.1532f32, 0.0475f32),
		);

		assert_component_eq(
			Component::unchecked(0.3127301, 0.32901987),
			gamut.xy_from_rgb(128, 128, 128),
			0.0001,
		);
	}

	fn assert_component_eq(a: Component, b: Component, d: f32) {
		if (a.x - b.x > d) || (b.x - a.x > d) {
			panic!("x components {} and {} outside of range {}", a.x, b.x, d);
		}
		if (a.y - b.y > d) || (b.y - a.y > d) {
			panic!("y components {} and {} outside of range {}", a.y, b.y, d);
		}
	}
}
