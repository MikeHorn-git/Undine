use std::{ffi::CStr, mem};
use windows_sys::{Win32::Foundation::*, Win32::System::Diagnostics::ToolHelp::*};

pub fn pid() -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            eprintln!("[!] Failed to create process snapshot");
            return None;
        }

        let mut entry = PROCESSENTRY32 {
            dwSize: mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        if Process32First(snapshot, &mut entry) != FALSE {
            loop {
                let exe = CStr::from_ptr(entry.szExeFile.as_ptr())
                    .to_string_lossy()
                    .to_ascii_lowercase();

                if exe == "yharnam.exe" {
                    CloseHandle(snapshot);
                    return Some(entry.th32ProcessID);
                }

                if Process32Next(snapshot, &mut entry) == FALSE {
                    break;
                }
            }
        } else {
            eprintln!("[!] Failed to get first process");
        }

        CloseHandle(snapshot);
        println!("[!] No matching process found");
        None
    }
}
