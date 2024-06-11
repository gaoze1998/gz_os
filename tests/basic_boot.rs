#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(gz_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gz_os::println;

#[test_case]
fn test_println_many() {
    println!("test_println_many output");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    gz_os::test_panic_handler(info);
}