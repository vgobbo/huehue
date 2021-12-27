use serde::{Deserialize, Serialize};

use crate::color::{Color, Temperature};
use crate::models::device_type::DeviceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
	pub archetype: String,
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct On {
	pub on: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimming {
	pub brightness: f32,
	pub min_dim_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLightsResponse {
	pub data: Option<Vec<GetLightsResponseItem>>,
	pub error: Option<crate::models::Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLightsResponseItem {
	#[serde(rename = "type")]
	pub r#type: String,

	pub id: uuid::Uuid,
	pub metadata: Metadata,
	pub dimming: Dimming,
	pub on: On,

	pub color: Option<Color>,
	pub color_temperature: Temperature,
}
