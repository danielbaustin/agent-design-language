use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use ::adl::adl;

use super::helpers::{EnvVarGuard, EnvVarGuardMulti};

pub(super) fn write_executable(path: &Path, contents: &str) -> io::Result<()> {
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

pub(super) fn block_incoming_localhost() -> EnvVarGuard {
    let key = "NO_PROXY";
    let old = std::env::var(key).ok();
    let mut new_val = old.clone().unwrap_or_default();
    if !new_val.is_empty() && !new_val.ends_with(',') {
        new_val.push(',');
    }
    new_val.push_str("127.0.0.1,localhost");
    EnvVarGuard::set(key, new_val)
}

pub(super) fn localhost_and_auth_env_guard(key: &str, value: &str) -> EnvVarGuardMulti {
    let old = std::env::var("NO_PROXY").ok();
    let mut no_proxy = old.clone().unwrap_or_default();
    if !no_proxy.is_empty() && !no_proxy.ends_with(',') {
        no_proxy.push(',');
    }
    no_proxy.push_str("127.0.0.1,localhost");
    EnvVarGuard::set_many(&[
        ("NO_PROXY", std::ffi::OsStr::new(&no_proxy)),
        (key, std::ffi::OsStr::new(value)),
    ])
}

pub(super) fn read_http_request(stream: &mut std::net::TcpStream) -> String {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .expect("set read timeout");
    let mut bytes = Vec::new();
    let mut buf = [0u8; 1024];
    let header_end = loop {
        let n = stream.read(&mut buf).expect("read request chunk");
        assert!(n > 0, "client closed before sending complete headers");
        bytes.extend_from_slice(&buf[..n]);
        if let Some(pos) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
            break pos + 4;
        }
    };

    let headers = String::from_utf8_lossy(&bytes[..header_end]).to_string();
    if headers.lines().any(|line| {
        line.to_ascii_lowercase()
            .starts_with("expect: 100-continue")
    }) {
        stream
            .write_all(b"HTTP/1.1 100 Continue\r\n\r\n")
            .expect("write continue response");
    }
    let content_length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().expect("valid content length"))
        })
        .unwrap_or(0);
    while bytes.len() < header_end + content_length {
        let n = stream.read(&mut buf).expect("read request body");
        assert!(n > 0, "client closed before sending complete body");
        bytes.extend_from_slice(&buf[..n]);
    }
    String::from_utf8_lossy(&bytes).to_string()
}

pub(super) fn make_mock_ollama_success(dir: &Path) -> io::Result<PathBuf> {
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

pub(super) fn make_mock_ollama_failure(dir: &Path) -> io::Result<PathBuf> {
    let bin = dir.join("mock_ollama_fail.sh");
    let script = r#"#!/bin/sh
set -eu
echo "something went wrong" 1>&2
exit 42
"#;
    write_executable(&bin, script)?;
    Ok(bin)
}

pub(super) fn make_mock_ollama_sleep(dir: &Path) -> io::Result<PathBuf> {
    let bin = dir.join("mock_ollama_sleep.sh");
    let script = r#"#!/bin/sh
set -eu
sleep 2
echo "MOCK_COMPLETION_SLOW"
"#;
    write_executable(&bin, script)?;
    Ok(bin)
}

pub(super) fn provider_spec_from_yaml(yaml: &str) -> adl::ProviderSpec {
    serde_yaml::from_str::<adl::ProviderSpec>(yaml).expect("failed to parse ProviderSpec YAML")
}

pub(super) fn adl_doc_from_yaml(yaml: &str) -> adl::AdlDoc {
    serde_yaml::from_str::<adl::AdlDoc>(yaml).expect("failed to parse AdlDoc YAML")
}
