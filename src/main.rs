#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");

    echo::init();

    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    println!("It didn't crash!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}