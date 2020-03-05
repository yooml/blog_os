
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;


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
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

//fn main() {
//}

#[no_mangle]
pub extern "C" fn _start() -> ! {
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
    #[cfg(test)]
        test_main();
    //panic!("Some panic message");
    loop {
        use blog_os::print;
        print!("-");
    }
}


