#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(your_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::{self, PanicInfo};
use your_os::println;


static HELLO: &[u8] = b"Hello World!";
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    your_os::test_panic_handler(info)
}
