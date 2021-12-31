use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::net::Ipv4Addr;

use huehue::models::device_type::DeviceType;
use huehue::Hue;
use huehue::HueError;
use serde::Serialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Serialize)]
struct Arguments {
	#[structopt(long)]
	pub device: String,

	#[structopt(long)]
	pub address: Ipv4Addr,
}

#[tokio::main]
async fn main() {
	let arguments = Arguments::from_args();

	let device_type = DeviceType::new("huehue".to_owned(), arguments.device).expect("Invalid device name.");

	println!("Attempting to authorize with {}.", arguments.address);

	let mut hue = Hue::new(arguments.address, device_type.clone())
		.await
		.expect("Failed to read bridge information.");

	if let Err(e) = hue.authorize().await {
		match e {
			HueError::Unauthorized => {
				println!("Link button not pressed. Press the button and re-run this.")
			},
			e => println!("Unexpected error {:?}.", e),
		}
	} else {
		let vars = format!(
			"RUES_DEVICE_TYPE='{}'\nRUES_APPLICATION_KEY='{}'\nRUES_BRIDGE='{}'\n",
			device_type.to_string(),
			hue.application_key().expect("Application key expected."),
			arguments.address
		);

		match OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open("huehue.env")
		{
			Ok(file) => {
				let mut writer = BufWriter::new(file);
				writer.write_all(vars.as_bytes()).expect("Failed to write data.");
			},
			Err(_) => {
				println!(
					"Failed to create an environment variable, so printing them here. It is recommended to save these \
					 to a file and source it."
				);
				println!("{}", vars);
			},
		}
	}
}
