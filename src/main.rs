#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(brevyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use brevyos::{
    allocator, hlt_loop,
    memory::{self, BootInfoFrameAllocator},
    print, println,
};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    brevyos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);
    print!("Welcome to brevyos! / # ");

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
