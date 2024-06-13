#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(echo::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::fmt::Write;
use kernel::{serial_println, framebuffer_print};
use core::panic::PanicInfo;
use bootloader_api::{BootInfo, entry_point};
use kernel::framebuffer::FrameBufferWriter;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init();

    // vga_println!("Hello World{}", "!");
    // vga_print!("AAAA");
    // vga_print!("BBBB");
    // vga_println!("The number is {} and answer is {}", 42, 1.0/3.0);
    // vga_print!("This text should hopefully wrap over multiple lines. The quick brown fox jumps over the lazy dog");

    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let framebuffer_info = framebuffer.info().clone();
    let framebuffer_buffer = framebuffer.buffer_mut();

    let mut writer = FrameBufferWriter::new(framebuffer_buffer, framebuffer_info, [255, 0, 255]);
    writer.write_str("Hello World!\n").expect("TODO: panic message");
    writer.set_color([0, 255, 0]);
    let _ = writer.write_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n");
    writer.set_color([0, 0, 255]);
    writer.write_str("This text is red :)").expect("TODO: panic message");

    framebuffer_print!("Test");
    // #[cfg(test)]
    // test_main();

    kernel::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    serial_println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}