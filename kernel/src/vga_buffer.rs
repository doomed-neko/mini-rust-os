use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Magenta, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        // we shift the background 4 bits to left and OR
        // it with foreground to produce the complete bit
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_code: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn set_color(&mut self, color: Color) {
        self.color_code = ColorCode::new(color, Color::Black);
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_code: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn backspace(&mut self) {
        let row = BUFFER_HEIGHT - 1;
        let col = self.column_position - 1;
        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar {
            ascii_code: 0x0,
            color_code,
        });
        self.column_position -= 1;
    }

    pub fn clear(&mut self) {
        self.buffer.chars = unsafe { core::mem::zeroed() };
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.as_bytes() {
            match byte {
                0x20..0x7e | b'\n' => self.write_byte(*byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_code: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! back_space {
    () => {
        $crate::vga_buffer::_backspace()
    };
}

#[doc(hidden)]
pub fn _print(_args: fmt::Arguments) {
    // use core::fmt::Write;
    // x86_64::instructions::interrupts::without_interrupts(|| {
    //     WRITER.lock().write_fmt(args).unwrap();
    // });
}

#[test_case]
fn test_one_println() {
    println!("test_println_simple output");
}

#[test_case]
fn test_many_println() {
    for _ in 0..200 {
        println!("test_println_simple output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    let s = "Test string";
    x86_64::instructions::interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{s}").expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let schar = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(c, char::from(schar.ascii_code));
        }
    })
}
