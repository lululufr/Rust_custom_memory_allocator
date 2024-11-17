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
                debug::print(b"\nINITIALISATION DE LA HEAP\n");
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

                debug::print(b"Addr alloue : ");
                debug::print_hex(self.alloc_ptr.get());

                debug::print(b"\nFIN INITIALISATION\n");

                self.alloc_ptr.get() as *mut u8
            } else {
                //TODO : c'est degeulasse ca !! a changer
                if layout.size() == FREEBLOCK_SIZE {
                    self.alloc_ptr.set((self.alloc_ptr.get()) + layout.size());
                    return ((self.alloc_ptr.get()) + layout.size()) as *mut u8;
                }

                let free_block = self.find_optimal_free_block(layout.size());

                //TODO : gerer la suppression du freeblock
                if !free_block.is_null() {
                    return free_block as *mut u8;
                }

                self.alloc_ptr.set((self.alloc_ptr.get()) + layout.size());
                ((self.alloc_ptr.get()) + layout.size()) as *mut u8
            }
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        debug::print(b"\nDeallocation  : ");
        debug::print_hex(ptr as usize);
        debug::print(b"\n");

        let free_block_size =
            Layout::from_size_align(FREEBLOCK_SIZE, core::mem::align_of::<Free_block>()).unwrap();

        let free_block = Free_block::new(self.free_list.get(), ptr as usize, layout.size());

        let ptr_free_block = unsafe { self.alloc(free_block_size) };

        unsafe { core::ptr::write(ptr_free_block as *mut Free_block, free_block) };

        self.free_list.set(ptr_free_block as *mut Free_block);
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
        let mut current = self.free_list.get();

        debug::print(b"\n");
        debug::print(b"\n---------LISTE chaine de free_block------------\n");
        while !current.is_null() {
            let block = unsafe { &*current }; // Déréférence le pointeur pour accéder au bloc

            debug::print(b"\n| Adresse libre: ");
            debug::print_hex(block.addr.get());
            debug::print(b" | Adresse du free_bloc: ");
            debug::print_hex(block as *const _ as usize);
            debug::print(b" | Taille: ");
            debug::print_hex(block.size.get());
            debug::print(b" | Next: ");
            debug::print_hex(block.next.get() as usize);

            // Passe au bloc suivant
            current = block.next.get();
        }

        debug::print(b"\n---------LISTE chaine de free_block------------\n");
    }

    pub unsafe fn find_optimal_free_block(&self, size: usize) -> *mut Free_block {
        debug::print(b"\nAllocation : Cherche le block le plus optimal...\n");
        debug::print(b"Taille demande : ");
        debug::print_hex(size);

        let mut current = self.free_list.get();
        let mut optimal_block: *mut Free_block = null_mut();
        let mut smallest_size_diff = usize::MAX; // Permet de trouver la plus petite différence de taille

        while !current.is_null() {
            let block = unsafe { &*current };

            let block_size = block.size.get();

            if size <= block_size {
                let size_diff = block_size - size;
                if size_diff < smallest_size_diff {
                    smallest_size_diff = size_diff;
                    optimal_block = current;

                    // Si on a un match parfait quit
                    if size_diff == 0 {
                        debug::print(b"Match parfait\n");
                        break;
                    }
                }
            }
            current = block.next.get();
        }

        debug::print(b"\nBlock optimal : ");
        debug::print_hex(optimal_block as usize);

        //debug::print(b"\nBlock optimal size  : ");
        //debug::print_hex(optimal_block.size.get() as usize);

        if optimal_block.is_null() {
            debug::print(b"Block optimal null\n");
            null_mut()
        } else {
            optimal_block
        }
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
