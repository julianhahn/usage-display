#![no_std]
#![no_main]

use core::panic::PanicInfo;

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[esp_hal::main]
fn main() -> ! {
    loop {}
}
