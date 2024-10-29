#![no_std]
#![no_main]

mod debug;
use debug::{print, print_hex};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::null_mut;
use core::u8;

pub struct Lululucator {
    heap_start: usize,
    heap_end: usize,
    init: bool,
    // free_list: *mut FreeBlock,
}

unsafe impl GlobalAlloc for Lululucator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut brk_addr: usize;

        asm!(
            "mov rax, 12",
            "mov rdi, 0",
            "syscall",
            "mov rdi, rax",

            "add rdi, {size}",
            "mov rax, 12",
            "syscall",

            size = in(reg) layout.size(),
            lateout("rax") brk_addr,
            out("rdi") _,
        );

        (brk_addr - layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

impl Lululucator {
    const fn new() -> Lululucator {
        Lululucator {
            heap_start: 0,
            heap_end: 0,
            init: false,
        }
    }
}

#[global_allocator]
static ALLOCATOR: Lululucator = Lululucator::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let luluint = Layout::from_size_align(1000, 1).unwrap();

    let ma_variable = unsafe { ALLOCATOR.alloc(luluint) };
    let ma_variable2 = unsafe { ALLOCATOR.alloc(luluint) };

    let addr = "prout test";
    debug::print_hex(&ma_variable as *const _ as usize);
    debug::print(b"\n");
    //ok
    debug::print_hex(&ma_variable2 as *const _ as usize);
    debug::print(b"\n");

    loop {}
}
