
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

extern crate alloc;

//use alloc::boxed::Box;
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    //use blog_os::memory::active_level_4_table;
    use blog_os::memory::translate_addr;
    use blog_os::memory;
    use x86_64::{structures::paging::MapperAllSizes, VirtAddr};
    use x86_64::{structures::paging::Page};
    use blog_os::memory::BootInfoFrameAllocator;
    use blog_os::allocator;


    println!("Hello World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    //let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    // as before

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    //let x = Box::new(41);
    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
        test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}

/*#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}*/

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    //loop {}
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

//fn main() {
//}

/*
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    blog_os::init();

    /*fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3(); // new
    */
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    let ptr = 0x2031b2 as *mut u32;

// read from a code page
    unsafe { let x = *ptr; }
    println!("read worked");

// write to a code page
    unsafe { *ptr = 42; }
    println!("write worked");
    #[cfg(test)]
        test_main();
    //panic!("Some panic message");
    /*loop {
        use blog_os::print;
        print!("-");
    }*/
    println!("It did not crash!");
    blog_os::hlt_loop();
}

*/