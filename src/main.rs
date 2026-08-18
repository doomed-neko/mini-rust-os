use std::env;
use std::process::Command;

fn main() {
    // read env variables that were set in build script
    // let uefi_path = env!("UEFI_PATH");
    // println!("UEFI PATH: {uefi_path}");
    let bios_path = env!("BIOS_PATH");
    println!("BIOS PATH: {bios_path}");
    let mut qemu = Command::new("qemu-system-x86_64");

    qemu.args(["-vga", "std"]);
    qemu.args(["-serial", "mon:stdio"]);
    qemu.args(["-audiodev", "pa,id=speaker"]);
    qemu.args(["-machine", "pcspk-audiodev=speaker"]);
    qemu.args(["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]);

    qemu.arg("-drive")
        .arg(format!("format=raw,file={bios_path}"));

    let mut run = qemu.spawn().expect("failed to start qemu-system-x86_64");

    let status = run.wait().expect("failed to wait for qemu");

    match status.code().unwrap_or(1) {
        0x10 => 0, // success
        0x11 => 1, // failure
        _ => 2,    // unknown fault
    };
}
