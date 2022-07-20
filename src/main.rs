extern crate prost;

use std::{ptr, alloc::{dealloc, Layout}};

use prost::{Enumeration, Message};
use bytes::{Bytes, BytesMut, Buf, BufMut};

//pub mod keeproto {
    include!(concat!(env!("OUT_DIR"), "/keeproto.rs"));
//}

#[link(name = "ezmacos")]
extern "C" {
    //
    // find if a window is active based on pid
    // finds pids based on filename

    pub fn is_process_running() -> bool;
    pub fn list_processes(length: &mut u32, out_bytes: &mut *mut u8);
}







fn main() {
    
    println!("Hello, world from rust!");
    unsafe { 
        //hello();
    
       // println!("is_process_running(): {}", is_process_running());
    }
    let mut counter = 0;
    loop { 
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
        let ptr_ptr = &mut  byte_ptr;
        
        list_processes(&mut in_length_val, ptr_ptr);
        // ?? Why  does align want a power of 2 while Swift tells me layou = 1?

        /* 
        let mut bytes = vec![];

        for i in 0..in_length_val {

           bytes.push( *(*ptr_ptr).offset(i as isize));
        }


        let mut buf = BytesMut::new();
        buf.put_slice(&bytes[..]);
 
        let mut kstringlist = KStringList{values:Vec::new()};


        kstringlist.merge(&mut buf);
        println!("{:?}", kstringlist);
     
        
       // dealloc appears to work correclty with this alignment
       // a  memory leak seems to exist around calling a property on
       //  NSWorkspace.shared.runningApplications
       // it's a searchable error but people seem to blame XCode
       */
       
       dealloc(byte_ptr, Layout::from_size_align_unchecked((in_length_val) as usize, 1));
       //println!("Length of byte ptr: {}", in_length_val);
       counter+=1;
       if counter % 100 == 0 {
        print!("counter:{}", counter);
        println!("Length of byte ptr: {}", in_length_val);
       }

    }
    //std::thread::sleep(std::time::Duration::from_millis(1));
    }   // endloop

}
