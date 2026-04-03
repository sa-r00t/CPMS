# Notes par rapport au projet :

--------
DLL:

une fois notre dll avec les 3 fonctions principales faites on pourra ajouter le reste du header de la trame et aussi songer a mettre un checksum pour le footer etc 




<img width="1665" height="860" alt="image" src="https://github.com/user-attachments/assets/0ea4f2ce-893b-4328-a843-ea2234deab0d" />


Il faut absolument que les fonctions exposent en c comme ça on peut utiliser le dll avec rust et cpp jcrois sa s'apelle extern c ou jsp quoi 

---------------------------------------------------------
WRAPPER:

faut tout foutre sur le wrapper histoire qu'on aie le moin de truc a écrire la ou on implemnt le protocole comme ça sa reste bien simple 

------------------------------------------
NETWORK:

quand on aura fini le dll de la V1 on pourra s'attauqer au network on fera des sockets qui envoyent notre contenu par tcp d'une machine A à B 

pour ce faire il faudra implementer la methode d'envoi par tcp sur le wrapper qui se chargera d'envoyer la data a une machine donnée







créer une sorte d'usine qui gere la creation et la supression de variable , par exemple la fonction encode quand elle a besoin de créer une variable elle apelle cette usine et dès que le free buffer est appelé elle rapelle cette usine  

tout cela pour que la variable ne sorte JAMAIS du dll 
