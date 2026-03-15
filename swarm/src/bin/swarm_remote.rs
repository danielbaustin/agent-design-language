use anyhow::Result;
use std::path::Path;

fn bind_arg_from_args(args: &[String]) -> String {
    args.get(1)
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:8787".to_string())
}

fn run_with_bind(bind: &str) -> Result<()> {
    eprintln!("swarm-remote listening on http://{bind}");
    ::adl::remote_exec::run_server(bind)
}

fn binary_stem_name(arg0: &std::ffi::OsStr) -> Option<String> {
    Path::new(arg0)
        .file_stem()
        .and_then(|stem| stem.to_str().map(|s| s.to_ascii_lowercase()))
}

fn is_legacy_swarm_remote_invocation_name(arg0: &std::ffi::OsStr) -> bool {
    binary_stem_name(arg0)
        .map(|name| is_legacy_swarm_remote_name(&name))
        .unwrap_or(false)
}

fn is_legacy_swarm_remote_invocation() -> bool {
    std::env::args_os()
        .next()
        .map(|arg0| is_legacy_swarm_remote_invocation_name(&arg0))
        .unwrap_or(false)
}

fn is_legacy_swarm_remote_name(name: &str) -> bool {
    name == "swarm_remote" || name == "swarm-remote"
}

fn run_main_with_args(args: &[String], legacy_invocation: bool) -> Result<()> {
    if legacy_invocation {
        eprintln!("DEPRECATION: 'swarm-remote' is deprecated; use 'adl-remote' instead.");
    }

    let bind = bind_arg_from_args(args);
    run_with_bind(&bind)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    run_main_with_args(&args, is_legacy_swarm_remote_invocation())
}

#[cfg(test)]
mod tests {
    use super::{
        binary_stem_name, bind_arg_from_args, is_legacy_swarm_remote_invocation,
        is_legacy_swarm_remote_invocation_name, is_legacy_swarm_remote_name, run_main_with_args,
        run_with_bind,
    };
    use std::{ffi::OsStr, path::Path};

    #[test]
    fn bind_arg_defaults_when_not_provided() {
        let args = vec!["swarm_remote".to_string()];
        assert_eq!(bind_arg_from_args(&args), "127.0.0.1:8787".to_string());
    }

    #[test]
    fn bind_arg_uses_first_cli_argument() {
        let args = vec!["swarm_remote".to_string(), "0.0.0.0:9000".to_string()];
        assert_eq!(bind_arg_from_args(&args), "0.0.0.0:9000".to_string());
    }

    #[test]
    fn run_with_bind_returns_error_for_invalid_address() {
        let err = run_with_bind("127.0.0.1:not-a-port").expect_err("invalid bind");
        assert!(err.to_string().contains("failed to bind remote server"));
    }

    #[test]
    fn binary_stem_name_normalizes_case_and_suffix() {
        assert_eq!(
            binary_stem_name(OsStr::new("/tmp/SWARM_REMOTE.exe")),
            Some("swarm_remote".to_string())
        );
    }

    #[test]
    fn binary_stem_name_returns_none_for_invalid_utf8() {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            assert_eq!(binary_stem_name(OsStr::from_bytes(&[0xff])), None);
        }
    }

    #[test]
    fn legacy_swarm_remote_detection_recognizes_current_binary_name() {
        let current_name = std::env::args_os()
            .next()
            .and_then(|arg0| Path::new(&arg0).file_stem().map(|s| s.to_owned()))
            .and_then(|stem| stem.to_str().map(|s| s.to_ascii_lowercase()))
            .expect("current binary name");

        if current_name == "swarm_remote" || current_name == "swarm-remote" {
            assert!(is_legacy_swarm_remote_invocation());
        }
    }

    #[test]
    fn legacy_swarm_remote_name_helper_is_precise() {
        assert!(is_legacy_swarm_remote_name("swarm_remote"));
        assert!(is_legacy_swarm_remote_name("swarm-remote"));
        assert!(!is_legacy_swarm_remote_name("adl_remote"));
        assert!(!is_legacy_swarm_remote_name("adl-remote"));
    }

    #[test]
    fn legacy_invocation_name_detects_legacy_binary_names() {
        assert!(is_legacy_swarm_remote_invocation_name(OsStr::new(
            "swarm_remote"
        )));
        assert!(is_legacy_swarm_remote_invocation_name(OsStr::new(
            "swarm-remote"
        )));
        assert!(!is_legacy_swarm_remote_invocation_name(OsStr::new(
            "adl_remote"
        )));
    }

    #[test]
    fn run_main_with_args_emits_legacy_path_with_explicit_invalid_bind() {
        let args = vec![
            "swarm_remote".to_string(),
            "127.0.0.1:not-a-port".to_string(),
        ];
        let err = run_main_with_args(&args, true).expect_err("invalid bind from legacy path");
        assert!(err.to_string().contains("failed to bind remote server"));
    }

    #[test]
    fn run_main_with_args_uses_explicit_bind() {
        let args = vec![
            "swarm_remote".to_string(),
            "127.0.0.1:not-a-port".to_string(),
        ];
        let err = run_main_with_args(&args, false).expect_err("invalid explicit bind");
        assert!(err.to_string().contains("failed to bind remote server"));
    }
}
