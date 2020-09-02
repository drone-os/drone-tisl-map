//! Mappings for General Purpose I/Os.
//!
//! For CC2538 System-on-Chip Solution.

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic GPIO port peripheral variant.
    pub trait GpioPortMap {}

    /// Generic GPIO port peripheral.
    pub struct GpioPortPeriph;

    GPIO {
        DATA {
            0x20 RwReg;
            DATA { RwRwRegFieldBits }
        }
        DIR {
            0x20 RwReg;
            DIR { RwRwRegFieldBits }
        }
        IS {
            0x20 RwReg;
            IS { RwRwRegFieldBits }
        }
        IBE {
            0x20 RwReg;
            IBE { RwRwRegFieldBits }
        }
        IEV {
            0x20 RwReg;
            IEV { RwRwRegFieldBits }
        }
        IE {
            0x20 RwReg;
            IE { RwRwRegFieldBits }
        }
        RIS {
            0x20 RoReg;
            RIS { RoRoRegFieldBits }
        }
        MIS {
            0x20 RoReg;
            MIS { RoRoRegFieldBits }
        }
        IC {
            0x20 WoReg;
            IC { WoWoRegFieldBits }
        }
        AFSEL {
            0x20 RwReg;
            AFSEL { RwRwRegFieldBits }
        }
        GPIOLOCK {
            0x20 RwReg;
            LOCK { RwRwRegFieldBits }
        }
        GPIOCR {
            0x20 RwReg;
            CR { RwRwRegFieldBits }
        }
        PMUX {
            0x20 RwReg;
            CKOEN { RwRwRegFieldBit }
            CKOPIN { RwRwRegFieldBit }
            DCEN { RwRwRegFieldBit }
            DCPIN { RwRwRegFieldBit }
        }
        P_EDGE_CTRL {
            0x20 RwReg;
            PDIRC7 { RwRwRegFieldBit }
            PDIRC6 { RwRwRegFieldBit }
            PDIRC5 { RwRwRegFieldBit }
            PDIRC4 { RwRwRegFieldBit }
            PDIRC3 { RwRwRegFieldBit }
            PDIRC2 { RwRwRegFieldBit }
            PDIRC1 { RwRwRegFieldBit }
            PDIRC0 { RwRwRegFieldBit }
            PCIRC7 { RwRwRegFieldBit }
            PCIRC6 { RwRwRegFieldBit }
            PCIRC5 { RwRwRegFieldBit }
            PCIRC4 { RwRwRegFieldBit }
            PCIRC3 { RwRwRegFieldBit }
            PCIRC2 { RwRwRegFieldBit }
            PCIRC1 { RwRwRegFieldBit }
            PCIRC0 { RwRwRegFieldBit }
            PBIRC7 { RwRwRegFieldBit }
            PBIRC6 { RwRwRegFieldBit }
            PBIRC5 { RwRwRegFieldBit }
            PBIRC4 { RwRwRegFieldBit }
            PBIRC3 { RwRwRegFieldBit }
            PBIRC2 { RwRwRegFieldBit }
            PBIRC1 { RwRwRegFieldBit }
            PBIRC0 { RwRwRegFieldBit }
            PAIRC7 { RwRwRegFieldBit }
            PAIRC6 { RwRwRegFieldBit }
            PAIRC5 { RwRwRegFieldBit }
            PAIRC4 { RwRwRegFieldBit }
            PAIRC3 { RwRwRegFieldBit }
            PAIRC2 { RwRwRegFieldBit }
            PAIRC1 { RwRwRegFieldBit }
            PAIRC0 { RwRwRegFieldBit }
        }
        PI_IEN {
            0x20 RwReg;
            PDIEN7 { RwRwRegFieldBit }
            PDIEN6 { RwRwRegFieldBit }
            PDIEN5 { RwRwRegFieldBit }
            PDIEN4 { RwRwRegFieldBit }
            PDIEN3 { RwRwRegFieldBit }
            PDIEN2 { RwRwRegFieldBit }
            PDIEN1 { RwRwRegFieldBit }
            PDIEN0 { RwRwRegFieldBit }
            PCIEN7 { RwRwRegFieldBit }
            PCIEN6 { RwRwRegFieldBit }
            PCIEN5 { RwRwRegFieldBit }
            PCIEN4 { RwRwRegFieldBit }
            PCIEN3 { RwRwRegFieldBit }
            PCIEN2 { RwRwRegFieldBit }
            PCIEN1 { RwRwRegFieldBit }
            PCIEN0 { RwRwRegFieldBit }
            PBIEN7 { RwRwRegFieldBit }
            PBIEN6 { RwRwRegFieldBit }
            PBIEN5 { RwRwRegFieldBit }
            PBIEN4 { RwRwRegFieldBit }
            PBIEN3 { RwRwRegFieldBit }
            PBIEN2 { RwRwRegFieldBit }
            PBIEN1 { RwRwRegFieldBit }
            PBIEN0 { RwRwRegFieldBit }
            PAIEN7 { RwRwRegFieldBit }
            PAIEN6 { RwRwRegFieldBit }
            PAIEN5 { RwRwRegFieldBit }
            PAIEN4 { RwRwRegFieldBit }
            PAIEN3 { RwRwRegFieldBit }
            PAIEN2 { RwRwRegFieldBit }
            PAIEN1 { RwRwRegFieldBit }
            PAIEN0 { RwRwRegFieldBit }
        }
        IRQ_DETECT_ACK {
            0x20 RwReg;
            PDIACK7 { RwRwRegFieldBit }
            PDIACK6 { RwRwRegFieldBit }
            PDIACK5 { RwRwRegFieldBit }
            PDIACK4 { RwRwRegFieldBit }
            PDIACK3 { RwRwRegFieldBit }
            PDIACK2 { RwRwRegFieldBit }
            PDIACK1 { RwRwRegFieldBit }
            PDIACK0 { RwRwRegFieldBit }
            PCIACK7 { RwRwRegFieldBit }
            PCIACK6 { RwRwRegFieldBit }
            PCIACK5 { RwRwRegFieldBit }
            PCIACK4 { RwRwRegFieldBit }
            PCIACK3 { RwRwRegFieldBit }
            PCIACK2 { RwRwRegFieldBit }
            PCIACK1 { RwRwRegFieldBit }
            PCIACK0 { RwRwRegFieldBit }
            PBIACK7 { RwRwRegFieldBit }
            PBIACK6 { RwRwRegFieldBit }
            PBIACK5 { RwRwRegFieldBit }
            PBIACK4 { RwRwRegFieldBit }
            PBIACK3 { RwRwRegFieldBit }
            PBIACK2 { RwRwRegFieldBit }
            PBIACK1 { RwRwRegFieldBit }
            PBIACK0 { RwRwRegFieldBit }
            PAIACK7 { RwRwRegFieldBit }
            PAIACK6 { RwRwRegFieldBit }
            PAIACK5 { RwRwRegFieldBit }
            PAIACK4 { RwRwRegFieldBit }
            PAIACK3 { RwRwRegFieldBit }
            PAIACK2 { RwRwRegFieldBit }
            PAIACK1 { RwRwRegFieldBit }
            PAIACK0 { RwRwRegFieldBit }
        }
        USB_IRQ_ACK {
            0x20 RwReg;
            USBACK { RwRwRegFieldBit }

        }
        IRQ_DETECT_UNMASK {
            0x20 RwReg;
            PDIACK7 { RwRwRegFieldBit }
            PDIACK6 { RwRwRegFieldBit }
            PDIACK5 { RwRwRegFieldBit }
            PDIACK4 { RwRwRegFieldBit }
            PDIACK3 { RwRwRegFieldBit }
            PDIACK2 { RwRwRegFieldBit }
            PDIACK1 { RwRwRegFieldBit }
            PDIACK0 { RwRwRegFieldBit }
            PCIACK7 { RwRwRegFieldBit }
            PCIACK6 { RwRwRegFieldBit }
            PCIACK5 { RwRwRegFieldBit }
            PCIACK4 { RwRwRegFieldBit }
            PCIACK3 { RwRwRegFieldBit }
            PCIACK2 { RwRwRegFieldBit }
            PCIACK1 { RwRwRegFieldBit }
            PCIACK0 { RwRwRegFieldBit }
            PBIACK7 { RwRwRegFieldBit }
            PBIACK6 { RwRwRegFieldBit }
            PBIACK5 { RwRwRegFieldBit }
            PBIACK4 { RwRwRegFieldBit }
            PBIACK3 { RwRwRegFieldBit }
            PBIACK2 { RwRwRegFieldBit }
            PBIACK1 { RwRwRegFieldBit }
            PBIACK0 { RwRwRegFieldBit }
            PAIACK7 { RwRwRegFieldBit }
            PAIACK6 { RwRwRegFieldBit }
            PAIACK5 { RwRwRegFieldBit }
            PAIACK4 { RwRwRegFieldBit }
            PAIACK3 { RwRwRegFieldBit }
            PAIACK2 { RwRwRegFieldBit }
            PAIACK1 { RwRwRegFieldBit }
            PAIACK0 { RwRwRegFieldBit }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_gpio_port {
    ($port_macro_doc:expr, $port_macro:ident, $port_ty_doc:expr, $port_ty:ident, $gpio:ident,) => {
        periph::map! {
            #[doc = $port_macro_doc]
            pub macro $port_macro;

            #[doc = $port_ty_doc]
            pub struct $port_ty;

            impl GpioPortMap for $port_ty {}

            drone_tisl_map_pieces::reg;
            crate;

            GPIO {
                $gpio;
                DATA {
                    DATA;
                    DATA { DATA }
                }
                DIR {
                    DIR;
                    DIR { DIR }
                }
                IS {
                    IS;
                    IS { IS }
                }
                IBE {
                    IBE;
                    IBE { IBE }
                }
                IEV {
                    IEV;
                    IEV { IEV }
                }
                IE {
                    IE;
                    IE { IE }
                }
                RIS {
                    RIS;
                    RIS { RIS }
                }
                MIS {
                    MIS;
                    MIS { MIS }
                }
                IC {
                    IC;
                    IC { IC }
                }
                AFSEL {
                    AFSEL;
                    AFSEL { AFSEL }
                }
                GPIOLOCK {
                    GPIOLOCK;
                    LOCK { LOCK }
                }
                GPIOCR {
                    GPIOCR;
                    CR { CR }
                }
                PMUX {
                    PMUX;
                    CKOEN { CKOEN }
                    CKOPIN { CKOPIN }
                    DCEN { DCEN }
                    DCPIN { DCPIN }
                }
                P_EDGE_CTRL {
                    P_EDGE_CTRL;
                    PDIRC7 { PDIRC7 }
                    PDIRC6 { PDIRC6 }
                    PDIRC5 { PDIRC5 }
                    PDIRC4 { PDIRC4 }
                    PDIRC3 { PDIRC3 }
                    PDIRC2 { PDIRC2 }
                    PDIRC1 { PDIRC1 }
                    PDIRC0 { PDIRC0 }
                    PCIRC7 { PCIRC7 }
                    PCIRC6 { PCIRC6 }
                    PCIRC5 { PCIRC5 }
                    PCIRC4 { PCIRC4 }
                    PCIRC3 { PCIRC3 }
                    PCIRC2 { PCIRC2 }
                    PCIRC1 { PCIRC1 }
                    PCIRC0 { PCIRC0 }
                    PBIRC7 { PBIRC7 }
                    PBIRC6 { PBIRC6 }
                    PBIRC5 { PBIRC5 }
                    PBIRC4 { PBIRC4 }
                    PBIRC3 { PBIRC3 }
                    PBIRC2 { PBIRC2 }
                    PBIRC1 { PBIRC1 }
                    PBIRC0 { PBIRC0 }
                    PAIRC7 { PAIRC7 }
                    PAIRC6 { PAIRC6 }
                    PAIRC5 { PAIRC5 }
                    PAIRC4 { PAIRC4 }
                    PAIRC3 { PAIRC3 }
                    PAIRC2 { PAIRC2 }
                    PAIRC1 { PAIRC1 }
                    PAIRC0 { PAIRC0 }
                }
                PI_IEN {
                    PI_IEN;
                    PDIEN7 { PDIEN7 }
                    PDIEN6 { PDIEN6 }
                    PDIEN5 { PDIEN5 }
                    PDIEN4 { PDIEN4 }
                    PDIEN3 { PDIEN3 }
                    PDIEN2 { PDIEN2 }
                    PDIEN1 { PDIEN1 }
                    PDIEN0 { PDIEN0 }
                    PCIEN7 { PCIEN7 }
                    PCIEN6 { PCIEN6 }
                    PCIEN5 { PCIEN5 }
                    PCIEN4 { PCIEN4 }
                    PCIEN3 { PCIEN3 }
                    PCIEN2 { PCIEN2 }
                    PCIEN1 { PCIEN1 }
                    PCIEN0 { PCIEN0 }
                    PBIEN7 { PBIEN7 }
                    PBIEN6 { PBIEN6 }
                    PBIEN5 { PBIEN5 }
                    PBIEN4 { PBIEN4 }
                    PBIEN3 { PBIEN3 }
                    PBIEN2 { PBIEN2 }
                    PBIEN1 { PBIEN1 }
                    PBIEN0 { PBIEN0 }
                    PAIEN7 { PAIEN7 }
                    PAIEN6 { PAIEN6 }
                    PAIEN5 { PAIEN5 }
                    PAIEN4 { PAIEN4 }
                    PAIEN3 { PAIEN3 }
                    PAIEN2 { PAIEN2 }
                    PAIEN1 { PAIEN1 }
                    PAIEN0 { PAIEN0 }
                }
                IRQ_DETECT_ACK {
                    IRQ_DETECT_ACK;
                    PDIACK7 { PDIACK7 }
                    PDIACK6 { PDIACK6 }
                    PDIACK5 { PDIACK5 }
                    PDIACK4 { PDIACK4 }
                    PDIACK3 { PDIACK3 }
                    PDIACK2 { PDIACK2 }
                    PDIACK1 { PDIACK1 }
                    PDIACK0 { PDIACK0 }
                    PCIACK7 { PCIACK7 }
                    PCIACK6 { PCIACK6 }
                    PCIACK5 { PCIACK5 }
                    PCIACK4 { PCIACK4 }
                    PCIACK3 { PCIACK3 }
                    PCIACK2 { PCIACK2 }
                    PCIACK1 { PCIACK1 }
                    PCIACK0 { PCIACK0 }
                    PBIACK7 { PBIACK7 }
                    PBIACK6 { PBIACK6 }
                    PBIACK5 { PBIACK5 }
                    PBIACK4 { PBIACK4 }
                    PBIACK3 { PBIACK3 }
                    PBIACK2 { PBIACK2 }
                    PBIACK1 { PBIACK1 }
                    PBIACK0 { PBIACK0 }
                    PAIACK7 { PAIACK7 }
                    PAIACK6 { PAIACK6 }
                    PAIACK5 { PAIACK5 }
                    PAIACK4 { PAIACK4 }
                    PAIACK3 { PAIACK3 }
                    PAIACK2 { PAIACK2 }
                    PAIACK1 { PAIACK1 }
                    PAIACK0 { PAIACK0 }
                }
                USB_IRQ_ACK {
                    USB_IRQ_ACK;
                    USBACK { USBACK }
                }
                IRQ_DETECT_UNMASK {
                    IRQ_DETECT_UNMASK;
                    PDIACK7 { PDIACK7 }
                    PDIACK6 { PDIACK6 }
                    PDIACK5 { PDIACK5 }
                    PDIACK4 { PDIACK4 }
                    PDIACK3 { PDIACK3 }
                    PDIACK2 { PDIACK2 }
                    PDIACK1 { PDIACK1 }
                    PDIACK0 { PDIACK0 }
                    PCIACK7 { PCIACK7 }
                    PCIACK6 { PCIACK6 }
                    PCIACK5 { PCIACK5 }
                    PCIACK4 { PCIACK4 }
                    PCIACK3 { PCIACK3 }
                    PCIACK2 { PCIACK2 }
                    PCIACK1 { PCIACK1 }
                    PCIACK0 { PCIACK0 }
                    PBIACK7 { PBIACK7 }
                    PBIACK6 { PBIACK6 }
                    PBIACK5 { PBIACK5 }
                    PBIACK4 { PBIACK4 }
                    PBIACK3 { PBIACK3 }
                    PBIACK2 { PBIACK2 }
                    PBIACK1 { PBIACK1 }
                    PBIACK0 { PBIACK0 }
                    PAIACK7 { PAIACK7 }
                    PAIACK6 { PAIACK6 }
                    PAIACK5 { PAIACK5 }
                    PAIACK4 { PAIACK4 }
                    PAIACK3 { PAIACK3 }
                    PAIACK2 { PAIACK2 }
                    PAIACK1 { PAIACK1 }
                    PAIACK0 { PAIACK0 }
                }
            }
        }
    };
}

map_gpio_port! {
    "Extracts GPIO port A register tokens.",
    periph_gpio_a,
    "GPIO port A peripheral variant.",
    GpioA,
    GPIO_A,
}

map_gpio_port! {
    "Extracts GPIO port B register tokens.",
    periph_gpio_b,
    "GPIO port B peripheral variant.",
    GpioB,
    GPIO_B,
}

map_gpio_port! {
    "Extracts GPIO port C register tokens.",
    periph_gpio_c,
    "GPIO port C peripheral variant.",
    GpioC,
    GPIO_C,
}

map_gpio_port! {
    "Extracts GPIO port D register tokens.",
    periph_gpio_d,
    "GPIO port D peripheral variant.",
    GpioD,
    GPIO_D,
}
