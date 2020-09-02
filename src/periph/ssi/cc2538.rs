//! Mappings for SSI.
//!
//! For CC2538 System-on-Chip Solution.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic SSI peripheral variant.
    pub trait SsiMap {}

    /// Generic SSI peripheral.
    pub struct SsiPeriph;

    SSI {
        CR0 {
            0x20 RwReg;
            DSS { RwRwRegFieldBits }
            FRF { RwRwRegFieldBits }
            SPO { RwRwRegFieldBit }
            SPH { RwRwRegFieldBit }
            SCR { RwRwRegFieldBits }
        }
        CR1 {
            0x20 RwReg;
            LBM { RwRwRegFieldBit }
            SSE { RwRwRegFieldBit }
            MS  { RwRwRegFieldBit }
            SOD { RwRwRegFieldBit }
        }
        DR {
            0x20 RwReg;
            DATA { RwRwRegFieldBits }
        }
        SR {
            0x20 RoReg;
            TFE { RoRoRegFieldBit }
            TNF { RoRoRegFieldBit }
            RNE { RoRoRegFieldBit }
            RFF { RoRoRegFieldBit }
            BSY { RoRoRegFieldBit }
        }
        CPSR {
            0x20 RwReg;
            CPSDVSR { RwRwRegFieldBits }
        }
        IM {
            0x20 RwReg;
            RORIM { RwRwRegFieldBit }
            RTIM  { RwRwRegFieldBit }
            RXIM  { RwRwRegFieldBit }
            TXIM  { RwRwRegFieldBit }
        }
        RIS {
            0x20 RoReg;
            RORRIS { RoRoRegFieldBit }
            RTRIS  { RoRoRegFieldBit }
            RXRIS  { RoRoRegFieldBit }
            TXRIS  { RoRoRegFieldBit }
        }
        MIS {
            0x20 RoReg;
            RORMIS { RoRoRegFieldBit }
            RTMIS  { RoRoRegFieldBit }
            RXMIS  { RoRoRegFieldBit }
            TXMIS  { RoRoRegFieldBit }
        }
        ICR {
            0x20 RwReg;
            RORIC { RwRwRegFieldBit }
            RTIC  { RwRwRegFieldBit }
        }
        DMACTL {
            0x20 RwReg;
            RXDMAE { RwRwRegFieldBit }
            TXDMAE { RwRwRegFieldBit }
        }
        CC {
            0x20 RwReg;
            CS { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_ssi {
    ($ssi_macro_doc:expr, $ssi_macro:ident, $ssi_ty_doc:expr, $ssi_ty:ident, $ssi_name:ident,) => {
        periph::map! {
            #[doc = $ssi_macro_doc]
            pub macro $ssi_macro;

            #[doc = $ssi_ty_doc]
            pub struct $ssi_ty;

            impl SsiMap for $ssi_ty {}

            drone_tisl_map_pieces::reg;
            crate;

            SSI {
                $ssi_name;
                CR0 {
                    CR0;
                    DSS { DSS }
                    FRF { FRF }
                    SPO { SPO }
                    SPH { SPH }
                    SCR { SCR }
                }
                CR1 {
                    CR1;
                    LBM { LBM }
                    SSE { SSE }
                    MS  { MS }
                    SOD { SOD }
                }
                DR {
                    DR;
                    DATA { DATA }
                }
                SR {
                    SR;
                    TFE { TFE }
                    TNF { TNF }
                    RNE { RNE }
                    RFF { RFF }
                    BSY { BSY }
                }
                CPSR {
                    CPSR;
                    CPSDVSR { CPSDVSR }
                }
                IM {
                    IM;
                    RORIM { RORIM }
                    RTIM  { RTIM }
                    RXIM  { RXIM }
                    TXIM  { TXIM }
                }
                RIS {
                    RIS;
                    RORRIS { RORRIS }
                    RTRIS  { RTRIS }
                    RXRIS  { RXRIS }
                    TXRIS  { TXRIS }
                }
                MIS {
                    MIS;
                    RORMIS { RORMIS }
                    RTMIS  { RTMIS }
                    RXMIS  { RXMIS }
                    TXMIS  { TXMIS }
                }
                ICR {
                    ICR;
                    RORIC { RORIC }
                    RTIC  { RTIC }
                }
                DMACTL {
                    DMACTL;
                    RXDMAE { RXDMAE }
                    TXDMAE { TXDMAE }
                }
                CC {
                    CC;
                    CS { CS }
                }
            }

        }
    };
}

#[cfg(tisl_mcu = "cc2538")]
map_ssi! {
    "Extracts SSI0 Registers tokens.",
    periph_ssi0,
    "SSI0 peripheral variant.",
    Ssi0,
    SSI0,
}

#[cfg(tisl_mcu = "cc2538")]
map_ssi! {
    "Extracts SSI1 Registers tokens.",
    periph_ssi1,
    "SSI1 peripheral variant.",
    Ssi1,
    SSI1,
}
