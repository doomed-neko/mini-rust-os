#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader_api::{BootInfo, BootloaderConfig, config::Mapping};
use core::panic::PanicInfo;
use embedded_graphics::{
    image::Image,
    mono_font::{MonoTextStyle, ascii::FONT_9X18_BOLD},
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use kernel::{
    framebuffer::{self, Color, Position, set_pixel_in},
    hlt_loop, println,
};
use tinytga::Tga;

extern crate alloc;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.mappings.framebuffer = Mapping::Dynamic;
    config
};
bootloader_api::entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let height = info.height;
        let width = info.width;
        for y in 0..height {
            for x in 0..width {
                set_pixel_in(framebuffer, Position::new(x, y), Color::new(30, 30, 46));
            }
        }
        let mut display = framebuffer::Display::new(framebuffer);
        let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb888::new(189, 147, 249));
        Text::new("HELL FUCKING YEAHAAAA", Point::new(20, 20), style)
            .draw(&mut display)
            .ok();
        let image = include_bytes!("../assets/rust.tga");
        let image: Tga<Rgb888> = Tga::from_slice(image).unwrap();
        let image = Image::new(&image, Point::new(width as i32 / 2, height as i32 / 2));
        image.draw(&mut display).ok();
    }
    // kernel::init();
    //
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    //
    // allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    //
    // // allocate a number on the heap
    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);
    // print!("Welcome to kernel! / # ");

    #[cfg(test)]
    test_main();

    hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use kernel::println;

    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
