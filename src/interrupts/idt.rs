use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use memory::gdt::DOUBLE_FAULT_IST_INDEX;


lazy_static!{
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
               .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    vga_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    err_code: u64)
{
    vga_println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    vga_println!("err_code: {}", err_code);

    loop {}
}
