#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("./bindings.rs");

use std::ptr;
use std::mem::MaybeUninit;
use std::ffi::CString;
use std::os::raw::c_void;

/*
extern "C" {
    fn init_container(x: i32) -> i32;
}
*/
/*
fn main() {
    // Statements here are executed when the compiled binary is called
    let m0_instance: *mut m0_client = ptr::null_mut();
    unsafe {
        let mut motr_container: MaybeUninit<m0_container> = MaybeUninit::uninit();
        let mut motr_conf: MaybeUninit<m0_config> = MaybeUninit::uninit();
        let mut motr_dix_conf: MaybeUninit<m0_idx_dix_config> = MaybeUninit::uninit();
        let mut motr_conf = m0_config {
            mc_is_oostore: true,
            mc_is_read_verify: false,
            mc_is_addb_init: false,
            mc_local_addr: CString::new("10.230.242.162@tcp:12345:4:1").unwrap().as_ptr(),
            mc_ha_addr: CString::new("10.230.242.162@tcp:12345:1:1").unwrap().as_ptr(),
            mc_process_fid: CString::new("0x7200000000000001:0x17").unwrap().as_ptr(),
            mc_profile: CString::new("0x7000000000000001:0x3d").unwrap().as_ptr(),
            mc_tm_recv_queue_min_len: M0_NET_TM_RECV_QUEUE_DEF_LEN,
            mc_max_rpc_msg_size: M0_RPC_DEF_MAX_RPC_MSG_SIZE,
            mc_layout_id: 0,
            mc_idx_service_id: m0_idx_service_type_M0_IDX_DIX as i32,
            mc_idx_service_conf: motr_dix_conf.as_mut_ptr() as *mut c_void,
            mc_addb_size: 0,
        };

        println!("{}", init_container(1));

        m0_client_init(&m0_instance as *const _ as  *mut _, 
            &motr_conf as *const _ as *mut _, true);
    }

    // Print text to the console
    println!("Hello World!");
}

*/


fn main() {
    unsafe {
//        println!("{}", init_container());
        println!("{}", object_read());
    }
}

