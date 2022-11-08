#![no_std] // don't link the Rust standard library (no libc)
#![no_main] // disable Rust entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*!
        The entry point, the linker looks for a function
        named `_start` by default.
    */

    // Cast integer into raw pointer
    let vga_buff = 0xb8000 as *mut u8;

    // Iterate over the bytes in `HELLO` byte string
    for (idx, &byte) in HELLO.iter().enumerate() {
        /* compiler cant prove the raw pointers are valid
        so put the operations in an unsafe block. */
        unsafe {
            // write the string byte and the corresponding color byte (0xb = light cyan)
            *vga_buff.offset(idx as isize * 2) = byte;
            *vga_buff.offset(idx as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //! Called on panic
    loop {}
}
