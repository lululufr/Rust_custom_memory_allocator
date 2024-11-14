use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::{null, null_mut};
use core::sync;
//use core::u8;
use core::cell::Cell;

use crate::debug::{self, print_hex};

pub struct Free_block {
    next: *mut Free_block,
    addr: usize,
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

                "add rdi, 0x500000",
                "mov rax, 12",
                "syscall",

                lateout("rax") brk_addr,
                out("rdi") _,
            );

            self.heap_ptr.set(brk_addr + layout.size());
            self.brk.set(brk_addr);
            self.init.set(true);

            ((brk_addr - 0x500000) + layout.size()) as *mut u8
        } else {
            self.heap_ptr.set(self.heap_ptr.get() + layout.size());

            (self.heap_ptr.get() + layout.size()) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        debug::print(b"valeur avant update free : ");
        debug::print_hex(self.free_list.get() as usize);
        debug::print(b"\n");

        debug::print(b"valeur ptr : ");
        debug::print_hex(ptr as usize);
        debug::print(b"\n");

        let mut fb = Free_block::new(self.free_list.get(), ptr as usize, layout.size());

        debug::print(b"valeur fb : ");
        debug::print_hex(&fb as *const _ as usize);
        debug::print(b"\n");

        self.free_list.set(&fb as *const _ as *mut Free_block);
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

    pub unsafe fn debug_free_blocks(&self) {
        let mut current = self.free_list.get();

        debug::print(b"\nFreeblock addr : ");
        debug::print_hex(current as usize);
        debug::print(b"\n");
    }
}

impl Free_block {
    pub const fn new(next: *mut Free_block, addr: usize, size: usize) -> Free_block {
        Free_block { next, addr, size }
    }
}
