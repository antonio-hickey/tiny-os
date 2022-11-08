#![no_std] // don't link the Rust standard library (libc)
#![no_main] // disable Rust entry points

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*!
        The entry point, the linker looks for a function
        named `_start` by default.
    */
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //! Called on panic
    loop {}
}
