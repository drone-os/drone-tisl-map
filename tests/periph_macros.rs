use drone_core::token::Token;
use drone_tisl_map::tisl_reg_tokens;

tisl_reg_tokens! {
    struct Regs;
}

#[test]
#[allow(unused_variables)]
fn perip_macros() {
    let reg = unsafe { Regs::take() };
    #[cfg(all(feature = "uart", any(tisl_mcu = "cc2538")))]
    {
        let uart0 = drone_tisl_map::periph::uart::periph_uart0!(reg);
        let uart1 = drone_tisl_map::periph::uart::periph_uart1!(reg);
    }
}
