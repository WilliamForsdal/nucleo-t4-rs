// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_halt;

use core::arch::asm;
use cortex_m_rt::entry;
use stm32g0::stm32g070;

// const PERIPH_BASE: u32 = 0x40000000u32;
// const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x00020000u32;
// const RCC_BASE: u32 = AHBPERIPH_BASE + 0x00001000u32;
// const IOPORT_BASE: u32 = 0x50000000u32;
// const GPIOA_BASE: u32 = IOPORT_BASE + 0x00000000u32;
// the program entry point
#[entry]
fn main() -> ! {
    let p = stm32g070::Peripherals::take().unwrap();
    // Enable clock for GPIOA
    p.RCC.iopenr.write(|w| w.iopaen().set_bit());



    // Set PA5 as output
    p.GPIOA.moder.write(|w| w.moder5().output());

    loop {
        // unsafe {
        //     *bsrr = 1 << 5;
        // }
        p.GPIOA.bsrr.write(|w| w.bs5().set_bit());
        for _ in 0..1000000 {
            unsafe {
                asm!("nop");
            }
        }

        p.GPIOA.bsrr.write(|w| w.br5().set_bit());

        for _ in 0..1000000 {
            unsafe {
                asm!("nop");
            }
        }
    }
}
