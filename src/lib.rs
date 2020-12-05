//! Texas Instruments SimpleLink™ peripheral mappings for Drone, an Embedded
//! Operating System.
//!
//! This crate uses CMSIS-SVD files based on TI CCS target db to automatically
//! generate Drone register and interrupt bindings. However only the
//! corresponding Product Specification is the single source of truth. A
//! difference between this crate bindings and the Product Specification is
//! considered a bug. Fixing such a bug is not a breaking change.
//!
//! This crate re-exports the contents of [`drone_cortexm::map`] module and is a
//! drop-in replacement for it.
//!
//! # Supported Devices
//!
//! | `tisl_mcu`  | Core name             | Product specification                                       | Available features                  |
//! |-------------|-----------------------|-------------------------------------------------------------|-------------------------------------|
//! | `cc2538`    | ARM® Cortex®-M3 r2p0  | [SWRU319C](https://www.ti.com/lit/ug/swru319c/swru319c.pdf) | `gpio` `ioc` `sysctrl` `tim` `uart` |
//!
//! `tisl_mcu` config flag should be set at the application level according to
//! this table.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-tisl-map/0.13/)
//!
//! The API documentation intentionally skips auto-generated [`reg`] and [`thr`]
//! bindings. Otherwise it would use several gigabytes of space and would be
//! very slow to render in a browser. One should refer to the Product
//! Specification instead. And to get an idea of what the API looks like on the
//! Drone side, look at the [`drone_cortexm::map`] module documentation.
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! drone-tisl-map = { version = "0.13.0", features = [...] }
//! ```
//!
//! Add or extend `std` feature as follows:
//!
//! ```toml
//! [features]
//! std = ["drone-tisl-map/std"]
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![no_std]

pub mod periph;
pub mod reg;
pub mod thr;

pub use drone_tisl_map_pieces::tisl_reg_tokens;
