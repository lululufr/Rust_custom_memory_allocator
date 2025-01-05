# memory_allocator

## Lucas MILLER 4SI4

Voici mon allocateur de mémoire.

```rust
pub struct Lululucator {
    alloc_ptr: Cell<usize>,
    heap_start: Cell<usize>,
    brk: Cell<usize>,
    init: Cell<bool>,
    free_list: Cell<*mut Free_block>,
}

```

Par défaut il y a de la compilation conditionnelle.

Il faut compiler en mode debug pour avoir l'affichage des blocs mémoires

Pour lancer en mode debug (mode par defaut):

Il va afficher en temps réel dans la console les actions mémoires qui sont effectué.

```
cargo run
```

Pour lancer en mode release :
Sans affichage dans la console.

```
cargo run --release
```

*Une rustdoc est également présente.*

```
cargo doc
```

### Mon architecture

j'ai décider de faire un allocateur de type linked list. 

J'ai également voulu y implémenter des features de type fixed-block allocator et du bump. 

C'est le LULULUCATOR !!!

#### L'alloc

  l'Allocation va se faire ainsi :

###### Si l'allocation n'est pas initialisé

  Alors on va initialiser mon allocateur :
c'est a dire qu'a travers une routine assembleur ( via le syscall brk) on va allouer TOUTE la taille de la heap. De facon a faire le moins de syscall possible plus tard.
Puis on va gérer via la structure les emplacements de mémoire disponible.
Puis on va renvoyer l'adresse disponible.

###### Si l'allocation est initialisé

  Alors on va simplement renvoyer le "alloc_ptr" qui est ni plus ni moins qu'un pointeur qui pointe la ou dans la heap ( cela suit un ordre chronologique un peu ) il peut écrire.
Puis remplacer alloc_ptr par l'adresse du futur endroit ou l'allocateur ira allouer. Soit alloc_ptr + taille alloué précédemment.

Dans le cas ou il y a un bloc free capable d'acceuillir le bloc qui va etre alloué :
Alors le bloc free va etre donné a cette allocation.
Si le bloc fait la meme taille, alors l'adresse est donné et le free_bloc supprimé de la liste chainé.

Si le bloc donné est plus grand , alors le free bloc en question va etre modifié de facon a pointer sur le reste de la taille non donné.
exemple :

je donne 100 -> Il n'y a pas de free_block -> j'alloue

ou

je donne 100 -> Il y a un freeblock de taille 100 -> je retourne l'adresse dispo -> je supprime le free_block de la linked list

ou

je donne 100 -> il y a un freeblock de 250 -> je retourne l'adresse dispo -> j'update le free_block pour le faire pointer vers les 150 restant. 


#### Le dealloc

###### La Deallocation  

  Elle va consitster simplement ajouter dans une liste chainé un élément de structure Free_Block.

  ```rust
pub struct Free_block {
    next: Cell<*mut Free_block>,
    addr: Cell<usize>,
    size: Cell<usize>,
}

```

###### Le Free_Block et la liste chainé  

Le free block prend juste une taille, une adresse et un next.
Je n'ai pas besoin d'un prévious pour le moment.
Mais lorsque j'implémenterai "realloc" je le rajouterai.

### Ressources

Je n'ai pas utilisé de ressources spécifique. La majorité des problemes ont surtout été avec le rust en lui meme, syntax et spécificité.
