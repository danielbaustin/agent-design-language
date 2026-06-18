#[cfg(not(test))]
#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[cfg(test)]
#[path = "../test_support.rs"]
mod test_support;

#[cfg(not(test))]
fn main() {
    cli::run_runtime_main();
}
