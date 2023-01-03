// don't link the rust standard library
#![no_std]

// disable all rust-level entry points
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Goodbye, world!";

// overwrites the os entry point to our own `_start` impl
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
