use crate::color::{Color, Temperature};
use crate::http::HueError;
use crate::models::lights::{GetLightsResponseItem, LightOnRequest};
use crate::models::GenericResponse;
use crate::{http, Client};

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

	pub async fn switch(&mut self, on: bool) -> Result<(), HueError> {
		let url = self
			.client
			.bridge()
			.url(format!("clip/v2/resource/light/{}", self.id).as_str());
		let application_key = self.client.application_key().clone().unwrap();
		let request_payload = LightOnRequest::new(on);

		match http::put_auth::<GenericResponse, LightOnRequest>(application_key, url, &request_payload).await {
			Ok(_) => {
				self.on = on;
				Ok(())
			},
			Err(e) => Err(e),
		}
	}
}
