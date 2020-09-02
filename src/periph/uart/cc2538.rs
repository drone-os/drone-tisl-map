//! Universal Asynchronous Receiver/Transmitter

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic UART peripheral variant.
    pub trait UartMap {}

    /// Generic UART peripheral.
    pub struct UartPeriph;

    UART {
        DR {
            0x20 RwRegBitBand;
            DATA { RwRwRegFieldBits }
            FE { RoRwRegFieldBitBand }
            PE { RoRwRegFieldBitBand }
            BE { RoRwRegFieldBitBand }
            OE { RoRwRegFieldBitBand }
        }
        RSR {
            0x20 RoRegBitBand;
            FE { RoRoRegFieldBitBand }
            PE { RoRoRegFieldBitBand }
            BE { RoRoRegFieldBitBand }
            OE { RoRoRegFieldBitBand }
        }
        ECR {
            0x20 WoReg;
            DATA { WoWoRegFieldBits }
        }
        FR {
            0x20 RoRegBitBand;
            CTS { RoRoRegFieldBit }
            BUSY { RoRoRegFieldBitBand }
            RXFE { RoRoRegFieldBitBand }
            TXFF { RoRoRegFieldBitBand }
            RXFF { RoRoRegFieldBitBand }
            TXFE { RoRoRegFieldBitBand }
        }
        ILPR {
            0x20 RwReg;
            ILPDVSR { RwRwRegFieldBits }
        }
        IBRD {
            0x20 RwReg;
            DIVINT { RwRwRegFieldBits }
        }
        FBRD {
            0x20 RwReg;
            DIVFRAC { RwRwRegFieldBits }
        }
        LCRH {
            0x20 RwRegBitBand;
            BRK { RwRwRegFieldBitBand }
            PEN { RwRwRegFieldBitBand }
            EPS { RwRwRegFieldBitBand }
            STP2 { RwRwRegFieldBitBand }
            FEN { RwRwRegFieldBitBand }
            WLEN { RwRwRegFieldBits }
            SPS { RwRwRegFieldBitBand }
        }
        CTL {
            0x20 RwRegBitBand;
            UARTEN { RwRwRegFieldBitBand }
            SIREN { RwRwRegFieldBitBand }
            SIRLP { RwRwRegFieldBitBand }
            EOT { RwRwRegFieldBitBand }
            HSE { RwRwRegFieldBitBand }
            LIN { RwRwRegFieldBitBand }
            LBE { RwRwRegFieldBitBand }
            TXE { RwRwRegFieldBitBand }
            RXE { RwRwRegFieldBitBand }
            RTSEN { RwRwRegFieldBitBand }
            CTSEN { RwRwRegFieldBitBand }
        }
        IFLS {
            0x20 RwReg;
            TXIFLSEL { RwRwRegFieldBits }
            RXIFLSEL { RwRwRegFieldBits }
        }
        IM {
            0x20 RwRegBitBand;
            RXIM { RwRwRegFieldBitBand }
            TXIM { RwRwRegFieldBitBand }
            RTIM { RwRwRegFieldBitBand }
            FEIM { RwRwRegFieldBitBand }
            PEIM { RwRwRegFieldBitBand }
            BEIM { RwRwRegFieldBitBand }
            OEIM { RwRwRegFieldBitBand }
            NINEBITIM { RwRwRegFieldBitBand }
            LMSBIM { RwRwRegFieldBitBand }
            LME1IM { RwRwRegFieldBitBand }
            LME5IM { RwRwRegFieldBitBand }
        }
        RIS {
            0x20 RoRegBitBand;
            RXRIS { RoRoRegFieldBitBand }
            TXRIS { RoRoRegFieldBitBand }
            RTRIS { RoRoRegFieldBitBand }
            FERIS { RoRoRegFieldBitBand }
            PERIS { RoRoRegFieldBitBand }
            BERIS { RoRoRegFieldBitBand }
            OERIS { RoRoRegFieldBitBand }
            NINEBITRIS { RoRoRegFieldBitBand }
            LMSBRIS { RoRoRegFieldBitBand }
            LME1RIS { RoRoRegFieldBitBand }
            LME5RIS { RoRoRegFieldBitBand }
        }
        MIS {
            0x20 RoRegBitBand;
            RXMIS { RoRoRegFieldBitBand }
            TXMIS { RoRoRegFieldBitBand }
            RTMIS { RoRoRegFieldBitBand }
            FEMIS { RoRoRegFieldBitBand }
            PEMIS { RoRoRegFieldBitBand }
            BEMIS { RoRoRegFieldBitBand }
            OEMIS { RoRoRegFieldBitBand }
            NINEBITMIS { RoRoRegFieldBitBand }
            LMSBMIS { RoRoRegFieldBitBand }
            LME1MIS { RoRoRegFieldBitBand }
            LME5MIS { RoRoRegFieldBitBand }

        }
        ICR {
            0x20 WoRegBitBand;
            RXIC { WoWoRegFieldBitBand }
            TXIC { WoWoRegFieldBitBand }
            RTIC { WoWoRegFieldBitBand }
            FEIC { WoWoRegFieldBitBand }
            PEIC { WoWoRegFieldBitBand }
            BEIC { WoWoRegFieldBitBand }
            OEIC { WoWoRegFieldBitBand }
            NINEBITIC { WoWoRegFieldBitBand }
            LMSBIC { WoWoRegFieldBitBand }
            LME1IC { WoWoRegFieldBitBand }
            LME5IC { WoWoRegFieldBitBand }
        }
        DMACTL {
            0x20 RwRegBitBand;
            RXDMAE { RwRwRegFieldBitBand }
            TXDMAE { RwRwRegFieldBitBand }
            DMAERR { RwRwRegFieldBitBand }
        }
        LCTL {
            0x20 RwRegBitBand;
            MASTER { RwRwRegFieldBitBand }
            BLEN { RwRwRegFieldBits }
        }
        LSS {
            0x20 RoReg;
            TSS { RoRoRegFieldBits }
        }
        LTIM {
            0x20 RoReg;
            TIMER { RoRoRegFieldBits }
        }
        NINEBITADDR {
            0x20 RwRegBitBand;
            ADDR { RwRwRegFieldBits }
            NINEBITEN { RwRwRegFieldBitBand }
        }
        NINEBITAMASK {
            0x20 RwReg;
            MASK { RwRwRegFieldBits }
            RANGE { RoRwRegFieldBits }
        }
        PP {
            0x20 RoRegBitBand;
            NB { RoRoRegFieldBitBand }
            SC { RoRoRegFieldBitBand }
        }
        CC {
            0x20 RwReg;
            CS { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_uart {
    ($uart_macro_doc:expr, $uart_macro:ident, $uart_ty_doc:expr, $uart_ty:ident, $uart:ident,) => {
        periph::map! {
            #[doc = $uart_macro_doc]
            pub macro $uart_macro;

            #[doc = $uart_ty_doc]
            pub struct $uart_ty;

            impl UartMap for $uart_ty {}

            drone_tisl_map_pieces::reg;
            crate;

            UART {
                $uart;
                DR {
                    DR;
                    DATA { DATA }
                    FE { FE }
                    PE { PE }
                    BE { BE }
                    OE { OE }
                }
                RSR {
                    RSR;
                    FE { FE }
                    PE { PE }
                    BE { BE }
                    OE { OE }
                }
                ECR {
                    ECR;
                    DATA { DATA }
                }
                FR {
                    FR;
                    CTS { CTS }
                    BUSY { BUSY }
                    RXFE { RXFE }
                    TXFF { TXFF }
                    RXFF { RXFF }
                    TXFE { TXFE }
                }
                ILPR {
                    ILPR;
                    ILPDVSR { ILPDVSR }
                }
                IBRD {
                    IBRD;
                    DIVINT { DIVINT }
                }
                FBRD {
                    FBRD;
                    DIVFRAC { DIVFRAC }
                }
                LCRH {
                    LCRH;
                    BRK { BRK }
                    PEN { PEN }
                    EPS { EPS }
                    STP2 { STP2 }
                    FEN { FEN }
                    WLEN { WLEN }
                    SPS { SPS }
                }
                CTL {
                    CTL;
                    UARTEN { UARTEN }
                    SIREN { SIREN }
                    SIRLP { SIRLP }
                    EOT { EOT }
                    HSE { HSE }
                    LIN { LIN }
                    LBE { LBE }
                    TXE { TXE }
                    RXE { RXE }
                    RTSEN { RTSEN }
                    CTSEN { CTSEN }
                }
                IFLS {
                    IFLS;
                    TXIFLSEL { TXIFLSEL }
                    RXIFLSEL { RXIFLSEL }
                }
                IM {
                    IM;
                    RXIM { RXIM }
                    TXIM { TXIM }
                    RTIM { RTIM }
                    FEIM { FEIM }
                    PEIM { PEIM }
                    BEIM { BEIM }
                    OEIM { OEIM }
                    NINEBITIM { NINEBITIM }
                    LMSBIM { LMSBIM }
                    LME1IM { LME1IM }
                    LME5IM { LME5IM }
                }
                RIS {
                    RIS;
                    RXRIS { RXRIS }
                    TXRIS { TXRIS }
                    RTRIS { RTRIS }
                    FERIS { FERIS }
                    PERIS { PERIS }
                    BERIS { BERIS }
                    OERIS { OERIS }
                    NINEBITRIS { NINEBITRIS }
                    LMSBRIS { LMSBRIS }
                    LME1RIS { LME1RIS }
                    LME5RIS { LME5RIS }
                }
                MIS {
                    MIS;
                    RXMIS { RXMIS }
                    TXMIS { TXMIS }
                    RTMIS { RTMIS }
                    FEMIS { FEMIS }
                    PEMIS { PEMIS }
                    BEMIS { BEMIS }
                    OEMIS { OEMIS }
                    NINEBITMIS { NINEBITMIS }
                    LMSBMIS { LMSBMIS }
                    LME1MIS { LME1MIS }
                    LME5MIS { LME5MIS }

                }
                ICR {
                    ICR;
                    RXIC { RXIC }
                    TXIC { TXIC }
                    RTIC { RTIC }
                    FEIC { FEIC }
                    PEIC { PEIC }
                    BEIC { BEIC }
                    OEIC { OEIC }
                    NINEBITIC { NINEBITIC }
                    LMSBIC { LMSBIC }
                    LME1IC { LME1IC }
                    LME5IC { LME5IC }
                }
                DMACTL {
                    DMACTL;
                    RXDMAE { RXDMAE }
                    TXDMAE { TXDMAE }
                    DMAERR { DMAERR }
                }
                LCTL {
                    LCTL;
                    MASTER { MASTER }
                    BLEN { BLEN }
                }
                LSS {
                    LSS;
                    TSS { TSS }
                }
                LTIM {
                    LTIM;
                    TIMER { TIMER }
                }
                NINEBITADDR {
                    NINEBITADDR;
                    ADDR { ADDR }
                    NINEBITEN { NINEBITEN }
                }
                NINEBITAMASK {
                    NINEBITAMASK;
                    MASK { MASK }
                    RANGE { RANGE }
                }
                PP {
                    PP;
                    NB { NB }
                    SC { SC }
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
map_uart! {
    "Extracts UART0 register tokens.",
    periph_uart0,
    "UART0 peripheral variant.",
    Uart0,
    UART0,
}

#[cfg(tisl_mcu = "cc2538")]
map_uart! {
    "Extracts UART1 register tokens.",
    periph_uart1,
    "UART1 peripheral variant.",
    Uart1,
    UART1,
}
