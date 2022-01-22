use std::collections::HashSet;

use crate::models::devices::GetDevicesResponseItem;
use crate::models::generic::{GenericIdentifier, ProductData};
use crate::Hue;

pub type Devices = Vec<Device>;

#[derive(Debug, Clone)]
pub struct Device {
	pub hue: Hue,
	pub id: uuid::Uuid,
	pub name: String,
	pub product: ProductData,
	pub services: HashSet<GenericIdentifier>,
}

impl Device {
	pub fn new(hue: &Hue, device: GetDevicesResponseItem) -> Device {
		Device {
			hue: hue.clone(),
			id: device.id,
			name: device.metadata.name,
			product: device.product_data,
			services: device.services,
		}
	}
}
