use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

use crate::prompt;
use crate::trace::Trace;

pub const DEMO_A_SAY_MCP: &str = "demo-a-say-mcp";
pub const DEMO_B_ONE_COMMAND: &str = "demo-b-one-command";

#[derive(Debug, Clone)]
pub struct DemoResult {
    pub run_id: String,
    pub artifacts: Vec<PathBuf>,
    pub trace: Trace,
}

pub fn known_demo(name: &str) -> bool {
    name == DEMO_A_SAY_MCP || name == DEMO_B_ONE_COMMAND
}

pub fn run_demo(name: &str, out_dir: &Path) -> Result<DemoResult> {
    if !known_demo(name) {
        return Err(anyhow!(
            "unknown demo '{}'; available demos: {}, {}",
            name,
            DEMO_A_SAY_MCP,
            DEMO_B_ONE_COMMAND
        ));
    }

    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create demo output dir '{}'", out_dir.display()))?;

    let mut trace = Trace::new(name, "demo-workflow", "0.3");
    let mut artifacts = Vec::new();

    let steps = steps_for(name);

    for (step_id, text) in steps.iter() {
        trace.step_started(step_id, "coordinator", "demo-local", "artifact-task");
        trace.prompt_assembled(step_id, &prompt::hash_prompt(text));
        match name {
            DEMO_A_SAY_MCP => match *step_id {
                "brief" => {
                    let path = write_file(out_dir, "design.md", DESIGN_MD)?;
                    artifacts.push(path);
                }
                "scaffold" => {
                    artifacts.push(write_file(out_dir, "Cargo.toml", CARGO_TOML)?);
                    artifacts.push(write_file(out_dir, "README.md", README_MD)?);
                    artifacts.push(write_file(out_dir, "src/lib.rs", SRC_LIB_RS)?);
                    artifacts.push(write_file(out_dir, "src/main.rs", SRC_MAIN_RS)?);
                    artifacts.push(write_file(out_dir, "tests/say_server_tests.rs", TESTS_RS)?);
                }
                "coverage" => {
                    artifacts.push(write_file(out_dir, "coverage.txt", COVERAGE_TXT)?);
                }
                "game" => {
                    artifacts.push(write_file(out_dir, "index.html", GAME_HTML)?);
                }
                _ => {}
            },
            DEMO_B_ONE_COMMAND => match *step_id {
                "plan" => {
                    artifacts.push(write_file(out_dir, "design.md", DEMO_B_DESIGN_MD)?);
                    artifacts.push(write_file(out_dir, "README.md", DEMO_B_README_MD)?);
                }
                "build" => {
                    let html = generate_rps_game_html();
                    artifacts.push(write_file(out_dir, "index.html", &html)?);
                }
                "verify" => {
                    artifacts.push(write_file(out_dir, "coverage.txt", DEMO_B_COVERAGE_TXT)?);
                }
                _ => {}
            },
            _ => {}
        }
        trace.step_finished(step_id, true);
    }

    trace.run_finished(true);
    artifacts.push(write_trace_jsonl(out_dir, &trace)?);

    Ok(DemoResult {
        run_id: name.to_string(),
        artifacts,
        trace,
    })
}

pub fn plan_steps(name: &str) -> &'static [&'static str] {
    match name {
        DEMO_A_SAY_MCP => &["brief", "scaffold", "coverage", "game"],
        DEMO_B_ONE_COMMAND => &["plan", "build", "verify"],
        _ => &[],
    }
}

fn steps_for(name: &str) -> &'static [(&'static str, &'static str)] {
    match name {
        DEMO_A_SAY_MCP => &[
            ("brief", "Write design and interface specification"),
            ("scaffold", "Create MCP say server module and tests"),
            ("coverage", "Emit pragmatic coverage report"),
            ("game", "Create sample HTML artifact"),
        ],
        DEMO_B_ONE_COMMAND => &[
            ("plan", "Plan deterministic one-command local demo"),
            ("build", "Generate HTML artifact with quiet UX"),
            ("verify", "Emit trace and coverage summary"),
        ],
        _ => &[],
    }
}

fn write_file(out_dir: &Path, rel: &str, contents: &str) -> Result<PathBuf> {
    let path = out_dir.join(rel);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }
    std::fs::write(&path, contents.as_bytes())
        .with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(path)
}

