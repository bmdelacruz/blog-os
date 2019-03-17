use crate::gdt;
use crate::serial_println;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint
      .set_handler_fn(breakpoint_handler);
    
    unsafe {
      idt.double_fault
        .set_handler_fn(double_fault_handler)
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }

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

extern "x86-interrupt" fn double_fault_handler(
  stack_frame: &mut InterruptStackFrame, error_code: u64
) {
  serial_println!("[DOUBLE_FAULT:{}] {:#?}", error_code, stack_frame);
  loop {}
}