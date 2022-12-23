// #![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]
// const PERIPH_BASE: u32 = 0x40000000u32;
// const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x00020000u32;
// const RCC_BASE: u32 = AHBPERIPH_BASE + 0x00001000u32;
// const IOPORT_BASE: u32 = 0x50000000u32;
// const GPIOA_BASE: u32 = IOPORT_BASE + 0x00000000u32;

// Just halt on panic
extern crate panic_halt;

use core::arch::asm;
use cortex_m_rt::{entry, exception};
use stm32g0::stm32g070::{self, CorePeripherals, Peripherals};

use core::cell::RefCell;
use cortex_m::interrupt::{self, CriticalSection, Mutex};

static MY_GPIO: Mutex<RefCell<Option<stm32g070::GPIOA>>> = Mutex::new(RefCell::new(None));

// the program entry point
#[entry]
fn main() -> ! {
    let mut p = stm32g070::Peripherals::take().unwrap();
    init(&mut p);
    interrupt::free(|cs| MY_GPIO.borrow(cs).replace(Some(p.GPIOA)));
    loop {}
}

const CLOCK_SPEED: u32 = 16000000; // 16MHz
const SYSTICK_FRQ: u32 = 1000;
const SYSTICK_LOAD: u32 = (CLOCK_SPEED / SYSTICK_FRQ) - 1;

fn init(p: &mut stm32g070::Peripherals) {
    p.FLASH.acr.write(|w| w.prften().set_bit());
    p.RCC.iopenr.write(|w| w.iopaen().set_bit());
    p.GPIOA.moder.write(|w| w.moder5().output());

    // Systick
    unsafe {
        p.STK.rvr.write(|w| w.bits(SYSTICK_LOAD));
        p.STK.cvr.write(|w| w.bits(0));
    }
    p.STK.csr.write(|w| {
        w.clksource()
            .set_bit()
            .tickint()
            .set_bit()
            .enable()
            .set_bit()
    });
}

static mut TICK_COUNT: u32 = 0;

#[exception]
fn SysTick() {
    let tick_count = unsafe {
        TICK_COUNT += 1;
        TICK_COUNT
    };

    interrupt::free(|cs| {
        let gpioa_tmp = MY_GPIO.borrow(cs).borrow();
        let gpioa = gpioa_tmp.as_ref().unwrap();
        if tick_count % 256 == 0 {
            gpioa.bsrr.write(|w| w.bs5().set_bit())
        } else {
            gpioa.bsrr.write(|w| w.br5().set_bit())
        }
    });
}
// fn pll_init(pll: &mut PLL)
