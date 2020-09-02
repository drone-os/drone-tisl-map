//! General-purpose I/O port pads.
//!
//! For CC2538 System-on-Chip Solutio

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic GPIO pad peripheral variant.
    pub trait IocPadMap { }

    /// Generic GPIO pad peripheral.
    pub struct IocPadPeriph;

    IOC  {
        PAD_SEL {
            0x20 RwReg;
            PAD_SEL { RwRwRegFieldBits }
        }
        PAD_OVER_BITS {
            0x20 RwReg Option;
            PAD_OVER_BITS { RwRwRegFieldBits }
        }
        PAD_OVER_BIT {
            0x20 RwReg Option;
            PAD_OVER_BIT { RwRwRegFieldBit }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_ioc_pad {
    (
        $port_ty:ident,
        $pad_macro_doc:expr,
        $pad_macro:ident,
        $pad_ty_doc:expr,
        $pad_ty:ident,
        $pad_sel_reg:ident,
        $pad_sel:ident,
        ($($pad_over_bits_reg:ident)?, $pad_over_bits:ident),
        ($($pad_over_bit_reg:ident)?, $pad_over_bit:ident),
    ) => {
        periph::map! {
            #[doc = $pad_macro_doc]
            pub macro $pad_macro;

            #[doc = $pad_ty_doc]
            pub struct $pad_ty;

            impl IocPadMap for $pad_ty {
                //type IocPortMap = super::super::ports::$port_ty;
            }

            drone_tisl_map_pieces::reg;
            crate::pads;
            IOC {
                IOC;
                PAD_SEL {
                    $pad_sel_reg;
                    PAD_SEL { $pad_sel }
                }
                PAD_OVER_BITS {
                    $(
                        $pad_over_bits_reg Option;
                        PAD_OVER_BITS { $pad_over_bits }
                    )*
                }
                PAD_OVER_BIT {
                    $(
                        $pad_over_bit_reg Option;
                        PAD_OVER_BIT { $pad_over_bit }
                    )*
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! map_ioc_pads_a {
    () => {
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA0 register tokens.",
            periph_ioc_a0,
            "IOC pad PA0 peripheral variant.",
            IocA0,
            PA0_SEL,
            PA0_sel,
            (PA0_OVER, PA0_over),
            (, PA0_over),
        }
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA1 register tokens.",
            periph_ioc_a1,
            "IOC pad PA1 peripheral variant.",
            IocA1,
            PA1_sel,
            PA1_SEL,
            (PA1_OVER, PA1_over),
            (, PA1_over),
        }
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA2 register tokens.",
            periph_ioc_a2,
            "IOC pad PA2 peripheral variant.",
            IocA2,
            PA2_sel,
            PA2_SEL,
            (PA2_OVER, PA2_over),
            (, PA2_over),
        }
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA3 register tokens.",
            periph_ioc_a3,
            "IOC pad PA3 peripheral variant.",
            IocA3,
            PA3_sel,
            PA3_SEL,
            (PA3_OVER, PA3_over),
            (, PA3_over),
        }
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA4 register tokens.",
            periph_ioc_a4,
            "IOC pad PA4 peripheral variant.",
            IocA4,
            PA4_sel,
            PA4_SEL,
            (PA4_OVER, PA4_over),
            (, PA4_over),
        }
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA5 register tokens.",
            periph_ioc_a5,
            "IOC pad PA5 peripheral variant.",
            IocA5,
            PA5_sel,
            PA5_SEL,
            (PA5_OVER, PA5_over),
            (, PA5_over),
        }
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA6 register tokens.",
            periph_ioc_a6,
            "IOC pad PA6 peripheral variant.",
            IocA6,
            PA6_sel,
            PA6_SEL,
            (PA6_OVER, PA6_over),
            (, PA6_over),
        }
        map_ioc_pad! {
            IocAPort,
            "Extracts IOC pad PA7 register tokens.",
            periph_ioc_a7,
            "IOC pad PA7 peripheral variant.",
            IocA7,
            PA7_sel,
            PA7_SEL,
            (PA7_OVER, PA7_over),
            (, PA7_over),
        }
    };
}

#[allow(unused_macros)]
macro_rules! map_ioc_pads_b {
    () => {
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PB0 register tokens.",
            periph_ioc_b0,
            "IOC pad PB0 peripheral variant.",
            IocB0,
            PB0_SEL,
            PB0_sel,
            (PB0_OVER, PB0_over),
            (, PB0_over),
        }
        map_ioc_pad! {
            IocBPort,
            "Extracts IOC pad PB1 register tokens.",
            periph_ioc_b1,
            "IOC pad PB1 peripheral variant.",
            IocB1,
            PB1_sel,
            PB1_SEL,
            (PB1_OVER, PB1_over),
            (, PB1_over),
        }
        map_ioc_pad! {
            IocBPort,
            "Extracts IOC pad PB2 register tokens.",
            periph_ioc_b2,
            "IOC pad PB2 peripheral variant.",
            IocB2,
            PB2_sel,
            PB2_SEL,
            (PB2_OVER, PB2_over),
            (, PB2_over),
        }
        map_ioc_pad! {
            IocBPort,
            "Extracts IOC pad PB3 register tokens.",
            periph_ioc_b3,
            "IOC pad PB3 peripheral variant.",
            IocB3,
            PB3_sel,
            PB3_SEL,
            (PB3_OVER, PB3_over),
            (, PB3_over),
        }
        map_ioc_pad! {
            IocBPort,
            "Extracts IOC pad PB4 register tokens.",
            periph_ioc_b4,
            "IOC pad PB4 peripheral variant.",
            IocB4,
            PB4_sel,
            PB4_SEL,
            (PB4_OVER, PB4_over),
            (, PB4_over),
        }
        map_ioc_pad! {
            IocBPort,
            "Extracts IOC pad PB5 register tokens.",
            periph_ioc_b5,
            "IOC pad PB5 peripheral variant.",
            IocB5,
            PB5_sel,
            PB5_SEL,
            (PB5_OVER, PB5_over),
            (, PB5_over),
        }
        map_ioc_pad! {
            IocBPort,
            "Extracts IOC pad PB6 register tokens.",
            periph_ioc_b6,
            "IOC pad PB6 peripheral variant.",
            IocB6,
            PB6_sel,
            PB6_SEL,
            (PB6_OVER, PB6_over),
            (, PB6_over),
        }
        map_ioc_pad! {
            IocBPort,
            "Extracts IOC pad PB7 register tokens.",
            periph_ioc_b7,
            "IOC pad PB7 peripheral variant.",
            IocB7,
            PB7_sel,
            PB7_SEL,
            (PB7_OVER, PB7_over),
            (, PB7_over),
        }
    };
}

