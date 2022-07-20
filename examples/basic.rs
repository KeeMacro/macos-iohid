use macos_iohid::Control;
use macos_iohid::OSController;

fn main() {

    let processes = Control::list_processes();
    for process in processes {
        println!("{:?}", process);
    }

    println!("Are we trusted? {}  ", Control::are_we_trusted());
    println!("IO Access? {} ", Control::check_io_access());
    println!("Is process running? {}", Control::is_process_running("Electron"));
}