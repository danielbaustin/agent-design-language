use anyhow::{anyhow, Result};
use serde::Serialize;

const DEFAULT_RUNTIME: RuntimeSelection = RuntimeSelection::V2;
const RUNTIME_V3_CONTROL_HOST: &str = "localhost";
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
    println!("{}", render_selection_report(&report, json)?);
    Ok(())
}

fn render_selection_report(report: &RuntimeV3SelectionReport, json: bool) -> Result<String> {
    if json {
        return Ok(serde_json::to_string_pretty(report)?);
    }

    Ok(format!(
        "SELECTED_RUNTIME={}\nDEFAULT_RUNTIME={}\nSELECTOR_SOURCE={}\nDEFAULT_CHANGED={}\nSELECTION_DIFFERS_FROM_DEFAULT={}\nRUNTIME_V3_CONTROL_ENDPOINT={}\nRUNTIME_V3_KERNEL_COMMAND={}",
        report.selected_runtime,
        report.default_runtime,
        report.selector_source,
        report.default_changed,
        report.selection_differs_from_default,
        report.runtime_v3_control_endpoint,
        report.runtime_v3_kernel_command.join(" ")
    ))
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
            "https://{RUNTIME_V3_CONTROL_HOST}:{RUNTIME_V3_CONTROL_PORT}"
        ),
        runtime_v3_kernel_command: vec![
            RUNTIME_V3_KERNEL_BIN,
            "serve",
            "--init",
            "infra/runtime-v3/runtime-init.toml",
        ],
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
  Runtime v3 uses the local control API endpoint https://localhost:20997.\n\
  Launch the Runtime v3 kernel with: adl-runtime-kernel serve --init infra/runtime-v3/runtime-init.toml"
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
        assert_eq!(
            report.runtime_v3_control_endpoint,
            "https://localhost:20997"
        );
        assert_eq!(
            report.runtime_v3_kernel_command,
            [
                "adl-runtime-kernel",
                "serve",
                "--init",
                "infra/runtime-v3/runtime-init.toml"
            ]
        );
    }

    #[test]
    fn unknown_runtime_selection_fails_closed() {
        let err = RuntimeSelection::parse("v4", "--runtime").expect_err("v4 must fail closed");
        assert!(err
            .to_string()
            .contains("unsupported runtime selection 'v4'"));
    }

    #[test]
    fn runtime_aliases_are_accepted() {
        assert_eq!(
            RuntimeSelection::parse("runtime-v2", "test").unwrap(),
            RuntimeSelection::V2
        );
        assert_eq!(
            RuntimeSelection::parse("runtime-v3", "test").unwrap(),
            RuntimeSelection::V3
        );
    }

    #[test]
    fn command_help_paths_succeed() {
        assert!(runtime_v3_usage().contains("https://localhost:20997"));
        real_runtime_v3(&["help".to_string()]).unwrap();
        select_runtime(&["--help".to_string()]).unwrap();
    }

    #[test]
    fn explicit_selection_exercises_text_and_json_reports() {
        let v2_report = selection_report(RuntimeSelection::V2, "--runtime");
        let text = render_selection_report(&v2_report, false).unwrap();
        assert!(text.contains("SELECTED_RUNTIME=runtime-v2"));
        assert!(text.contains("RUNTIME_V3_CONTROL_ENDPOINT=https://localhost:20997"));
        select_runtime(&["--runtime".to_string(), "v2".to_string()]).unwrap();

        let v3_report = selection_report(RuntimeSelection::V3, "--runtime");
        let json = render_selection_report(&v3_report, true).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["selected_runtime"], "runtime-v3");
        assert_eq!(
            value["runtime_v3_control_endpoint"],
            "https://localhost:20997"
        );
        select_runtime(&[
            "--runtime".to_string(),
            "runtime-v3".to_string(),
            "--json".to_string(),
        ])
        .unwrap();
    }

    #[test]
    fn command_arguments_fail_closed() {
        let unknown_command = real_runtime_v3(&["launch".to_string()]).unwrap_err();
        assert!(unknown_command
            .to_string()
            .contains("unknown runtime-v3 command 'launch'"));

        let missing_runtime = select_runtime(&["--runtime".to_string()]).unwrap_err();
        assert!(missing_runtime
            .to_string()
            .contains("requires --runtime <v2|v3>"));

        let unknown_arg = select_runtime(&["--bogus".to_string()]).unwrap_err();
        assert!(unknown_arg
            .to_string()
            .contains("unknown arg for runtime-v3 select: --bogus"));
    }
}
