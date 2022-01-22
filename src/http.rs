use reqwest::header::HeaderMap;
use reqwest::{Certificate, Client, ClientBuilder, Error, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::certificate::CERTIFICATE;
use crate::models::error::ErrorCode;

const HUE_APPLICATION_KEY_HEADER: &str = "hue-application-key";

#[derive(Debug)]
pub enum HueError {
	Unauthorized,
	AlreadyAuthorized,
	Connection,
	Response(reqwest::Error),
	Unsupported,
	Unexpected,
	Unknown,
}

fn build_base() -> ClientBuilder {
	reqwest::Client::builder()
		.add_root_certificate(Certificate::from_pem(CERTIFICATE.as_bytes()).unwrap())
		.danger_accept_invalid_hostnames(true)
}

pub fn build() -> Client {
	build_base().build().unwrap()
}

pub fn build_with_key(application_key: String) -> Client {
	let mut headers = HeaderMap::new();
	headers.insert(HUE_APPLICATION_KEY_HEADER, application_key.parse().unwrap());

	build_base().default_headers(headers).build().unwrap()
}

pub async fn get_auth<R>(application_key: String, url: Url) -> Result<R, HueError>
where
	R: DeserializeOwned,
{
	let client = build_with_key(application_key);

	let response = match client.get(url).send().await {
		Ok(response) => response,
		Err(e) => return Err(HueError::from(e)),
	};

	match response.json::<R>().await {
		Ok(payload) => Ok(payload),
		Err(e) => Err(HueError::from(e)),
	}
}

#[allow(unused)]
pub async fn get_text(application_key: String, url: Url) -> Result<String, HueError> {
	let client = build_with_key(application_key);

	let response = match client.get(url).send().await {
		Ok(response) => response,
		Err(e) => return Err(HueError::from(e)),
	};

	match response.text().await {
		Ok(payload) => Ok(payload),
		Err(e) => Err(HueError::from(e)),
	}
}

pub async fn put_auth<R, T>(application_key: String, url: Url, object: &T) -> Result<R, HueError>
where
	T: Serialize,
	R: DeserializeOwned,
{
	let client = build_with_key(application_key);

	let response = match client.put(url).json(&object).send().await {
		Ok(response) => response,
		Err(e) => return Err(HueError::from(e)),
	};

	match response.json::<R>().await {
		Ok(payload) => Ok(payload),
		Err(e) => Err(HueError::from(e)),
	}
}

impl From<Error> for HueError {
	fn from(e: Error) -> Self {
		if e.is_status() {
			match e.status().unwrap() {
				StatusCode::UNAUTHORIZED => HueError::Unauthorized,
				_ => HueError::Unknown,
			}
		} else if e.is_connect() {
			return HueError::Connection;
		} else if e.is_decode() {
			return HueError::Response(e);
		} else {
			HueError::Unknown
		}
	}
}

impl From<ErrorCode> for HueError {
	fn from(ec: ErrorCode) -> Self {
		match ec {
			ErrorCode::Unauthorized => HueError::Unauthorized,
			ErrorCode::LinkButtonNotPressed => HueError::Unauthorized,
			_ => HueError::Unknown,
		}
	}
}
