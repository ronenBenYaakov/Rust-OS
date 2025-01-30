#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod vga_buffer;
mod libs;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::{Page, Translate};
use x86_64::VirtAddr;
use crate::libs::memory;
use crate::libs::memory::BootInfoFrameAllocator;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");
    libs::init();

    let phys_mem_offset = VirtAddr::new(boot_info.memory_map[0].range.start_frame_number);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);



    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    let addresses = [
        //the identity mapped VGA buffer
        0xb8000,
        //some code page
        0x201008,
        //some stack page
        0x0100_0020_1a10,
        //virtual address mapped to physical address 0
        boot_info.memory_map[0].range.start_frame_number
    ];

    /*for &address in &addresses {
        let virt = VirtAddr::new(address);
        // new: use the `mapper.translate_addr` method
        let phys = mapper.translate_addr(virt).unwrap();
        println!("{:?} -> {:?}", virt, phys);
    }*/


    /*let phys_mem_offset = VirtAddr::new(boot_info.memory_map[0].range.start_frame_number);



    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }*/

    /*let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.memory_map[0].range.start_frame_number;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe{ &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
     */
    loop {}
}

