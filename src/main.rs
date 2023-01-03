// don't link the rust standard library
#![no_std]

// disable all rust-level entry points
#![no_main]

use core::panic::PanicInfo;

// overwrites the os entry point to our own `_start` impl
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
