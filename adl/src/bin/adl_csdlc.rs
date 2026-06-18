extern crate adl;

#[allow(dead_code)]
#[path = "../cli/mod.rs"]
mod cli;
#[cfg(test)]
#[path = "../test_support.rs"]
mod test_support;

#[cfg(not(test))]
fn main() {
    cli::run_csdlc_main();
}

#[cfg(test)]
fn binary_help_probe() -> String {
    cli::csdlc_usage().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn adl_csdlc_cli_binary_links_to_csdlc_dispatch_surface() {
        let output = super::binary_help_probe();

        assert!(output.contains("adl-csdlc - ADL C-SDLC compatibility binary"));
        assert!(output.contains("adl-csdlc issue run <issue>"));
        assert!(output.contains("adl/tools/pr.sh remains the canonical"));
    }
}
