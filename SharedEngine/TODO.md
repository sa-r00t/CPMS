# Shared engine 
Rust example Encoder x

Rust example Decoder x

Rust implementable decoder/encoder scripts *a faire* 


# Core engine



# DLL

il faut 3 grandes fonctions pour le dll 

# La Fonction Encode

fonction qui vien encoder la trame et la former (header,payload,footer)

# La Fonction Decode

fonction qui vien décoder la trame et éxtraire les donées de celle-ci 

# La Fonction Free-Buffer

fonction qui vien liberer la ram et la nettoyer pour éviter un memory leak 




faire en sorte que le decode soit utilisable avec un truc du genre decoded.magic,decoded.type,decoded.payload etc etc


# Wrapper

pour nous faciliter la tache quand on implémente le programme il faut qu'on fasse un Wrapper qui va faire le sale boulot a notre place et va servir de middleman entre le programme ou on implémente cpms et le dll on pourra juste faire encode(contenu) decode(contenu) a notre guise et méme le free-buffer sa sera géré par le wrapper donc sa va étre trés trés facile d'utilisation


on peut aller encore plus loin et voir si on peut pas transformer ça en crate rust ou libraire cpp ou module python pour rendre le tout trés compact et facile a intégrer



# Network 

tcp socket

