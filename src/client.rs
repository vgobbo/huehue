use reqwest::Error;

use crate::models::create_user::{CreateUserRequest, CreateUserResponse};
use crate::models::device_type::DeviceType;
use crate::models::error::ErrorCode;
use crate::{http, Bridge};

#[derive(Debug)]
pub enum AuthorizationError {
	Hue(ErrorCode),
	AlreadyAuthorized,
	Connection,
	Response,
	Unexpected,
	Unknown,
}

pub struct Client {
	bridge: Bridge,
	device_type: DeviceType,
	application_key: Option<String>,
}

impl Client {
	pub fn new(bridge: Bridge, device_type: DeviceType) -> Client {
		Client {
			bridge,
			device_type,
			application_key: None,
		}
	}

	pub fn device_type(&self) -> &DeviceType {
		&self.device_type
	}

	pub fn application_key(&self) -> Option<String> {
		self.application_key.clone()
	}

	pub async fn authorize(&mut self) -> Result<(), AuthorizationError> {
		if self.application_key.is_some() {
			return Err(AuthorizationError::AlreadyAuthorized);
		}

		let request = CreateUserRequest::new(self.device_type.clone());

		let response = match http::client().post(self.bridge.url("api")).json(&request).send().await {
			Ok(response) => response,
			Err(e) => return Err(AuthorizationError::from(e)),
		};

		let payload = match response.json::<CreateUserResponse>().await {
			Ok(data) => data,
			Err(e) => return Err(AuthorizationError::from(e)),
		};
		if payload.len() != 1 {
			return Err(AuthorizationError::Unexpected);
		}

		let data = payload.get(0).unwrap();
		if let Some(data) = &data.success {
			self.application_key = Some(data.username.to_owned());
			return Ok(());
		}
		if let Some(error) = &data.error {
			return Err(AuthorizationError::Hue(error.r#type.clone()));
		}

		Err(AuthorizationError::Unknown)
	}
}

impl From<Error> for AuthorizationError {
	fn from(e: Error) -> Self {
		if e.is_connect() {
			return AuthorizationError::Connection;
		}
		if e.is_decode() {
			return AuthorizationError::Response;
		}

		AuthorizationError::Unknown
	}
}
