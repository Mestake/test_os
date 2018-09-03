pub mod idt;

pub fn init() {
    idt::init();
}
