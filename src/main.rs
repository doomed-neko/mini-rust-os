#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(brevyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use brevyos::{hlt_loop, print, println};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("{:?}", boot_info);
    println!("Hello World{}", "!");

    brevyos::init();

    print!("Welcome to BrevyOS / #");

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
