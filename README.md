# memory_allocator

# Lucas MILLER 4SI4

Voici mon allocateur de mémoire.

Par défaut il y a de la compilation conditionnelle.
Il faut compiler en mode debug pour avoir l'affichage des blocs mémoires

### Mon architecture

```
j'ai décider de faire un allocateur simple. 

#### L'alloc 

  l'Allocation va se faire ainsi :

###### Si l'allocation n'est pas initialisé : 

  Alors on va initialiser mon allocateur : 
c'est a dire qu'a travers une routine assembleur ( via le syscall brk) on va allouer TOUTE la taille de la heap.
Puis on va gérer via la structure les emplacements de mémoire disponible. 
Puis on va renvoyer l'adresse disponible. 

###### Si l'allocation est initialisé : 

  Alors on va simplement renvoyer le "alloc_ptr" qui est ni plus ni moins qu'un pointeur qui pointe la ou dans la heap ( cela suit un ordre chronologique un peu ) il peut écrire. 
Puis remplacer alloc_ptr par l'adresse du futur endroit ou l'allocateur ira allouer. Soit alloc_ptr + taille alloué précédemment. 

#### Le dealloc

###### La Deallocation  
  Elle va consitster simplement ajouter dans une liste chainé un élément de structure Free_Block qui indique une adresse et une taille. 

Puis lors de l'allocation avant d'allouer la ou alloc_ptr pointe il va etre vérifié dans cette liste qu'il n'y a pas deja un endroit capable de prendre la taille donné

###### Le Free_Block et la liste chainé  
Le free block prend juste une taille, une adresse et un next.
Je n'ai pas besoin d'un prévious pour le moment. 
Mais lorsque j'implémenterai "realloc" je le rajouterai. 


### Ressources

Je n'ai pas utilisé de ressources spécifique. La majorité des problemes ont surtout été avec le rust en lui meme, syntax et spécificité. 




