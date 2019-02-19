#![no_std]

use bindings;
use cty;

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        bindings::printk(b"Hello from Rust".as_ptr() as *const cty::c_char);
    }
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}