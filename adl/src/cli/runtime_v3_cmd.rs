use anyhow::{anyhow, Result};
use serde::Serialize;

const DEFAULT_RUNTIME: RuntimeSelection = RuntimeSelection::V2;
const RUNTIME_V3_CONTROL_HOST: &str = "127.0.0.1";
const RUNTIME_V3_CONTROL_PORT: u16 = 20_997;
const RUNTIME_V3_KERNEL_BIN: &str = "adl-runtime-kernel";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum RuntimeSelection {
    V2,
    V3,
}

impl RuntimeSelection {
    fn parse(value: &str, source: &str) -> Result<Self> {
        match value {
            "v2" | "runtime-v2" => Ok(Self::V2),
            "v3" | "runtime-v3" => Ok(Self::V3),
            other => Err(anyhow!(
                "unsupported runtime selection '{other}' from {source}; expected v2, runtime-v2, v3, or runtime-v3"
            )),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::V2 => "runtime-v2",
            Self::V3 => "runtime-v3",
        }
    }
}

#[derive(Debug, Serialize)]
struct RuntimeV3SelectionReport {
    schema: &'static str,
    selected_runtime: &'static str,
    default_runtime: &'static str,
    selector_source: &'static str,
    default_changed: bool,
    selection_differs_from_default: bool,
    runtime_v2_available: bool,
    runtime_v3_available: bool,
    runtime_v3_control_host: &'static str,
    runtime_v3_control_port: u16,
    runtime_v3_control_endpoint: String,
    runtime_v3_kernel_command: Vec<&'static str>,
    compatibility_boundary: &'static str,
}

pub(crate) fn real_runtime_v3(args: &[String]) -> Result<()> {
    match args.first().map(String::as_str) {
        Some("--help" | "-h" | "help") => {
            println!("{}", runtime_v3_usage());
            Ok(())
        }
        Some("select") => select_runtime(&args[1..]),
        None => select_runtime(&[]),
        Some(other) => Err(anyhow!(
            "unknown runtime-v3 command '{other}'. Expected select, help, or --help."
        )),
    }
}

fn select_runtime(args: &[String]) -> Result<()> {
    let mut explicit_runtime: Option<RuntimeSelection> = None;
    let mut json = false;
    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--runtime" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(anyhow!("runtime-v3 select requires --runtime <v2|v3>"));
                };
                explicit_runtime = Some(RuntimeSelection::parse(value, "--runtime")?);
                i += 1;
            }
            "--json" => {
                json = true;
            }
            "--help" | "-h" => {
                println!("{}", runtime_v3_usage());
                return Ok(());
            }
            other => {
                return Err(anyhow!("unknown arg for runtime-v3 select: {other}"));
            }
        }
        i += 1;
    }

    let (selected, source) = match explicit_runtime {
        Some(selection) => (selection, "--runtime"),
        None => match std::env::var("ADL_RUNTIME_SELECTION") {
            Ok(value) if !value.trim().is_empty() => (
                RuntimeSelection::parse(value.trim(), "ADL_RUNTIME_SELECTION")?,
                "ADL_RUNTIME_SELECTION",
            ),
            _ => (DEFAULT_RUNTIME, "default"),
        },
    };

    let report = selection_report(selected, source);
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("SELECTED_RUNTIME={}", report.selected_runtime);
        println!("DEFAULT_RUNTIME={}", report.default_runtime);
        println!("SELECTOR_SOURCE={}", report.selector_source);
        println!("DEFAULT_CHANGED={}", report.default_changed);
        println!(
            "SELECTION_DIFFERS_FROM_DEFAULT={}",
            report.selection_differs_from_default
        );
        println!(
            "RUNTIME_V3_CONTROL_ENDPOINT={}",
            report.runtime_v3_control_endpoint
        );
        println!(
            "RUNTIME_V3_KERNEL_COMMAND={}",
            report.runtime_v3_kernel_command.join(" ")
        );
    }
    Ok(())
}

fn selection_report(
    selected: RuntimeSelection,
    selector_source: &'static str,
) -> RuntimeV3SelectionReport {
    RuntimeV3SelectionReport {
        schema: "adl.runtime_v3.entrypoint_selection.v1",
        selected_runtime: selected.label(),
        default_runtime: DEFAULT_RUNTIME.label(),
        selector_source,
        default_changed: false,
        selection_differs_from_default: selected != DEFAULT_RUNTIME,
        runtime_v2_available: true,
        runtime_v3_available: true,
        runtime_v3_control_host: RUNTIME_V3_CONTROL_HOST,
        runtime_v3_control_port: RUNTIME_V3_CONTROL_PORT,
        runtime_v3_control_endpoint: format!(
            "http://{RUNTIME_V3_CONTROL_HOST}:{RUNTIME_V3_CONTROL_PORT}"
        ),
        runtime_v3_kernel_command: vec![RUNTIME_V3_KERNEL_BIN, "serve"],
        compatibility_boundary:
            "explicit selector only; Runtime v2 remains the default until the cutover gate changes it",
    }
}

pub(crate) fn runtime_v3_usage() -> &'static str {
    "adl runtime-v3 - explicit Runtime v3 entrypoint selection\n\n\
Usage:\n\
  adl runtime-v3 select [--runtime v2|v3] [--json]\n\
  adl runtime-v3 --help\n\n\
Environment:\n\
  ADL_RUNTIME_SELECTION=v3 selects Runtime v3 when --runtime is omitted.\n\n\
Notes:\n\
  Runtime v2 remains the default unless --runtime v3 or ADL_RUNTIME_SELECTION=v3 is supplied.\n\
  Runtime v3 uses the local control API endpoint http://127.0.0.1:20997.\n\
  Launch the Runtime v3 kernel with: adl-runtime-kernel serve"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_selection_keeps_runtime_v2() {
        let report = selection_report(RuntimeSelection::V2, "default");
        assert_eq!(report.selected_runtime, "runtime-v2");
        assert_eq!(report.default_runtime, "runtime-v2");
        assert!(!report.default_changed);
        assert!(!report.selection_differs_from_default);
        assert_eq!(report.runtime_v3_control_port, 20_997);
    }

    #[test]
    fn explicit_v3_selection_reports_cutover_boundary() {
        let report = selection_report(RuntimeSelection::V3, "--runtime");
        assert_eq!(report.selected_runtime, "runtime-v3");
        assert_eq!(report.default_runtime, "runtime-v2");
        assert!(!report.default_changed);
        assert!(report.selection_differs_from_default);
        assert!(report.runtime_v2_available);
        assert!(report.runtime_v3_available);
        assert_eq!(report.runtime_v3_control_endpoint, "http://127.0.0.1:20997");
        assert_eq!(
            report.runtime_v3_kernel_command,
            ["adl-runtime-kernel", "serve"]
        );
    }

    #[test]
    fn unknown_runtime_selection_fails_closed() {
        let err = RuntimeSelection::parse("v4", "--runtime").expect_err("v4 must fail closed");
        assert!(err
            .to_string()
            .contains("unsupported runtime selection 'v4'"));
    }
}
