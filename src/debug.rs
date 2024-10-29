use core::arch::asm;

pub fn print(buf: &[u8]) {
    let _ret: i32;
    unsafe {
        asm!(
            "syscall",               // Effectuer l'appel système.
            inlateout("rax") 1 => _ret,       // Numéro de syscall pour write (1).
            in("rdi") 1,                   // Premier argument : file descriptor.
            in("rsi") buf.as_ptr(),         // Deuxième argument : pointeur vers le buffer.
            in("rdx") buf.len(),            // Troisième argument : longueur du buffer.
            lateout("rcx") _,               // Registres écrasés par syscall.
            lateout("r11") _,
        );
    }
}
pub fn print_hex(mut addr: usize) -> () {
    //prefix
    print(b"0x");

    //récupération du nombre de nibbles
    let mut nibbles = count_nibbles(addr) as isize;

    //affichage des nibbles
    while nibbles > -1 {
        // let num = (addr >> 4) & 0xF;
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
            10 => b'A',
            11 => b'B',
            12 => b'C',
            13 => b'D',
            14 => b'E',
            15 => b'F',
            _ => b'?',
        };
        print(&[c]);

        nibbles -= 1;
    }
}

pub fn count_nibbles(mut num: usize) -> usize {
    if num == 0 {
        return 1; // Zero nécessite au moins un nibble
    }
    let mut count = 0;
    while num > 0 {
        num >>= 4; // Décale `num` de 4 bits vers la droite (équivalent à diviser par 16)
        count += 1;
    }
    count
}
