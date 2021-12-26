use serde::{Deserialize, Serialize};

use crate::models::device_type::DeviceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
	pub devicetype: String,
}

pub type CreateUserResponse = Vec<CreateUserResponseItem>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponseItem {
	pub success: Option<CreateUserResponseItemSub>,
	pub error: Option<crate::models::Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponseItemSub {
	pub username: String,
}

impl CreateUserRequest {
	pub fn new(device_type: DeviceType) -> CreateUserRequest {
		CreateUserRequest {
			devicetype: device_type.to_string(),
		}
	}
}
