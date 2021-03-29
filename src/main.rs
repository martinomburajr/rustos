#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //The VGA buffer is located at address 0xb8000 and that each character cell
    // consists of an ASCII byte and a color byte. First, we cast the integer
    // 0xb8000 into a raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        //Note that there's an unsafe block around all memory writes. 
        // The reason is that the Rust compiler can't prove that the raw
        // pointers we create are valid. They could point anywhere and lead to
        // data corruption. By putting them into an unsafe block we're basically
        // telling the compiler that we are absolutely sure that the operations are valid.
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}