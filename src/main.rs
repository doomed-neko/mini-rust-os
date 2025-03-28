#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(brevyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use brevyos::{hlt_loop, init, println};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    init();

    println!("It did not crash!");

    #[cfg(test)]
    test_main();

    hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    brevyos::test_panic_handler(info)
}
