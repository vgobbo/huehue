use std::time::Duration;

use rues::client::AuthorizationError;
use rues::models::device_type::DeviceType;
use rues::models::error::ErrorCode;
use rues::{Bridge, Client};

#[tokio::test]
pub async fn authorize_link_button_not_pressed() {
	let bridges = Bridge::discover(Duration::from_secs(3)).await;
	if bridges.is_empty() {
		// no bridge to try to connect.
		return;
	}

	let bridge = bridges.get(0).unwrap().clone();
	let mut client = Client::new(bridge);
	let result = client
		.authorize(DeviceType::new("rues".to_owned(), "authorize_test".to_owned()).unwrap())
		.await;
	assert!(result.is_err());
	if let AuthorizationError::Hue(code) = result.unwrap_err() {
		assert_eq!(code, ErrorCode::LinkButtonNotPressed);
	} else {
		assert!(false, "Expected unauthorized error.");
	}
}
