use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
	pub devicetype: String,
}

pub type CreateUserResponse = Vec<CreateUserResponseItem>;

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponseItem {
	pub success: CreateUserResponseItemSub,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponseItemSub {
	pub username: String,
}
