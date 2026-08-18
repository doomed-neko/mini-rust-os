#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(brevyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use alloc::boxed::Box;
use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping, entry_point};
use brevyos::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    print, println,
    task::{Task, executor::Executor, keyboard},
};
use core::panic::PanicInfo;
use x86_64::VirtAddr;

extern crate alloc;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

// add a `config` argument to the `entry_point` macro call
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!("Hello World{}", "!");
    brevyos::init();

    let phys_mem_offset = VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("phys mem offset"),
    );
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);
    print!("Welcome to brevyos! / # ");

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use brevyos::hlt_loop;

    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    brevyos::test_panic_handler(info)
}
