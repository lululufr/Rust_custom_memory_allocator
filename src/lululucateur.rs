use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::ptr::{null, null_mut};
use core::sync;
//use core::u8;
use core::cell::Cell;
use core::mem::size_of;

use crate::debug::{self, print_hex};

/// Heap size
/// J'ai fais le choix d'allouer 1024 * 1024 octets pour la heap soit 1Mo
/// Lors de la création du kernel je vais allouer beaucoup
const HEAP_SIZE: usize = 1024 * 1024;
const FREEBLOCK_SIZE: usize = size_of::<Free_block>();
const HEAP_FREEBLOCK_SIZE: usize = HEAP_SIZE / 0x10;

/// Structure de mes blocs libres
/// C'est une liste chainé, je n'ai pas besoin de faire des opérations compliqué alors je ne prend
/// que le un next ( bloc suivant ) l'addr (l'adresse libre en mémoire) et size (la taille disponible)
/// Ici Cell me permet de faciliter l'acces a mes valeurs , Crédit a chatGPT qui m'a donné cette méthode
pub struct Free_block {
    next: Cell<*mut Free_block>,
    addr: Cell<usize>,
    size: Cell<usize>,
}

///La structure de mon allocateur
/// J'ai choisis de faire un allocateur simple , ici alloc_ptr est un pointeur qui pointe a
/// l'endroit de la heap ou va etre alloué le prochain block ( si pas de block free disponible)
/// heap_start est l'adresse de début de la heap
/// brk est l'adresse de fin de la heap
/// init qui permet de savoir si la heap a été initialisé
pub struct Lululucator {
    alloc_ptr: Cell<usize>,
    heap_start: Cell<usize>,
    brk: Cell<usize>,
    init: Cell<bool>,
    free_list: Cell<*mut Free_block>,
}

///ici je déclare mon allocateur global en l'implémentant via la structure GlobalAlloc
#[allow(clippy::unnecessary_operation)]
#[allow(unsafe_code)]
unsafe impl GlobalAlloc for Lululucator {
    ///Alloc ici ma fonction va allouer un block de mémoire de taille layout.size()
    ///Si la heap n'a pas été initialisé elle va l'initialiser en donnant un gros block de mémoire
    ///( pour ne pas avoir a faire des syscall a chaque fois (se qui va etre utile lors du kernelage du kernel)
    ///L'initialisation de la heap se fait en faisant un syscall 12 ( brk ) qui permet de déplacer
    ///le pointeur de fin de heap et de retourner l'adresse de brk
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut brk_addr: usize;
        unsafe {
            if !self.init.get() {
                if cfg!(debug_assertions) {
                    debug::print(b"\nINITIALISATION DE LA HEAP\n");
                }

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

                if cfg!(debug_assertions) {
                    debug::print(b"Addr alloue : ");
                    debug::print_hex(self.alloc_ptr.get());

                    debug::print(b"\nFIN INITIALISATION\n");
                }
                self.alloc_ptr.get() as *mut u8
            } else {
                //TODO : c'est degeulasse ca !! a changer
                if layout.size() == FREEBLOCK_SIZE {
                    self.alloc_ptr.set((self.alloc_ptr.get()) + layout.size());
                    return ((self.alloc_ptr.get()) + layout.size()) as *mut u8;
                }

                let free_block = self.find_optimal_free_block(layout.size()); // Recherche
                                                                              // du block optimal pour l'allocation

                if !free_block.is_null() {
                    //suppréssion freeblock de la liste
                    let mut current = free_block;
                    self.remove_free_block(free_block);
                    return current as *mut u8;
                }

                self.alloc_ptr.set((self.alloc_ptr.get()) + layout.size());

                if cfg!(debug_assertions) {
                    debug::print(b"\nAllocation : ");
                    debug::print_hex((self.alloc_ptr.get()) + layout.size());
                    debug::print(b"\n");
                }

                // a la fin on va renvoyer l'adresse du block alloué soit l'address du alloc_ptr + la taille du block
                ((self.alloc_ptr.get()) + layout.size()) as *mut u8
            }
        }
    }
    ///Pour dealloc c'est la que ca devient complex !!
    ///La fonction va prendre un pointeur et une taille !
    ///Elle va créer un free_block avec ces informations et l'ajouter a la liste chainé de free_block
    ///Le point etant qu'il faut pouvoir allouer pour faire ca ! en tout ca j'alloue un block de taille FREEBLOCK_SIZE
    ///Je le remplis avec les informations et je l'ajoute a la liste chainé
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if cfg!(debug_assertions) {
            debug::print(b"\nDeallocation  : ");
            debug::print_hex(ptr as usize);
            debug::print(b"\n");
        }
        let free_block_size =
            Layout::from_size_align(FREEBLOCK_SIZE, core::mem::align_of::<Free_block>()).unwrap();

        let free_block = Free_block::new(self.free_list.get(), ptr as usize, layout.size());

        let ptr_free_block = unsafe { self.alloc(free_block_size) };

        unsafe { core::ptr::write(ptr_free_block as *mut Free_block, free_block) }; //cette fontion
                                                                                    //est magique !! parce que écrir a partir d'un pointeur en rust est apparemment pas si
                                                                                    //simple

        self.free_list.set(ptr_free_block as *mut Free_block);

        if cfg!(debug_assertions) {
            unsafe { self.debug_free_blocks() };
        }
    }
}

