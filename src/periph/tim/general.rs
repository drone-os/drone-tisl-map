//! General-purpose timers.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic general-purpose timer peripheral variant.
    pub trait GeneralTimMap {}

    /// Generic general-purpose timer peripheral.
    pub struct GeneralTimPeriph;

    GPTIMER {
        CFG {
            0x20 RwReg;
            GPTMCFG { RwRwRegFieldBits }
        }
        TAMR {
            0x20 RwReg;
            TAMR    { RwRwRegFieldBits }
            TACMR   { RwRwRegFieldBit }
            TAAMS   { RwRwRegFieldBit }
            TACDIR  { RwRwRegFieldBit }
            TAMIE   { RwRwRegFieldBit }
            TAWOT   { RwRwRegFieldBit }
            TASNAPS { RwRwRegFieldBit }
            TAILD   { RwRwRegFieldBit }
            TAPWMIE { RwRwRegFieldBit }
            TAMRSU  { RwRwRegFieldBit }
            TAPLO   { RwRwRegFieldBit }
        }
        TBMR {
            0x20 RwReg;
            TBMR    { RwRwRegFieldBits }
            TBCMR   { RwRwRegFieldBit }
            TBAMS   { RwRwRegFieldBit }
            TBCDIR  { RwRwRegFieldBit }
            TBMIE   { RwRwRegFieldBit }
            TBWOT   { RwRwRegFieldBit }
            TBSNAPS { RwRwRegFieldBit }
            TBILD   { RwRwRegFieldBit }
            TBPWMIE { RwRwRegFieldBit }
            TBMRSU  { RwRwRegFieldBit }
            TBPLO   { RwRwRegFieldBit }
        }
        CTL {
            0x20 RwReg;
            TAEN    { RwRwRegFieldBit }
            TASTALL { RwRwRegFieldBit }
            TAEVENT { RwRwRegFieldBits }
            TAOTE   { RwRwRegFieldBit }
            TAPWML  { RwRwRegFieldBit }
            TBEN    { RwRwRegFieldBit }
            TBSTALL { RwRwRegFieldBit }
            TBEVENT { RwRwRegFieldBits }
            TBOTE   { RwRwRegFieldBit }
            TBPWML  { RwRwRegFieldBit }
        }
        SYNC {
            0x20 RwReg;
            SYNC0 { RwRwRegFieldBits }
            SYNC1 { RwRwRegFieldBits }
            SYNC2 { RwRwRegFieldBits }
            SYNC3 { RwRwRegFieldBits }
        }
        IMR {
            0x20 RwReg;
            TATOIM { RwRwRegFieldBit }
            CAMIM { RwRwRegFieldBit }
            CAEIM { RwRwRegFieldBit }
            TAMIM { RwRwRegFieldBit }
            TBTOIM { RwRwRegFieldBit }
            CBMIM { RwRwRegFieldBit }
            CBEIM { RwRwRegFieldBit }
            TBMIM { RwRwRegFieldBit }
        }
        RIS {
            0x20 RwReg;
            TATORIS { RoRwRegFieldBit }
            CAMRIS { RoRwRegFieldBit }
            CAERIS { RoRwRegFieldBit }
            TAMRIS { RoRwRegFieldBit }
            TBTORIS { RoRwRegFieldBit }
            CBMRIS { RoRwRegFieldBit }
            CBERIS { RoRwRegFieldBit }
            TBMRIS { RoRwRegFieldBit }
        }
        MIS {
            0x20 RwReg;
            TATOMIS { RoRwRegFieldBit }
            CAMMIS { RoRwRegFieldBit }
            CAEMIS { RoRwRegFieldBit }
            TAMRIS { RoRwRegFieldBit }
            TBTOMIS { RoRwRegFieldBit }
            CBMMIS { RoRwRegFieldBit }
            CBEMIS { RoRwRegFieldBit }
            TBMMIS { RoRwRegFieldBit }
        }
        ICR {
            0x20 RwReg;
            TATOCINT { RwRwRegFieldBit }
            CAMCINT { RwRwRegFieldBit }
            CAECINT { RwRwRegFieldBit }
            TAMCINT { RwRwRegFieldBit }
            TBTOCINT { RwRwRegFieldBit }
            CBMCINT { RwRwRegFieldBit }
            CBECINT { RwRwRegFieldBit }
            TBMCINT { RwRwRegFieldBit }
            WUECINT { RwRwRegFieldBit }
        }
        TAILR {
            0x20 RwReg;
            TAILR { RwRwRegFieldBits }
        }
        TBILR {
            0x20 RwReg;
            TBILR { RwRwRegFieldBits }
        }
        TAMATCHR {
            0x20 RwReg;
            TAMR { RwRwRegFieldBits }
        }
        TBMATCHR {
            0x20 RwReg;
            TBMR { RwRwRegFieldBits}
        }
        TAPR {
            0x20 RwReg;
            TAPSR { RwRwRegFieldBits }
        }
        TBPR {
            0x20 RwReg;
            TBPSR { RwRwRegFieldBits }
        }
        TAPMR {
            0x20 RwReg;
            TAPSR { RwRwRegFieldBits }
        }
        TBPMR {
            0x20 RwReg;
            TBPSR { RwRwRegFieldBits }
        }
        TAR {
            0x20 RwReg;
            TAR { RoRwRegFieldBits }
        }
        TBR {
            0x20 RwReg;
            TBR { RoRwRegFieldBits }
        }
        TAV {
            0x20 RwReg;
            TAV { RwRwRegFieldBits }
        }
        TBV {
            0x20 RwReg;
            TBV { RwRwRegFieldBits }
            PRE { RoRwRegFieldBits }
        }
        TAPS {
            0x20 RwReg;
            PSS { RoRwRegFieldBits }
        }
        TBPS {
            0x20 RwReg;
            PSS { RoRwRegFieldBits }
        }
        TAPV {
            0x20 RwReg;
            PSV { RoRwRegFieldBits }
        }
        TBPV {
            0x20 RwReg;
            PSV { RoRwRegFieldBits }
        }
        PP {
            0x20 RwReg;
            SIZE { RoRwRegFieldBits }
            CHAIN { RoRwRegFieldBit }
            SYNCNT { RoRwRegFieldBit }
            ALTCLK { RoRwRegFieldBit }
        }
    }
}

