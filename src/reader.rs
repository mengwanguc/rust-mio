#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;


use anyhow::Result;
use std::os::raw::c_char;
use std::{slice, str};


use std::io::Write;


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub fn read() {
        unsafe {
//            println!("{}", object_read());
        }
    }

struct MotrReader {
    init_instance_done: bool
}


impl MotrReader {
    pub fn new() -> Self {
        unsafe {
            m0_init_instance();
            MotrReader{
                init_instance_done: true,
            }
        }
    }

    pub fn read_object(&self) -> *mut read_result{
        unsafe {
            println!("readning object in rust\n");
            m0_object_read(4099, 8188)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[test]
    fn test_read() {
        unsafe {
            let reader = MotrReader::new();
            println!("\nreadingnow\n\n\n");
            reader.read_object();
            println!("\nreadingnow\n\n\n");
            reader.read_object();
            println!("\nreadingnow\n\n\n");
            let rres = reader.read_object();

//            let slice = CStr::from_ptr(mystr);            

            println!("string buffer size without nul terminator: {}", (*rres).len);

//            let s = str::from_utf8_unchecked(slice::from_raw_parts((*rres).data as *const u8, (*rres).len as _));

            let res_bytes = slice::from_raw_parts((*rres).data as *const u8, (*rres).len as _);

            let mut file = std::fs::File::create("temp.txt").expect("create failed");
            file.write_all(res_bytes).expect("write failed");            

//            println!("string buffer size without nul terminator: {}", strlen(mystr));
/*            println!("{}", m0_object_read());
            println!("{}", m0_init_instance());
            println!("{}", m0_object_read());
            println!("\n\n\n");
            println!("{}", m0_object_read());
            println!("\n\n\n");
            println!("{}", m0_object_read());
*/
        }
    }

}

