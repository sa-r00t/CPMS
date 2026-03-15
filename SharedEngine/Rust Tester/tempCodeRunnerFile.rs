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