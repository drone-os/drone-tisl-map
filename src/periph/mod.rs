//! Texas Instruments SimpleLink™ peripheral mappings.

#[doc(no_inline)]
pub use drone_cortexm::map::periph::*;

#[cfg(feature = "tim")]
pub extern crate drone_tisl_map_periph_tim as tim;
