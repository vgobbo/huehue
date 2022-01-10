use std::net::Ipv4Addr;
use std::str::FromStr;

use huehue::models::device_type::DeviceType;
use huehue::{Hue, Light};
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

	if let Some(brightness) = &light.brightness {
		println!("\tBrightness: {}", brightness);
	}

	if let Some(color) = &light.color {
		println!("\tColor: ({}, {})", color.xy.x, color.xy.y);
		println!(
			"\tGamut: R={},{} G={},{} B={},{}",
			color.gamut.red.x,
			color.gamut.red.y,
			color.gamut.green.x,
			color.gamut.green.y,
			color.gamut.blue.x,
			color.gamut.blue.y
		);
	}

	if let Some(temperature) = &light.temperature {
		if let Some(value) = &temperature.mirek {
			let mirek_schema = &temperature.mirek_schema;
			println!(
				"\tTemperature: {} (min: {}, max {})",
				value, mirek_schema.mirek_minimum, mirek_schema.mirek_maximum
			);
		}
	}
}

#[tokio::main]
async fn main() {
	let arguments = Arguments::from_args();

	let device_type = DeviceType::from_str(arguments.device_type.as_str()).expect("Invalid device name.");
	let hue = Hue::new_with_key(arguments.bridge, device_type, arguments.application_key)
		.await
		.expect("Failed to read bridge information.");

	match hue.lights().await {
		Ok(lights) => lights.iter().for_each(|light| print_lights(light)),
		Err(e) => println!("Unexpected Hue error {:?}.", e),
	}
}
