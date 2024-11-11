use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::{null, null_mut};
use core::sync;
//use core::u8;
use core::cell::Cell;

pub struct Free_block {
    next: *mut Free_block,
    size: usize,
}

pub struct Lululucator {
    heap_ptr: Cell<usize>,
    brk: Cell<usize>,
    init: Cell<bool>,
    free_list: Cell<*mut Free_block>,
}

#[allow(clippy::unnecessary_operation)]
#[allow(unsafe_code)]
unsafe impl GlobalAlloc for Lululucator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut brk_addr: usize;

        if !self.init.get() {
            asm!(
                "mov rax, 12",
                "mov rdi, 0",
                "syscall",
                "mov rdi, rax",

                "add rdi, 0x5000",
                "mov rax, 12",
                "syscall",

                lateout("rax") brk_addr,
                out("rdi") _,
            );

            &self.heap_ptr.set(brk_addr + layout.size());
            &self.brk.set(brk_addr);
            &self.init.set(true);
            ((brk_addr - 0x5000) + layout.size()) as *mut u8
        } else if (self.heap_ptr.get() + layout.size()) > self.brk.get() {
            asm!(
                "mov rax, 12",
                "mov rdi, 0",
                "syscall",

                "mov rdi, rax",
                "add rdi, 0x5000",
                "mov rax, 12",
                "syscall",

                lateout("rax") brk_addr,
                out("rdi") _,
            );
            &self.brk.set(brk_addr);
            &self.heap_ptr.set(brk_addr + layout.size());
            ((brk_addr - 0x5000) + layout.size()) as *mut u8
        } else {
            &self.heap_ptr.set(&self.heap_ptr.get() + layout.size());

            (&self.heap_ptr.get() + layout.size()) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let free_block = ptr as *mut Free_block;

        (*free_block).size = layout.size();
        (*free_block).next = self.free_list.get();

        self.free_list.set(free_block);
    }
}

impl Lululucator {
    pub const fn new() -> Lululucator {
        Lululucator {
            heap_ptr: Cell::new(0),
            brk: Cell::new(0),
            init: Cell::new(false),
            free_list: Cell::new(null_mut()),
        }
    }
}

impl Free_block {
    pub const fn new() -> Free_block {
        Free_block {
            next: null_mut(),
            size: 0,
        }
    }
}
