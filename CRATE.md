# Huehue

A Rust wrapper for [Hue API v2](https://developers.meethue.com/develop/hue-api-v2/).

Note that the Hue API v2 is in early access at the time of writing, so an upgrade to it could break applications in
unpredictable way.

## Features
- Hue Bridge certificate validation.
- Discovery through mDNS.
- Discovery through [discovery.meethue.com](https://discovery.meethue.com).
- Light:
    - Switch on/off.
    - Color in the [CIE 1931 color space](https://en.wikipedia.org/wiki/CIE_1931_color_space).
- XY to RGB and RGB to XY conversion.

## Examples

The `examples` folder has fully functional sample applications to demonstrate some implemented features. The examples
aim to be trivial to understand by focusing on specific functionality, and should be easy to copy and build your own
application.

Simply use `cargo` to run the desired example:
```bash
$ cargo run --example scan

Scanning for bridges for 5 seconds.
1 bridges found.

> Bridge #1:
        Identifier: 1231231231231234
        Model: BSB002
        Version: 1948086000
        Address: 192.168.2.124
        Supported: true
```