//! A Rust wrapper for [Hue API v2](https://developers.meethue.com/develop/hue-api-v2/).
//!
//! Note that the Hue API v2 is in early access at the time of writing, so an upgrade to it could break applications in
//! unpredictable way.
//!
//! ## Features
//! - Hue Bridge certificate validation.
//! - Bridge discovery:
//!   - through mDNS.
//!   - through [discovery.meethue.com](https://discovery.meethue.com).
//!   - user specified IPv4 address.
//! - Devices:
//!   - list devices.
//! - Light:
//!   - switch on/off.
//!   - color in the [CIE 1931 color space](https://en.wikipedia.org/wiki/CIE_1931_color_space).
//!   - color in the sRGB color space.
//!   - dimming.
//! - XY to RGB and RGB to XY conversion.
//!
//! ## Discovery
//! Bridges can be discovered by simply running:
//! ```no_run
//! # use huehue::Hue;
//! # use std::time::Duration;
//! #
//! # async fn discover() {
//! let bridges = Hue::bridges(Duration::from_secs(5)).await;
//! # }
//! ```
//!
//! Both MDNS and [discovery.meehue.com](discovery.meehue.com) will be used during the discovery.
//!
//! ## Authorization
//! Once a bridge is chosen, you made need to generate an application key with it, so you can interact with it.
//!
//! Simply create a [`Hue`] struct specifying the bridge address and a device, and call [`Hue::authorize`]:
//!
//! Before the application is run, you must press the round button in the bridge. Else, you will get an unauthorized
//! error.
//! ```no_run
//! # use std::net::Ipv4Addr;
//! # use std::time::Duration;
//! # use huehue::{Hue, HueError};
//! # use huehue::models::device_type::DeviceType;
//! #
//! # async fn authorize(ip: Ipv4Addr) -> String {
//! let device_type = DeviceType::new("my_app".to_owned(), "my_device".to_owned()).unwrap();
//! let mut hue = Hue::new(ip, device_type)
//! 	.await
//! 	.expect("Failed to run bridge information.");
//! hue.authorize().await.expect("Failed to link with the bridge.");
//! hue.application_key()
//! 	.expect("When successfully authorized, this must always be valid.")
//! # }
//! ```
//!
//! The [`Hue::application_key()`] field should be saved so authorization is no longer required.
//! ```no_run
//! # use std::net::Ipv4Addr;
//! # use std::time::Duration;
//! # use huehue::{Hue, HueError};
//! # use huehue::models::device_type::DeviceType;
//! #
//! # async fn connect(ip: Ipv4Addr, device_type: DeviceType, application_key: String) -> Hue {
//! Hue::new_with_key(ip, device_type, application_key)
//! 	.await
//! 	.expect("Failed to run bridge information.")
//! # }
//! ```
//!
//! With the resulting [`Hue`] instance you can interact with the bridge.

pub mod bridge;
mod certificate;
pub mod color;
pub mod device;
mod discover;
mod http;
pub mod hue;
pub mod light;
pub mod models;

pub use bridge::Bridge;
pub use http::HueError;
pub use hue::Hue;
pub use light::Light;
