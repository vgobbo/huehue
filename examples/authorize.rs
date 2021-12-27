use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::net::Ipv4Addr;

use rues::client::AuthorizationError;
use rues::models::device_type::DeviceType;
use rues::models::error::ErrorCode;
use rues::{Bridge, Client};
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

	let device_type = DeviceType::new("rues".to_owned(), arguments.device).expect("Invalid device name.");

	println!("Attempting to authorize with {}.", arguments.address);

	let bridge = Bridge::new(arguments.address)
		.await
		.expect("Failed to read bridge information.");

	let mut client = Client::new(bridge, device_type.clone());

	if let Err(e) = client.authorize().await {
		match e {
			AuthorizationError::Hue(ErrorCode::LinkButtonNotPressed) => {
				println!("Link button not pressed. Press the button and re-run this.")
			},
			AuthorizationError::Hue(e) => println!("Unexpected Hue error {:?}.", e),
			e => println!("Unexpected error {:?}.", e),
		}
	} else {
		let vars = format!(
			"RUES_DEVICE_TYPE='{}'\nRUES_CLIENT_KEY='{}'\n",
			device_type.to_string(),
			client.client_key().expect("Client key expected.")
		);

		match OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open("rues.env")
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
