use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
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
  fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
  ascii_character: u8,
  color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
  chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
  row_position: usize,
  column_position: usize,
  has_pending_new_line: bool,
  color_code: ColorCode,
  buffer: &'static mut Buffer,
}

impl Writer {
  pub fn write_string(&mut self, s: &str) {
    for byte in s.bytes() {
      match byte {
        0x20...0x7e | b'\n' => self.write_byte(byte),
        _ => self.write_byte(0xfe),
      }
    }
  }

  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.postpone_new_line(),
      byte => {
        if self.has_pending_new_line || self.column_position >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = self.row_position;
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

  fn postpone_new_line(&mut self) {
    if self.has_pending_new_line {
      self.new_line();
    }
    self.has_pending_new_line = true;
  }

  fn new_line(&mut self) {
    if self.row_position >= BUFFER_HEIGHT - 1 {
      for row in 1..BUFFER_HEIGHT {
        for col in 0..BUFFER_WIDTH {
          let character = self.buffer.chars[row][col].read();
          self.buffer.chars[row - 1][col].write(character);
        }
      }
      self.clear_row(BUFFER_HEIGHT - 1);
    } else {
      self.row_position += 1;
    }

    self.column_position = 0;
    self.has_pending_new_line = false;
  }

  fn clear_row(&mut self, row: usize) {
    for col in 0..BUFFER_WIDTH {
      self.buffer.chars[row][col].write(ScreenChar {
        ascii_character: b' ',
        color_code: self.color_code,
      });
    }
  }
}

impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);

    Ok(())
  }
}

lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    row_position: 0,
    column_position: 0,
    has_pending_new_line: false,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
  use core::fmt::Write;
  WRITER.lock().write_fmt(args).unwrap();
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