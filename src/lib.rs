extern crate prost;

use std::{alloc::{dealloc, Layout}};
use bytes::{BufMut,BytesMut};
use prost::{Message};

#[cfg(target_os="windows")]
use winapi::{shared::minwindef::{BOOL,DWORD, LPARAM, UINT,WPARAM}, um::winuser::{VK_CONTROL, SetWinEventHook, EVENT_SYSTEM_FOREGROUND, GetForegroundWindow}};
#[cfg(target_os="windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os="windows")]
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, HANDLE, PROCESS_VM_WRITE};
#[cfg(target_os="windows")]
use winapi::um::winuser::{GetWindowThreadProcessId, SendMessageW, WM_KEYDOWN, WM_KEYUP, VK_MENU,VK_SHIFT};
#[cfg(target_os="windows")]
use winapi:: {
    um::{
        processthreadsapi::OpenProcess,
        winuser::{EnumWindows}
    }
}; 

//#[macro_use]
extern crate lazy_static;

// bring proto defs into this namespace
include!(concat!(env!("OUT_DIR"), "/keeproto.rs"));

#[derive(Debug,Copy,Clone)]
pub enum ActionTargetType<'a> {
    Window(&'a ProcessWindowHandles),
    Process(i32)
}


#[derive(Debug)]
pub struct ProcessWindowHandles {
    pub pid: u32,
    #[cfg(target_os="windows")]
    pub window_handles:Vec::<HWND> 
}

#[cfg(target_os = "windows")]
unsafe impl Send for ProcessWindowHandles {}
#[cfg(target_os = "windows")]
unsafe impl Sync for ProcessWindowHandles {}

#[cfg(target_os="windows")]
unsafe extern "system" fn enum_windows_callback(window_handle: HWND, l_param: LPARAM) -> BOOL {
    let mut process_id = 0; //DWORD
    GetWindowThreadProcessId(window_handle, &mut process_id);
    let window_handle_ptr:*mut HWND = std::mem::transmute_copy(&window_handle);
    let window_handles:&mut ProcessWindowHandles = std::mem::transmute(l_param);
    //window_handles.push(*window_handle_ptr);
  
    // println!("{:#?}", window_handles.pid);
    if window_handles.pid == process_id { 
      
        if !window_handle_ptr.is_null() && winapi::um::winuser::IsWindow(window_handle)  != 0 {
           //println!("{}", process_id);
           //println!("{:#?}",window_handle);
           //println!("{:#?}",window_handle.clone());
           window_handles.window_handles.push(window_handle.clone());
           //window_handlespush(window_handle.clone());
        }
    }
    1
}

#[cfg(target_os="windows")]
pub fn get_window_handles(pid:u32) -> ProcessWindowHandles{
    unsafe{ 
     
        let mut window_handles = Vec::<HWND>::new();
        let mut handles = ProcessWindowHandles { pid, window_handles};
        let lparam: LPARAM = std::mem::transmute_copy(& &mut handles);
        EnumWindows(Some(enum_windows_callback),lparam);
        handles
    }
}

// trait can be used for Mac/Windows/Linux implementations
pub trait OSController {
    fn is_process_running(suffix: &str) -> bool;
    fn list_processes() -> Vec<String>;
    fn send_key_to_target(target: ActionTargetType, virtual_key: u16);
    fn send_key_up_to_target(target: ActionTargetType, virtual_key: u16);
    fn send_key_down_to_target(target: ActionTargetType, virtual_key: u16, shift: bool, alt: bool, control: bool);
    fn are_we_trusted() -> bool;
    fn acquire_privileges() -> bool;
    fn request_io_access();
    fn check_io_access() -> bool;
    fn is_process_active(suffix: &str) -> bool;
    fn app_focus_change(cb: unsafe extern "C" fn() -> ());
}

pub struct Control {}

#[cfg(target_os="windows")] 
impl Control {
    fn win_send_key_to_target(target: ActionTargetType, key: u16) {
        //SendMessageW(target_handle,WM_KEYDOWN , VK_SHIFT.try_into().unwrap(), 0);
         

    }
}


#[cfg(target_os="windows")]
impl OSController for Control {
  
    fn is_process_running(suffix: &str) -> bool {
        todo!()
    }

    fn list_processes() -> Vec<String> {
        todo!()
    }

    fn send_key_to_target(target: ActionTargetType, virtual_key: u16) {
        if let ActionTargetType::Window(hwnd) = target {

        } 
     
    }

    fn send_key_up_to_target(target: ActionTargetType, virtual_key: u16) {
        println!("key_up_to_target");
        if let ActionTargetType::Window(target_handle) = target {
            unsafe {
   
                SendMessageW(target_handle.window_handles[0],WM_KEYUP , virtual_key.try_into().unwrap(), 0);
            }
        }
    }

