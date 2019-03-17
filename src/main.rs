#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use blog_os::println;
use blog_os::serial_println;
use bootloader::BootInfo;
use bootloader::entry_point;
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  
  blog_os::hlt_loop();
}

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
  blog_os::gdt::init();
  blog_os::interrupts::init_idt();
  blog_os::interrupts::init_pics();
  blog_os::interrupts::enable_interrupts();

  let mut mapper = unsafe {
    blog_os::memory::init(boot_info.physical_memory_offset)
  };
  let mut frame_allocator = blog_os::memory::EmptyFrameAllocator;

  let page = Page::containing_address(VirtAddr::new(0xdea_dbee_f000));
  blog_os::memory::create_example_mapping(
    page, &mut mapper, &mut frame_allocator
  );
  let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
  unsafe {
    page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e)
  };

  blog_os::hlt_loop();
}