#![no_main]
#![no_std]


extern crate panic_halt;


use cortex_m_rt::entry;


use lpc8xx_hal::{
    prelude::*,
    Peripherals,
};


#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();

    let swm = p.SWM.split();

    let gpio_enabled = {
        let mut syscon = p.SYSCON.split();
        p.GPIO.enable(&mut syscon.handle)
    };

    let mut red_led = swm.pins.pio1_2
        .into_gpio_pin(&gpio_enabled)
        .into_output();

    let mut blue_led = swm.pins.pio1_1
        .into_gpio_pin(&gpio_enabled)
        .into_output();

    let mut green_led = swm.pins.pio1_0
        .into_gpio_pin(&gpio_enabled)
        .into_output();

    red_led.set_low();
    blue_led.set_low();
    // green_led.set_low();

    let mut red_on = true;
    loop {
        green_led.set_low();
        for _ in 0..10_000 {
        }
        green_led.set_high();
        for _ in 0..10_000 {
        }
        if red_on {
            red_led.set_high();
        } else {
            red_led.set_low();
        }
        red_on = !red_on;

    }
}
