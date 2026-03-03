use anyhow::Result;
use ::adl as swarm;

fn bind_arg_from_args(args: &[String]) -> String {
    args.get(1)
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:8787".to_string())
}

fn run_with_bind(bind: &str) -> Result<()> {
    eprintln!("adl-remote listening on http://{bind}");
    swarm::remote_exec::run_server(bind)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let bind = bind_arg_from_args(&args);
    run_with_bind(&bind)
}

#[cfg(test)]
mod tests {
    use super::{bind_arg_from_args, run_with_bind};

    #[test]
    fn bind_arg_defaults_when_not_provided() {
        let args = vec!["adl_remote".to_string()];
        assert_eq!(bind_arg_from_args(&args), "127.0.0.1:8787".to_string());
    }

    #[test]
    fn bind_arg_uses_first_cli_argument() {
        let args = vec!["adl_remote".to_string(), "0.0.0.0:9000".to_string()];
        assert_eq!(bind_arg_from_args(&args), "0.0.0.0:9000".to_string());
    }

    #[test]
    fn run_with_bind_returns_error_for_invalid_address() {
        let err = run_with_bind("127.0.0.1:not-a-port").expect_err("invalid bind");
        assert!(err.to_string().contains("failed to bind remote server"));
    }
}
