use crate::println;
use crate::gdt;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(interrupt_stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", interrupt_stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(interrupt_stack_frame: InterruptStackFrame, _error_code: u64) -> !{
    // Todo: 🙏 that this works
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", interrupt_stack_frame);
}

pub fn init_idt() {
    IDT.load();
}
