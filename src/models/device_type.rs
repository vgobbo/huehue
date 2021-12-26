use regex::Regex;
use serde::{Deserialize, Serialize};

const APPLICATION_NAME_REGEX: &str = r"^\w{1,20}$";
const DEVICE_NAME_REGEX: &str = r"^\w{1,19}$";

#[derive(Debug)]
pub enum Error {
	ApplicationName,
	DeviceName,
}

/// Represents the weirdly named `devicetype`, used as identification during authentication.
#[derive(Serialize, Deserialize)]
pub struct DeviceType {
	pub application_name: String,
	pub device_name: String,
}

impl DeviceType {
	pub fn new(application_name: String, device_name: String) -> Result<DeviceType, Error> {
		let app_name_re = Regex::new(APPLICATION_NAME_REGEX).unwrap();
		if !app_name_re.is_match(application_name.as_str()) {
			return Err(Error::ApplicationName);
		}

		let dev_name_re = Regex::new(DEVICE_NAME_REGEX).unwrap();
		if !dev_name_re.is_match(device_name.as_str()) {
			return Err(Error::DeviceName);
		}

		Ok(DeviceType {
			application_name,
			device_name,
		})
	}
}

impl ToString for DeviceType {
	fn to_string(&self) -> String {
		format!("{}#{}", self.application_name, self.device_name)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		assert!(DeviceType::new("some".to_owned(), "thing".to_owned()).is_ok());
		assert!(DeviceType::new("123_some".to_owned(), "thing_345".to_owned()).is_ok());

		assert!(DeviceType::new("some".to_owned(), "".to_owned()).is_err());
		assert!(DeviceType::new("".to_owned(), "thing".to_owned()).is_err());
	}

	#[test]
	fn to_string() {
		let dt = DeviceType::new("some".to_owned(), "thing".to_owned());
		assert!(dt.is_ok());
		assert_eq!(dt.unwrap().to_string().as_str(), "some#thing");
	}
}
