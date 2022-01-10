use std::net::Ipv4Addr;
use std::str::FromStr;

use huehue::color::{Component, RGB8};
use huehue::models::device_type::DeviceType;
use huehue::{Hue, Light};
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Serialize)]
struct XyArguments {
	#[structopt(long)]
	pub x: f32,

	#[structopt(long)]
	pub y: f32,
}

#[derive(Debug, StructOpt, Serialize)]
struct RgbArguments {
	#[structopt(long)]
	pub r: u8,

	#[structopt(long)]
	pub g: u8,

	#[structopt(long)]
	pub b: u8,
}

#[derive(Debug, StructOpt, Serialize)]
struct DimmArguments {
	#[structopt(long)]
	pub value: f32,
}

#[derive(Debug, StructOpt, Serialize)]
enum ActionArguments {
	Switch,
	Dimm(DimmArguments),
	Xy(XyArguments),
	Rgb(RgbArguments),
}

#[derive(Debug, StructOpt, Serialize)]
struct Arguments {
	#[structopt(subcommand)]
	pub action: ActionArguments,

	#[structopt(long, env = "RUES_BRIDGE")]
	pub bridge: Ipv4Addr,

	#[structopt(long, env = "RUES_DEVICE_TYPE")]
	pub device_type: String,

	#[structopt(long, env = "RUES_APPLICATION_KEY")]
	pub application_key: String,

	#[structopt(long)]
	pub id: uuid::Uuid,
}

fn print_light(light: &Light) {
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

	let light_opt = match hue.lights().await {
		Ok(lights) => lights.into_iter().filter(|light| light.id == arguments.id).next(),
		Err(e) => {
			println!("Unexpected Hue error {:?}.", e);
			return;
		},
	};

	let mut light = match light_opt {
		Some(light) => light,
		None => {
			println!("Light {} not found.", arguments.id);
			return;
		},
	};

	match arguments.action {
		ActionArguments::Switch => {
			match light.switch(!light.on).await {
				Ok(_) => (),
				Err(e) => {
					println!("Unexpected Hue error {:?}.", e);
					return;
				},
			}
		},
		ActionArguments::Xy(color) => {
			match light
				.set_color(Component::new(color.x, color.y).expect("Invalid color."))
				.await
			{
				Ok(_) => (),
				Err(e) => {
					println!("Unexpected Hue error {:?}.", e);
					return;
				},
			}
		},
		ActionArguments::Rgb(color) => {
			match light.set_color_rgb(RGB8::new(color.r, color.g, color.b)).await {
				Ok(_) => (),
				Err(e) => {
					println!("Unexpected Hue error {:?}.", e);
					return;
				},
			}
		},
		ActionArguments::Dimm(dimm) => {
			match light.dimm(dimm.value).await {
				Ok(_) => (),
				Err(e) => {
					println!("Unexpected Hue error {:?}.", e);
					return;
				},
			}
		},
	}

	print_light(&light);
}
