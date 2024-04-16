#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(echo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use echo::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    echo::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
