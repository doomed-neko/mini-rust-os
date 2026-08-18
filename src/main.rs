use std::env;

fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    println!("BIOS PATH: {bios_path}");
    println!("UEFI PATH: {uefi_path}");
}
