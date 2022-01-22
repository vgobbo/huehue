use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
	pub archetype: String,
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductData {
	pub certified: bool,
	pub model_id: String,
	pub manufacturer_name: String,
	pub product_archetype: String,
	pub product_name: String,
	pub software_version: String,
}
