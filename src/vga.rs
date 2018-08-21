use core::fmt;

use volatile::Volatile;
use spin::Mutex;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


#[allow(dead_code)]
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        let code = (background as u8) << 4 | foreground as u8;
        ColorCode(code)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

type VgaBuffer = [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT];

pub struct Writer {
    color_code: ColorCode,
    buffer: &'static mut VgaBuffer,
    col: usize,
    row: usize,
}


impl Writer {
    pub fn write_byte(&mut self, ascii_char: u8) {
        match ascii_char {
            b'\n' => self.newline(),
            byte => {
                let ch = ScreenChar { ascii_character: byte, color_code: self.color_code };
                self.buffer[self.row][self.col].write(ch);

                self.col += 1;
                if self.col >= BUFFER_WIDTH {
                    self.newline();
                }
            }
        }
    }

    pub fn newline(&mut self) {
        self.col = 0;

        if self.row >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let ch = self.buffer[row][col].read();
                    self.buffer[row - 1][col].write(ch);
                }
            }

            for col in 0..BUFFER_WIDTH {
                let space = ScreenChar { ascii_character: b' ', color_code: self.color_code };
                self.buffer[BUFFER_HEIGHT - 1][col].write(space);
            }
        } else {
            self.row += 1;
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.write_byte(*byte);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}

impl Default for Writer {
    fn default() -> Self {
        let buffer_addr = 0xb8000 as *mut VgaBuffer;
        Writer {
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *buffer_addr },
            col: 0,
            row: 0
        }
    }
}


lazy_static! {
    pub static ref VGA_BUFFER: Mutex<Writer> = Mutex::new(Writer::default());
}

pub fn print(args: fmt::Arguments) {
    use ::core::fmt::Write;
    VGA_BUFFER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! vga_print {
    ($($args:expr),*) => {{
        $crate::vga::print(format_args!($($args),*));
    }};
}

#[macro_export]
macro_rules! vga_println {
    () => { vga_print!("\n") };
    ($fmt:expr) => { vga_print!(concat!($fmt, "\n")); };
    ($fmt:expr, $($args:expr),*) => { vga_print!(concat!($fmt, "\n"), $($args),*); };
}



#[cfg(test)]
mod test {
    use super::*;
    use super::Color::*;
    use array_init::array_init;
    use volatile::Volatile;
    use std::boxed::Box;

    fn empty_char() -> ScreenChar {
        let color_code = ColorCode::new(White, Black);
        ScreenChar { ascii_character: b' ', color_code }
    }

    fn make_writer() -> Writer {
        let color_code = ColorCode::new(White, Black);
        let space = empty_char();
        let buffer: VgaBuffer = array_init(|_| {
            array_init(|_|Volatile::new(space))
        });
        let buffer: &'static mut VgaBuffer = Box::leak(Box::new(buffer));

        Writer {
            color_code,
            buffer,
            col: 0,
            row: 0,
        }
    }

    #[test]
    fn write_byte() {
        let mut writer = make_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == 0 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == 0 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn write_formatted() {
        use core::fmt::Write;

        let mut writer = make_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "b{}", "c").unwrap();

        for (i, row) in writer.buffer.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == 0 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == 1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'c');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
}