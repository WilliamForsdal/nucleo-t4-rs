// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use core::arch::asm;

use rt::entry;

const PERIPH_BASE: u32 = 0x40000000u32;
const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x00020000u32;
const RCC_BASE: u32 = AHBPERIPH_BASE + 0x00001000u32;
const IOPORT_BASE: u32 = 0x50000000u32;
const GPIOA_BASE: u32 = IOPORT_BASE + 0x00000000u32;
// the program entry point
#[entry]
fn main() -> ! {
    unsafe {
        // Enable clock for GPIOA: RCC->IOPENR |= RCC_IOPENR_GPIOAEN
        let iopenr = (RCC_BASE + 0x34) as *mut u32;
        *iopenr |= 1;
        let moder = (GPIOA_BASE + 0x00) as *mut u32;
        let bsrr = (GPIOA_BASE + 0x18) as *mut u32;
        let brr = (GPIOA_BASE + 0x28) as *mut u32;
        *moder = 0xFFFFF7FF; // PA5 output

        loop {
            *bsrr = 1 << 5;
            for _ in 0..1000000 {
                asm!("nop");
            }
            *brr = 1 << 5;
            for _ in 0..1000000 {
                asm!("nop");
            }
        }
    }
}
