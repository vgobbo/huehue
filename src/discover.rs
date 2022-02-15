use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::time::Duration;

use mdns_sd::{ServiceDaemon, ServiceEvent};
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "_hue._tcp.local.";

#[derive(Serialize, Deserialize)]
pub struct Discovery {
	pub id: String,

	#[serde(rename = "internalipaddress")]
	pub internal_ip_address: String,
	pub port: u16,
}

pub async fn discover(timeout: Duration) -> HashSet<Ipv4Addr> {
	discover_mdns(timeout)
		.await
		.into_iter()
		.chain(discover_meethue().await)
		.collect()
}

async fn discover_mdns(timeout: Duration) -> HashSet<Ipv4Addr> {
	let result = tokio::spawn(async move {
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

			// If shutdown fails there is not much we can do, so just accept it.
			drop(mdns.shutdown());
		}

		ips
	})
	.await;

	match result {
		Ok(ips) => ips,
		Err(_) => HashSet::new(),
	}
}

async fn discover_meethue() -> HashSet<Ipv4Addr> {
	if let Ok(response) = reqwest::get("https://discovery.meethue.com").await {
		if let Ok(bridges) = response.json::<Vec<Discovery>>().await {
			return bridges
				.into_iter()
				.filter_map(|bridge| bridge.internal_ip_address.parse().ok())
				.collect();
		}
	}

	HashSet::new()
}
