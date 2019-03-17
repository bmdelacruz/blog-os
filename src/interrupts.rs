use crate::serial_println;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt
  };
}

pub fn init_idt() {
  IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
  stack_frame: &mut InterruptStackFrame
) {
  serial_println!("[BREAKPOINT] {:#?}", stack_frame);
}