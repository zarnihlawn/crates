use std::{
    os::windows::process::ExitStatusExt,
    process::{Command, ExitStatus, Output},
};

pub fn get_hostname() -> String {
    match (cfg!(target_os = "windows"),) {
        (true,) => {
            let output = Command::new("cmd")
                .args(&["/C", "hostname"])
                .output()
                .unwrap_or_else(|_| Output {
                    status: ExitStatus::from_raw(1),
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                });
            String::from_utf8(output.stdout)
                .unwrap_or_else(|_| "unknown".to_string())
                .trim()
                .to_string()
        }
        (false,) => "unknown".to_string(),
    }
}
