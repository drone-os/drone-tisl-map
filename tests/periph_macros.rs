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
    #[cfg(all(feature = "tim", any(tisl_mcu = "cc2538")))]
    {
        let gptim0 = drone_tisl_map::periph::tim::periph_gptimer0!(reg);
        let gptim1 = drone_tisl_map::periph::tim::periph_gptimer1!(reg);
        let gptim2 = drone_tisl_map::periph::tim::periph_gptimer2!(reg);
        let gptim3 = drone_tisl_map::periph::tim::periph_gptimer3!(reg);
    }
}
