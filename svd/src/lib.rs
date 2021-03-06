//! Texas Instruments SimpleLink™ SVD to bindings for Drone, an Embedded
//! Operating System.

#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

pub use anyhow::{bail, Result};

use drone_svd::{Config, Device};
use std::{env, fs::File, path::Path};

/// Generates code for register mappings.
pub fn generate_regs(pool_number: usize, pool_size: usize) -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut output = File::create(out_dir.join("svd_regs.rs"))?;
    svd_config().generate_regs(&mut output, dev, pool_number, pool_size)
}

/// Generates code for register tokens struct.
pub fn generate_index() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut reg_output = File::create(out_dir.join("svd_reg_index.rs"))?;
    svd_config().generate_index(&mut reg_output, dev)
}

fn svd_config() -> Config<'static> {
    let mut options = Config::new("tisl_reg_tokens");
    options.bit_band(0x4000_0000..0x4010_0000);
    options.exclude_peripherals(&["FPU", "FPU_CPACR", "ITM", "MPU", "NVIC", "SCB", "STK", "TPIU"]);
    options
}

fn svd_deserialize() -> Result<Device> {
    drone_svd::rerun_if_env_changed();
    match env::var("CARGO_CFG_TISL_MCU")?.as_ref() {
        "cc2538" => parse_svd("CC2538.svd"),
        _ => bail!("invalid `tisl_mcu` cfg flag"),
    }
}

fn parse_svd(path: &str) -> Result<Device> {
    drone_svd::parse(format!("{}/files/{}", env!("CARGO_MANIFEST_DIR"), path))
}
