use anyhow::Result;
use std::ffi::OsString;
use std::process::Command;

fn adl_remote_binary_name() -> &'static str {
    if cfg!(windows) {
        "adl-remote.exe"
    } else {
        "adl-remote"
    }
}

fn main() -> Result<()> {
    eprintln!("DEPRECATION: 'swarm-remote' is deprecated; use 'adl-remote' instead.");

    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let current_exe = std::env::current_exe()?;
    let adl_remote_exe = current_exe.with_file_name(adl_remote_binary_name());

    let status = Command::new(&adl_remote_exe).args(args).status()?;
    std::process::exit(status.code().unwrap_or(1));
}
