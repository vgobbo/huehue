use std::net::Ipv4Addr;
use std::str::FromStr;

use rues::models::device_type::DeviceType;
use rues::{Bridge, Client, Light};
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

fn print_lights(light: &Light) {
	println!("> Light {}:", light.name);
	println!("\tIdentifier: {}", light.id);
	println!("\tOn: {}", light.on);
	println!("\tBrightness: {}", light.brightness);

	if let Some(color) = &light.color {
		println!("\tColor: ({}, {})", color.xy.x, color.xy.y);
	}

	if let Some(value) = &light.temperature.mirek {
		let mirek_schema = &light.temperature.mirek_schema;
		println!(
			"\tTemperature: {} (min: {}, max {})",
			value, mirek_schema.mirek_minimum, mirek_schema.mirek_maximum
		);
	}
}

#[tokio::main]
async fn main() {
	let arguments = Arguments::from_args();

	let device_type = DeviceType::from_str(arguments.device_type.as_str()).expect("Invalid device name.");
	let bridge = Bridge::new(arguments.bridge)
		.await
		.expect("Failed to read bridge information.");

	let client = Client::new_with_key(bridge, device_type.clone(), arguments.application_key);
	match client.lights().await {
		Ok(lights) => lights.iter().for_each(|light| print_lights(light)),
		Err(e) => println!("Unexpected Hue error {:?}.", e),
	}
}
