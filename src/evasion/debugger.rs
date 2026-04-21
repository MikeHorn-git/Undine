/*
   https://nu11busters.github.io/rust-maldev-course/evasion/anti-debug-with-isdebuggerpresent/
*/

use std::{process, thread, time::Duration};
use windows_sys::Win32::{Foundation::TRUE, System::Diagnostics::Debug::IsDebuggerPresent};

pub fn debugger() {
    unsafe {
        if IsDebuggerPresent() == TRUE {
            eprintln!("Debugger detected");
            process::exit(1);
        }
    }
}

pub fn thread() {
    thread::spawn(|| {
        loop {
            debugger();
            thread::sleep(Duration::from_secs(1)); // Check every second
        }
    });
}
