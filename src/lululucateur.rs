use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::{null, null_mut};
use core::sync;
//use core::u8;
use core::cell::Cell;
use core::mem::size_of;

use crate::debug::{self, print_hex};

const HEAP_SIZE: usize = 1024 * 1024;
const FREEBLOCK_SIZE: usize = size_of::<Free_block>();
const HEAP_FREEBLOCK_SIZE: usize = HEAP_SIZE / 0x10;

pub struct Free_block {
    next: Cell<*mut Free_block>,
    addr: Cell<usize>,
    size: Cell<usize>,
}

pub struct Lululucator {
    alloc_ptr: Cell<usize>,
    heap_start: Cell<usize>,
    brk: Cell<usize>,
    init: Cell<bool>,
    free_list: Cell<*mut Free_block>,
}

#[allow(clippy::unnecessary_operation)]
#[allow(unsafe_code)]
unsafe impl GlobalAlloc for Lululucator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut brk_addr: usize;
        unsafe {
            if !self.init.get() {
                asm!(
                    "mov rax, 12",
                    "mov rdi, 0",
                    "syscall",
                    "mov rdi, rax",

                    "add rdi, {heap_size}",
                    "mov rax, 12",
                    "syscall",

                    heap_size = const HEAP_SIZE,

                    lateout("rax") brk_addr,
                    out("rdi") _,
                );

                self.heap_start.set(brk_addr - HEAP_SIZE);
                self.alloc_ptr.set((brk_addr - HEAP_SIZE) + layout.size());
                self.brk.set(brk_addr);
                self.init.set(true);

                self.alloc_ptr.get() as *mut u8
            } else {
                self.alloc_ptr.set((self.alloc_ptr.get()) + layout.size());

                ((self.alloc_ptr.get()) + layout.size()) as *mut u8
            }
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        debug::print(b"-----BLOC FREE-----\n");

        debug::print(b"Addr 1er bloc : ");
        debug::print_hex(self.free_list.get() as usize);
        debug::print(b"\n");

        debug::print(b"Addr vide disponible : ");
        debug::print_hex(ptr as usize);
        debug::print(b"\n");

        let free_block_size =
            Layout::from_size_align(FREEBLOCK_SIZE, core::mem::align_of::<Free_block>()).unwrap();

        let free_block = Free_block::new(self.free_list.get(), ptr as usize, layout.size());

        let ptr_free_block = unsafe { self.alloc(free_block_size) };

        unsafe { core::ptr::write(ptr_free_block as *mut Free_block, free_block) };

        debug::print(b"Addr du freeblock : ");
        debug::print_hex(ptr_free_block as usize);
        debug::print(b"\n");

        self.free_list.set(ptr_free_block as *mut Free_block);

        debug::print(b"\n");
    }
}

impl Lululucator {
    pub const fn new() -> Lululucator {
        Lululucator {
            alloc_ptr: Cell::new(0),
            heap_start: Cell::new(0),
            brk: Cell::new(0),
            init: Cell::new(false),
            free_list: Cell::new(null_mut()),
        }
    }

    pub unsafe fn debug_free_blocks(&self) {
        //let mut current = self.free_list.get();

        //debug::print(b"\nFreeblock addr : ");
        //debug::print_hex(current as usize);
        //debug::print(b"\n");
    }
}

impl Free_block {
    pub const fn new(next: *mut Free_block, addr: usize, size: usize) -> Free_block {
        Free_block {
            next: Cell::new(next),
            addr: Cell::new(addr),
            size: Cell::new(size),
        }
    }
}
