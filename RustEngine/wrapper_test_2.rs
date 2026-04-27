use std::ffi::{CString, c_void};
use std::os::raw::c_char;
use std::sync::OnceLock;

use libloading::{Library, Symbol};


#[repr(C)]
struct Data {
    size: u64,
    ptr: *mut c_void,
}


type EncodeFn =
    unsafe extern "C" fn(*const c_char, usize) -> Data;

type DecodeFn =
    unsafe extern "C" fn(*const u8, usize) -> Data;

type FreeBufferFn =
    unsafe extern "C" fn(*mut c_void);



static LIB: OnceLock<Library> = OnceLock::new();


fn lib() -> &'static Library {

    LIB.get_or_init(|| unsafe {

        Library::new("SharedEngine.dll")
            .expect("UNABLE TO LOAD DLL")

    })
}



fn free_buffer(ptr: *mut c_void) {

    unsafe {

        let free: Symbol<FreeBufferFn> =
            lib().get(b"FreeBuffer")
            .expect("FreeBuffer Not Found");

        free(ptr);
    }
}



pub fn encode(content: &str) -> Vec<u8> {

    unsafe {

        let encode: Symbol<EncodeFn> =
            lib().get(b"Encode")
            .expect("Encode Not Found");


        let c_text =
            CString::new(content)
                .expect("CString failed");


        let encoded =
            encode(
                c_text.as_ptr(),
                content.len()
            );


        let slice =
            std::slice::from_raw_parts(
                encoded.ptr as *const u8,
                encoded.size as usize
            );


        let result =
            slice.to_vec();


        free_buffer(encoded.ptr); // meilleure gestion de la mémoire, on libère le buffer après utilisation la ou la v1 laissait le buffer alloué (memory leak)

        result
    }
}



pub fn decode(encoded: &[u8]) -> String {

    unsafe {

        let decode: Symbol<DecodeFn> =
            lib().get(b"Decode")
            .expect("Decode Not Found");


        let decoded =
            decode(
                encoded.as_ptr(),
                encoded.len()
            );


        let slice =
            std::slice::from_raw_parts(
                decoded.ptr as *const u8,
                decoded.size as usize
            );


        let result =
            String::from_utf8_lossy(slice)
                .to_string();


        free_buffer(decoded.ptr);

        result
    }
}