///Rien de fou ici , l'implémentation de new()
///on initialise tout a 0
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

    ///Cette fonction me sert juste a debug elle affiche ma liste chainé de bloc free
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

    ///cette fonction est la pour trouver si il y a un block free de disponible ( et de taille suffisante)
    ///elle va parcourir la liste chainé de free_block
    /// Je peux améliorer cette fonction encore
    pub unsafe fn find_optimal_free_block(&self, size: usize) -> *mut Free_block {
        if cfg!(debug_assertions) {
            debug::print(b"\nAllocation : Cherche le block le plus optimal...\n");
            debug::print(b"Taille demande : ");
            debug::print_hex(size);
        }

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
                        if cfg!(debug_assertions) {
                            debug::print(b" Match parfait\n");
                        }
                        break;
                    }
                }
            }
            current = block.next.get();
        }
        if cfg!(debug_assertions) {
            debug::print(b"\nBlock optimal : ");
            debug::print_hex(optimal_block as usize);
        }
        if optimal_block.is_null() {
            if cfg!(debug_assertions) {
                debug::print(b"Block optimal null\n");
            }
            null_mut()
        } else {
            optimal_block
        }
    }

    ///Cette fonction me permet de supprimer un free_block de la liste
    ///C'est la seul opération que je fais sur la freelist
    ///Je pourrais dans le futur ajouter un realloc et donc faire d'autre opération comme la fusion
    ///de block etc...
    ///Mais pour le moment on fait simple
    pub fn remove_free_block(&self, block: *mut Free_block) {
        if cfg!(debug_assertions) {
            debug::print(b"\nSuppression du freeblock : \n");
            debug::print_hex(block as usize);
        }
        let mut current = self.free_list.get();
        let mut prev: *mut Free_block = null_mut();
        while !current.is_null() {
            if current == block {
                unsafe {
                    if !prev.is_null() {
                        (*prev).next.set((*block).next.get());
                    } else {
                        self.free_list.set((*block).next.get());
                    }
                }
                break;
            }

            unsafe {
                prev = current;
                current = (*current).next.get();
            }
        }
    }
}

///Implem du freeblock .. j'ai rien a dire la dessus
impl Free_block {
    pub const fn new(next: *mut Free_block, addr: usize, size: usize) -> Free_block {
        Free_block {
            next: Cell::new(next),
            addr: Cell::new(addr),
            size: Cell::new(size),
        }
    }
}
