use std::process::Command;
use crate::app::AppError;


pub fn run_script() -> Result<(), AppError> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "adaptor.exe"])
            .spawn()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("./adaptor")
            .spawn()?
    };

    Ok(())
}