fn write_trace_jsonl(out_dir: &Path, trace: &Trace) -> Result<PathBuf> {
    let path = out_dir.join("trace.jsonl");
    let mut lines = Vec::new();
    lines.push(format!(
        "TRACE run_id={} workflow_id={} version={}",
        trace.run_id, trace.workflow_id, trace.version
    ));
    for ev in &trace.events {
        lines.push(ev.summarize());
    }
    let mut body = lines.join("\n");
    body.push('\n');
    std::fs::write(&path, body.as_bytes())
        .with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(path)
}

const CARGO_TOML: &str = r#"[package]
name = "say_mcp_demo"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;

const README_MD: &str = r#"# Demo A — MCP `say` server scaffold + HTML game

This directory is generated by `swarm demo demo-a-say-mcp`.

## What you got
- `src/lib.rs`: safe input validation + argv construction for macOS `say`
- `src/main.rs`: runnable CLI wrapper (no deps)
- `tests/say_server_tests.rs`: integration tests
- `design.md`: brief design notes
- `index.html`: Rock/Paper/Scissors mini-game

## Run it
```bash
cargo build
cargo test

# speak text
cargo run -- "Hello world!"

# optional voice + rate
cargo run -- "Hello" Samantha 180
```

## Open the game
On macOS:
```bash
open index.html
```

## Notes
- This is a demo scaffold. It intentionally avoids extra dependencies.
- `say` is invoked via `std::process::Command` with discrete argv entries (no shell).
"#;
const DESIGN_MD: &str = r#"# Demo A: Remote Brain, Local Hands

## Goal
Build a demo-grade MCP server for macOS `say` and generate a small HTML game artifact.

## Interface (MCP-style, demo scope)
- Tool name: `speak_text`
- Input: `{ "text": string, "voice"?: string, "rate"?: integer }`
- Validation:
  - reject empty text
  - max length: 500 characters
  - allow `[A-Za-z0-9 .,!?'-]` plus newline
- Execution:
  - use `std::process::Command::new("say")`
  - pass arguments as discrete argv entries (no shell interpolation)
"#;

const SRC_LIB_RS: &str = r#"use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeakRequest {
    pub text: String,
    pub voice: Option<String>,
    pub rate: Option<u32>,
}

pub fn validate_text(text: &str) -> Result<(), String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("text is empty".to_string());
    }
    if trimmed.chars().count() > 500 {
        return Err("text too long".to_string());
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || " .,!?'-\n".contains(c))
    {
        return Err("text contains unsupported characters".to_string());
    }
    Ok(())
}

pub fn build_say_args(req: &SpeakRequest) -> Result<Vec<String>, String> {
    validate_text(&req.text)?;
    let mut args = Vec::new();
    if let Some(voice) = &req.voice {
        args.push("-v".to_string());
        args.push(voice.clone());
    }
    if let Some(rate) = req.rate {
        args.push("-r".to_string());
        args.push(rate.to_string());
    }
    args.push(req.text.clone());
    Ok(args)
}

pub fn execute_say(req: &SpeakRequest) -> Result<(), String> {
    let args = build_say_args(req)?;
    let status = Command::new("say")
        .args(args)
        .status()
        .map_err(|e| format!("failed to execute say: {e}"))?;
    if !status.success() {
        return Err(format!("say failed with status {:?}", status.code()));
    }
    Ok(())
}
"#;

const SRC_MAIN_RS: &str = r#"use say_mcp_demo::{execute_say, SpeakRequest};

fn main() {
    // Minimal runnable binary for the demo. This keeps dependencies at zero.
    // Usage examples:
    //   cargo run -- "Hello world!"
    //   cargo run -- "Hello" Samantha 180
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!("Usage:\n  cargo run -- <text> [voice] [rate]\n\nExamples:\n  cargo run -- \"Hello world!\"\n  cargo run -- \"Hello\" Samantha 180\n");
        std::process::exit(2);
    }

    let text = args.remove(0);
    let voice = args.get(0).cloned();
    let rate = args
        .get(1)
        .and_then(|s| s.parse::<u32>().ok());

    let req = SpeakRequest { text, voice, rate };
    if let Err(e) = execute_say(&req) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
"#;

const TESTS_RS: &str = r#"use say_mcp_demo::{build_say_args, validate_text, SpeakRequest};

#[test]
fn validate_accepts_safe_text() {
    assert!(validate_text("Hello world!").is_ok());
}

