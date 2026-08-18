use crate::{inb, outb};

const SPEAKER_PORT: u16 = 0x61;
const SPEAKER_ENABLED: u8 = 0x03;
const SPEAKER_DISABLED: u8 = 0xFC;

pub fn start_sound(frequency: u32) {
    stop_sound();

    let divider = 1193180 / frequency;
    outb(0x43, 0xb6);
    outb(0x42, divider as u8);
    outb(0x42, (divider >> 8) as u8);
    // clk::set_pit_frequency(divider, SPEAKER_CHANNEL);

    let tmp = inb(SPEAKER_PORT);
    if tmp != tmp | SPEAKER_ENABLED {
        outb(SPEAKER_PORT, tmp | SPEAKER_ENABLED);
    }
}

pub fn stop_sound() {
    let tmp = inb(SPEAKER_PORT);
    outb(SPEAKER_PORT, tmp & SPEAKER_DISABLED);
}
