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
}
