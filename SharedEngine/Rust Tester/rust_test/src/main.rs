use std::ffi::CString;
use std::os::raw::c_char;
use libloading::{Library, Symbol};

type EncodeFn = unsafe extern "C" fn(input: *const c_char, size: usize, output: *mut c_char);

fn main() {
    // tout le chargement de la DLL doit être dans unsafe
    let lib = unsafe { Library::new("SharedEngine.dll").expect("Impossible de charger la DLL") };

    unsafe {
        let encode: Symbol<EncodeFn> = lib.get(b"Encode").expect("Impossible de trouver Encode");

        let text = "Hello World!";
        let c_text = CString::new(text).unwrap();
        let size = text.len();

        let mut buffer = vec![0u8; 1024];
        let ptr_output = buffer.as_mut_ptr() as *mut c_char;

        encode(c_text.as_ptr(), size, ptr_output);

        let result = &buffer[..size];
        println!("Encoded bytes: {:02X?}", result);
        println!("Décodé comme string: {}", String::from_utf8_lossy(result));
    }
}