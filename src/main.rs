#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info : &PanicInfo) -> ! {
    loop {}
}

use core::alloc::{GlobalAlloc, Layout};

pub struct GreatAllocator;

unsafe impl GlobalAlloc for GreatAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //do shit

        0 as *mut u8

    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        //do shit

    }
}


#[global_allocator]
static ALLOCATOR: GreatAllocator = GreatAllocator;

#[no_mangle]
unsafe extern "C" fn _start() {

    ALLOCATOR.alloc(Layout::new::<u8>());


}
