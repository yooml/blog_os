#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(blog_os::test_runner)]

use core::panic::PanicInfo;


#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    //init();
    test_main();

    loop {}
}

/*fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}*/

/*#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}*/

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

use blog_os::{println, serial_print, serial_println, init};

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}