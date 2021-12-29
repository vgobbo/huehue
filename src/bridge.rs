use std::net::Ipv4Addr;

use crate::models;

const VERSION_MIN: &str = "1948086000";

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Error {
	Connection,
	NotHue,
	UnknownModel,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Model {
	BSB001,
	BSB002,
	Unknown,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Bridge {
	pub id: String,
	pub model: Model,
	pub version: String,
	pub address: Ipv4Addr,
	pub supported: bool,
}

impl From<(Ipv4Addr, models::Config)> for Bridge {
	fn from(data: (Ipv4Addr, models::Config)) -> Self {
		Bridge {
			id: data.1.bridgeid,
			model: Model::from(&data.1.modelid),
			version: data.1.swversion.clone(),
			address: data.0,
			supported: data.1.swversion.as_str() >= VERSION_MIN,
		}
	}
}

impl From<&String> for Model {
	fn from(value: &String) -> Self {
		match value.as_str() {
			"BSB001" => Model::BSB001,
			"BSB002" => Model::BSB002,
			_ => Model::Unknown,
		}
	}
}
