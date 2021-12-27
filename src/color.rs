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
