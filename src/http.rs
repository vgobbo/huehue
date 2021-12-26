use reqwest::{Certificate, Client};

use crate::certificate::CERTIFICATE;

pub fn client() -> Client {
	reqwest::Client::builder()
		.add_root_certificate(Certificate::from_pem(CERTIFICATE.as_bytes()).unwrap())
		.danger_accept_invalid_hostnames(true)
		.build()
		.unwrap()
}
