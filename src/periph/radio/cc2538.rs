//! Radio Core

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic Radio peripheral variant.
    pub trait RadioMap {}

    /// Generic Radio peripheral.
    pub struct RadioPeriph;

    RFCORE_FFSM {
        SRCRESMASK0 {
            0x20 RwReg;
            SRCRESMASK0 { RwRwRegFieldBits }
        }
        SRCRESMASK1 {
            0x20 RwReg;
            SRCRESMASK1 { RwRwRegFieldBits }
        }
        SRCRESMASK2 {
            0x20 RwReg;
            SRCRESMASK2 { RwRwRegFieldBits }
        }
        SRCRESINDEX {
            0x20 RwReg;
            SRCRESINDEX { RwRwRegFieldBits }
        }
        SRCEXTPENDEN0 {
            0x20 RwReg;
            SRCEXTPENDEN0 { RwRwRegFieldBits }
        }
        SRCEXTPENDEN1 {
            0x20 RwReg;
            SRCEXTPENDEN1 { RwRwRegFieldBits }
        }
        SRCEXTPENDEN2 {
            0x20 RwReg;
            SRCEXTPENDEN2 { RwRwRegFieldBits }
        }
        SRCSHORTPENDEN0 {
            0x20 RwReg;
            SRCSHORTPENDEN0 { RwRwRegFieldBits }
        }
        SRCSHORTPENDEN1 {
            0x20 RwReg;
            SRCSHORTPENDEN1 { RwRwRegFieldBits }
        }
        SRCSHORTPENDEN2 {
            0x20 RwReg;
            SRCSHORTPENDEN2 { RwRwRegFieldBits }
        }
        EXT_ADDR0 {
            0x20 RwReg;
            EXT_ADDR0 { RwRwRegFieldBits }
        }
        EXT_ADDR1 {
            0x20 RwReg;
            EXT_ADDR1 { RwRwRegFieldBits }
        }
        EXT_ADDR2 {
            0x20 RwReg;
            EXT_ADDR2 { RwRwRegFieldBits }
        }
        EXT_ADDR3 {
            0x20 RwReg;
            EXT_ADDR3 { RwRwRegFieldBits }
        }
        EXT_ADDR4 {
            0x20 RwReg;
            EXT_ADDR4 { RwRwRegFieldBits }
        }
        EXT_ADDR5 {
            0x20 RwReg;
            EXT_ADDR5 { RwRwRegFieldBits }
        }
        EXT_ADDR6 {
            0x20 RwReg;
            EXT_ADDR6 { RwRwRegFieldBits }
        }
        EXT_ADDR7 {
            0x20 RwReg;
            EXT_ADDR7 { RwRwRegFieldBits }
        }
        PAN_ID0 {
            0x20 RwReg;
            PAN_ID0 { RwRwRegFieldBits }
        }
        PAN_ID1 {
            0x20 RwReg;
            PAN_ID1 { RwRwRegFieldBits }
        }
        SHORT_ADDR0 {
            0x20 RwReg;
            SHORT_ADDR0 { RwRwRegFieldBits }
        }
        SHORT_ADDR1 {
            0x20 RwReg;
            SHORT_ADDR1 { RwRwRegFieldBits }
        }
    }

    RFCORE_XREG {
        FRMFILT0 {
            0x20 RwReg;
            FRAME_FILTER_EN { RwRwRegFieldBit }
            PAN_COORDINATOR { RwRwRegFieldBit }
            MAX_FRAME_VERSION { RwRwRegFieldBits }
        }
        FRMFILT1 {
            0x20 RwReg;
            MODIFY_FT_FILTER { RwRwRegFieldBits }
            ACCEPT_FT_0_BEACON { RwRwRegFieldBit }
            ACCEPT_FT_1_DATA { RwRwRegFieldBit }
            ACCEPT_FT_2_ACK { RwRwRegFieldBit }
            ACCEPT_FT_3_MAC_CMD { RwRwRegFieldBit }
        }
        SRCMATCH {
            0x20 RwReg;
            SRC_MATCH_EN { RwRwRegFieldBit }
            AUTOPEND { RwRwRegFieldBit }
            PEND_DATAREQ_ONLY { RwRwRegFieldBit }
        }
        SRCSHORTEN0 {
            0x20 RwReg;
            SHORT_ADDR_EN { RwRwRegFieldBits }
        }
        SRCSHORTEN1 {
            0x20 RwReg;
            SHORT_ADDR_EN { RwRwRegFieldBits }
        }
        SRCSHORTEN2 {
            0x20 RwReg;
            SHORT_ADDR_EN { RwRwRegFieldBits }
        }
        SRCEXTEN0 {
            0x20 RwReg;
            EXT_ADDR_EN { RwRwRegFieldBits }
        }
        SRCEXTEN1 {
            0x20 RwReg;
            EXT_ADDR_EN { RwRwRegFieldBits }
        }
        SRCEXTEN2 {
            0x20 RwReg;
            EXT_ADDR_EN { RwRwRegFieldBits }
        }
        FRMCTRL0 {
            0x20 RwReg;
            TX_MODE { RwRwRegFieldBits }
            RX_MODE { RwRwRegFieldBits }
            ENERGY_SCAN { RwRwRegFieldBit }
            AUTOACK { RwRwRegFieldBit }
            AUTOCRC { RwRwRegFieldBit }
            APPEND_DATA_MODE { RwRwRegFieldBit }
        }
        FRMCTRL1 {
            0x20 RwReg;
            SET_RXENMASK_ON_TX { RwRwRegFieldBit }
            IGNORE_TX_UNDERF { RwRwRegFieldBit }
            PENDING_OR { RwRwRegFieldBit }
        }
        RXENABLE {
            0x20 RoReg;
            RXENMASK { RoRoRegFieldBits }
        }
        RXMASKSET {
            0x20 RwReg;
            RXENMASKSET { RwRwRegFieldBits }
        }
        RXMASKCLR {
            0x20 RwReg;
            RXENMASKCLR { RwRwRegFieldBits }
        }
        FREQTUNE {
            0x20 RwReg;
            XOSC32M_TUNE { RwRwRegFieldBits }
        }
        FREQCTRL {
            0x20 RwReg;
            FREQ { RwRwRegFieldBits }
        }
        TXPOWER {
            0x20 RwReg;
            PA_BIAS { RwRwRegFieldBits }
            PA_POWER { RwRwRegFieldBits }
        }
        TXCTRL {
            0x20 RwReg;
            TXMIX_CURRENT { RwRwRegFieldBits }
            DAC_DC { RwRwRegFieldBits }
            DAC_CURR { RwRwRegFieldBits }
        }
        FSMSTAT0 {
            0x20 RoReg;
            FSM_FFCTRL_STATE { RoRoRegFieldBits }
            CAL_RUNNING { RoRoRegFieldBit }
            CAL_DONE { RoRoRegFieldBit }
        }
        FSMSTAT1 {
            0x20 RoReg;
            RX_ACTIVE { RoRoRegFieldBit }
            TX_ACTIVE { RoRoRegFieldBit }
            LOCK_STATUS { RoRoRegFieldBit }
            SAMPLED_CCA { RoRoRegFieldBit }
            CCA { RoRoRegFieldBit }
            SFD { RoRoRegFieldBit }
            FIFOP { RoRoRegFieldBit }
            FIFO { RoRoRegFieldBit }
        }
        FIFOPCTRL {
            0x20 RwReg;
            FIFOP_THR { RwRwRegFieldBits }
        }
        FSMCTRL {
            0x20 RwReg;
            RX2RX_TIME_OFF { RwRwRegFieldBit }
            SLOTTED_ACK { RwRwRegFieldBit }
        }
        CCACTRL0 {
            0x20 RwReg;
            CCA_THR { RwRwRegFieldBits }
        }
        CCACTRL1 {
            0x20 RwReg;
            CCA_HYST { RwRwRegFieldBits }
            CCA_MODE { RwRwRegFieldBits }
        }
        RSSI {
            0x20 RoReg;
            RSSI_VAL { RoRoRegFieldBits }
        }
        RSSISTAT {
            0x20 RoReg;
            RSSI_VALID { RoRoRegFieldBit }
        }
        RXFIRST {
            0x20 RoReg;
            DATA { RoRoRegFieldBits }
        }
        RXFIFOCNT {
            0x20 RoReg;
            RXFIFOCNT { RoRoRegFieldBits }
        }
        TXFIFOCNT {
            0x20 RoReg;
            TXFIFOCNT { RoRoRegFieldBits }
        }
        RXFIRST_PTR {
            0x20 RoReg;
            RXFIRST_PTR { RoRoRegFieldBits }
        }
        RXLAST_PTR {
            0x20 RoReg;
            RXLAST_PTR { RoRoRegFieldBits }
        }
        RXP1_PTR {
            0x20 RoReg;
            RXP1_PTR { RoRoRegFieldBits }
        }
        TXFIRST_PTR {
            0x20 RoReg;
            TXFIRST_PTR { RoRoRegFieldBits }
        }
        TXLAST_PTR {
            0x20 RoReg;
            TXLAST_PTR { RoRoRegFieldBits }
        }
        RFIRQM0 {
            0x20 RwReg;
            RFIRQM { RwRwRegFieldBits }
        }
        RFIRQM1 {
            0x20 RwReg;
            RFIRQM { RwRwRegFieldBits }
        }
        RFERRM {
            0x20 RwReg;
            RFERRM { RwRwRegFieldBits }
        }
        RFRND {
            0x20 RoReg;
            IRND { RoRoRegFieldBit }
            QRND { RoRoRegFieldBit }
        }
        MDMCTRL0 {
            0x20 RwReg;
            TX_FILTER { RwRwRegFieldBit }
            PREAMBLE_LENGTH { RwRwRegFieldBits }
            DEMOD_AVG_MODE { RwRwRegFieldBit }
            DEM_NUM_ZEROS { RwRwRegFieldBits }
        }
        MDMCTRL1 {
            0x20 RwReg;
            CORR_THR { RwRwRegFieldBits }
            CORR_THR_SFD { RwRwRegFieldBit }
        }
        FREQEST {
            0x20 RoReg;
            FREQEST { RoRoRegFieldBits }
        }
        RXCTRL {
            0x20 RwReg;
            MIX_CURRENT { RwRwRegFieldBits }
            GBIAS_LNA_REF { RwRwRegFieldBits }
            GBIAS_LNA2_REF { RwRwRegFieldBits }
        }
        FSCTRL {
            0x20 RwReg;
            LODIV_CURRENT { RwRwRegFieldBits }
            LODIV_BUF_CURRENT_RX { RwRwRegFieldBits }
            LODIV_BUF_CURRENT_TX { RwRwRegFieldBits }
            PRE_CURRENT { RwRwRegFieldBits }
        }
        FSCAL0 {
            0x20 RwReg;
            BW_BOOST_MODE { RwRwRegFieldBits }
            CHP_CURRENT { RwRwRegFieldBits }
            CHP_DISABLE { RwRwRegFieldBit }
            VCO_CURR_COMP_EN_OV { RwRwRegFieldBit }
        }
        FSCAL1 {
            0x20 RwReg;
            VCO_CURR { RwRwRegFieldBits }
            VCO_CURR_CAL { RwRwRegFieldBits }
            VCO_CURR_CAL_OE { RwRwRegFieldBit }
        }
        FSCAL2 {
            0x20 RwReg;
            VCO_CAPARR { RwRwRegFieldBits }
            VCO_CAPARR_OE { RwRwRegFieldBit }
        }
        FSCAL3 {
            0x20 RwReg;
            VCO_CAPARR_CAL_CTRL { RwRwRegFieldBits }
            VCO_VC_DAC { RwRwRegFieldBits }
            VCO_DAC_EN_OV { RwRwRegFieldBit }
        }
        AGCCTRL0 {
            0x20 RwReg;
            AGC_DR_XTND_THR { RwRwRegFieldBits }
            AGC_DR_XTND_EN { RwRwRegFieldBit }
        }
        AGCCTRL1 {
            0x20 RwReg;
            AGC_REF { RwRwRegFieldBits }
        }
        AGCCTRL2 {
            0x20 RwReg;
            LNA_CURRENT_OE { RwRwRegFieldBit }
            LNA3_CURRENT { RwRwRegFieldBits }
            LNA2_CURRENT { RwRwRegFieldBits }
            LNA1_CURRENT { RwRwRegFieldBits }
        }
        AGCCTRL3 {
            0x20 RwReg;
            AAF_RP_OE { RwRwRegFieldBit }
            AAF_RP { RwRwRegFieldBits }
            AGC_WIN_SIZE { RwRwRegFieldBits }
            AGC_SETTLE_WAIT { RwRwRegFieldBits }
        }
        ADCTEST0 {
            0x20 RwReg;
            ADC_DAC2_EN { RwRwRegFieldBit }
            ADC_GM_ADJ { RwRwRegFieldBits }
            ADC_QUANT_ADJ { RwRwRegFieldBits }
            ADC_VREF_ADJ { RwRwRegFieldBits }
        }
        ADCTEST1 {
            0x20 RwReg;
            ADC_C3_ADJ { RwRwRegFieldBits }
            ADC_C2_ADJ { RwRwRegFieldBits }
            ADC_TEST_CTRL { RwRwRegFieldBits }
        }
        ADCTEST2 {
            0x20 RwReg;
            ADC_DAC_ROT { RwRwRegFieldBit }
            ADC_FF_ADJ { RwRwRegFieldBits }
            AAF_RS { RwRwRegFieldBits }
            ADC_TEST_MODE { RwRwRegFieldBits }
        }
        MDMTEST0 {
            0x20 RwReg;
            DC_BLOCK_MODE { RwRwRegFieldBits }
            DC_WIN_SIZE { RwRwRegFieldBits }
            TX_TONE { RwRwRegFieldBits }
        }
        MDMTEST1 {
            0x20 RwReg;
            LOOPBACK_EN { RwRwRegFieldBit }
            RFC_SNIFF_EN { RwRwRegFieldBit }
            RAMP_AMP { RwRwRegFieldBit }
            MOD_IF { RwRwRegFieldBit }
            USEMIRROR_IF { RwRwRegFieldBit }
        }
        DACTEST0 {
            0x20 RwReg;
            DAC_Q_O { RwRwRegFieldBits }
        }
        DACTEST1 {
            0x20 RwReg;
            DAC_I_O { RwRwRegFieldBits }
        }
        DACTEST2 {
            0x20 RwReg;
            DAC_SRC { RwRwRegFieldBits }
            DAC_CASC_CTRL { RwRwRegFieldBits }
            DAC_DEM_EN { RwRwRegFieldBit }
        }
        ATEST {
            0x20 RwReg;
            ATEST_CTRL { RwRwRegFieldBits }
        }
        PTEST0 {
            0x20 RwReg;
            AAF_PD { RwRwRegFieldBit }
            TXMIX_PD { RwRwRegFieldBit }
            LNA_PD { RwRwRegFieldBits }
            DAC_PD { RwRwRegFieldBit }
            ADC_PD { RwRwRegFieldBit }
            CHP_PD { RwRwRegFieldBit }
            PRE_PD { RoRwRegFieldBit }
        }
        PTEST1 {
            0x20 RwReg;
            LODIV_PD { RwRwRegFieldBit }
            VCO_PD { RwRwRegFieldBit }
            PA_PD { RwRwRegFieldBit }
            PD_OVERRIDE { RwRwRegFieldBit }
        }
        CSPPROG_0 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_1 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_2 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_3 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_4 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_5 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_6 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_7 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_8 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_9 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_10 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_11 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_12 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_13 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_14 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_15 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_16 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_17 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_18 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_19 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_20 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_21 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_22 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPPROG_23 {
            0x20 RoReg;
            CSP_INSTR { RoRoRegFieldBits }
        }
        CSPCTRL  {
            0x20 RwReg;
            MCU_CTRL { RwRwRegFieldBit}
        }
        CSPSTAT {
            0x20 RoReg;
            CSP_PC { RoRoRegFieldBits }
            CSP_RUNNING { RoRoRegFieldBit }
        }
        CSPX {
            0x20 RoReg;
            CSPX { RoRoRegFieldBits }
        }
        CSPY {
            0x20 RoReg;
            CSPY { RoRoRegFieldBits }
        }
        CSPZ {
            0x20 RoReg;
            CSPZ { RoRoRegFieldBits }
        }
        CSPT {
            0x20 RoReg;
            CSPT { RoRoRegFieldBits }
        }
        RFC_OBS_CTRL0 {
            0x20 RwReg;
            RFC_OBS_MUX0 { RwRwRegFieldBits }
            RFC_OBS_POL0 { RwRwRegFieldBit }
        }
        RFC_OBS_CTRL1 {
            0x20 RwReg;
            RFC_OBS_MUX1 { RwRwRegFieldBits }
            RFC_OBS_POL1 { RwRwRegFieldBit }
        }
        RFC_OBS_CTRL2 {
            0x20 RwReg;
            RFC_OBS_MUX2 { RwRwRegFieldBits }
            RFC_OBS_POL2 { RwRwRegFieldBit }
        }
        TXFILTCFG {
            0x20 RwReg;
            FC { RwRwRegFieldBits }
        }
    }

    RFCORE_SFR {
        RFDATA {
            0x20 RwReg;
            RFD { RwRwRegFieldBits }
        }
        RFERRF {
            0x20 RwReg;
            NLOCK { RwRwRegFieldBit }
            RXABO { RwRwRegFieldBit }
            RXOVERF { RwRwRegFieldBit }
            RXUNDERF { RwRwRegFieldBit }
            TXOVERF { RwRwRegFieldBit }
            TXUNDERF { RwRwRegFieldBit }
            STROBEERR { RwRwRegFieldBit }
        }
        RFIRQF1 {
            0x20 RwReg;
            TXACKDONE { RwRwRegFieldBit }
            TXDONE { RwRwRegFieldBit }
            RFIDLE { RwRwRegFieldBit }
            CSP_MANINT { RwRwRegFieldBit }
            CSP_STOP { RwRwRegFieldBit }
            CSP_WAIT { RwRwRegFieldBit }
        }
        RFIRQF0 {
            0x20 RwReg;
            ACT_UNUSED { RwRwRegFieldBit }
            SFD { RwRwRegFieldBit }
            FIFOP { RwRwRegFieldBit }
            SRC_MATCH_DONE { RwRwRegFieldBit }
            SRC_MATCH_FOUND { RwRwRegFieldBit }
            FRAME_ACCEPTED { RwRwRegFieldBit }
            RXPKTDONE { RwRwRegFieldBit }
            RXMASKZERO { RwRwRegFieldBit }
        }
        RFST {
            0x20 RwReg;
            INSTR { RwRwRegFieldBits }
        }
    }

    CCTEST {
        IO {
            0x20 RwReg;
            SC { RwRwRegFieldBit }
        }
        OBSSEL0 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        OBSSEL1 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        OBSSEL2 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        OBSSEL3 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        OBSSEL4 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        OBSSEL5 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        OBSSEL6 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        OBSSEL7 {
            0x20 RwReg;
            SEL { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        TR0 {
            0x20 RwReg;
            ADCTM { RwRwRegFieldBit }
        }
        USBCTRL {
            0x20 RwReg;
            USB_STB { RwRwRegFieldBit }
        }
    }

    ANA_REGS {
        IVCTRL {
            0x20 RwReg;
            PA_BIAS_CTRL { RwRwRegFieldBits }
            TXMIX_DC_CTRL { RwRwRegFieldBit }
            LODIV_BIAS_CTRL { RwRwRegFieldBit }
            DAC_CURR_CTRL { RwRwRegFieldBits }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_radio {
    (
        $radio_macro_doc:expr,
        $radio_macro:ident,
        $radio_ty_doc:expr,
        $radio_ty:ident,
        $radio_ffsm:ident,
        $radio_xreg:ident,
        $radio_sfr:ident,
        $radio_cctest:ident,
        $radio_ana_regs:ident,
    ) => {
        periph::map! {
            #[doc = $radio_macro_doc]
            pub macro $radio_macro;

            #[doc = $radio_ty_doc]
            pub struct $radio_ty;

            impl RadioMap for $radio_ty {}

            drone_tisl_map_pieces::reg;
            crate;

            RFCORE_FFSM {
                $radio_ffsm;
                SRCRESMASK0 {
                    SRCRESMASK0;
                    SRCRESMASK0 { SRCRESMASK0 }
                }
                SRCRESMASK1 {
                    SRCRESMASK1;
                    SRCRESMASK1 { SRCRESMASK1 }
                }
                SRCRESMASK2 {
                    SRCRESMASK2;
                    SRCRESMASK2 { SRCRESMASK2 }
                }
                SRCRESINDEX {
                    SRCRESINDEX;
                    SRCRESINDEX { SRCRESINDEX }
                }
                SRCEXTPENDEN0 {
                    SRCEXTPENDEN0;
                    SRCEXTPENDEN0 { SRCEXTPENDEN0 }
                }
                SRCEXTPENDEN1 {
                    SRCEXTPENDEN1;
                    SRCEXTPENDEN1 { SRCEXTPENDEN1 }
                }
                SRCEXTPENDEN2 {
                    SRCEXTPENDEN2;
                    SRCEXTPENDEN2 { SRCEXTPENDEN2 }
                }
                SRCSHORTPENDEN0 {
                    SRCSHORTPENDEN0;
                    SRCSHORTPENDEN0 { SRCSHORTPENDEN0 }
                }
                SRCSHORTPENDEN1 {
                    SRCSHORTPENDEN1;
                    SRCSHORTPENDEN1 { SRCSHORTPENDEN1 }
                }
                SRCSHORTPENDEN2 {
                    SRCSHORTPENDEN2;
                    SRCSHORTPENDEN2 { SRCSHORTPENDEN2 }
                }
                EXT_ADDR0 {
                    EXT_ADDR0;
                    EXT_ADDR0 { EXT_ADDR0 }
                }
                EXT_ADDR1 {
                    EXT_ADDR1;
                    EXT_ADDR1 { EXT_ADDR1 }
                }
                EXT_ADDR2 {
                    EXT_ADDR2;
                    EXT_ADDR2 { EXT_ADDR2 }
                }
                EXT_ADDR3 {
                    EXT_ADDR3;
                    EXT_ADDR3 { EXT_ADDR3 }
                }
                EXT_ADDR4 {
                    EXT_ADDR4;
                    EXT_ADDR4 { EXT_ADDR4 }
                }
                EXT_ADDR5 {
                    EXT_ADDR5;
                    EXT_ADDR5 { EXT_ADDR5 }
                }
                EXT_ADDR6 {
                    EXT_ADDR6;
                    EXT_ADDR6 { EXT_ADDR6 }
                }
                EXT_ADDR7 {
                    EXT_ADDR7;
                    EXT_ADDR7 { EXT_ADDR7 }
                }
                PAN_ID0 {
                    PAN_ID0;
                    PAN_ID0 { PAN_ID0 }
                }
                PAN_ID1 {
                    PAN_ID1;
                    PAN_ID1 { PAN_ID1 }
                }
                SHORT_ADDR0 {
                    SHORT_ADDR0;
                    SHORT_ADDR0 { SHORT_ADDR0 }
                }
                SHORT_ADDR1 {
                    SHORT_ADDR1;
                    SHORT_ADDR1 { SHORT_ADDR1 }
                }
            }

            RFCORE_XREG {
                $radio_xreg;
                FRMFILT0 {
                    FRMFILT0;
                    FRAME_FILTER_EN { FRAME_FILTER_EN }
                    PAN_COORDINATOR { PAN_COORDINATOR }
                    MAX_FRAME_VERSION { MAX_FRAME_VERSION }
                }
                FRMFILT1 {
                    FRMFILT1;
                    MODIFY_FT_FILTER { MODIFY_FT_FILTER }
                    ACCEPT_FT_0_BEACON { ACCEPT_FT_0_BEACON }
                    ACCEPT_FT_1_DATA { ACCEPT_FT_1_DATA }
                    ACCEPT_FT_2_ACK { ACCEPT_FT_2_ACK }
                    ACCEPT_FT_3_MAC_CMD { ACCEPT_FT_3_MAC_CMD }
                }
                SRCMATCH {
                    SRCMATCH;
                    SRC_MATCH_EN { SRC_MATCH_EN }
                    AUTOPEND { AUTOPEND }
                    PEND_DATAREQ_ONLY { PEND_DATAREQ_ONLY }
                }
                SRCSHORTEN0 {
                    SRCSHORTEN0;
                    SHORT_ADDR_EN { SHORT_ADDR_EN }
                }
                SRCSHORTEN1 {
                    SRCSHORTEN1;
                    SHORT_ADDR_EN { SHORT_ADDR_EN }
                }
                SRCSHORTEN2 {
                    SRCSHORTEN2;
                    SHORT_ADDR_EN { SHORT_ADDR_EN }
                }
                SRCEXTEN0 {
                    SRCEXTEN0;
                    EXT_ADDR_EN { EXT_ADDR_EN }
                }
                SRCEXTEN1 {
                    SRCEXTEN1;
                    EXT_ADDR_EN { EXT_ADDR_EN }
                }
                SRCEXTEN2 {
                    SRCEXTEN2;
                    EXT_ADDR_EN { EXT_ADDR_EN }
                }
                FRMCTRL0 {
                    FRMCTRL0;
                    TX_MODE { TX_MODE }
                    RX_MODE { RX_MODE }
                    ENERGY_SCAN { ENERGY_SCAN }
                    AUTOACK { AUTOACK }
                    AUTOCRC { AUTOCRC }
                    APPEND_DATA_MODE { APPEND_DATA_MODE }
                }
                FRMCTRL1 {
                    FRMCTRL1;
                    SET_RXENMASK_ON_TX { SET_RXENMASK_ON_TX }
                    IGNORE_TX_UNDERF { IGNORE_TX_UNDERF }
                    PENDING_OR { PENDING_OR }
                }
                RXENABLE {
                    RXENABLE;
                    RXENMASK { RXENMASK }
                }
                RXMASKSET {
                    RXMASKSET;
                    RXENMASKSET { RXENMASKSET }
                }
                RXMASKCLR {
                    RXMASKCLR;
                    RXENMASKCLR { RXENMASKCLR }
                }
                FREQTUNE {
                    FREQTUNE;
                    XOSC32M_TUNE { XOSC32M_TUNE }
                }
                FREQCTRL {
                    FREQCTRL;
                    FREQ { FREQ }
                }
                TXPOWER {
                    TXPOWER;
                    PA_BIAS { PA_BIAS }
                    PA_POWER { PA_POWER }
                }
                TXCTRL {
                    TXCTRL;
                    TXMIX_CURRENT { TXMIX_CURRENT }
                    DAC_DC { DAC_DC }
                    DAC_CURR { DAC_CURR }
                }
                FSMSTAT0 {
                    FSMSTAT0;
                    FSM_FFCTRL_STATE { FSM_FFCTRL_STATE }
                    CAL_RUNNING { CAL_RUNNING }
                    CAL_DONE { CAL_DONE }
                }
                FSMSTAT1 {
                    FSMSTAT1;
                    RX_ACTIVE { RX_ACTIVE }
                    TX_ACTIVE { TX_ACTIVE }
                    LOCK_STATUS { LOCK_STATUS }
                    SAMPLED_CCA { SAMPLED_CCA }
                    CCA { CCA }
                    SFD { SFD }
                    FIFOP { FIFOP }
                    FIFO { FIFO }
                }
                FIFOPCTRL {
                    FIFOPCTRL;
                    FIFOP_THR { FIFOP_THR }
                }
                FSMCTRL {
                    FSMCTRL;
                    RX2RX_TIME_OFF { RX2RX_TIME_OFF }
                    SLOTTED_ACK { SLOTTED_ACK }
                }
                CCACTRL0 {
                    CCACTRL0;
                    CCA_THR { CCA_THR }
                }
                CCACTRL1 {
                    CCACTRL1;
                    CCA_HYST { CCA_HYST }
                    CCA_MODE { CCA_MODE }
                }
                RSSI {
                    RSSI;
                    RSSI_VAL { RSSI_VAL }
                }
                RSSISTAT {
                    RSSISTAT;
                    RSSI_VALID { RSSI_VALID }
                }
                RXFIRST {
                    RXFIRST;
                    DATA { DATA }
                }
                RXFIFOCNT {
                    RXFIFOCNT;
                    RXFIFOCNT { RXFIFOCNT }
                }
                TXFIFOCNT {
                    TXFIFOCNT;
                    TXFIFOCNT { TXFIFOCNT }
                }
                RXFIRST_PTR {
                    RXFIRST_PTR;
                    RXFIRST_PTR { RXFIRST_PTR }
                }
                RXLAST_PTR {
                    RXLAST_PTR;
                    RXLAST_PTR { RXLAST_PTR }
                }
                RXP1_PTR {
                    RXP1_PTR;
                    RXP1_PTR { RXP1_PTR }
                }
                TXFIRST_PTR {
                    TXFIRST_PTR;
                    TXFIRST_PTR { TXFIRST_PTR }
                }
                TXLAST_PTR {
                    TXLAST_PTR;
                    TXLAST_PTR { TXLAST_PTR }
                }
                RFIRQM0 {
                    RFIRQM0;
                    RFIRQM { RFIRQM }
                }
                RFIRQM1 {
                    RFIRQM1;
                    RFIRQM { RFIRQM }
                }
                RFERRM {
                    RFERRM;
                    RFERRM { RFERRM }
                }
                RFRND {
                    RFRND;
                    IRND { IRND }
                    QRND { QRND }
                }
                MDMCTRL0 {
                    MDMCTRL0;
                    TX_FILTER { TX_FILTER }
                    PREAMBLE_LENGTH { PREAMBLE_LENGTH }
                    DEMOD_AVG_MODE { DEMOD_AVG_MODE }
                    DEM_NUM_ZEROS { DEM_NUM_ZEROS }
                }
                MDMCTRL1 {
                    MDMCTRL1;
                    CORR_THR { CORR_THR }
                    CORR_THR_SFD { CORR_THR_SFD }
                }
                FREQEST {
                    FREQEST;
                    FREQEST { FREQEST }
                }
                RXCTRL {
                    RXCTRL;
                    MIX_CURRENT { MIX_CURRENT }
                    GBIAS_LNA_REF { GBIAS_LNA_REF }
                    GBIAS_LNA2_REF { GBIAS_LNA2_REF }
                }
                FSCTRL {
                    FSCTRL;
                    LODIV_CURRENT { LODIV_CURRENT }
                    LODIV_BUF_CURRENT_RX { LODIV_BUF_CURRENT_RX }
                    LODIV_BUF_CURRENT_TX { LODIV_BUF_CURRENT_TX }
                    PRE_CURRENT { PRE_CURRENT }
                }
                FSCAL0 {
                    FSCAL0;
                    BW_BOOST_MODE { BW_BOOST_MODE }
                    CHP_CURRENT { CHP_CURRENT }
                    CHP_DISABLE { CHP_DISABLE }
                    VCO_CURR_COMP_EN_OV { VCO_CURR_COMP_EN_OV }
                }
                FSCAL1 {
                    FSCAL1;
                    VCO_CURR { VCO_CURR }
                    VCO_CURR_CAL { VCO_CURR_CAL }
                    VCO_CURR_CAL_OE { VCO_CURR_CAL_OE }
                }
                FSCAL2 {
                    FSCAL2;
                    VCO_CAPARR { VCO_CAPARR }
                    VCO_CAPARR_OE { VCO_CAPARR_OE }
                }
                FSCAL3 {
                    FSCAL3;
                    VCO_CAPARR_CAL_CTRL { VCO_CAPARR_CAL_CTRL }
                    VCO_VC_DAC { VCO_VC_DAC }
                    VCO_DAC_EN_OV { VCO_DAC_EN_OV }
                }
                AGCCTRL0 {
                    AGCCTRL0;
                    AGC_DR_XTND_THR { AGC_DR_XTND_THR }
                    AGC_DR_XTND_EN { AGC_DR_XTND_EN }
                }
                AGCCTRL1 {
                    AGCCTRL1;
                    AGC_REF { AGC_REF }
                }
                AGCCTRL2 {
                    AGCCTRL2;
                    LNA_CURRENT_OE { LNA_CURRENT_OE }
                    LNA3_CURRENT { LNA3_CURRENT }
                    LNA2_CURRENT { LNA2_CURRENT }
                    LNA1_CURRENT { LNA1_CURRENT }
                }
                AGCCTRL3 {
                    AGCCTRL3;
                    AAF_RP_OE { AAF_RP_OE }
                    AAF_RP { AAF_RP }
                    AGC_WIN_SIZE { AGC_WIN_SIZE }
                    AGC_SETTLE_WAIT { AGC_SETTLE_WAIT }
                }
                ADCTEST0 {
                    ADCTEST0;
                    ADC_DAC2_EN { ADC_DAC2_EN }
                    ADC_GM_ADJ { ADC_GM_ADJ }
                    ADC_QUANT_ADJ { ADC_QUANT_ADJ }
                    ADC_VREF_ADJ { ADC_VREF_ADJ }
                }
                ADCTEST1 {
                    ADCTEST1;
                    ADC_C3_ADJ { ADC_C3_ADJ }
                    ADC_C2_ADJ { ADC_C2_ADJ }
                    ADC_TEST_CTRL { ADC_TEST_CTRL }
                }
                ADCTEST2 {
                    ADCTEST2;
                    ADC_DAC_ROT { ADC_DAC_ROT }
                    ADC_FF_ADJ { ADC_FF_ADJ }
                    AAF_RS { AAF_RS }
                    ADC_TEST_MODE { ADC_TEST_MODE }
                }
                MDMTEST0 {
                    MDMTEST0;
                    DC_BLOCK_MODE { DC_BLOCK_MODE }
                    DC_WIN_SIZE { DC_WIN_SIZE }
                    TX_TONE { TX_TONE }
                }
                MDMTEST1 {
                    MDMTEST1;
                    LOOPBACK_EN { LOOPBACK_EN }
                    RFC_SNIFF_EN { RFC_SNIFF_EN }
                    RAMP_AMP { RAMP_AMP }
                    MOD_IF { MOD_IF }
                    USEMIRROR_IF { USEMIRROR_IF }
                }
                DACTEST0 {
                    DACTEST0;
                    DAC_Q_O { DAC_Q_O }
                }
                DACTEST1 {
                    DACTEST1;
                    DAC_I_O { DAC_I_O }
                }
                DACTEST2 {
                    DACTEST2;
                    DAC_SRC { DAC_SRC }
                    DAC_CASC_CTRL { DAC_CASC_CTRL }
                    DAC_DEM_EN { DAC_DEM_EN }
                }
                ATEST {
                    ATEST;
                    ATEST_CTRL { ATEST_CTRL }
                }
                PTEST0 {
                    PTEST0;
                    AAF_PD { AAF_PD }
                    TXMIX_PD { TXMIX_PD }
                    LNA_PD { LNA_PD }
                    DAC_PD { DAC_PD }
                    ADC_PD { ADC_PD }
                    CHP_PD { CHP_PD }
                    PRE_PD { PRE_PD }
                }
                PTEST1 {
                    PTEST1;
                    LODIV_PD { LODIV_PD }
                    VCO_PD { VCO_PD }
                    PA_PD { PA_PD }
                    PD_OVERRIDE { PD_OVERRIDE }
                }
                CSPPROG_0 {
                    CSPPROG_0;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_1 {
                    CSPPROG_1;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_2 {
                    CSPPROG_2;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_3 {
                    CSPPROG_3;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_4 {
                    CSPPROG_4;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_5 {
                    CSPPROG_5;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_6 {
                    CSPPROG_6;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_7 {
                    CSPPROG_7;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_8 {
                    CSPPROG_8;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_9 {
                    CSPPROG_9;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_10 {
                    CSPPROG_10;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_11 {
                    CSPPROG_11;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_12 {
                    CSPPROG_12;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_13 {
                    CSPPROG_13;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_14 {
                    CSPPROG_14;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_15 {
                    CSPPROG_15;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_16 {
                    CSPPROG_16;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_17 {
                    CSPPROG_17;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_18 {
                    CSPPROG_18;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_19 {
                    CSPPROG_19;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_20 {
                    CSPPROG_20;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_21 {
                    CSPPROG_21;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_22 {
                    CSPPROG_22;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPPROG_23 {
                    CSPPROG_23;
                    CSP_INSTR { CSP_INSTR }
                }
                CSPCTRL  {
                    CSPCTRL;
                    MCU_CTRL { MCU_CTRL }
                }
                CSPSTAT {
                    CSPSTAT;
                    CSP_PC { CSP_PC }
                    CSP_RUNNING { CSP_RUNNING }
                }
                CSPX {
                    CSPX;
                    CSPX { CSPX }
                }
                CSPY {
                    CSPY;
                    CSPY { CSPY }
                }
                CSPZ {
                    CSPZ;
                    CSPZ { CSPZ }
                }
                CSPT {
                    CSPT;
                    CSPT { CSPT }
                }
                RFC_OBS_CTRL0 {
                    RFC_OBS_CTRL0;
                    RFC_OBS_MUX0 { RFC_OBS_MUX0 }
                    RFC_OBS_POL0 { RFC_OBS_POL0 }
                }
                RFC_OBS_CTRL1 {
                    RFC_OBS_CTRL1;
                    RFC_OBS_MUX1 { RFC_OBS_MUX1 }
                    RFC_OBS_POL1 { RFC_OBS_POL1 }
                }
                RFC_OBS_CTRL2 {
                    RFC_OBS_CTRL2;
                    RFC_OBS_MUX2 { RFC_OBS_MUX2 }
                    RFC_OBS_POL2 { RFC_OBS_POL2 }
                }
                TXFILTCFG {
                    TXFILTCFG;
                    FC { FC }
                }
            }

            RFCORE_SFR {
                $radio_sfr;
                RFDATA {
                    RFDATA;
                    RFD { RFD }
                }
                RFERRF {
                    RFERRF;
                    NLOCK { NLOCK }
                    RXABO { RXABO }
                    RXOVERF { RXOVERF }
                    RXUNDERF { RXUNDERF }
                    TXOVERF { TXOVERF }
                    TXUNDERF { TXUNDERF }
                    STROBEERR { STROBEERR }
                }
                RFIRQF1 {
                    RFIRQF1;
                    TXACKDONE { TXACKDONE }
                    TXDONE { TXDONE }
                    RFIDLE { RFIDLE }
                    CSP_MANINT { CSP_MANINT }
                    CSP_STOP { CSP_STOP }
                    CSP_WAIT { CSP_WAIT }
                }
                RFIRQF0 {
                    RFIRQF0;
                    ACT_UNUSED { ACT_UNUSED }
                    SFD { SFD }
                    FIFOP { FIFOP }
                    SRC_MATCH_DONE { SRC_MATCH_DONE }
                    SRC_MATCH_FOUND { SRC_MATCH_FOUND }
                    FRAME_ACCEPTED { FRAME_ACCEPTED }
                    RXPKTDONE { RXPKTDONE }
                    RXMASKZERO { RXMASKZERO }
                }
                RFST {
                    RFST;
                    INSTR { INSTR }
                }
            }

            CCTEST {
                $radio_cctest;
                IO {
                    IO;
                    SC { SC }
                }
                OBSSEL0 {
                    OBSSEL0;
                    SEL { SEL }
                    EN { EN }
                }
                OBSSEL1 {
                    OBSSEL1;
                    SEL { SEL }
                    EN { EN }
                }
                OBSSEL2 {
                    OBSSEL2;
                    SEL { SEL }
                    EN { EN }
                }
                OBSSEL3 {
                    OBSSEL3;
                    SEL { SEL }
                    EN { EN }
                }
                OBSSEL4 {
                    OBSSEL4;
                    SEL { SEL }
                    EN { EN }
                }
                OBSSEL5 {
                    OBSSEL5;
                    SEL { SEL }
                    EN { EN }
                }
                OBSSEL6 {
                    OBSSEL6;
                    SEL { SEL }
                    EN { EN }
                }
                OBSSEL7 {
                    OBSSEL7;
                    SEL { SEL }
                    EN { EN }
                }
                TR0 {
                    TR0;
                    ADCTM { ADCTM }
                }
                USBCTRL {
                    USBCTRL;
                    USB_STB { USB_STB }
                }
            }

            ANA_REGS {
                $radio_ana_regs;
                IVCTRL {
                    IVCTRL;
                    PA_BIAS_CTRL { PA_BIAS_CTRL }
                    TXMIX_DC_CTRL { TXMIX_DC_CTRL }
                    LODIV_BIAS_CTRL { LODIV_BIAS_CTRL }
                    DAC_CURR_CTRL { DAC_CURR_CTRL }
                }
            }
        }
    };
}

#[cfg(tisl_mcu = "cc2538")]
map_radio! {
    "Extracts RF Core Register tokens.",
    periph_radio,
    "RF Core peripheral variant.",
    RfCore,
    RFCORE_FFSM,
    RFCORE_XREG,
    RFCORE_SFR,
    CCTEST,
    ANA_REGS,
}
