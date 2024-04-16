#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");
    print!("AAAA");
    print!("BBBB");
    println!("THe number is {} and answer is {}", 42, 1.0/3.0);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}