#[allow(unused_macros)]
macro_rules! map_ioc_pads_c {
    () => {
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC0 register tokens.",
            periph_ioc_c0,
            "IOC pad PC0 peripheral variant.",
            IocC0,
            PC0_SEL,
            PC0_sel,
            (, PC0_over),
            (PC0_OVER, PC0_over),
        }
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC1 register tokens.",
            periph_ioc_c1,
            "IOC pad PC1 peripheral variant.",
            IocC1,
            PC1_sel,
            PC1_SEL,
            (, PC1_over),
            (PC1_OVER, PC1_over),
        }
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC2 register tokens.",
            periph_ioc_c2,
            "IOC pad PC2 peripheral variant.",
            IocC2,
            PC2_sel,
            PC2_SEL,
            (, PC2_over),
            (PC2_OVER, PC2_over),
        }
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC3 register tokens.",
            periph_ioc_c3,
            "IOC pad PC3 peripheral variant.",
            IocC3,
            PC3_sel,
            PC3_SEL,
            (, PC3_over),
            (PC3_OVER, PC3_over),
        }
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC4 register tokens.",
            periph_ioc_c4,
            "IOC pad PC4 peripheral variant.",
            IocC4,
            PC4_sel,
            PC4_SEL,
            (PC4_OVER, PC4_over),
            (, PC4_over),
        }
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC5 register tokens.",
            periph_ioc_c5,
            "IOC pad PC5 peripheral variant.",
            IocC5,
            PC5_sel,
            PC5_SEL,
            (PC5_OVER, PC5_over),
            (, PC5_over),
        }
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC6 register tokens.",
            periph_ioc_c6,
            "IOC pad PC6 peripheral variant.",
            IocC6,
            PC6_sel,
            PC6_SEL,
            (PC6_OVER, PC6_over),
            (, PC6_over),
        }
        map_ioc_pad! {
            IocCPort,
            "Extracts IOC pad PC7 register tokens.",
            periph_ioc_c7,
            "IOC pad PC7 peripheral variant.",
            IocC7,
            PC7_sel,
            PC7_SEL,
            (PC7_OVER, PC7_over),
            (, PC7_over),
        }
    };
}

#[allow(unused_macros)]
macro_rules! map_ioc_pads_d {
    () => {
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD0 register tokens.",
            periph_ioc_d0,
            "IOC pad PD0 peripheral variant.",
            IocD0,
            PD0_SEL,
            PD0_sel,
            (PD0_OVER, PD0_over),
            (, PD0_over),
        }
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD1 register tokens.",
            periph_ioc_d1,
            "IOC pad PD1 peripheral variant.",
            IocD1,
            PD1_sel,
            PD1_SEL,
            (PD1_OVER, PD1_over),
            (, PD1_over),
        }
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD2 register tokens.",
            periph_ioc_d2,
            "IOC pad PD2 peripheral variant.",
            IocD2,
            PD2_sel,
            PD2_SEL,
            (PD2_OVER, PD2_over),
            (, PD2_over),
        }
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD3 register tokens.",
            periph_ioc_d3,
            "IOC pad PD3 peripheral variant.",
            IocD3,
            PD3_sel,
            PD3_SEL,
            (PD3_OVER, PD3_over),
            (, PD3_over),
        }
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD4 register tokens.",
            periph_ioc_d4,
            "IOC pad PD4 peripheral variant.",
            IocD4,
            PD4_sel,
            PD4_SEL,
            (PD4_OVER, PD4_over),
            (, PD4_over),
        }
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD5 register tokens.",
            periph_ioc_d5,
            "IOC pad PD5 peripheral variant.",
            IocD5,
            PD5_sel,
            PD5_SEL,
            (PD5_OVER, PD5_over),
            (, PD5_over),
        }
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD6 register tokens.",
            periph_ioc_d6,
            "IOC pad PD6 peripheral variant.",
            IocD6,
            PD6_sel,
            PD6_SEL,
            (PD6_OVER, PD6_over),
            (, PD6_over),
        }
        map_ioc_pad! {
            IocDPort,
            "Extracts IOC pad PD7 register tokens.",
            periph_ioc_d7,
            "IOC pad PD7 peripheral variant.",
            IocD7,
            PD7_sel,
            PD7_SEL,
            (PD7_OVER, PD7_over),
            (, PD7_over),
        }
    };
}

map_ioc_pads_a! {}

map_ioc_pads_b! {}

map_ioc_pads_c! {}

map_ioc_pads_d! {}
