version trés basique de l'encoder de la trame qui n'a qu'un header et la payload qu'on peut entrer puis le programme vien former la trame aprés reception de la payload 

Magic (dans notre cas MS) 2 bytes

Version (1) 1 byte

TYPE (On numérote au fur et a mesure vu que le texte c'est notre premier type de donnée pour l'instant on met 1) 1 byte


Hello world! -> 4D 53 01 01 48 65 6C 6C 6F 20 77 6F 72 6C 64 21
