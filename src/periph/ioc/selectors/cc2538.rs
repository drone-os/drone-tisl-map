//! I/O port selectors.
//!
//! For CC2538 System-on-Chip Solutio

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic IOC port selector variant.
    pub trait IocSelectorMap {}

    /// Generic port selector peripheral.
    pub struct IocSelectorPeriph;

    IOC  {
        UARTRXD_UART0 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        UARTCTS_UART1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        UARTRXD_UART1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        CLK_SSI_SSI0 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        SSIRXD_SSI0 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        SSIFSSIN_SSI0 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        CLK_SSIIN_SSI0 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        CLK_SSI_SSI1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        SSIRXD_SSI1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        SSIFSSIN_SSI1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        CLK_SSIIN_SSI1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        I2CMSSDA {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        I2CMSSCL {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT0OCP1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT0OCP2 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT1OCP1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT1OCP2 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT2OCP1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT2OCP2 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT3OCP1 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
        GPT3OCP2 {
            0x20 RwReg;
            INPUT_SEL { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_ioc_selectors {
    ($sel_macro_doc:expr, $sel_macro:ident, $sel_ty_doc:expr, $sel_ty:ident,) => {
        periph::map! {
            #[doc = $sel_macro_doc]
            pub macro $sel_macro;

            #[doc = $sel_ty_doc]
            pub struct $sel_ty;

            impl IocSelectorMap for $sel_ty {}

            drone_tisl_map_pieces::reg;
            crate::selectors;

            IOC {
                IOC;
                UARTRXD_UART0 {
                    UARTRXD_UART0;
                    INPUT_SEL { INPUT_SEL }
                }
                UARTCTS_UART1 {
                    UARTCTS_UART1;
                    INPUT_SEL { INPUT_SEL }
                }
                UARTRXD_UART1 {
                    UARTRXD_UART1;
                    INPUT_SEL { INPUT_SEL }
                }
                CLK_SSI_SSI0 {
                    CLK_SSI_SSI0;
                    INPUT_SEL { INPUT_SEL }
                }
                SSIRXD_SSI0 {
                    SSIRXD_SSI0;
                    INPUT_SEL { INPUT_SEL }
                }
                SSIFSSIN_SSI0 {
                    SSIFSSIN_SSI0;
                    INPUT_SEL { INPUT_SEL }
                }
                CLK_SSIIN_SSI0 {
                    CLK_SSIIN_SSI0;
                    INPUT_SEL { INPUT_SEL }
                }
                CLK_SSI_SSI1 {
                    CLK_SSI_SSI1;
                    INPUT_SEL { INPUT_SEL }
                }
                SSIRXD_SSI1 {
                    SSIRXD_SSI1;
                    INPUT_SEL { INPUT_SEL }
                }
                SSIFSSIN_SSI1 {
                    SSIFSSIN_SSI1;
                    INPUT_SEL { INPUT_SEL }
                }
                CLK_SSIIN_SSI1 {
                    CLK_SSIIN_SSI1;
                    INPUT_SEL { INPUT_SEL }
                }
                I2CMSSDA {
                    I2CMSSDA;
                    INPUT_SEL { INPUT_SEL }
                }
                I2CMSSCL {
                    I2CMSSCL;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT0OCP1 {
                    GPT0OCP1;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT0OCP2 {
                    GPT0OCP2;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT1OCP1 {
                    GPT1OCP1;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT1OCP2 {
                    GPT1OCP2;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT2OCP1 {
                    GPT2OCP1;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT2OCP2 {
                    GPT2OCP2;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT3OCP1 {
                    GPT3OCP1;
                    INPUT_SEL { INPUT_SEL }
                }
                GPT3OCP2 {
                    GPT3OCP2;
                    INPUT_SEL { INPUT_SEL }
                }
            }
        }
    };
}

map_ioc_selectors! {
    "Extract IOC selector tokens",
    periph_ioc_selectors,
    "IOC selectors",
    IocSel,
}
