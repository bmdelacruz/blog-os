#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
  print!("The big brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet consectetur adipiscing elit. ");
  serial_print!("The big brown fox jumps over the lazy dog. Lorem ipsum dolor sit amet consectetur adipiscing elit.");

  loop {}
}