
pub mod interrupts;
pub mod gdt;
pub mod memory;

extern crate alloc;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}