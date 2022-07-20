use macos_iohid::Control;
use macos_iohid::OSController;

fn main() {

    let processes = Control::list_processes();
    for process in processes {
        println!("{:?}", process);
    }
}