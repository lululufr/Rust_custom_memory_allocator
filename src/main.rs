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

/// ON DECLAE UN ALLOCATOR GLOBAL
#[global_allocator]
static mut ALLOCATOR: Lululucator = Lululucator::new();

/// Il y a de la compilation conditionnelle ici
/// je ne l'ai fait que pour débugguer mais j'aurais pu faire mieux en utilisant des macros
/// pour les tests
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let lulusmall = Layout::from_size_align(512, core::mem::align_of::<Free_block>()).unwrap();
    let luluint = Layout::from_size_align(1024, core::mem::align_of::<Free_block>()).unwrap();
    let lululong = Layout::from_size_align(2048, core::mem::align_of::<Free_block>()).unwrap();
    // on alloue 3 bloc, ici les unsafes sont obligatoire car on touche a la mémoire
    let ma_variable = unsafe { ALLOCATOR.alloc(luluint) };
    let ma_variable2 = unsafe { ALLOCATOR.alloc(luluint) };
    let ma_variable3 = unsafe { ALLOCATOR.alloc(luluint) };
    let ma_variable7 = unsafe { ALLOCATOR.alloc(lululong) };

    let addr = "prout test";

    // on libère les 3 blocs , ici pareil pour le unsafe, pas le choix, car mon dealloc alloue de
    // la mémoire
    unsafe {
        ALLOCATOR.dealloc(ma_variable, luluint);
        ALLOCATOR.dealloc(ma_variable2, luluint);
        ALLOCATOR.dealloc(ma_variable3, luluint);
    }

    // on alloue un bloc
    let ma_variable4 = unsafe { ALLOCATOR.alloc(luluint) };

    if cfg!(debug_assertions) {
        unsafe {
            ALLOCATOR.debug_free_blocks();
        }

        // on alloue un bloc
        let ma_variable5 = unsafe { ALLOCATOR.alloc(luluint) };
        unsafe {
            ALLOCATOR.dealloc(ma_variable7, lululong);
        }

        let ma_variable8 = unsafe { ALLOCATOR.alloc(lululong) };

        unsafe {
            ALLOCATOR.debug_free_blocks();
        }

        let ma_variable9 = unsafe { ALLOCATOR.alloc(lulusmall) };

        unsafe {
            ALLOCATOR.debug_free_blocks();
        }
    }
    loop {}
}
