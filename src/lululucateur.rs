use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::{null, null_mut};
use core::sync;
//use core::u8;
use core::cell::Cell;

use crate::debug;

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

            &self.heap_ptr.set(brk_addr + layout.size());
            &self.brk.set(brk_addr);
            &self.init.set(true);

            ((brk_addr - 0x500000) + layout.size()) as *mut u8
        } else {
            &self.heap_ptr.set(&self.heap_ptr.get() + layout.size());

            (&self.heap_ptr.get() + layout.size()) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut free_block = Free_block::new(self.free_list.get(), ptr as usize, layout.size());

        self.free_list
            .set(&free_block as *const _ as *mut Free_block);
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

        debug::print(b"AFFICHAGE LISTE CHAINE DE FREE \n");

        while !current.is_null() {
            let block = &*current;
            debug::print_hex(block.next as usize);
            debug::print(b"\n");

            current = block.next;
        }
    }
}

impl Free_block {
    pub const fn new(fb: *mut Free_block, addr: usize, si: usize) -> Free_block {
        Free_block {
            next: fb,
            addr: addr,
            size: si,
        }
    }
}

