#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(your_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> () {
    test_main();

    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    your_os::test_panic_handler(info)
}

use your_os::println;

#[test_case]
fn test_println() {
    println!("trest_println output");
}