macro_rules! map_gp_tim {
    ($tim_macro_doc:expr, $tim_macro:ident, $tim_ty_doc:expr, $tim_ty:ident, $tim:ident,) => {
        periph::map! {
            #[doc = $tim_macro_doc]
            pub macro $tim_macro;

            #[doc = $tim_ty_doc]
            pub struct $tim_ty;

            impl GeneralTimMap for $tim_ty {}

            drone_tisl_map_pieces::reg;
            crate::general;

            GPTIMER {
                $tim;
                CFG {
                    CFG;
                    GPTMCFG { GPTMCFG }
                }
                TAMR {
                    TAMR;
                    TAMR    { TAMR }
                    TACMR   { TACMR }
                    TAAMS   { TAAMS }
                    TACDIR  { TACDIR }
                    TAMIE   { TAMIE }
                    TAWOT   { TAWOT }
                    TASNAPS { TASNAPS }
                    TAILD   { TAILD }
                    TAPWMIE { TAPWMIE }
                    TAMRSU  { TAMRSU }
                    TAPLO   { TAPLO }
                }
                TBMR {
                    TBMR;
                    TBMR    { TBMR }
                    TBCMR   { TBCMR }
                    TBAMS   { TBAMS }
                    TBCDIR  { TBCDIR }
                    TBMIE   { TBMIE }
                    TBWOT   { TBWOT }
                    TBSNAPS { TBSNAPS }
                    TBILD   { TBILD }
                    TBPWMIE { TBPWMIE }
                    TBMRSU  { TBMRSU }
                    TBPLO   { TBPLO }
                }
                CTL {
                    CTL;
                    TAEN    { TAEN }
                    TASTALL { TASTALL }
                    TAEVENT { TAEVENT }
                    TAOTE   { TAOTE }
                    TAPWML  { TAPWML }
                    TBEN    { TBEN }
                    TBSTALL { TBSTALL }
                    TBEVENT { TBEVENT }
                    TBOTE   { TBOTE }
                    TBPWML  { TBPWML }
                }
                SYNC {
                    SYNC;
                    SYNC0 { SYNC0 }
                    SYNC1 { SYNC1 }
                    SYNC2 { SYNC2 }
                    SYNC3 { SYNC3 }
                }
                IMR {
                    IMR;
                    TATOIM { TATOIM }
                    CAMIM { CAMIM }
                    CAEIM { CAEIM }
                    TAMIM { TAMIM }
                    TBTOIM { TBTOIM }
                    CBMIM { CBMIM }
                    CBEIM { CBEIM }
                    TBMIM { TBMIM }
                }
                RIS {
                    RIS;
                    TATORIS { TATORIS }
                    CAMRIS { CAMRIS }
                    CAERIS { CAERIS }
                    TAMRIS { TAMRIS }
                    TBTORIS { TBTORIS }
                    CBMRIS { CBMRIS }
                    CBERIS { CBERIS }
                    TBMRIS { TBMRIS }
                }
                MIS {
                    MIS;
                    TATOMIS { TATOMIS }
                    CAMMIS { CAMMIS }
                    CAEMIS { CAEMIS }
                    TAMRIS { TAMRIS }
                    TBTOMIS { TBTOMIS }
                    CBMMIS { CBMMIS }
                    CBEMIS { CBEMIS }
                    TBMMIS { TBMMIS }
                }
                ICR {
                    ICR;
                    TATOCINT { TATOCINT }
                    CAMCINT { CAMCINT }
                    CAECINT { CAECINT }
                    TAMCINT { TAMCINT }
                    TBTOCINT { TBTOCINT }
                    CBMCINT { CBMCINT }
                    CBECINT { CBECINT }
                    TBMCINT { TBMCINT }
                    WUECINT { WUECINT }
                }
                TAILR {
                    TAILR;
                    TAILR { TAILR }
                }
                TBILR {
                    TBILR;
                    TBILR { TBILR }
                }
                TAMATCHR {
                    TAMATCHR;
                    TAMR { TAMR }
                }
                TBMATCHR {
                    TBMATCHR;
                    TBMR { TBMR }
                }
                TAPR {
                    TAPR;
                    TAPSR { TAPSR }
                }
                TBPR {
                    TBPR;
                    TBPSR { TBPSR }
                }
                TAPMR {
                    TAPMR;
                    TAPSR { TAPSR }
                }
                TBPMR {
                    TBPMR;
                    TBPSR { TBPSR }
                }
                TAR {
                    TAR;
                    TAR { TAR }
                }
                TBR {
                    TBR;
                    TBR { TBR }
                }
                TAV {
                    TAV;
                    TAV { TAV }
                }
                TBV {
                    TBV;
                    TBV { TBV }
                    PRE { PRE }
                }
                TAPS {
                    TAPS;
                    PSS { PSS }
                }
                TBPS {
                    TBPS;
                    PSS { PSS }
                }
                TAPV {
                    TAPV;
                    PSV { PSV }
                }
                TBPV {
                    TBPV;
                    PSV { PSV }
                }
                PP {
                    PP;
                    SIZE { SIZE }
                    CHAIN { CHAIN }
                    SYNCNT { SYNCNT }
                    ALTCLK { ALTCLK }
                }
            }
        }
    };
}

#[cfg(tisl_mcu = "cc2538")]
map_gp_tim! {
    "Extracts GPTIMER0 register tokens.",
    periph_gptimer0,
    "GPTIMER0 perhipheral variant.",
    GpTimer0,
    GPTIMER0,
}

#[cfg(tisl_mcu = "cc2538")]
map_gp_tim! {
    "Extracts GPTIMER1 register tokens.",
    periph_gptimer1,
    "GPTIMER0 perhipheral variant.",
    GpTimer1,
    GPTIMER1,
}

#[cfg(tisl_mcu = "cc2538")]
map_gp_tim! {
    "Extracts GPTIMER2 register tokens.",
    periph_gptimer2,
    "GPTIMER0 perhipheral variant.",
    GpTimer2,
    GPTIMER2,
}

#[cfg(tisl_mcu = "cc2538")]
map_gp_tim! {
    "Extracts GPTIMER3 register tokens.",
    periph_gptimer3,
    "GPTIMER0 perhipheral variant.",
    GpTimer3,
    GPTIMER3,
}
