use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::null_mut;
use core::sync;
use core::u8;

pub struct FreeBlock {
    next: *mut FreeBlock,
    size: usize,
}

pub struct Lululucator {
    heap_start: usize,
    heap_end: usize,
    init: bool,
    free_list: *mut FreeBlock,
}

unsafe impl Sync for Lululucator {}

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

//unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}

impl Lululucator {
    pub const fn new() -> Lululucator {
        Lululucator {
            heap_start: 0,
            heap_end: 0,
            init: false,
            free_list: null_mut(),
        }
    }
    pub unsafe fn free(&mut self, addr: *mut u8, layout: Layout) {
        let freeblock = addr as *mut FreeBlock;
        (*freeblock).size = layout.size();
        (*freeblock).next = self.free_list;
        self.free_list = freeblock;
    }
}
