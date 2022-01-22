use std::net::Ipv4Addr;
use std::str::FromStr;

use huehue::device::Device;
use huehue::models::device_type::DeviceType;
use huehue::Hue;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Serialize)]
struct Arguments {
	#[structopt(long, env = "RUES_BRIDGE")]
	pub bridge: Ipv4Addr,

	#[structopt(long, env = "RUES_DEVICE_TYPE")]
	pub device_type: String,

	#[structopt(long, env = "RUES_APPLICATION_KEY")]
	pub application_key: String,
}

fn print_device(device: &Device) {
	println!("> Device {}:", device.name);
	println!("\tIdentifier: {}", device.id);

	println!("\tModel: {}", device.product.model_id);
	println!("\tManufacturer: {}", device.product.manufacturer_name);
	println!("\tArchetype: {}", device.product.product_archetype);
	println!("\tProduct: {}", device.product.product_name);
	println!("\tSoftware version: {}", device.product.software_version);
	println!("\tCertified: {}", device.product.certified);

	device.services.iter().for_each(|service| {
		println!("\tService: rid={}, rtype={}", service.rid, service.rtype);
	});
}

#[tokio::main]
async fn main() {
	let arguments = Arguments::from_args();

	let device_type = DeviceType::from_str(arguments.device_type.as_str()).expect("Invalid device name.");
	let hue = Hue::new_with_key(arguments.bridge, device_type, arguments.application_key)
		.await
		.expect("Failed to read bridge information.");

	match hue.devices().await {
		Ok(devices) => devices.iter().for_each(|device| print_device(device)),
		Err(e) => println!("Unexpected Hue error {:?}.", e),
	}
}
