use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type Errors = Vec<Error>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ErrorCode {
	Unauthorized = 1,
	InvalidJson = 2,
	ResourceNotAvailable = 3,
	ResourceMethodNotAvailable = 4,
	ParameterMissing = 5,
	ParameterNotAvailable = 6,
	ParameterValue = 7,
	ParameterNotModifiable = 8,
	TooManyItems = 11,
	PortalConnectionRequired = 12,
	InternalError = 901,

	LinkButtonNotPressed = 101,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
	#[serde(rename = "type")]
	pub r#type: ErrorCode,

	pub address: String,
	pub description: String,
}
