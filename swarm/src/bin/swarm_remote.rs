use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let bind = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:8787".to_string());
    eprintln!("swarm-remote listening on http://{bind}");
    swarm::remote_exec::run_server(&bind)
}
