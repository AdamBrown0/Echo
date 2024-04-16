#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");

    echo::init();
    x86_64::instructions::interrupts::int3();

    println!("It didn't crash!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}