use std::ffi::{CString, c_void};
use std::os::raw::c_char;

use libloading::{Library, Symbol};


#[repr(C)]
pub struct Data {
    pub size: u64,
    pub ptr: *mut std::ffi::c_void,
}


type EncodeFn =
    unsafe extern "C" fn(*const c_char, usize) -> Data;

type DecodeFn =
    unsafe extern "C" fn(*const u8, usize) -> Data;

type FreeBufferFn =
    unsafe extern "C" fn(*mut c_void);



pub fn encode(content: &str) -> Data {

    let lib = unsafe {
        Library::new("SharedEngine.dll")
            .expect("UNABLE TO LOAD DLL")
    };

    unsafe {

        let encode: Symbol<EncodeFn> =
            lib.get(b"Encode")
            .expect("Encode Not Found");


        let c_text =
            CString::new(content).unwrap();


        encode(c_text.as_ptr(), content.len())
    }
}



pub fn decode(data: Data) -> String {

    let lib = unsafe {
        Library::new("SharedEngine.dll")
            .expect("UNABLE TO LOAD DLL")
    };

    unsafe {

        let decode: Symbol<DecodeFn> =
            lib.get(b"Decode")
            .expect("Decode Not Found");

        let free_buffer: Symbol<FreeBufferFn> =
            lib.get(b"FreeBuffer")
            .expect("FreeBuffer Not Found");


        let decoded =
            decode(data.ptr as *const u8, data.size as usize);


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
