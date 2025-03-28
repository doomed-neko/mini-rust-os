#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(brevyos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

use brevyos::println;
use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    brevyos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