    fn send_key_down_to_target(target: ActionTargetType, virtual_key: u16, shift: bool, alt: bool, control: bool) {
        println!("key_down_to_target");
        if let ActionTargetType::Window(target_handle) = target {
            unsafe {
                if shift {
                    SendMessageW(target_handle.window_handles[0],WM_KEYDOWN , VK_SHIFT.try_into().unwrap(), 0);
                }
                if alt {
                    SendMessageW(target_handle.window_handles[0],WM_KEYDOWN , VK_MENU.try_into().unwrap(), 0);
                }

                if control {
                    SendMessageW(target_handle.window_handles[0],WM_KEYDOWN , VK_CONTROL.try_into().unwrap(), 0);
                }
                SendMessageW(target_handle.window_handles[0],WM_KEYDOWN , virtual_key.try_into().unwrap(), 0);
                
                if shift {
                    SendMessageW(target_handle.window_handles[0],WM_KEYUP , VK_SHIFT.try_into().unwrap(), 0);
                }
                if alt {
                    SendMessageW(target_handle.window_handles[0],WM_KEYUP , VK_MENU.try_into().unwrap(), 0);
                }

                if control {
                    SendMessageW(target_handle.window_handles[0],WM_KEYUP , VK_CONTROL.try_into().unwrap(), 0);
                }

                //std::thread::sleep(std::time::Duration::from_millis(300));
                //SendMessageW(target_handle,WM_KEYUP, vk_code, 0);
                //SendMessageW(target_handle,WM_KEYUP , VK_SHIFT.try_into().unwrap(), 0);
            }
        }
    }

    fn are_we_trusted() -> bool {
      true
    }

    fn acquire_privileges() -> bool {
      true
    }

    fn request_io_access() {
       
    }

    fn check_io_access() -> bool {
        true
    }

    fn is_process_active(suffix: &str) -> bool {
        unsafe{ 
            let focused_window = GetForegroundWindow();
     
        
            if let Ok(pid) = suffix.parse() {
                let handles = get_window_handles(pid);
                if handles.window_handles.contains(&focused_window) {
                     return true;
                }
            }
        }

        false
    }

    fn app_focus_change(cb: unsafe extern "C" fn() -> ()) {
        unsafe { WINDOW_FOCUS_PTR = cb; }
        unsafe { 
            SetWinEventHook(EVENT_SYSTEM_FOREGROUND,EVENT_SYSTEM_FOREGROUND,
            std::ptr::null_mut(),Some(window_foreground_change),0,0,0);
          
            }
    }
}

unsafe extern "C" fn _x() {

}

// need a cleaner way to pass the callback
#[cfg(target_os="windows")]
static mut WINDOW_FOCUS_PTR: unsafe extern "C" fn() = _x;

#[cfg(target_os="windows")]
unsafe extern "system" fn window_foreground_change(
    event_hook: winapi::shared::windef::HWINEVENTHOOK,
    event: u32,
    window: HWND,
    object: i32,
    child_wind:i32,
    thread: u32,
    _time: u32
) {
    println!("window focus change!");
    WINDOW_FOCUS_PTR();
}

#[cfg(target_os="macos")]    
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
    fn app_focus_change(cb: unsafe extern "C" fn() -> ());
}

#[cfg(target_os="macos")]
impl OSController for Control {


    // TODO
    fn is_process_active(suffix: &str) -> bool {
        let kstring = KString {value: suffix.to_string()};
        let mut buf:Vec<u8> = Vec::new();
        kstring.encode(&mut buf).unwrap();
        let in_bytes:&[u8] = &buf;
        let length = buf.len();

        unsafe { 
            //let raw = std::ffi::CString::new(suffix).unwrap().into_raw();
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


                let _ = kstringlist.merge(&mut buf);
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
    }

    fn send_key_to_target(target: ActionTargetType, virtual_key: u16) {
        unsafe {
            if let ActionTargetType::Process(pid) = target { 
                send_key_to_pid(pid, virtual_key);
            }
        }
    }

    fn send_key_up_to_target(target: ActionTargetType, virtual_key: u16) {
        unsafe {
            if let ActionTargetType::Process(pid) = target {
                send_key_up_to_pid(pid, virtual_key);
         
            }
        }
    }
    fn send_key_down_to_target(target: ActionTargetType, virtual_key: u16, shift: bool, alt: bool, control: bool) {
        unsafe {
            if let ActionTargetType::Process(pid) = target {
                send_key_down_to_pid(pid, virtual_key, shift, alt, control);
            }
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

    fn app_focus_change(cb: unsafe extern "C" fn() -> ()) {
        unsafe { app_focus_change(cb); }
    }
}
