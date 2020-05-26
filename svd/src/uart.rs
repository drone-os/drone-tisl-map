//! UART peripheral patches.

use anyhow::Result;
use drone_svd::{Access, Device};

/// Remove reserved fields, because these fields have no purpose.
pub fn remove_reserved(dev: &mut Device) -> Result<()> {
    dev.periph("UART0").reg("CC").remove_field("Reserved29");
    dev.periph("UART0").reg("PP").remove_field("Reserved30");
    dev.periph("UART0").reg("PP").remove_field("SC");
    dev.periph("UART0").reg("NINEBITAMASK").remove_field("Reserved16");
    dev.periph("UART0").reg("NINEBITADDR").remove_field("Reserved16");
    dev.periph("UART0").reg("NINEBITADDR").remove_field("Reserved7");
    dev.periph("UART0").reg("LTIM").remove_field("Reserved16");
    dev.periph("UART0").reg("LSS").remove_field("Reserved16");
    dev.periph("UART0").reg("LCTL").remove_field("Reserved3");
    dev.periph("UART0").reg("LCTL").remove_field("Reserved26");
    dev.periph("UART0").reg("DMACTL").remove_field("Reserved29");
    dev.periph("UART0").reg("ICR").remove_field("Reserved1");
    dev.periph("UART0").reg("ICR").remove_field("Reserved4");
    dev.periph("UART0").reg("ICR").remove_field("Reserved16");
    dev.periph("UART0").reg("MIS").remove_field("Reserved1");
    dev.periph("UART0").reg("MIS").remove_field("Reserved4");
    dev.periph("UART0").reg("MIS").remove_field("Reserved16");
    dev.periph("UART0").reg("RIS").remove_field("Reserved1");
    dev.periph("UART0").reg("RIS").remove_field("Reserved4");
    dev.periph("UART0").reg("RIS").remove_field("Reserved16");
    dev.periph("UART0").reg("IM").remove_field("Reserved1");
    dev.periph("UART0").reg("IM").remove_field("Reserved4");
    dev.periph("UART0").reg("IM").remove_field("Reserved16");
    dev.periph("UART0").reg("IFLS").remove_field("Reserved26");
    dev.periph("UART0").reg("CTL").remove_field("Reserved1");
    dev.periph("UART0").reg("CTL").remove_field("Reserved4");
    dev.periph("UART0").reg("CTL").remove_field("Reserved6");
    dev.periph("UART0").reg("LCRH").remove_field("Reserved24");
    dev.periph("UART0").reg("FBRD").remove_field("Reserved26");
    dev.periph("UART0").reg("IBRD").remove_field("Reserved16");
    dev.periph("UART0").reg("ILPR").remove_field("Reserved24");
    dev.periph("UART0").reg("FR").remove_field("Reserved2");
    dev.periph("UART0").reg("FR").remove_field("Reserved24");
    dev.periph("UART0").reg("ECR").remove_field("Reserved24");
    dev.periph("UART0").reg("RSR").remove_field("Reserved28");
    dev.periph("UART0").reg("DR").remove_field("Reserved20");

    dev.periph("UART1").reg("CC").remove_field("Reserved29");
    dev.periph("UART1").reg("PP").remove_field("Reserved30");
    dev.periph("UART1").reg("PP").remove_field("SC");
    dev.periph("UART1").reg("NINEBITAMASK").remove_field("Reserved16");
    dev.periph("UART1").reg("NINEBITADDR").remove_field("Reserved16");
    dev.periph("UART1").reg("NINEBITADDR").remove_field("Reserved7");
    dev.periph("UART1").reg("LTIM").remove_field("Reserved16");
    dev.periph("UART1").reg("LSS").remove_field("Reserved16");
    dev.periph("UART1").reg("LCTL").remove_field("Reserved3");
    dev.periph("UART1").reg("LCTL").remove_field("Reserved26");
    dev.periph("UART1").reg("DMACTL").remove_field("Reserved29");
    dev.periph("UART1").reg("ICR").remove_field("Reserved1");
    dev.periph("UART1").reg("ICR").remove_field("Reserved4");
    dev.periph("UART1").reg("ICR").remove_field("Reserved16");
    dev.periph("UART1").reg("MIS").remove_field("Reserved1");
    dev.periph("UART1").reg("MIS").remove_field("Reserved4");
    dev.periph("UART1").reg("MIS").remove_field("Reserved16");
    dev.periph("UART1").reg("RIS").remove_field("Reserved1");
    dev.periph("UART1").reg("RIS").remove_field("Reserved4");
    dev.periph("UART1").reg("RIS").remove_field("Reserved16");
    dev.periph("UART1").reg("IM").remove_field("Reserved1");
    dev.periph("UART1").reg("IM").remove_field("Reserved4");
    dev.periph("UART1").reg("IM").remove_field("Reserved16");
    dev.periph("UART1").reg("IFLS").remove_field("Reserved26");
    dev.periph("UART1").reg("CTL").remove_field("Reserved1");
    dev.periph("UART1").reg("CTL").remove_field("Reserved4");
    dev.periph("UART1").reg("CTL").remove_field("Reserved6");
    dev.periph("UART1").reg("LCRH").remove_field("Reserved24");
    dev.periph("UART1").reg("FBRD").remove_field("Reserved26");
    dev.periph("UART1").reg("IBRD").remove_field("Reserved16");
    dev.periph("UART1").reg("ILPR").remove_field("Reserved24");
    dev.periph("UART1").reg("FR").remove_field("Reserved2");
    dev.periph("UART1").reg("FR").remove_field("Reserved24");
    dev.periph("UART1").reg("ECR").remove_field("Reserved24");
    dev.periph("UART1").reg("RSR").remove_field("Reserved28");
    dev.periph("UART1").reg("DR").remove_field("Reserved20");

    dev.periph("UART0").reg("RSR").access = Some(Access::ReadOnly);
    dev.periph("UART0").reg("ECR").access = Some(Access::WriteOnly);
    dev.periph("UART0").reg("FR").access = Some(Access::ReadOnly);
    dev.periph("UART0").reg("RIS").access = Some(Access::ReadOnly);
    dev.periph("UART0").reg("MIS").access = Some(Access::ReadOnly);
    dev.periph("UART0").reg("ICR").access = Some(Access::WriteOnly);
    dev.periph("UART0").reg("LSS").access = Some(Access::ReadOnly);
    dev.periph("UART0").reg("LTIM").access = Some(Access::ReadOnly);
    dev.periph("UART0").reg("PP").access = Some(Access::ReadOnly);

    dev.periph("UART1").reg("RSR").access = Some(Access::ReadOnly);
    dev.periph("UART1").reg("ECR").access = Some(Access::WriteOnly);
    dev.periph("UART1").reg("FR").access = Some(Access::ReadOnly);
    dev.periph("UART1").reg("RIS").access = Some(Access::ReadOnly);
    dev.periph("UART1").reg("MIS").access = Some(Access::ReadOnly);
    dev.periph("UART1").reg("ICR").access = Some(Access::WriteOnly);
    dev.periph("UART1").reg("LSS").access = Some(Access::ReadOnly);
    dev.periph("UART1").reg("LTIM").access = Some(Access::ReadOnly);
    dev.periph("UART1").reg("PP").access = Some(Access::ReadOnly);

    Ok(())
}
