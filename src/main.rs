#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use cortex_m_rt;

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
