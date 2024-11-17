use core::arch::asm;

pub fn print(buf: &[u8]) {
    let _ret: i32;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 1 => _ret,
            in("rdi") 1,
            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
            lateout("rcx") _,
            lateout("r11") _,
        );
    }
}
pub fn print_hex(mut addr: usize) -> () {
    unsafe {
        let mut cpt = 0;

        let mut nibbles = count_nibbles(addr) as isize;

        while nibbles > -1 {
            let num = ((addr >> 4 * nibbles) & 0xF) as u8;
            let c = match num {
                0 => b'0',
                1 => b'1',
                2 => b'2',
                3 => b'3',
                4 => b'4',
                5 => b'5',
                6 => b'6',
                7 => b'7',
                8 => b'8',
                9 => b'9',
                10 => b'a',
                11 => b'b',
                12 => b'c',
                13 => b'd',
                14 => b'e',
                15 => b'f',
                _ => b'?',
            };
            print(&[c]);
            cpt += 1;
            if cpt == 1 {
                print(b"x");
            }

            nibbles -= 1;
        }
    }
}

pub fn count_nibbles(mut num: usize) -> usize {
    unsafe {
        if num == 0 {
            return 1;
        }
        let mut count = 0;
        while num > 0 {
            num >>= 4;
            count += 1;
        }
        count
    }
}
