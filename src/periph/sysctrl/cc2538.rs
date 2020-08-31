//! Extended interrupts and events controller
//! for Texas Instruments CC2538(TM) family of products

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic SYS_CTRL peripheral variant.
    pub trait SysCtrlMap {}

    /// Generic SYS_CTRL peripheral.
    pub struct SysCtrlPeriph;

    SYS_CTRL {
        CLOCK_CTRL {
            0x20 RwRegBitBand;
            OSC32K_CALDIS { RwRwRegFieldBitBand }
            OSC32K { RwRwRegFieldBitBand }
            AMP_DET { RwRwRegFieldBitBand }
            OSC_PD { RwRwRegFieldBitBand }
            OSC { RwRwRegFieldBitBand }
            IO_DIV { RwRwRegFieldBits }
            SYS_DIV { RwRwRegFieldBits }
        }
        CLOCK_STA {
            0x20 RoRegBitBand;
            SYNC_32K { RoRoRegFieldBitBand }
            OSC32K_CALDIS { RoRoRegFieldBitBand }
            OSC32K { RoRoRegFieldBitBand }
            RST { RoRoRegFieldBits }
            SOURCE_CHANGE { RoRoRegFieldBitBand }
            XOSC_STB { RoRoRegFieldBitBand }
            HSOSC_STB { RoRoRegFieldBitBand }
            OSC_PD { RoRoRegFieldBitBand }
            OSC { RoRoRegFieldBitBand }
            IO_DIV { RoRoRegFieldBits }
            RTCLK_FREQ { RoRoRegFieldBits }
            SYS_DIV { RoRoRegFieldBits }
        }
        RCGCGPT {
            0x20 RwRegBitBand;
            GPT3 { RwRwRegFieldBitBand }
            GPT2 { RwRwRegFieldBitBand }
            GPT1 { RwRwRegFieldBitBand }
            GPT0 { RwRwRegFieldBitBand }
        }
        SCGCGPT {
            0x20 RwRegBitBand;
            GPT3 { RwRwRegFieldBitBand }
            GPT2 { RwRwRegFieldBitBand }
            GPT1 { RwRwRegFieldBitBand }
            GPT0 { RwRwRegFieldBitBand }
        }
        DCGCGPT {
            0x20 RwRegBitBand;
            GPT3 { RwRwRegFieldBitBand }
            GPT2 { RwRwRegFieldBitBand }
            GPT1 { RwRwRegFieldBitBand }
            GPT0 { RwRwRegFieldBitBand }
        }
        SRGPT {
            0x20 RwRegBitBand;
            GPT3 { RwRwRegFieldBitBand }
            GPT2 { RwRwRegFieldBitBand }
            GPT1 { RwRwRegFieldBitBand }
            GPT0 { RwRwRegFieldBitBand }
        }
        RCGCSSI {
            0x20 RwRegBitBand;
            SSI1 { RwRwRegFieldBitBand }
            SSI0 { RwRwRegFieldBitBand }
        }
        SCGCSSI {
            0x20 RwRegBitBand;
            SSI1 { RwRwRegFieldBitBand }
            SSI0 { RwRwRegFieldBitBand }
        }
        DCGCSSI {
            0x20 RwRegBitBand;
            SSI1 { RwRwRegFieldBitBand }
            SSI0 { RwRwRegFieldBitBand }
        }
        SRSSI {
            0x20 RwRegBitBand;
            SSI1 { RwRwRegFieldBitBand }
            SSI0 { RwRwRegFieldBitBand }
        }
        RCGCUART {
            0x20 RwRegBitBand;
            UART1 { RwRwRegFieldBitBand }
            UART0 { RwRwRegFieldBitBand }
        }
        SCGCUART {
            0x20 RwRegBitBand;
            UART1 { RwRwRegFieldBitBand }
            UART0 { RwRwRegFieldBitBand }
        }
        DCGCUART {
            0x20 RwRegBitBand;
            UART1 { RwRwRegFieldBitBand }
            UART0 { RwRwRegFieldBitBand }
        }
        SRUART {
            0x20 RwRegBitBand;
            UART1 { RwRwRegFieldBitBand }
            UART0 { RwRwRegFieldBitBand }
        }
        RCGCI2C {
            0x20 RwRegBitBand;
            I2C0 { RwRwRegFieldBitBand }
        }
        SCGCI2C {
            0x20 RwRegBitBand;
            I2C0 { RwRwRegFieldBitBand }
        }
        DCGCI2C {
            0x20 RwRegBitBand;
            I2C0 { RwRwRegFieldBitBand }
        }
        SRI2C {
            0x20 RwRegBitBand;
            I2C0 { RwRwRegFieldBitBand }
        }
        RCGCSEC {
            0x20 RwRegBitBand;
            AES { RwRwRegFieldBitBand }
            PKA { RwRwRegFieldBitBand }
        }
        SCGCSEC {
            0x20 RwRegBitBand;
            AES { RwRwRegFieldBitBand }
            PKA { RwRwRegFieldBitBand }
        }
        DCGCSEC {
            0x20 RwRegBitBand;
            AES { RwRwRegFieldBitBand }
            PKA { RwRwRegFieldBitBand }
        }
        SRSEC {
            0x20 RwRegBitBand;
            AES { RwRwRegFieldBitBand }
            PKA { RwRwRegFieldBitBand }
        }
        PMCTL {
            0x20 RwReg;
            PM { RwRwRegFieldBits }
        }
        SRCRC {
            0x20 RwRegBitBand;
            CRC_REN_USB { RwRwRegFieldBitBand }
            CRC_REN_RF { RwRwRegFieldBitBand }
        }
        PWRDBG {
            0x20 RwRegBitBand;
            FORCE_WARM_RESET { RwRwRegFieldBitBand }
        }
        CLD {
            0x20 RwRegBitBand;
            VALID { RoRwRegFieldBitBand }
            EN { RwRwRegFieldBitBand }
        }
        IWE {
            0x20 RwRegBitBand;
            SM_TIMER_IWE { RwRwRegFieldBitBand }
            USB_IWE { RwRwRegFieldBitBand }
            PORT_D_IWE { RwRwRegFieldBitBand }
            PORT_C_IWE { RwRwRegFieldBitBand }
            PORT_B_IWE { RwRwRegFieldBitBand }
            PORT_A_IWE { RwRwRegFieldBitBand }
        }
        I_MAP {
            0x20 RwRegBitBand;
            ALTMAP { RwRwRegFieldBitBand }
        }
        RCGCRFC {
            0x20 RwRegBitBand;
            RFC0 { RwRwRegFieldBitBand }
        }
        SCGCRFC {
            0x20 RwRegBitBand;
            RFC0 { RwRwRegFieldBitBand }
        }
        DCGCRFC {
            0x20 RwRegBitBand;
            RFC0 { RwRwRegFieldBitBand }
        }
        EMUOVR {
            0x20 RwRegBitBand;
            ICEPICK_FORCE_CLOCK_CG { RwRwRegFieldBitBand }
            ICEPICK_FORCE_POWER_CG { RwRwRegFieldBitBand }
            ICEPICK_INHIBIT_SLEEP_CG { RwRwRegFieldBitBand }
            ICEMELTER_WKUP_CG { RwRwRegFieldBitBand }
            ICEPICK_FORCE_CLOCK_PM { RwRwRegFieldBitBand }
            ICEPICK_FORCE_POWER_PM { RwRwRegFieldBitBand }
            ICEPICK_INHIBIT_SLEEP_PM { RwRwRegFieldBitBand }
            ICEMELTER_WKUP_PM { RwRwRegFieldBitBand }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_sysctrl {
    (
        $sysctrl_macro_doc:expr,
        $sysctrl_macro:ident,
        $sysctrl_ty_doc:expr,
        $sysctrl_ty:ident,
        $sysctrl:ident,
    ) => {
        periph::map! {
            #[doc = $sysctrl_macro_doc]
            pub macro $sysctrl_macro;

            #[doc = $sysctrl_ty_doc]
            pub struct $sysctrl_ty;

            impl SysCtrlMap for $sysctrl_ty {}

            drone_tisl_map_pieces::reg;
            crate;

            SYS_CTRL {
                $sysctrl;
                CLOCK_CTRL {
                    CLOCK_CTRL;
                    OSC32K_CALDIS { OSC32K_CALDIS }
                    OSC32K { OSC32K }
                    AMP_DET { AMP_DET }
                    OSC_PD { OSC_PD }
                    OSC { OSC }
                    IO_DIV { IO_DIV }
                    SYS_DIV { SYS_DIV }
                }
                CLOCK_STA {
                    CLOCK_STA;
                    SYNC_32K { SYNC_32K }
                    OSC32K_CALDIS { OSC32K_CALDIS }
                    OSC32K { OSC32K }
                    RST { RST }
                    SOURCE_CHANGE { SOURCE_CHANGE }
                    XOSC_STB { XOSC_STB }
                    HSOSC_STB { HSOSC_STB }
                    OSC_PD { OSC_PD }
                    OSC { OSC }
                    IO_DIV { IO_DIV }
                    RTCLK_FREQ { RTCLK_FREQ }
                    SYS_DIV { SYS_DIV }
                }
                RCGCGPT {
                    RCGCGPT;
                    GPT3 { GPT3 }
                    GPT2 { GPT2 }
                    GPT1 { GPT1 }
                    GPT0 { GPT0 }
                }
                SCGCGPT {
                    SCGCGPT;
                    GPT3 { GPT3 }
                    GPT2 { GPT2 }
                    GPT1 { GPT1 }
                    GPT0 { GPT0 }
                }
                DCGCGPT {
                    DCGCGPT;
                    GPT3 { GPT3 }
                    GPT2 { GPT2 }
                    GPT1 { GPT1 }
                    GPT0 { GPT0 }
                }
                SRGPT {
                    SRGPT;
                    GPT3 { GPT3 }
                    GPT2 { GPT2 }
                    GPT1 { GPT1 }
                    GPT0 { GPT0 }
                }
                RCGCSSI {
                    RCGCSSI;
                    SSI1 { SSI1 }
                    SSI0 { SSI0 }
                }
                SCGCSSI {
                    SCGCSSI;
                    SSI1 { SSI1 }
                    SSI0 { SSI0 }
                }
                DCGCSSI {
                    DCGCSSI;
                    SSI1 { SSI1 }
                    SSI0 { SSI0 }
                }
                SRSSI {
                    SRSSI;
                    SSI1 { SSI1 }
                    SSI0 { SSI0 }
                }
                RCGCUART {
                    RCGCUART;
                    UART1 { UART1 }
                    UART0 { UART0 }
                }
                SCGCUART {
                    SCGCUART;
                    UART1 { UART1 }
                    UART0 { UART0 }
                }
                DCGCUART {
                    DCGCUART;
                    UART1 { UART1 }
                    UART0 { UART0 }
                }
                SRUART {
                    SRUART;
                    UART1 { UART1 }
                    UART0 { UART0 }
                }
                RCGCI2C {
                    RCGCI2C;
                    I2C0 { I2C0 }
                }
                SCGCI2C {
                    SCGCI2C;
                    I2C0 { I2C0 }
                }
                DCGCI2C {
                    DCGCI2C;
                    I2C0 { I2C0 }
                }
                SRI2C {
                    SRI2C;
                    I2C0 { I2C0 }
                }
                RCGCSEC {
                    RCGCSEC;
                    AES { AES }
                    PKA { PKA }
                }
                SCGCSEC {
                    SCGCSEC;
                    AES { AES }
                    PKA { PKA }
                }
                DCGCSEC {
                    DCGCSEC;
                    AES { AES }
                    PKA { PKA }
                }
                SRSEC {
                    SRSEC;
                    AES { AES }
                    PKA { PKA }
                }
                PMCTL {
                    PMCTL;
                    PM { PM }
                }
                SRCRC {
                    SRCRC;
                    CRC_REN_USB { CRC_REN_USB }
                    CRC_REN_RF { CRC_REN_RF }
                }
                PWRDBG {
                    PWRDBG;
                    FORCE_WARM_RESET { FORCE_WARM_RESET }
                }
                CLD {
                    CLD;
                    VALID { VALID }
                    EN { EN }
                }
                IWE {
                    IWE;
                    SM_TIMER_IWE { SM_TIMER_IWE }
                    USB_IWE { USB_IWE }
                    PORT_D_IWE { PORT_D_IWE }
                    PORT_C_IWE { PORT_C_IWE }
                    PORT_B_IWE { PORT_B_IWE }
                    PORT_A_IWE { PORT_A_IWE }
                }
                I_MAP {
                    I_MAP;
                    ALTMAP { ALTMAP }
                }
                RCGCRFC {
                    RCGCRFC;
                    RFC0 { RFC0 }
                }
                SCGCRFC {
                    SCGCRFC;
                    RFC0 { RFC0 }
                }
                DCGCRFC {
                    DCGCRFC;
                    RFC0 { RFC0 }
                }
                EMUOVR {
                    EMUOVR;
                    ICEPICK_FORCE_CLOCK_CG { ICEPICK_FORCE_CLOCK_CG }
                    ICEPICK_FORCE_POWER_CG { ICEPICK_FORCE_POWER_CG }
                    ICEPICK_INHIBIT_SLEEP_CG { ICEPICK_INHIBIT_SLEEP_CG }
                    ICEMELTER_WKUP_CG { ICEMELTER_WKUP_CG }
                    ICEPICK_FORCE_CLOCK_PM { ICEPICK_FORCE_CLOCK_PM }
                    ICEPICK_FORCE_POWER_PM { ICEPICK_FORCE_POWER_PM }
                    ICEPICK_INHIBIT_SLEEP_PM { ICEPICK_INHIBIT_SLEEP_PM }
                    ICEMELTER_WKUP_PM { ICEMELTER_WKUP_PM }
                }
            }
        }
    };
}

map_sysctrl! {
    "Extracts SYS_CTRL register tokens.",
    periph_sysctrl,
    "SYS_CTRL peripheral variant.",
    Sysctrl,
    SYS_CTRL,
}
