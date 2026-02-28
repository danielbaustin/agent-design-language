use anyhow::Result;
use std::path::Path;

fn bind_arg_from_args(args: &[String]) -> String {
    args.get(1)
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:8787".to_string())
}

fn run_with_bind(bind: &str) -> Result<()> {
    eprintln!("swarm-remote listening on http://{bind}");
    swarm::remote_exec::run_server(bind)
}

fn is_legacy_swarm_remote_invocation() -> bool {
    std::env::args_os()
        .next()
        .and_then(|arg0| Path::new(&arg0).file_stem().map(|s| s.to_owned()))
        .and_then(|stem| stem.to_str().map(|s| s.to_ascii_lowercase()))
        .map(|name| name == "swarm_remote" || name == "swarm-remote")
        .unwrap_or(false)
}

fn main() -> Result<()> {
    if is_legacy_swarm_remote_invocation() {
        eprintln!("DEPRECATION: 'swarm-remote' is deprecated; use 'adl-remote' instead.");
    }

    let args: Vec<String> = std::env::args().collect();
    let bind = bind_arg_from_args(&args);
    run_with_bind(&bind)
}

#[cfg(test)]
mod tests {
    use super::{bind_arg_from_args, is_legacy_swarm_remote_invocation, run_with_bind};
    use std::path::Path;

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
}
