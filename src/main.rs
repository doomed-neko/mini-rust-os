#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use ps2::Controller;
#[cfg(test)]
use testing::{exit_qemu, QemuExitCode};
use text::constants::*;
use vga_buffer::WRITER;

mod serial;
mod testing;
mod text;
mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    let mut is_ctrl_down: bool = false;
    let mut controller = unsafe { Controller::new() };
    {
        let _ = (controller.disable_mouse(), controller.read_data());
    }

    print!("Welcome to pasta_os! $");
    #[allow(clippy::empty_loop)]
    loop {
        if let Ok(byte) = controller.read_data() {
            match byte {
                // x if (x >> 7) & 1 == 1 => (),
                // x if x >= 30 || x < 44 => print!("{}", (x + 67) as char),
                A => print!("a"),
                B => print!("b"),
                C => print!("c"),
                D => print!("d"),
                E => print!("e"),
                F => print!("f"),
                G => print!("g"),
                H => print!("h"),
                I => print!("i"),
                J => print!("j"),
                K => print!("k"),
                L => print!("l"),
                M => print!("m"),
                N => print!("n"),
                O => print!("o"),
                P => print!("p"),
                Q => print!("q"),
                R => print!("r"),
                S => print!("s"),
                T => print!("t"),
                U => print!("u"),
                V => print!("v"),
                W => print!("w"),
                X => print!("x"),
                Y => print!("y"),
                Z => print!("z"),
                ENTER => println!(),
                SPACE => print!(" "),
                BACKSLASH => print!("\\"),
                SLASH => print!("/"),
                PERIOD => print!("."),
                SEMICOLON => print!(";"),
                COMMA => print!(","),
                EQUALS => print!("="),
                MINUS => print!("-"),
                ONE if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::LightGray),
                TWO if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Blue),
                THREE if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Green),
                FOUR if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Cyan),
                FIVE if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Red),
                SIX if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Magenta),
                SEVEN if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Brown),
                EIGHT if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Pink),
                NINE if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::Yellow),
                ZERO if is_ctrl_down => WRITER.lock().set_color(vga_buffer::Color::White),
                ONE => print!("1"),
                TWO => print!("2"),
                THREE => print!("3"),
                FOUR => print!("4"),
                FIVE => print!("5"),
                SIX => print!("6"),
                SEVEN => print!("7"),
                EIGHT => print!("8"),
                NINE => print!("9"),
                ZERO => print!("0"),
                BACKSPACE => back_space!(),
                LCTRL_DOWN => is_ctrl_down = true,
                LCTRL_UP => is_ctrl_down = false,
                _ => (),
            }
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn test_func() {
    assert_ne!(1, 2);
}
