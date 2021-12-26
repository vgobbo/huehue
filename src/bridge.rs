use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::time::Duration;

use mdns_sd::{ServiceDaemon, ServiceEvent};
use reqwest::Url;

use crate::{http, models};

const SERVICE_NAME: &str = "_hue._tcp.local.";
const VERSION_MIN: &str = "1948086000";

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Error {
	Connection,
	NotHue,
	UnknownModel,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Model {
	BSB001,
	BSB002,
	Unknown,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
		let client = http::client();
		client
			.get(url.to_string())
			.send()
			.await
			.ok()?
			.json::<models::Config>()
			.await
			.ok()
	}

	pub fn url(&self, path: &str) -> url::Url {
		Url::parse(format!("https://{}/{}", self.address.to_string(), path).as_str()).unwrap()
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
