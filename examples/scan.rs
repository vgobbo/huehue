use std::time::Duration;

use rues::Hue;

#[tokio::main]
async fn main() {
	println!("Scanning for bridges for 5 seconds.");
	let bridges = Hue::bridges(Duration::from_secs(5)).await;
	println!("{} bridges found.\n", bridges.len());

	let mut i = 1;
	for bridge in &bridges {
		println!("> Bridge #{}:", i);
		i += 1;

		println!("\tIdentifier: {}", bridge.id);
		println!("\tModel: {:?}", bridge.model);
		println!("\tVersion: {}", bridge.version);
		println!("\tAddress: {}", bridge.address);
		println!("\tSupported: {}", bridge.supported);
	}
}
