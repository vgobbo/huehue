use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::time::Duration;

use mdns_sd::{ServiceDaemon, ServiceEvent};
use reqwest::{Certificate, Url};

use crate::models;

const SERVICE_NAME: &str = "_hue._tcp.local.";
const VERSION_MIN: &str = "1948086000";
const CERTIFICATE: &str = "-----BEGIN CERTIFICATE-----
MIICMjCCAdigAwIBAgIUO7FSLbaxikuXAljzVaurLXWmFw4wCgYIKoZIzj0EAwIw
OTELMAkGA1UEBhMCTkwxFDASBgNVBAoMC1BoaWxpcHMgSHVlMRQwEgYDVQQDDAty
b290LWJyaWRnZTAiGA8yMDE3MDEwMTAwMDAwMFoYDzIwMzgwMTE5MDMxNDA3WjA5
MQswCQYDVQQGEwJOTDEUMBIGA1UECgwLUGhpbGlwcyBIdWUxFDASBgNVBAMMC3Jv
b3QtYnJpZGdlMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEjNw2tx2AplOf9x86
aTdvEcL1FU65QDxziKvBpW9XXSIcibAeQiKxegpq8Exbr9v6LBnYbna2VcaK0G22
jOKkTqOBuTCBtjAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNV
HQ4EFgQUZ2ONTFrDT6o8ItRnKfqWKnHFGmQwdAYDVR0jBG0wa4AUZ2ONTFrDT6o8
ItRnKfqWKnHFGmShPaQ7MDkxCzAJBgNVBAYTAk5MMRQwEgYDVQQKDAtQaGlsaXBz
IEh1ZTEUMBIGA1UEAwwLcm9vdC1icmlkZ2WCFDuxUi22sYpLlwJY81Wrqy11phcO
MAoGCCqGSM49BAMCA0gAMEUCIEBYYEOsa07TH7E5MJnGw557lVkORgit2Rm1h3B2
sFgDAiEA1Fj/C3AN5psFMjo0//mrQebo0eKd3aWRx+pQY08mk48=
-----END CERTIFICATE-----";

pub enum Error {
	Connection,
	NotHue,
	UnknownModel,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Model {
	BSB001,
	BSB002,
	Unknown,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Bridge {
	pub id: String,
	pub model: Model,
	pub version: String,
	pub address: Ipv4Addr,
	pub supported: bool,
}

impl Bridge {
	pub async fn new(ip: Ipv4Addr) -> Result<Bridge, Error> {
		match Self::get_config(&ip).await {
			Some(config) => Ok(Bridge::from((ip, config))),
			None => Err(Error::Connection),
		}
	}

	pub async fn discover(timeout: Duration) -> Vec<Bridge> {
		let ips: HashSet<Ipv4Addr> = Self::discover_mdns(timeout)
			.await
			.into_iter()
			.chain(Self::discover_meethue().await)
			.collect();

		let mut bridges = Vec::new();
		for ip in ips {
			match Self::get_config(&ip).await {
				Some(config) => bridges.push(Bridge::from((ip, config))),
				None => (),
			}
		}

		bridges
	}

	async fn discover_mdns(timeout: Duration) -> HashSet<Ipv4Addr> {
		let mut ips = HashSet::new();

		if let Ok(mdns) = ServiceDaemon::new() {
			if let Ok(receiver) = mdns.browse(SERVICE_NAME) {
				let end_time = std::time::SystemTime::now() + timeout;
				while std::time::SystemTime::now() < end_time {
					if let Ok(event) = receiver.recv_timeout(Duration::from_secs(1)) {
						match event {
							ServiceEvent::ServiceResolved(info) => {
								info.get_addresses().iter().for_each(|ip| drop(ips.insert(ip.to_std())))
							},
							_ => (),
						}
					}
				}
			}
		}

		ips
	}

	async fn discover_meethue() -> HashSet<Ipv4Addr> {
		if let Ok(response) = reqwest::get("https://discovery.meethue.com").await {
			if let Ok(bridges) = response.json::<Vec<models::meethue::Discovery>>().await {
				return bridges
					.into_iter()
					.filter_map(|bridge| bridge.internalipaddress.parse().ok())
					.collect();
			}
		}

		HashSet::new()
	}

	async fn get_config(ip: &Ipv4Addr) -> Option<models::Config> {
		let url = Url::parse(format!("https://{}/api/0/config", ip.to_string()).as_str()).unwrap();
		let client = reqwest::Client::builder()
			.add_root_certificate(Certificate::from_pem(CERTIFICATE.as_bytes()).unwrap())
			.danger_accept_invalid_hostnames(true)
			.build()
			.unwrap();
		client
			.get(url.to_string())
			.send()
			.await
			.ok()?
			.json::<models::Config>()
			.await
			.ok()
	}
}

impl From<(Ipv4Addr, models::Config)> for Bridge {
	fn from(data: (Ipv4Addr, models::Config)) -> Self {
		Bridge {
			id: data.1.bridgeid,
			model: Model::from(&data.1.modelid),
			version: data.1.swversion.clone(),
			address: data.0,
			supported: data.1.swversion.as_str() >= VERSION_MIN,
		}
	}
}

impl From<&String> for Model {
	fn from(value: &String) -> Self {
		match value.as_str() {
			"BSB001" => Model::BSB001,
			"BSB002" => Model::BSB002,
			_ => Model::Unknown,
		}
	}
}
