extern crate prost;


use std::{
    alloc::{dealloc, Layout},
    ptr, os::{unix::process, raw::c_char}, io::Read,
};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use prost::{Enumeration, Message};

// bring proto defs into this namespace
include!(concat!(env!("OUT_DIR"), "/keeproto.rs"));

#[link(name = "ezmacos")]
extern "C" {
    fn list_processes(length: &mut u32, out_bytes: &mut *mut u8);
    fn send_key_to_pid(pid: i32, virtual_key: u16);
    fn send_key_up_to_pid(pid: i32, virtual_key: u16);
    fn send_key_down_to_pid(pid: i32, virtual_key: u16, shift: bool, alt: bool, control: bool);
    fn are_we_trusted() -> bool;
    fn acquire_privileges() -> bool;
    fn request_io_access();
    fn check_io_access() -> bool;
    fn is_process_active(length: i64, in_bytes: &u8 ) -> bool;
}

// trait can be used for Mac/Windows/Linux implementations
pub trait OSController {
    fn is_process_running(suffix: &str) -> bool;
    fn list_processes() -> Vec<String>;
    fn send_key_to_pid(pid: i32, virtual_key: u16);
    fn send_key_up_to_pid(pid: i32, virtual_key: u16);
    fn send_key_down_to_pid(pid: i32, virtual_key: u16, shift: bool, alt: bool, control: bool);
    fn are_we_trusted() -> bool;
    fn acquire_privileges() -> bool;
    fn request_io_access();
    fn check_io_access() -> bool;
    fn is_process_active(suffix: &str) -> bool;
}

pub struct Control {}

impl OSController for Control {
    // TODO
    fn is_process_active(suffix: &str) -> bool {
        let kstring = KString {value: suffix.to_string()};
        let mut buf:Vec<u8> = Vec::new();
        kstring.encode(&mut buf).unwrap();
        let in_bytes:&[u8] = &buf;
        let length = buf.len();

        unsafe { 
            let raw = std::ffi::CString::new(suffix).unwrap().into_raw();
            return is_process_active(length as i64, &in_bytes[0]);
        }
    }
    
    fn is_process_running(suffix: &str) -> bool {
        let processes = Self::list_processes();
        let is_running=false;
        for process in processes {
            if process.ends_with(suffix) {
                return true
            }
        } 

        is_running
    }

    fn list_processes() -> Vec<String> {
        let mut processes = Vec::new();
        println!("Hello, world from rust!");
        unsafe {
            //hello();

            // println!("is_process_running(): {}", is_process_running());
        }
   
      
            unsafe {
                /*
                    let kstring = KString{ value:"asdf".to_owned()};
                    let mut kstringlist = KStringList{values:Vec::new()};
                    kstringlist.values.push(kstring);
                    let mut buf = BytesMut::with_capacity(1024);
                    kstringlist.encode(&mut buf);

                    let x = KStringList::decode(&mut buf);
                    print!("{:?}", x);
                */
                let mut in_length_val = 0;
                //let in_length =  &mut in_length_val;

                let mut byte_ptr: *mut u8 = std::ptr::null_mut();
                let ptr_ptr = &mut byte_ptr;

                list_processes(&mut in_length_val, ptr_ptr);
                // ?? Why  does align want a power of 2 while Swift tells me layou = 1?

                
                 let mut bytes = vec![];

                 for i in 0..in_length_val {

                    bytes.push( *(*ptr_ptr).offset(i as isize));
                 }


                 let mut buf = BytesMut::new();
                 buf.put_slice(&bytes[..]);

                 let mut kstringlist = KStringList{values:Vec::new()};


                 kstringlist.merge(&mut buf);
                 for x in kstringlist.values.iter() {
                    processes.push(x.value.clone());
                 }
                // dealloc appears to work correclty with this alignment
                // a  memory leak seems to exist around calling a property on
                //  NSWorkspace.shared.runningApplications
                // it's a searchable error but people seem to blame XCode
                
                dealloc(byte_ptr,Layout::from_size_align_unchecked((in_length_val) as usize, 1),);
     
          
            }
            processes
            //std::thread::sleep(std::time::Duration::from_millis(1));
     
    }

    fn send_key_to_pid(pid: i32, virtual_key: u16) {
        unsafe {
            send_key_to_pid(pid, virtual_key);
        }
    }

    fn send_key_up_to_pid(pid: i32, virtual_key: u16) {
        unsafe {
            send_key_up_to_pid(pid, virtual_key);
        }
    }
    fn send_key_down_to_pid(pid: i32, virtual_key: u16, shift: bool, alt: bool, control: bool) {
        unsafe {
            send_key_down_to_pid(pid, virtual_key, shift, alt, control);
        }
    }

    fn are_we_trusted() -> bool {
        unsafe {
            return are_we_trusted();
        }
    }

    fn acquire_privileges() -> bool {
        unsafe {
            return acquire_privileges()
        }
    }

    fn request_io_access() {
        unsafe {
            request_io_access();
        }
    }

    fn check_io_access() -> bool {
        unsafe {
            return check_io_access()
        }
    }
}
