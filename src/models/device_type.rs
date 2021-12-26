use serde::{Deserialize, Serialize};

/// Represents the weirdly named `devicetype`, used as identification during authentication.
#[derive(Serialize, Deserialize)]
pub struct DeviceType {
	pub application_name: String,
	pub device_name: String,
}

impl ToString for DeviceType {
	fn to_string(&self) -> String {
		format!("{}#{}", self.application_name, self.device_name)
	}
}
