use serde::{Deserialize, Serialize};

use crate::models::error::Errors;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericResponse {
	pub errors: Option<Errors>,
}
