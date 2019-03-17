use x86_64::PhysAddr;
use x86_64::VirtAddr;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{
  FrameAllocator, Mapper, Page, PageTable, PageTableFlags as Flags,
  PhysFrame, Size4KiB, MapperAllSizes, MappedPageTable,
};

pub unsafe fn init(physical_memory_offset: u64) -> impl MapperAllSizes {
  let level_4_table = active_level_4_table(physical_memory_offset);
  let phys_to_virt = move |frame: PhysFrame| -> *mut PageTable {
    let phys = frame.start_address().as_u64();
    let virt = VirtAddr::new(phys + physical_memory_offset);
    virt.as_mut_ptr()
  };
  MappedPageTable::new(level_4_table, phys_to_virt)
}

unsafe fn active_level_4_table(
  physical_memory_offset: u64
) -> &'static mut PageTable {
  let (level_4_table_frame, _) = Cr3::read();
  let physical_addr = level_4_table_frame.start_address();
  let virtual_addr = VirtAddr::new(
    physical_addr.as_u64() + physical_memory_offset
  );
  let page_table_ptr: *mut PageTable = virtual_addr.as_mut_ptr();
  &mut *page_table_ptr
}

pub fn create_example_mapping(
  page: Page, mapper: &mut impl Mapper<Size4KiB>,
  frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
  let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
  let flags = Flags::PRESENT | Flags::WRITABLE;
  let map_to_result = unsafe {
    mapper.map_to(page, frame, flags, frame_allocator)
  };
  map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
  fn allocate_frame(&mut self) -> Option<PhysFrame> {
    None
  }
}