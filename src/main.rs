#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(echo::test_runner)]
#![reexport_test_harness_main = "test_main"]

use echo::print;
use echo::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");
    print!("AAAA");
    print!("BBBB");
    println!("The number is {} and answer is {}", 42, 1.0/3.0);

    echo::init();

    fn stack_overflow() {
        stack_overflow();
    }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    echo::test_panic_handler(info);
}