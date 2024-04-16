#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    echo::init();

    println!("Hello World{}", "!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}