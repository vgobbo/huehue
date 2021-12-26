use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Discovery {
	pub id: String,
	pub internalipaddress: String,
	pub port: u16,
}
