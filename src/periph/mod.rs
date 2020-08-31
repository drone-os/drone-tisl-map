//! Texas Instruments SimpleLink™ peripheral mappings.

#[doc(no_inline)]
pub use drone_cortexm::map::periph::*;

#[cfg(feature = "ioc")]
pub extern crate drone_tisl_map_periph_ioc as ioc;
#[cfg(feature = "radio")]
pub extern crate drone_tisl_map_periph_radio as radio;
#[cfg(feature = "gpio")]
pub extern crate drone_tisl_map_periph_gpio as gpio;
#[cfg(feature = "sysctrl")]
pub extern crate drone_tisl_map_periph_sysctrl as sysctrl;
#[cfg(feature = "tim")]
pub extern crate drone_tisl_map_periph_tim as tim;
#[cfg(feature = "uart")]
pub extern crate drone_tisl_map_periph_uart as uart;
