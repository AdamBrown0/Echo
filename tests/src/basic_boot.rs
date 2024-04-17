#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(echo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
use kernel::{framebuffer, framebuffer_println};

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    framebuffer_println!("test_println output");
}
