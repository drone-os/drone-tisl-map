use drone_core::token::Token;
use drone_tisl_map::tisl_reg_tokens;

tisl_reg_tokens! {
    struct Regs;
}

#[test]
#[allow(unused_variables)]
fn periph_macros1() {
    let reg = unsafe { Regs::take() };
    #[cfg(all(
        feature = "gpio",
        any(
            tisl_mcu = "cc2538",
        )
    ))]
    {
        let gpio_a = drone_tisl_map::periph::gpio::periph_gpio_a!(reg);
        let gpio_b = drone_tisl_map::periph::gpio::periph_gpio_b!(reg);
        let gpio_c = drone_tisl_map::periph::gpio::periph_gpio_c!(reg);
        let gpio_d = drone_tisl_map::periph::gpio::periph_gpio_d!(reg);
    }
    #[cfg(all(
        feature = "ioc",
        any(
            tisl_mcu = "cc2538",
        )
    ))]
    {
        let ioc_a0 = drone_tisl_map::periph::ioc::periph_ioc_a0!(reg);
        let ioc_a1 = drone_tisl_map::periph::ioc::periph_ioc_a1!(reg);
        let ioc_a2 = drone_tisl_map::periph::ioc::periph_ioc_a2!(reg);
        let ioc_a3 = drone_tisl_map::periph::ioc::periph_ioc_a3!(reg);
        let ioc_a4 = drone_tisl_map::periph::ioc::periph_ioc_a4!(reg);
        let ioc_a5 = drone_tisl_map::periph::ioc::periph_ioc_a5!(reg);
        let ioc_a6 = drone_tisl_map::periph::ioc::periph_ioc_a6!(reg);
        let ioc_a7 = drone_tisl_map::periph::ioc::periph_ioc_a7!(reg);
        let ioc_b0 = drone_tisl_map::periph::ioc::periph_ioc_b0!(reg);
        let ioc_b1 = drone_tisl_map::periph::ioc::periph_ioc_b1!(reg);
        let ioc_b2 = drone_tisl_map::periph::ioc::periph_ioc_b2!(reg);
        let ioc_b3 = drone_tisl_map::periph::ioc::periph_ioc_b3!(reg);
        let ioc_b4 = drone_tisl_map::periph::ioc::periph_ioc_b4!(reg);
        let ioc_b5 = drone_tisl_map::periph::ioc::periph_ioc_b5!(reg);
        let ioc_b6 = drone_tisl_map::periph::ioc::periph_ioc_b6!(reg);
        let ioc_b7 = drone_tisl_map::periph::ioc::periph_ioc_b7!(reg);
        let ioc_c0 = drone_tisl_map::periph::ioc::periph_ioc_c0!(reg);
        let ioc_c1 = drone_tisl_map::periph::ioc::periph_ioc_c1!(reg);
        let ioc_c2 = drone_tisl_map::periph::ioc::periph_ioc_c2!(reg);
        let ioc_c3 = drone_tisl_map::periph::ioc::periph_ioc_c3!(reg);
        let ioc_c4 = drone_tisl_map::periph::ioc::periph_ioc_c4!(reg);
        let ioc_c5 = drone_tisl_map::periph::ioc::periph_ioc_c5!(reg);
        let ioc_c6 = drone_tisl_map::periph::ioc::periph_ioc_c6!(reg);
        let ioc_c7 = drone_tisl_map::periph::ioc::periph_ioc_c7!(reg);
        let ioc_d0 = drone_tisl_map::periph::ioc::periph_ioc_d0!(reg);
        let ioc_d1 = drone_tisl_map::periph::ioc::periph_ioc_d1!(reg);
        let ioc_d2 = drone_tisl_map::periph::ioc::periph_ioc_d2!(reg);
        let ioc_d3 = drone_tisl_map::periph::ioc::periph_ioc_d3!(reg);
        let ioc_d4 = drone_tisl_map::periph::ioc::periph_ioc_d4!(reg);
        let ioc_d5 = drone_tisl_map::periph::ioc::periph_ioc_d5!(reg);
        let ioc_d6 = drone_tisl_map::periph::ioc::periph_ioc_d6!(reg);
        let ioc_d7 = drone_tisl_map::periph::ioc::periph_ioc_d7!(reg);
        let ioc_selectors = drone_tisl_map::periph::ioc::periph_ioc_selectors!(reg);
    }
    #[cfg(all(
        feature = "uart",
        any(
            tisl_mcu = "cc2538",
        )
    ))]
    {
        let uart0 = drone_tisl_map::periph::uart::periph_uart0!(reg);
        let uart1 = drone_tisl_map::periph::uart::periph_uart1!(reg);
    }
     #[cfg(all(
        feature = "tim",
        any(
            tisl_mcu = "cc2538",
        )
    ))]
    {
        let tim0 = drone_tisl_map::periph::tim::periph_gptimer0!(reg);
        let tim1 = drone_tisl_map::periph::tim::periph_gptimer1!(reg);
        let tim2 = drone_tisl_map::periph::tim::periph_gptimer2!(reg);
        let tim3 = drone_tisl_map::periph::tim::periph_gptimer3!(reg);
    }
    #[cfg(all(
        feature = "sysctrl",
        any(
            tisl_mcu = "cc2538",
        )
    ))]
    {
        let sysctrl = drone_tisl_map::periph::sysctrl::periph_sysctrl!(reg);
    }
    #[cfg(all(
        feature = "radio",
        any(
            tisl_mcu = "cc2538",
        )
    ))]
    {
        let radio = drone_tisl_map::periph::radio::periph_radio!(reg);
    }

}
