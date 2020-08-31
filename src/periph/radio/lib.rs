//! Radio Core

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

pub mod cc2538;

pub use self::cc2538::*;
