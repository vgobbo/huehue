use std::net::Ipv4Addr;
use std::time::Duration;

use url::Url;

use crate::device::{Device, Devices};
use crate::http::HueError;
use crate::light::Lights;
use crate::models::create_user::{CreateUserRequest, CreateUserResponse};
use crate::models::device_type::DeviceType;
use crate::models::devices::GetDevicesResponse;
use crate::models::lights::GetLightsResponse;
use crate::{discover, http, models, Bridge, Light};

#[derive(Debug, Clone)]
pub struct Hue {
	bridge: Bridge,
	device_type: DeviceType,
	application_key: Option<String>,
}

impl Hue {
	pub async fn new(ip: Ipv4Addr, device_type: DeviceType) -> Result<Hue, HueError> {
		let bridge = match Self::get_config(&ip).await {
			Some(config) => Bridge::from((ip, config)),
			None => return Err(HueError::Connection),
		};

		Ok(Hue {
			bridge,
			device_type,
			application_key: None,
		})
	}

	pub async fn new_with_key(ip: Ipv4Addr, device_type: DeviceType, application_key: String) -> Result<Hue, HueError> {
		let bridge = match Self::get_config(&ip).await {
			Some(config) => Bridge::from((ip, config)),
			None => return Err(HueError::Connection),
		};

		Ok(Hue {
			bridge,
			device_type,
			application_key: Some(application_key),
		})
	}

	pub fn bridge(&self) -> &Bridge {
		&self.bridge
	}

	pub fn url(&self, path: &str) -> url::Url {
		Url::parse(format!("https://{}/{}", self.bridge.address.to_string(), path).as_str()).unwrap()
	}

	pub fn device_type(&self) -> &DeviceType {
		&self.device_type
	}

	pub fn application_key(&self) -> Option<String> {
		self.application_key.clone()
	}

	pub async fn bridges(timeout: Duration) -> Vec<Bridge> {
		let ips = discover::discover(timeout).await;
		let mut bridges = Vec::new();
		for ip in ips {
			match Self::get_config(&ip).await {
				Some(config) => bridges.push(Bridge::from((ip, config))),
				None => (),
			}
		}

		bridges
	}

	fn check_authorization(&self) -> Result<(), HueError> {
		if self.application_key.is_some() {
			Ok(())
		} else {
			Err(HueError::AlreadyAuthorized)
		}
	}

	async fn get_config(ip: &Ipv4Addr) -> Option<models::Config> {
		let url = Url::parse(format!("https://{}/api/0/config", ip.to_string()).as_str()).unwrap();
		let client = http::build();
		client
			.get(url.to_string())
			.send()
			.await
			.ok()?
			.json::<models::Config>()
			.await
			.ok()
	}

	pub async fn authorize(&mut self) -> Result<(), HueError> {
		if self.application_key.is_some() {
			return Err(HueError::AlreadyAuthorized);
		}

		let request = CreateUserRequest::new(self.device_type.clone());

		let response = match http::build().post(self.url("api")).json(&request).send().await {
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

		let response: GetLightsResponse = http::get_auth(
			self.application_key.clone().unwrap(),
			self.url("clip/v2/resource/light"),
		)
		.await?;

		if let Some(data) = response.data {
			return Ok(data.into_iter().map(|datum| Light::new(self, datum)).collect());
		}
		if let Some(error) = response.error {
			return Err(HueError::from(error.r#type.clone()));
		}

		Err(HueError::Unknown)
	}

	pub async fn devices(&self) -> Result<Devices, HueError> {
		self.check_authorization()?;

		let response: GetDevicesResponse = http::get_auth(
			self.application_key.clone().unwrap(),
			self.url("clip/v2/resource/device"),
		)
		.await?;

		if let Some(data) = response.data {
			return Ok(data.into_iter().map(|datum| Device::new(self, datum)).collect());
		}
		if let Some(error) = response.error {
			return Err(HueError::from(error.r#type.clone()));
		}

		Err(HueError::Unknown)
	}
}