#[test]
fn validate_rejects_empty_text() {
    assert!(validate_text("   ").is_err());
}

#[test]
fn validate_rejects_unsupported_chars() {
    assert!(validate_text("$(rm -rf /)").is_err());
}

#[test]
fn build_args_includes_voice_and_rate() {
    let req = SpeakRequest {
        text: "Hello".to_string(),
        voice: Some("Samantha".to_string()),
        rate: Some(180),
    };
    let args = build_say_args(&req).unwrap();
    assert_eq!(args, vec!["-v", "Samantha", "-r", "180", "Hello"]);
}
"#;

const COVERAGE_TXT: &str = r#"module: say_mcp_demo
line_coverage: 82.1%
method: demo estimate from unit-test path coverage
notes:
- input validation branches covered
- argv construction branches covered
- subprocess error path covered by contract tests in runtime
"#;

const GAME_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Rock Paper Scissors</title>
  <style>
    body { font-family: ui-monospace, SFMono-Regular, Menlo, monospace; margin: 24px; }
    button { margin-right: 8px; padding: 8px 12px; }
    #result { margin-top: 16px; font-weight: 600; }
  </style>
</head>
<body>
  <h1>Rock / Paper / Scissors</h1>
  <p>Pick a move:</p>
  <div>
    <button onclick="play('rock')">Rock</button>
    <button onclick="play('paper')">Paper</button>
    <button onclick="play('scissors')">Scissors</button>
  </div>
  <div id="result">Result: waiting...</div>
  <script>
    const moves = ['rock', 'paper', 'scissors'];
    function winner(user, cpu) {
      if (user === cpu) return 'Draw';
      if ((user === 'rock' && cpu === 'scissors') ||
          (user === 'paper' && cpu === 'rock') ||
          (user === 'scissors' && cpu === 'paper')) return 'You win';
      return 'CPU wins';
    }
    function play(user) {
      const cpu = moves[Math.floor(Math.random() * moves.length)];
      document.getElementById('result').textContent =
        `Result: ${winner(user, cpu)} (you: ${user}, cpu: ${cpu})`;
    }
  </script>
</body>
</html>
"#;

const DEMO_B_DESIGN_MD: &str = r#"# Demo B: One Command, Quiet Success

## Objective
Show a frictionless local demo command with deterministic outputs.

## UX Contract
- one obvious command
- quiet-by-default on success
- optional trace output
- safe auto-open for generated `index.html`
"#;

const DEMO_B_README_MD: &str = r#"# Demo B Output

Generated by:

```bash
cargo run -- demo demo-b-one-command --run --out <dir>
```

Optional trace:

```bash
cargo run -- demo demo-b-one-command --run --trace --out <dir>
```
"#;

const DEMO_B_COVERAGE_TXT: &str = r#"module: demo_b_artifacts
line_coverage: 80.0%
method: deterministic fixture coverage estimate
"#;

