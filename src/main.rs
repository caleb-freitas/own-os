// don't link the rust standard library
#![no_std]

// disable all rust-level entry points
#![no_main]

use core::panic::PanicInfo;

// overwrites the os entry point to our own `_start` impl
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Goodbye, world!");
    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;


#[feature(custom_test_frameworks)]
#[test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
