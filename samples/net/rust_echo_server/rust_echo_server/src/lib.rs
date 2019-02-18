#![no_std]

#[no_mangle]
pub extern "C" fn rust_main() {
    panic!();
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop{}
}