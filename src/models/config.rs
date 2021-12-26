use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub name: String,
	pub datastoreversion: String,
	pub swversion: String,
	pub apiversion: String,
	pub mac: String,
	pub bridgeid: String,
	pub factorynew: bool,
	pub replacesbridgeid: Option<String>,
	pub modelid: String,
	pub starterkitid: Option<String>,
}
