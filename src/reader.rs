#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CStr;
use std::ffi::CString;

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

    pub fn new(
        ha_addr: *const ::std::os::raw::c_char,
        local_addr: *const ::std::os::raw::c_char,
        profile_fid: *const ::std::os::raw::c_char,
        process_fid: *const ::std::os::raw::c_char,
    ) -> Self {
        unsafe {
//            println!("{}", CStr::from_ptr(ha_addr).to_str());
            m0_init_instance(ha_addr, local_addr, profile_fid, process_fid);
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

    pub fn str_to_c_char(string: &str) -> *mut c_char {
        let bytes: Vec<u8> = String::from(string).into_bytes();
        let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();

        c_chars.push(0); // null terminator

        let ptr: *mut c_char = c_chars.as_mut_ptr();
        ptr
    }

    #[test]
    fn test_read() {
            let ha_addr = CString::new("172.31.12.177@tcp:12345:34:1").expect("CString::new failed");
            let ha_addr_ptr = ha_addr.as_ptr();
        unsafe {
            let local_addr = CString::new("172.31.12.177@tcp:12345:33:1000").expect("CString::new failed");
            let profile_fid = CString::new("0x7000000000000001:0").expect("CString::new failed");
            let process_fid = CString::new("0x7200000000000001:64").expect("CString::new failed");
         //   let local_addr = str_to_c_char("172.31.12.177@tcp:12345:33:1000");
         //   let profile_fid = str_to_c_char("0x7000000000000001:0");
         //   let process_fid = str_to_c_char("0x7200000000000001:64");
            let reader = MotrReader::new(ha_addr_ptr, local_addr.as_ptr(), 
                                         profile_fid.as_ptr(), process_fid.as_ptr());
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

