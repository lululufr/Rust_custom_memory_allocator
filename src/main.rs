#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
extern "C" fn _start() -> i32 {

    let letter = 33+ 34;

    return letter;
}
