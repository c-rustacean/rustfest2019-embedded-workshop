#![no_main]
#![no_std]


extern crate panic_halt;


use cortex_m_rt::entry;


use lpc8xx_hal::Peripherals;


#[entry]
fn main() -> ! {
    let _p = Peripherals::take().unwrap();

    loop {}
}
