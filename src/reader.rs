#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;


use anyhow::Result;
use std::os::raw::c_char;

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

    pub fn read_object(&self) -> *mut c_char{
        unsafe {
            println!("readning object in rust\n");
            m0_object_read()
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
            let mystr = reader.read_object();

            let slice = CStr::from_ptr(mystr);            

            println!("string buffer size without nul terminator: {}", slice.to_bytes().len());
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

