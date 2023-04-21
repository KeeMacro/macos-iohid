use winapi::shared::minwindef::{BOOL,DWORD, LPARAM, UINT,WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, HANDLE, PROCESS_VM_WRITE};
use winapi::um::winuser::{GetWindowThreadProcessId, SendMessageW, WM_KEYDOWN, WM_KEYUP, VK_SHIFT};
use winapi:: {
    um::{
        processthreadsapi::OpenProcess,
        winuser::{EnumWindows}
    }
};    
fn print_window_names_by_pid() {

}  

#[derive(Debug)]
struct ProcessWindowHandles {
    pid: u32,
    window_handles:Vec::<HWND> 
}

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

fn get_window_handles(pid:u32) -> ProcessWindowHandles{
    unsafe{ 
     
        let mut window_handles = Vec::<HWND>::new();
        let mut handles = ProcessWindowHandles { pid, window_handles};
        let lparam: LPARAM = std::mem::transmute_copy(& &mut handles);
        EnumWindows(Some(enum_windows_callback),lparam);
        handles
    }
}
 

fn main() {
    let pid = 11468;
    let handles = get_window_handles(pid);
    println!("{:#?}", handles);

    let vk_code = 0x39;// A0x57;
    let target_handle =handles.window_handles[0];
    unsafe { 

            // wParam 
            SendMessageW(target_handle,WM_KEYDOWN , VK_SHIFT.try_into().unwrap(), 0);
            SendMessageW(target_handle,WM_KEYDOWN , vk_code, 0);
            std::thread::sleep(std::time::Duration::from_millis(300));
            SendMessageW(target_handle,WM_KEYUP, vk_code, 0);
            SendMessageW(target_handle,WM_KEYUP , VK_SHIFT.try_into().unwrap(), 0);
     
    }
   

}