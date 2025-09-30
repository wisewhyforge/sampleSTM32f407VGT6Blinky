#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Print panic message to probe console
use panic_probe as _;


use cortex_m_rt::entry;
use stm32f4xx_hal::{
    pac,
    prelude::*,
};

#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let gpioc = p.GPIOC.split(&mut rcc);
    let gpiod = p.GPIOD.split(&mut rcc);
    let mut led = gpiod.pd13.into_push_pull_output();

    loop {
        for _ in 0..100_000 {
            led.set_high();
        }
        for _ in 0..100_000 {
            led.set_low();
        }
    }
}
