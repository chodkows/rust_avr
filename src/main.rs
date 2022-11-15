#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};

#[no_mangle]
pub extern fn main() {
    port::D6::set_output();
    port::D6::set_high();

    port::D7::set_output();
    port::B0::set_output();

    loop {
        port::D7::set_low();
        port::B0::set_high();
        ruduino::delay::delay_ms(3000);
        port::D7::set_high();
        port::B0::set_low();
        ruduino::delay::delay_ms(1000);
    }
}
