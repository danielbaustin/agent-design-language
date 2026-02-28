use std::ffi::OsString;
use std::process::Command;

fn adl_binary_name() -> &'static str {
    if cfg!(windows) {
        "adl.exe"
    } else {
        "adl"
    }
}

fn main() {
    eprintln!("DEPRECATION: 'swarm' CLI is deprecated; use 'adl' instead.");

    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    let current_exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("failed to resolve current executable path: {err}");
            std::process::exit(1);
        }
    };
    let adl_exe = current_exe.with_file_name(adl_binary_name());

    let status = match Command::new(&adl_exe).args(args).status() {
        Ok(status) => status,
        Err(err) => {
            eprintln!(
                "failed to launch 'adl' shim target '{}': {err}",
                adl_exe.display()
            );
            std::process::exit(1);
        }
    };

    std::process::exit(status.code().unwrap_or(1));
}
