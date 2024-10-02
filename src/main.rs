#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}


use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;

pub struct GreatAllocator;

unsafe impl GlobalAlloc for GreatAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        let brk_addr: usize;

        asm!(
        "mov rax, 0xc",
        "mov rdi, 0",
        "syscall",
        
        "mov rdi, rax",
        "mov rax, 0xc",
        "add rdi, 1000",
        "syscall",
        lateout("rax") brk_addr,
        );

        brk_addr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //do shit

    }
}


#[global_allocator]
static ALLOCATOR: GreatAllocator = GreatAllocator;

#[no_mangle]
unsafe extern "C" fn _start() -> usize{

    let addr = ALLOCATOR.alloc(Layout::new::<u8>());

    0




}
