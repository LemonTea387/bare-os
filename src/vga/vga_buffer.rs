use core::fmt;

use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const VGA_TEXT_BUFFER_MEM: usize = 0xb8000;

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
// Ensure struct has same data layout as u8
#[repr(transparent)]
struct VGAColorCode(u8);
impl VGAColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Field ordering in rust is undefined
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: VGAColorCode,
}

#[repr(transparent)]
struct ScreenBuffer {
    /// Has to be volatile to make sure it's not optimized away
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct ScreenWriter {
    column_position: usize,
    color_code: VGAColorCode,
    /// Since the buffer is a mem mapped addr, it is always avail.
    buffer: &'static mut ScreenBuffer,
}
impl Default for ScreenWriter {
    fn default() -> Self {
        ScreenWriter {
            column_position: 0,
            color_code: VGAColorCode::new(Color::White, Color::Black),
            // buffer: unsafe { &mut *(VGA_TEXT_BUFFER_MEM as *mut ScreenBuffer) },
            buffer: unsafe { &mut *(VGA_TEXT_BUFFER_MEM as *mut ScreenBuffer) },
        }
    }
}

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl ScreenWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
    fn new_line(&mut self) {
        self.push_rows();
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn push_rows(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        assert!(row < BUFFER_HEIGHT);
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank)
        }
    }
}
