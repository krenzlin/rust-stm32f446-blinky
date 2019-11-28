#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use crate::hal::prelude::*;

#[rt::entry]
fn main() -> ! {
    if let Some(peripherals) = hal::stm32::Peripherals::take() {
        let gpioa = peripherals.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();
        led.set_high().unwrap();
    }
    loop {}
}
