use crate::models::lights::GetLightsResponseItem;

pub type Lights = Vec<Light>;

#[derive(Debug, Clone)]
pub struct Light {
	pub id: uuid::Uuid,
	pub name: String,
	pub on: bool,
	pub brightness: f32,
}

impl Light {}

impl From<GetLightsResponseItem> for Light {
	fn from(value: GetLightsResponseItem) -> Self {
		Light {
			id: value.id,
			name: value.metadata.name,
			on: value.on.on,
			brightness: value.dimming.brightness,
		}
	}
}
