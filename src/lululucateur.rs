use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::{null, null_mut};
use core::sync;
//use core::u8;
use core::cell::Cell;

pub struct Lululucator {
    heap_ptr: Cell<usize>,
    heap_end: Cell<usize>,
    init: Cell<bool>,
}

unsafe impl GlobalAlloc for Lululucator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut brk_addr: usize;

        if !self.init.get() {
            asm!(
                "mov rax, 12",
                "mov rdi, 0",
                "syscall",
                "mov rdi, rax",

                "add rdi, 0x3000",
                "mov rax, 12",
                "syscall",

                lateout("rax") brk_addr,
                out("rdi") _,
            );

            &self.heap_ptr.set(brk_addr + layout.size());
            &self.heap_end.set(brk_addr);
            &self.init.set(true);
            ((brk_addr - 0x3000) + layout.size()) as *mut u8
        } else {
            &self.heap_ptr.set(&self.heap_ptr.get() + layout.size());

            (&self.heap_ptr.get() + layout.size()) as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

impl Lululucator {
    pub const fn new() -> Lululucator {
        Lululucator {
            heap_ptr: Cell::new(0),
            heap_end: Cell::new(0),
            init: Cell::new(false),
        }
    }
    pub fn free(&mut self, addr: *mut u8, layout: Layout) {}
}
