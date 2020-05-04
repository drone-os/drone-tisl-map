[![crates.io](https://img.shields.io/crates/v/drone-tisl-map.svg)](https://crates.io/crates/drone-tisl-map)
![maintenance](https://img.shields.io/badge/maintenance-experimental-brightgreen.svg)

# drone-tisl-map

Texas Instruments SimpleLink™ peripheral mappings for Drone, an Embedded
Operating System.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-tisl-map = { version = "0.12.0", features = [...] }
```

Add or extend `std` feature as follows:

```toml
[features]
std = ["drone-tisl-map/std"]
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
