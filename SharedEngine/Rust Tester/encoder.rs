/*fn main() {
    //t'a capté il fait la fonction main
    const MAGIC: [u8; 2] = [0x4D, 0x53];
    //la tema j'ai fait une constante pour le magic avec 2 valeurs hex pour "MS"
    const VERSION: u8 = 1;  
    // frr ça c une constante pour la version toujours en hex la avc le u8 
    const TYPE: u8 = 1;
    //pareil mais pr le type de msg

    let mut text = String::new();
    //guette la jlui dit "ohh fait gaffe la jvais drop une valeur dans la variable texte qui est mutable dans la console tu connais
    
    std::io::stdin().read_line(&mut text).unwrap();
    // la il lis ce nigga
    
    let text = text.trim().to_string();
    // la je néttoie un peu le texte pour que sa soit mieu lisible
    
    let mut frame: Vec<u8> = Vec::new(); 
    // la pour former la trame finale frr je un vecteur et jlui balance chaque partie a chaque fois 
    
    frame.extend_from_slice(&MAGIC);
    //la jlui balance le magic 
    
    frame.push(VERSION);
    //la jlui balance la version
    
    frame.push(TYPE);
    //la jlui balance le type
    
    frame.extend_from_slice(text.as_bytes());  
    //la wallah j'ai oublié sa fesait quoi mais jcrois sa le rend en byte comme ça je peux le retourner

    let hex_string: Vec<String> = frame.iter().map(|b| format!("{:02X}", b)).collect();
    //la sa fait une variable finale avc tt dedans jcrois
    
    println!("{}", hex_string.join(" "));   
    //et la sa renvoie
}*/

use std::ffi::CString;
use std::os::raw::c_char;
use libloading::{Library, Symbol}; // tu auras besoin de libloading

type EncodeFn = unsafe extern "C" fn(input: *const c_char, size: usize, output: *mut c_char);

fn main() {
    // 1. Charger la DLL
    let lib = Library::new("SharedEngine.dll").expect("Impossible de charger la DLL");

    unsafe {
        let encode: Symbol<EncodeFn> = lib.get(b"Encode").expect("Impossible de trouver Encode");

        // 2. Préparer le texte
        let text = "Hello World!";
        let c_text = CString::new(text).unwrap();
        let size = text.len();

        // 3. Buffer pour le résultat
        let mut buffer = vec![0u8; 1024];
        let ptr_output = buffer.as_mut_ptr() as *mut c_char;

        // 4. Appel de la fonction
        encode(c_text.as_ptr(), size, ptr_output);

        // 5. Lire les octets encodés
        let result = &buffer[..size];
        println!("Encoded bytes: {:?}", result);
        println!("Decoded as string: {}", String::from_utf8_lossy(result));
    }
}