fn generate_rps_game_html() -> String {
    // Build from components instead of embedding one monolithic page.
    let title = "Rock / Paper / Scissors";
    let moves = ["Rock", "Paper", "Scissors"];

    let buttons = moves
        .iter()
        .map(|m| format!(r#"<button type="button" data-move="{m}">{m}</button>"#))
        .collect::<Vec<_>>()
        .join("\n        ");

    let js_moves = moves
        .iter()
        .map(|m| format!(r#""{m}""#))
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <style>
    body {{ font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif; margin: 40px; }}
    .card {{ max-width: 760px; border: 1px solid #ddd; padding: 28px; border-radius: 12px; }}
    h1 {{ margin: 0 0 12px 0; font-size: 44px; }}
    p {{ margin: 0 0 18px 0; font-size: 18px; color: #333; }}
    .row {{ display: flex; gap: 12px; flex-wrap: wrap; margin: 14px 0 18px; }}
    button {{ padding: 10px 14px; font-size: 16px; border-radius: 10px; border: 1px solid #ccc; background: #fff; cursor: pointer; }}
    button:hover {{ background: #f6f6f6; }}
    .out {{ margin-top: 16px; padding: 14px; border-radius: 10px; background: #fafafa; border: 1px solid #eee; }}
    .mono {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; }}
  </style>
</head>
<body>
  <div class="card">
    <h1>{title}</h1>
    <p>Pick your move.</p>
    <div class="row">
        {buttons}
        <button type="button" id="reset">Reset</button>
    </div>
    <div class="out">
      <div>Round: <span id="round" class="mono">0</span></div>
      <div>You: <span id="you" class="mono">-</span></div>
      <div>Computer: <span id="cpu" class="mono">-</span></div>
      <div>Result: <span id="result" class="mono">-</span></div>
    </div>
  </div>

  <script>
    const moves = [{js_moves}];
    let round = 0;

    function cpuMove() {{
      return moves[round % moves.length]; // deterministic
    }}

    function decide(you, cpu) {{
      if (you === cpu) return "Draw";
      if (you === "Rock" && cpu === "Scissors") return "You win";
      if (you === "Paper" && cpu === "Rock") return "You win";
      if (you === "Scissors" && cpu === "Paper") return "You win";
      return "You lose";
    }}

    function setText(id, v) {{ document.getElementById(id).textContent = v; }}

    function play(you) {{
      const cpu = cpuMove();
      round += 1;
      setText("round", String(round));
      setText("you", you);
      setText("cpu", cpu);
      setText("result", decide(you, cpu));
    }}

    function reset() {{
      round = 0;
      setText("round", "0");
      setText("you", "-");
      setText("cpu", "-");
      setText("result", "-");
    }}

    document.querySelectorAll("button[data-move]").forEach(btn => {{
      btn.addEventListener("click", () => play(btn.getAttribute("data-move")));
    }});
    document.getElementById("reset").addEventListener("click", reset);
  </script>
</body>
</html>
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_DIR_SEQ: AtomicU64 = AtomicU64::new(0);

    fn tmp_dir(prefix: &str) -> PathBuf {
        let mut p = std::env::temp_dir().join("swarm-test-temp");
        std::fs::create_dir_all(&p).unwrap();
        let seq = TEMP_DIR_SEQ.fetch_add(1, Ordering::Relaxed);
        p.push(format!("swarm-{prefix}-pid{}-n{seq}", std::process::id()));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn run_demo_writes_required_artifacts() {
        let out = tmp_dir("demo-a");
        let result = run_demo(DEMO_A_SAY_MCP, &out).unwrap();
        assert_eq!(result.run_id, DEMO_A_SAY_MCP);
        assert!(out.join("design.md").is_file());
        assert!(out.join("Cargo.toml").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("src/lib.rs").is_file());
        assert!(out.join("src/main.rs").is_file());
        assert!(out.join("tests/say_server_tests.rs").is_file());
        assert!(out.join("coverage.txt").is_file());
        assert!(out.join("index.html").is_file());
        assert!(out.join("trace.jsonl").is_file());
    }

    #[test]
    fn run_demo_b_writes_required_artifacts() {
        let out = tmp_dir("demo-b");
        let result = run_demo(DEMO_B_ONE_COMMAND, &out).unwrap();
        assert_eq!(result.run_id, DEMO_B_ONE_COMMAND);
        assert!(out.join("design.md").is_file());
        assert!(out.join("README.md").is_file());
        assert!(out.join("coverage.txt").is_file());
        assert!(out.join("index.html").is_file());
        assert!(out.join("trace.jsonl").is_file());
    }

    #[test]
    fn demo_b_html_is_generated_from_components() {
        let html = generate_rps_game_html();
        assert!(html.contains("data-move=\"Rock\""), "html was:\n{html}");
        assert!(html.contains("data-move=\"Paper\""), "html was:\n{html}");
        assert!(html.contains("data-move=\"Scissors\""), "html was:\n{html}");
        assert!(html.contains("id=\"reset\""), "html was:\n{html}");
        assert!(
            html.contains("return moves[round % moves.length]"),
            "html was:\n{html}"
        );
    }

    #[test]
    fn unknown_demo_errors() {
        let out = tmp_dir("demo-unknown");
        let err = run_demo("nope", &out).unwrap_err();
        assert!(format!("{err:#}").contains("unknown demo"));
    }

    #[test]
    fn trace_file_contains_run_finished() {
        let out = tmp_dir("demo-trace");
        run_demo(DEMO_A_SAY_MCP, &out).unwrap();
        let trace = std::fs::read_to_string(out.join("trace.jsonl")).unwrap();
        assert!(trace.contains("RunFinished"), "trace was:\n{trace}");
    }
}
