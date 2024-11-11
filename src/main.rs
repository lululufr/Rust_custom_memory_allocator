#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(unused)]
#![warn(unsafe_op_in_unsafe_fn)]

use core::alloc::{GlobalAlloc, Layout};

mod debug;
use debug::{print, print_hex};

mod lululucateur;
use core::panic::PanicInfo;
use lululucateur::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static mut ALLOCATOR: Lululucator = Lululucator::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let luluint = Layout::from_size_align(1024, 1).unwrap();

    let ma_variable = unsafe { ALLOCATOR.alloc(luluint) };
    let ma_variable2 = unsafe { ALLOCATOR.alloc(luluint) };
    let ma_variable3 = unsafe { ALLOCATOR.alloc(luluint) };

    let addr = "prout test";
    debug::print_hex(ma_variable as *const _ as usize);
    debug::print(b"\n");
    //ok
    debug::print_hex(ma_variable2 as *const _ as usize);
    debug::print(b"\n");

    debug::print_hex(ma_variable3 as *const _ as usize);
    debug::print(b"\n");

    unsafe {
        ALLOCATOR.free(ma_variable, luluint);
    }

    loop {}
}
