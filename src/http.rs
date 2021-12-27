use reqwest::header::HeaderMap;
use reqwest::{Certificate, Client, ClientBuilder};

use crate::certificate::CERTIFICATE;

const HUE_APPLICATION_KEY_HEADER: &str = "hue-application-key";

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
