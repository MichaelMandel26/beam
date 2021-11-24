use std::io::Result;
use std::process::{Command, ExitStatus};

pub fn connect(host: String, username: String) -> Result<ExitStatus> {
    let host_string = format!("{}@{}", username, host);
    Command::new("tsh")
        .args(["ssh", host_string.as_str()])
        .spawn()?
        .wait()
}
