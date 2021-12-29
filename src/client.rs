use crate::http::HueError;
use crate::light::Lights;
use crate::models::create_user::{CreateUserRequest, CreateUserResponse};
use crate::models::device_type::DeviceType;
use crate::models::lights::GetLightsResponse;
use crate::{http, Bridge, Light};

#[derive(Debug, Clone)]
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

	pub fn new_with_key(bridge: Bridge, device_type: DeviceType, application_key: String) -> Client {
		Client {
			bridge,
			device_type,
			application_key: Some(application_key),
		}
	}

	pub fn bridge(&self) -> &Bridge {
		&self.bridge
	}

	pub fn device_type(&self) -> &DeviceType {
		&self.device_type
	}

	pub fn application_key(&self) -> Option<String> {
		self.application_key.clone()
	}

	fn check_authorization(&self) -> Result<(), HueError> {
		if self.application_key.is_some() {
			Ok(())
		} else {
			Err(HueError::AlreadyAuthorized)
		}
	}

	pub async fn authorize(&mut self) -> Result<(), HueError> {
		if self.application_key.is_some() {
			return Err(HueError::AlreadyAuthorized);
		}

		let request = CreateUserRequest::new(self.device_type.clone());

		let response = match http::build().post(self.bridge.url("api")).json(&request).send().await {
			Ok(response) => response,
			Err(e) => return Err(HueError::from(e)),
		};

		let payload = match response.json::<CreateUserResponse>().await {
			Ok(data) => data,
			Err(e) => return Err(HueError::from(e)),
		};
		if payload.len() != 1 {
			return Err(HueError::Unexpected);
		}

		let data = payload.get(0).unwrap();
		if let Some(data) = &data.success {
			self.application_key = Some(data.username.to_owned());
			return Ok(());
		}
		if let Some(error) = &data.error {
			return Err(HueError::from(error.r#type.clone()));
		}

		Err(HueError::Unknown)
	}

	pub async fn lights(&self) -> Result<Lights, HueError> {
		self.check_authorization()?;

		let response = match http::build_with_key(self.application_key.clone().unwrap())
			.get(self.bridge.url("clip/v2/resource/light"))
			.send()
			.await
		{
			Ok(response) => response,
			Err(e) => return Err(HueError::from(e)),
		};

		let payload = match response.json::<GetLightsResponse>().await {
			Ok(payload) => payload,
			Err(e) => return Err(HueError::from(e)),
		};

		if let Some(data) = payload.data {
			return Ok(data.into_iter().map(|datum| Light::new(self, datum)).collect());
		}
		if let Some(error) = payload.error {
			return Err(HueError::from(error.r#type.clone()));
		}

		Err(HueError::Unknown)
	}
}
