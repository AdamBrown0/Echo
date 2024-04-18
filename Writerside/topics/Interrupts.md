# Interrupts

## Interrupt Descriptor Table
The Interrupt Descriptor Table allows us to specify a handler function for all CPU
exceptions. Important CPU exceptions include:
- **Page Fault:** When an instruction attempts to illegally access memory
- **Invalid Opcode:** When an instruction is not valid
- **Double Fault:** Called when the IDT does not have a handler registered for an exception
- **Triple Fault:** Called when an exception occurs while the CPU tries to call the double
  fault handler - this will result in rebooting the OS

We will use the IDT implementation from x86_64. The IDT needs to live for the duration
of Echo's uptime to prevent errors from immediately rebooting the system, but it will
not do this if it's allocated on the stack and therefore the memory is reused after
leaving the `init_idt` method. To fix this, we have to ensure it's
stored as a `&'static`. To do this, we use the `lazy_static` macro to perform
initialization the first time the IDT is referenced.
```text
use x86_64::structures::idt::{InterruptDescriptorTable};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        [ ... ]
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
```

## Exception Handlers
```text
idt.breakpoint.set_handler_fn(breakpoint_handler);
[ ... ]
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
```