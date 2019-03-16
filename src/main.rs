#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use blog_os::println;
use blog_os::serial_println;

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
  println!("system started up");
  serial_println!("system started up");

  blog_os::interrupts::init_idt();

  x86_64::instructions::interrupts::int3();

  println!("it did not fucking crash!!");

  loop {}
}