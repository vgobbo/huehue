use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::generic::{GenericIdentifier, Metadata, ProductData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDevicesResponseItem {
	pub id: Uuid,
	pub metadata: Metadata,
	pub product_data: ProductData,
	pub services: Vec<GenericIdentifier>,

	#[serde(rename = "type")]
	pub device_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDevicesResponse {
	pub data: Option<Vec<GetDevicesResponseItem>>,
	pub error: Option<super::Error>,
}
