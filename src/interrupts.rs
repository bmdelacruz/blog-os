use crate::gdt;
use crate::hlt_loop;
use crate::print;
use crate::serial_println;
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::PageFaultErrorCode;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    
    unsafe {
      idt.double_fault
        .set_handler_fn(double_fault_handler)
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt[InterruptIndex::Timer.as_usize()]
      .set_handler_fn(timer_interrupt_handler);
    idt[InterruptIndex::Keyboard.as_usize()]
      .set_handler_fn(keyboard_interrupt_handler);

    idt
  };
}

pub fn init_idt() {
  IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
  stack_frame: &mut InterruptStackFrame
) {
  serial_println!("[BREAKPOINT] {:?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
  stack_frame: &mut InterruptStackFrame, error_code: u64
) {
  serial_println!("[DOUBLE_FAULT:{}] {:?}", error_code, stack_frame);

  hlt_loop();
}

extern "x86-interrupt" fn page_fault_handler(
  stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode
) {
  use x86_64::registers::control::Cr2;

  serial_println!(
    "[PAGE_FAULT:{:?}:{:?}] {:?}", Cr2::read(), error_code, stack_frame
  );

  hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(
  _stack_frame: &mut InterruptStackFrame
) {
  unsafe {
    PICS.lock().notify_end_of_interrupt(
      InterruptIndex::Timer.as_u8(),
    );
  }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
  _stack_frame: &mut InterruptStackFrame
) {
  use pc_keyboard::Keyboard;
  use pc_keyboard::ScancodeSet1;
  use pc_keyboard::DecodedKey;
  use pc_keyboard::layouts;
  use spin::Mutex;
  use x86_64::instructions::port::Port;

  lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
      Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
  }

  let mut keyboard = KEYBOARD.lock();
  let port = Port::new(0x60);
  let scancode: u8 = unsafe { port.read() };
  
  if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
    if let Some(key) = keyboard.process_keyevent(key_event) {
      match key {
        DecodedKey::Unicode(character) => print!("{}", character),
        DecodedKey::RawKey(key) => print!("{:?}", key),
      }
    }
  }

  unsafe {
    PICS.lock().notify_end_of_interrupt(
      InterruptIndex::Keyboard.as_u8(),
    );
  }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {
  ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
});

pub fn init_pics() {
  unsafe {
    PICS.lock().initialize();
  }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
  Timer = PIC_1_OFFSET,
  Keyboard,
}

impl InterruptIndex {
  fn as_u8(self) -> u8 {
    self as u8
  }

  fn as_usize(self) -> usize {
    usize::from(self.as_u8())
  }
}

pub fn enable_interrupts() {
  x86_64::instructions::interrupts::enable();
}