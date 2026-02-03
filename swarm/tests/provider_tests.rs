use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use swarm::adl;
use swarm::provider::{build_provider, OllamaProvider};

mod helpers;
use helpers::EnvVarGuard;

fn unique_temp_dir(prefix: &str) -> io::Result<PathBuf> {
    let mut dir = env::temp_dir();
    let uniq = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        // nanos since epoch, good enough for tests
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    dir.push(uniq);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn write_executable(path: &Path, contents: &str) -> io::Result<()> {
    fs::write(path, contents)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
    }
    Ok(())
}

fn make_mock_ollama_success(dir: &Path) -> io::Result<PathBuf> {
    let bin = dir.join("mock_ollama_ok.sh");
    // Mimic: `ollama run <model>` and read prompt from stdin.
    // We ignore args but verify shape is reasonable.
    let script = r#"#!/bin/sh
set -eu
# Expect: run <model>
if [ "${1:-}" != "run" ]; then
  echo "expected arg1=run, got '${1:-}'" 1>&2
  exit 2
fi
if [ -z "${2:-}" ]; then
  echo "expected model arg2" 1>&2
  exit 2
fi
# Consume stdin (the prompt)
cat >/dev/null
# Emit a deterministic response
echo "MOCK_COMPLETION_OK"
"#;
    write_executable(&bin, script)?;
    Ok(bin)
}

fn make_mock_ollama_failure(dir: &Path) -> io::Result<PathBuf> {
    let bin = dir.join("mock_ollama_fail.sh");
    let script = r#"#!/bin/sh
set -eu
echo "something went wrong" 1>&2
exit 42
"#;
    write_executable(&bin, script)?;
    Ok(bin)
}

fn provider_spec_from_yaml(yaml: &str) -> adl::ProviderSpec {
    serde_yaml::from_str::<adl::ProviderSpec>(yaml).expect("failed to parse ProviderSpec YAML")
}

#[test]
fn build_provider_rejects_unknown_kind() {
    let spec = provider_spec_from_yaml(
        r#"
type: definitely_not_supported
config: {}
"#,
    );

    let err = match build_provider(&spec) {
        Ok(_) => panic!("expected build_provider to fail for unknown kind"),
        Err(e) => e,
    };
    let msg = format!("{err:#}");
    assert!(
        msg.contains("unsupported provider kind"),
        "expected unsupported-kind error, got: {msg}"
    );
}

#[test]
fn ollama_from_spec_defaults_model_when_missing() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config: {}
"#,
    );

    let p = OllamaProvider::from_spec(&spec).expect("from_spec failed");
    // provider.rs uses this default
    assert_eq!(p.model, "llama3.1:8b");
    assert!(p.temperature.is_none());
}

#[test]
fn ollama_from_spec_parses_temperature_float() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
  temperature: 0.7
"#,
    );

    let p = OllamaProvider::from_spec(&spec).expect("from_spec failed");
    assert_eq!(p.model, "llama3.1:8b");
    assert_eq!(p.temperature, Some(0.7_f32));
}

#[test]
fn ollama_from_spec_parses_temperature_int() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
  temperature: 1
"#,
    );

    let p = OllamaProvider::from_spec(&spec).expect("from_spec failed");
    assert_eq!(p.temperature, Some(1.0_f32));
}

#[test]
fn ollama_from_spec_parses_temperature_string() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
  temperature: "0.25"
"#,
    );

    let p = OllamaProvider::from_spec(&spec).expect("from_spec failed");
    assert_eq!(p.temperature, Some(0.25_f32));
}

#[test]
fn build_provider_builds_ollama_provider() {
    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    let p = build_provider(&spec).expect("build_provider failed");
    let out = p
        .complete("hello") // will try real ollama unless SWARM_OLLAMA_BIN is set; just sanity check it is callable type-wise
        .err();

    // We don't actually want to require ollama installed here; this test just verifies construction works.
    // If Ollama is installed, this might return Ok(..); if not, it will Err(..). Either is fine.
    // What we *do* want is: no panic, no unsupported-kind error.
    if let Some(e) = out {
        let msg = format!("{e:#}");
        assert!(
            msg.contains("failed to spawn") || msg.contains("ollama"),
            "unexpected error from complete(): {msg}"
        );
    }
}

#[test]
fn provider_complete_uses_mock_binary_success() {
    let dir = unique_temp_dir("swarm-provider-tests").unwrap();
    let bin = make_mock_ollama_success(&dir).unwrap();

    let _env_guard = EnvVarGuard::set("SWARM_OLLAMA_BIN", &bin);

    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    let p = build_provider(&spec).expect("build_provider failed");
    let out = p
        .complete("test prompt")
        .expect("complete() should succeed with mock");
    assert!(
        out.contains("MOCK_COMPLETION_OK"),
        "expected mock output, got: {out:?}"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn provider_complete_surfaces_stderr_on_failure() {
    let dir = unique_temp_dir("swarm-provider-tests").unwrap();
    let bin = make_mock_ollama_failure(&dir).unwrap();

    let _env_guard = EnvVarGuard::set("SWARM_OLLAMA_BIN", &bin);

    let spec = provider_spec_from_yaml(
        r#"
type: ollama
config:
  model: llama3.1:8b
"#,
    );

    let p = build_provider(&spec).expect("build_provider failed");
    let err = p.complete("test prompt").unwrap_err();
    let msg = format!("{err:#}");

    assert!(
        msg.contains("ollama run failed"),
        "expected failure to mention ollama run failed, got: {msg}"
    );
    assert!(
        msg.contains("something went wrong"),
        "expected stderr to be included, got: {msg}"
    );

    let _ = fs::remove_dir_all(dir);
}

#[test]
fn env_var_guard_restores_previous_value() {
    let key = "SWARM_TEST_ENV_GUARD_RESTORE";
    let original = std::env::var_os(key);

    {
        let _guard = EnvVarGuard::set(key, "temporary");
        let val = std::env::var(key).expect("env var should be set");
        assert_eq!(val, "temporary");
    }

    {
        let _guard = EnvVarGuard::unset(key);
        assert!(std::env::var_os(key).is_none());
    }

    assert_eq!(std::env::var_os(key), original);
}
