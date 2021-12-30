use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericIdentifier {
	pub rid: uuid::Uuid,
	pub rtype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericError {
	pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericResponse {
	pub errors: Option<Vec<GenericError>>,
	pub data: Option<Vec<GenericIdentifier>>,
}
