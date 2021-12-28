use crate::color::{Color, Temperature};
use crate::models::lights::GetLightsResponseItem;
use crate::Client;

pub type Lights = Vec<Light>;

#[derive(Debug, Clone)]
pub struct Light {
	pub client: Client,
	pub id: uuid::Uuid,
	pub name: String,
	pub on: bool,
	pub brightness: f32,
	pub color: Option<Color>,
	pub temperature: Temperature,
}

impl Light {
	pub fn new(client: &Client, light: GetLightsResponseItem) -> Light {
		Light {
			client: client.clone(),
			id: light.id,
			name: light.metadata.name,
			on: light.on.on,
			brightness: light.dimming.brightness,
			color: light.color,
			temperature: light.color_temperature,
		}
	}
}
