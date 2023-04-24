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

fn main() {
    let pid = 11468;
    let handles = macos_iohid::get_window_handles(pid);
    
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