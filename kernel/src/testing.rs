#![allow(dead_code)]

use crate::{serial_print, serial_println};
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testble]) {
    use crate::serial_println;

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testble {
    fn run(&self);
}

impl<T> Testble for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("Running {}\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
