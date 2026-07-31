use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::Write,
    net::{SocketAddr, ToSocketAddrs},
    path::{Path, PathBuf},
    process::{ExitCode, Stdio},
    sync::Arc,
    time::{Duration, Instant},
};

use adl_runtime::guardian::{GuardianOutcome, GuardianTerminalState};
use adl_runtime_kernel::verify_live_continuity_lineage;
use ed25519_dalek::{SigningKey, VerifyingKey};
use fs2::FileExt;
use rcgen::{date_time_ymd, BasicConstraints, CertificateParams, CertifiedIssuer, IsCa, KeyPair};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStderr, ChildStdout, Command};
use tokio_rustls::rustls::{
    pki_types::{CertificateDer, ServerName},
    ClientConfig, RootCertStore,
};

#[cfg(windows)]
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows_sys::Win32::System::{
    Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT},
    Threading::{
        OpenProcess, TerminateProcess, CREATE_NEW_PROCESS_GROUP, PROCESS_QUERY_LIMITED_INFORMATION,
        PROCESS_TERMINATE,
    },
};

const REPORT_SCHEMA: &str = "adl.runtime_v3.lifecycle_soak.v1";
const REQUIRED_CYCLES: u64 = 10_000;
const STRESS_RUNS: u64 = 100;
const STRESS_SECONDS: u64 = 10;
const ENDURANCE_RUNS: u64 = 10;
const ENDURANCE_SECONDS: u64 = 600;
const PLATFORM_PROOF_SCHEMA: &str = "adl.wp12.platform_proof.v1";

#[tokio::main]
async fn main() -> ExitCode {
    let raw_args = std::env::args().skip(1).collect::<Vec<_>>();
    if raw_args.iter().any(|arg| arg == "--aggregate-platform") {
        return match AggregateArgs::parse(raw_args.into_iter()) {
            Ok(args) => aggregate_platform(&args),
            Err(error) => {
                eprintln!("{error}");
                ExitCode::from(64)
            }
        };
    }

    let args = match Args::parse(raw_args.into_iter()) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    if let Err(error) = prepare_state_root(&args.state_root) {
        eprintln!("{error}");
        return ExitCode::from(64);
    }

    let started = Instant::now();
    let kernel_sha256 = match file_sha256(&args.kernel) {
        Ok(digest) => digest,
        Err(error) => {
            eprintln!("failed hashing Runtime v3 kernel: {error}");
            return ExitCode::from(66);
        }
    };
    let fixture = match ProductionFixture::create(
        &args.state_root,
        &args.init_template,
        &args.kernel,
        &args.vector,
        args.suite,
        &args.revision,
    ) {
        Ok(fixture) => fixture,
        Err(error) => {
            eprintln!("failed preparing production Runtime v3 launch: {error}");
            return ExitCode::from(66);
        }
    };
    let _qualification_lock = match QualificationLock::acquire(&args.init_template, fixture.address)
    {
        Ok(lock) => lock,
        Err(error) => {
            eprintln!("failed acquiring lifecycle qualification lock: {error}");
            return ExitCode::from(75);
        }
    };

    let execution = match execute_suite(&args, &fixture, started).await {
        Ok(execution) => execution,
        Err(failure) => return fail(&args, &kernel_sha256, started, failure),
    };

    let report = report(&args, &kernel_sha256, started, "pass", &execution, None);
    if let Err(error) = write_report(&args.report, &report) {
        eprintln!("failed writing lifecycle report: {error}");
        return ExitCode::from(66);
    }
    println!("{report}");
    ExitCode::SUCCESS
}

struct Args {
    guardian: PathBuf,
    kernel: PathBuf,
    vector: PathBuf,
    init_template: PathBuf,
    state_root: PathBuf,
    report: PathBuf,
    revision: String,
    suite: Suite,
}

struct ProductionFixture {
    address: SocketAddr,
    init: PathBuf,
    continuity_root: PathBuf,
    local_state_root: PathBuf,
    observability_root: PathBuf,
    master_log: PathBuf,
    log_audit: PathBuf,
    tls_connector: tokio_rustls::TlsConnector,
    continuity_verifying_key: VerifyingKey,
    observatory_token: String,
    readiness_timeout: Duration,
    readiness_poll: Duration,
    shutdown_wait: Duration,
}

#[derive(Debug)]
struct QualificationLock {
    file: File,
}

impl QualificationLock {
    fn acquire(init_template: &Path, address: SocketAddr) -> Result<Self, String> {
        let init_template = init_template.canonicalize().map_err(|error| {
            format!(
                "init template {} could not be canonicalized: {error}",
                init_template.display()
            )
        })?;
        let repository_root = init_template
            .ancestors()
            .find(|candidate| candidate.join(".git").exists())
            .ok_or_else(|| {
                format!(
                    "init template {} is not inside a Git worktree",
                    init_template.display()
                )
            })?;
        let lock_dir = repository_root
            .join(".adl")
            .join("runtime-v3")
            .join("qualification");
        std::fs::create_dir_all(&lock_dir).map_err(|error| {
            format!(
                "could not create qualification lock directory {}: {error}",
                lock_dir.display()
            )
        })?;
        let address_key = address.to_string().replace([':', '[', ']'], "_");
        Self::acquire_at(&lock_dir.join(format!("api-{address_key}.lock")), address)
    }

    fn acquire_at(path: &Path, address: SocketAddr) -> Result<Self, String> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)
            .map_err(|error| {
                format!(
                    "could not open qualification lock {}: {error}",
                    path.display()
                )
            })?;
        file.try_lock_exclusive().map_err(|error| {
            format!(
                "another lifecycle qualification owns configured API address {address} \
                 (lock {}): {error}",
                path.display()
            )
        })?;
        file.set_len(0)
            .and_then(|()| write!(file, "pid={}\naddress={address}\n", std::process::id()))
            .and_then(|()| file.sync_data())
            .map_err(|error| {
                let _ = FileExt::unlock(&file);
                format!(
                    "could not record qualification lock owner in {}: {error}",
                    path.display()
                )
            })?;
        Ok(Self { file })
    }
}

impl Drop for QualificationLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

impl ProductionFixture {
    fn create(
        state_root: &Path,
        init_template: &Path,
        kernel: &Path,
        vector: &Path,
        suite: Suite,
        revision: &str,
    ) -> Result<Self, String> {
        let template_text = std::fs::read_to_string(init_template).map_err(|error| {
            format!(
                "could not read init template {}: {error}",
                init_template.display()
            )
        })?;
        let mut init_document = toml::from_str::<toml::Value>(&template_text).map_err(|error| {
            format!("invalid init template {}: {error}", init_template.display())
        })?;
        let configured_address = toml_string(&init_document, &["api", "address"])?;
        let readiness_timeout = Duration::from_millis(toml_u64(
            &init_document,
            &["qualification", "readiness_timeout_millis"],
        )?);
        let readiness_poll = Duration::from_millis(toml_u64(
            &init_document,
            &["qualification", "readiness_poll_millis"],
        )?);
        let shutdown_wait = Duration::from_millis(toml_u64(
            &init_document,
            &["qualification", "shutdown_wait_millis"],
        )?);
        let address = configured_address
            .to_socket_addrs()
            .map_err(|error| format!("invalid configured API address: {error}"))?
            .find(SocketAddr::is_ipv4)
            .or_else(|| configured_address.to_socket_addrs().ok()?.next())
            .ok_or_else(|| "configured API address did not resolve".to_owned())?;
        let state_root = state_root
            .canonicalize()
            .map_err(|error| format!("state root could not be canonicalized: {error}"))?;
        let tls_root = state_root.join(toml_string(&init_document, &["paths", "tls_dir"])?);
        let continuity_root =
            state_root.join(toml_string(&init_document, &["paths", "continuity_dir"])?);
        let credentials_root =
            state_root.join(toml_string(&init_document, &["paths", "credentials_dir"])?);
        let observability_root = state_root.join(toml_string(
            &init_document,
            &["paths", "observability_dir"],
        )?);
        let master_log = observability_root.join(toml_string(
            &init_document,
            &["observability_pipeline", "master_log_path"],
        )?);
        let log_audit = observability_root.join(toml_string(
            &init_document,
            &["observability_pipeline", "audit_path"],
        )?);
        for path in [
            &tls_root,
            &continuity_root,
            &credentials_root,
            &observability_root,
        ] {
            std::fs::create_dir_all(path)
                .map_err(|error| format!("could not create {}: {error}", path.display()))?;
        }

        let mut ca_params = CertificateParams::new(["adl-runtime-v3-wp12-ca".to_owned()])
            .map_err(|error| error.to_string())?;
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        ca_params.not_before = date_time_ymd(2026, 1, 1);
        ca_params.not_after = date_time_ymd(2036, 1, 1);
        let ca_key = KeyPair::generate().map_err(|error| error.to_string())?;
        let ca =
            CertifiedIssuer::self_signed(ca_params, ca_key).map_err(|error| error.to_string())?;
        let leaf_key = KeyPair::generate().map_err(|error| error.to_string())?;
        let mut leaf_params = CertificateParams::new([
            "localhost".to_owned(),
            "127.0.0.1".to_owned(),
            "::1".to_owned(),
        ])
        .map_err(|error| error.to_string())?;
        leaf_params.not_before = date_time_ymd(2026, 1, 1);
        leaf_params.not_after = date_time_ymd(2036, 1, 1);
        let leaf = leaf_params
            .signed_by(&leaf_key, &ca)
            .map_err(|error| error.to_string())?;
        let certificate = tls_root.join(toml_file_name(
            &init_document,
            &["api", "tls", "certificate_chain_path"],
        )?);
        let private_key = tls_root.join(toml_file_name(
            &init_document,
            &["api", "tls", "private_key_path"],
        )?);
        std::fs::write(&certificate, leaf.pem()).map_err(|error| error.to_string())?;
        write_secret(&private_key, leaf_key.serialize_pem().as_bytes())
            .map_err(|error| error.to_string())?;

        let control_key = SigningKey::from_bytes(&[17_u8; 32]);
        let operation_key = SigningKey::from_bytes(&[29_u8; 32]);
        let continuity_key = SigningKey::from_bytes(&[23_u8; 32]);
        let control_public_key = hex::encode(control_key.verifying_key().as_bytes());
        let operation_public_key = hex::encode(operation_key.verifying_key().as_bytes());
        let continuity_signing_key = hex::encode([23_u8; 32]);
        let observatory_token = "wp12-observatory-token-000000000001".to_owned();
        let control_public_key_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "control_public_key_path"],
        )?);
        let operation_public_key_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "operation_public_key_path"],
        )?);
        let continuity_signing_key_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "continuity_signing_key_path"],
        )?);
        let observatory_token_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "observatory_token_path"],
        )?);
        std::fs::write(&control_public_key_path, &control_public_key)
            .map_err(|error| error.to_string())?;
        std::fs::write(&operation_public_key_path, &operation_public_key)
            .map_err(|error| error.to_string())?;
        write_secret(
            &continuity_signing_key_path,
            continuity_signing_key.as_bytes(),
        )
        .map_err(|error| error.to_string())?;
        write_secret(&observatory_token_path, observatory_token.as_bytes())
            .map_err(|error| error.to_string())?;

        set_toml_string(&mut init_document, &["state_root"], toml_path(&state_root)?)?;
        set_toml_string(
            &mut init_document,
            &["binaries", "kernel_path"],
            toml_path(kernel)?,
        )?;
        set_toml_string(
            &mut init_document,
            &["observability_pipeline", "vector_binary_path"],
            toml_path(vector)?,
        )?;
        set_toml_string(
            &mut init_document,
            &["api", "tls", "certificate_chain_path"],
            toml_path(&certificate)?,
        )?;
        set_toml_string(
            &mut init_document,
            &["api", "tls", "private_key_path"],
            toml_path(&private_key)?,
        )?;
        for (field, path) in [
            ("control_public_key_path", &control_public_key_path),
            ("operation_public_key_path", &operation_public_key_path),
            ("continuity_signing_key_path", &continuity_signing_key_path),
            ("observatory_token_path", &observatory_token_path),
        ] {
            set_toml_string(
                &mut init_document,
                &["credentials", field],
                toml_path(path)?,
            )?;
        }
        for (field, value) in [
            ("revision", revision),
            ("lifecycle_suite", suite.name()),
            ("lifecycle_run", revision),
            ("lifecycle_cycle", suite.name()),
        ] {
            set_toml_string(
                &mut init_document,
                &["observability_pipeline", field],
                value.to_owned(),
            )?;
        }
        let init = state_root.join("runtime-init.toml");
        std::fs::write(
            &init,
            toml::to_string_pretty(&init_document).map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;

        let mut roots = RootCertStore::empty();
        roots
            .add(CertificateDer::from(ca.der().to_vec()))
            .map_err(|error| error.to_string())?;
        let client_config = ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        Ok(Self {
            address,
            init,
            continuity_root,
            local_state_root: state_root,
            observability_root,
            master_log,
            log_audit,
            tls_connector: tokio_rustls::TlsConnector::from(Arc::new(client_config)),
            continuity_verifying_key: continuity_key.verifying_key(),
            observatory_token,
            readiness_timeout,
            readiness_poll,
            shutdown_wait,
        })
    }

    fn configure_cycle(
        &self,
        args: &Args,
        run: u64,
        cycle: u64,
        minimum_generation: u64,
    ) -> Result<(), String> {
        let text = std::fs::read_to_string(&self.init)
            .map_err(|error| format!("runtime init became unreadable: {error}"))?;
        let mut document = toml::from_str::<toml::Value>(&text)
            .map_err(|error| format!("runtime init became invalid: {error}"))?;
        set_toml_string(
            &mut document,
            &["observability_pipeline", "lifecycle_run"],
            format!("{}:run-{run}", args.revision),
        )?;
        set_toml_string(
            &mut document,
            &["observability_pipeline", "lifecycle_cycle"],
            format!("{}:run-{run}:cycle-{cycle}", args.suite.name()),
        )?;
        set_toml_integer(
            &mut document,
            &["credentials", "continuity_min_generation"],
            minimum_generation,
        )?;
        std::fs::write(
            &self.init,
            toml::to_string_pretty(&document)
                .map_err(|error| format!("runtime init could not be encoded: {error}"))?,
        )
        .map_err(|error| format!("runtime init cycle update failed: {error}"))
    }
}

fn toml_string<'a>(document: &'a toml::Value, path: &[&str]) -> Result<&'a str, String> {
    let mut value = document;
    for segment in path {
        value = value
            .get(*segment)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    value
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            format!(
                "init template {} must be a non-empty string",
                path.join(".")
            )
        })
}

fn toml_u64(document: &toml::Value, path: &[&str]) -> Result<u64, String> {
    let mut value = document;
    for segment in path {
        value = value
            .get(*segment)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    value
        .as_integer()
        .and_then(|value| u64::try_from(value).ok())
        .filter(|value| *value > 0)
        .ok_or_else(|| {
            format!(
                "init template {} must be a positive integer",
                path.join(".")
            )
        })
}

fn toml_file_name(document: &toml::Value, path: &[&str]) -> Result<std::ffi::OsString, String> {
    Path::new(toml_string(document, path)?)
        .file_name()
        .map(std::ffi::OsStr::to_os_string)
        .ok_or_else(|| format!("init template {} has no file name", path.join(".")))
}

fn set_toml_string(document: &mut toml::Value, path: &[&str], value: String) -> Result<(), String> {
    let (field, parents) = path
        .split_last()
        .ok_or_else(|| "empty TOML path".to_owned())?;
    let mut table = document
        .as_table_mut()
        .ok_or_else(|| "init template root must be a TOML table".to_owned())?;
    for segment in parents {
        table = table
            .get_mut(*segment)
            .and_then(toml::Value::as_table_mut)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    let slot = table
        .get_mut(*field)
        .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    *slot = toml::Value::String(value);
    Ok(())
}

fn set_toml_integer(document: &mut toml::Value, path: &[&str], value: u64) -> Result<(), String> {
    let value = i64::try_from(value).map_err(|_| format!("{} overflowed", path.join(".")))?;
    let (field, parents) = path
        .split_last()
        .ok_or_else(|| "empty TOML path".to_owned())?;
    let mut table = document
        .as_table_mut()
        .ok_or_else(|| "init template root must be a TOML table".to_owned())?;
    for segment in parents {
        table = table
            .get_mut(*segment)
            .and_then(toml::Value::as_table_mut)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    let slot = table
        .get_mut(*field)
        .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    *slot = toml::Value::Integer(value);
    Ok(())
}

#[derive(Clone, Copy)]
enum Suite {
    Preflight,
    Lifecycle { cycles: u64 },
    Stress { runs: u64, seconds: u64 },
    Endurance { runs: u64, seconds: u64 },
}

impl Suite {
    fn name(self) -> &'static str {
        match self {
            Self::Preflight => "preflight_1x",
            Self::Lifecycle { .. } => "lifecycle_10000",
            Self::Stress { .. } => "stress_100x10s",
            Self::Endurance { .. } => "endurance_10x600s",
        }
    }
}

impl Args {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut guardian = None;
        let mut kernel = None;
        let mut vector = None;
        let mut init_template = None;
        let mut state_root = None;
        let mut report = None;
        let mut revision = None;
        let mut suite = None;
        while let Some(argument) = args.next() {
            let value = |args: &mut dyn Iterator<Item = String>, name: &str| {
                args.next()
                    .ok_or_else(|| format!("{name} requires a value"))
            };
            match argument.as_str() {
                "--guardian" => guardian = Some(PathBuf::from(value(&mut args, "--guardian")?)),
                "--kernel" => kernel = Some(PathBuf::from(value(&mut args, "--kernel")?)),
                "--vector" => vector = Some(PathBuf::from(value(&mut args, "--vector")?)),
                "--init-template" => {
                    init_template = Some(PathBuf::from(value(&mut args, "--init-template")?))
                }
                "--state-root" => {
                    state_root = Some(PathBuf::from(value(&mut args, "--state-root")?))
                }
                "--report" => report = Some(PathBuf::from(value(&mut args, "--report")?)),
                "--revision" => revision = Some(value(&mut args, "--revision")?),
                "--suite" => {
                    if suite.is_some() {
                        return Err("--suite accepts exactly one value".to_owned());
                    }
                    suite = Some(match value(&mut args, "--suite")?.as_str() {
                        "preflight" | "preflight_1x" => Suite::Preflight,
                        "lifecycle" | "lifecycle_10000" => Suite::Lifecycle {
                            cycles: REQUIRED_CYCLES,
                        },
                        "stress" | "stress_100x10s" => Suite::Stress {
                            runs: STRESS_RUNS,
                            seconds: STRESS_SECONDS,
                        },
                        "endurance" | "endurance_10x600s" => Suite::Endurance {
                            runs: ENDURANCE_RUNS,
                            seconds: ENDURANCE_SECONDS,
                        },
                        other => return Err(format!("unsupported lifecycle soak suite: {other}")),
                    });
                }
                _ => return Err(format!("unknown lifecycle soak option: {argument}")),
            }
        }
        let guardian = guardian.ok_or_else(|| "--guardian is required".to_owned())?;
        let kernel = kernel.ok_or_else(|| "--kernel is required".to_owned())?;
        let vector = vector.ok_or_else(|| "--vector is required".to_owned())?;
        let init_template =
            init_template.ok_or_else(|| "--init-template is required".to_owned())?;
        let state_root = state_root.ok_or_else(|| "--state-root is required".to_owned())?;
        let report = report.ok_or_else(|| "--report is required".to_owned())?;
        let revision = revision.ok_or_else(|| "--revision is required".to_owned())?;
        if !guardian.is_absolute() || !guardian.is_file() {
            return Err("--guardian must be an absolute existing file".to_owned());
        }
        if !kernel.is_absolute() || !kernel.is_file() {
            return Err("--kernel must be an absolute existing file".to_owned());
        }
        if !vector.is_absolute() || !vector.is_file() {
            return Err("--vector must be an absolute existing file".to_owned());
        }
        if !init_template.is_absolute() || !init_template.is_file() {
            return Err("--init-template must be an absolute existing file".to_owned());
        }
        if !state_root.is_absolute() || !report.is_absolute() {
            return Err("--state-root and --report must be absolute paths".to_owned());
        }
        let suite = suite.unwrap_or(Suite::Lifecycle {
            cycles: REQUIRED_CYCLES,
        });
        if revision.len() != 40
            || !revision
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err("--revision must be a lowercase 40-character Git SHA".to_owned());
        }
        Ok(Self {
            guardian,
            kernel,
            vector,
            init_template,
            state_root,
            report,
            revision,
            suite,
        })
    }
}

struct AggregateArgs {
    preflight_report: PathBuf,
    lifecycle_report: PathBuf,
    stress_report: PathBuf,
    endurance_report: PathBuf,
    output: PathBuf,
}

impl AggregateArgs {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut aggregate = false;
        let mut preflight_report = None;
        let mut lifecycle_report = None;
        let mut stress_report = None;
        let mut endurance_report = None;
        let mut output = None;
        while let Some(argument) = args.next() {
            let value = |args: &mut dyn Iterator<Item = String>, name: &str| {
                args.next()
                    .ok_or_else(|| format!("{name} requires a value"))
            };
            match argument.as_str() {
                "--aggregate-platform" => aggregate = true,
                "--preflight-report" => {
                    preflight_report = Some(PathBuf::from(value(&mut args, "--preflight-report")?))
                }
                "--lifecycle-report" => {
                    lifecycle_report = Some(PathBuf::from(value(&mut args, "--lifecycle-report")?))
                }
                "--stress-report" => {
                    stress_report = Some(PathBuf::from(value(&mut args, "--stress-report")?))
                }
                "--endurance-report" => {
                    endurance_report = Some(PathBuf::from(value(&mut args, "--endurance-report")?))
                }
                "--output" => output = Some(PathBuf::from(value(&mut args, "--output")?)),
                _ => return Err(format!("unknown platform aggregation option: {argument}")),
            }
        }
        if !aggregate {
            return Err("--aggregate-platform is required".to_owned());
        }
        let args = Self {
            preflight_report: preflight_report
                .ok_or_else(|| "--preflight-report is required".to_owned())?,
            lifecycle_report: lifecycle_report
                .ok_or_else(|| "--lifecycle-report is required".to_owned())?,
            stress_report: stress_report.ok_or_else(|| "--stress-report is required".to_owned())?,
            endurance_report: endurance_report
                .ok_or_else(|| "--endurance-report is required".to_owned())?,
            output: output.ok_or_else(|| "--output is required".to_owned())?,
        };
        for path in [
            &args.preflight_report,
            &args.lifecycle_report,
            &args.stress_report,
            &args.endurance_report,
        ] {
            if !path.is_file() {
                return Err(format!("report does not exist: {}", path.display()));
            }
        }
        if !args.output.is_absolute() {
            return Err(
                "--output must be an absolute path for atomic platform proof writes".to_owned(),
            );
        }
        Ok(args)
    }
}

struct Execution {
    completed_runs: u64,
    completed_cycles: u64,
    continuity_generation: u64,
    minimum_cycles_per_run: u64,
    guardian_pids: BTreeSet<u32>,
    runtime_instance_ids: BTreeSet<String>,
    guardian_launches: u64,
    runtime_starts: u64,
    anti_rollback_minimum_enforced: bool,
    restart_budget_exercised: bool,
    total_restarts: u64,
    log_checked_cycles: u64,
    log_proof: Option<LogProof>,
}

impl Execution {
    fn new(completed_runs: u64, continuity_generation: u64, minimum_cycles_per_run: u64) -> Self {
        Self {
            completed_runs,
            completed_cycles: 0,
            continuity_generation,
            minimum_cycles_per_run,
            guardian_pids: BTreeSet::new(),
            runtime_instance_ids: BTreeSet::new(),
            guardian_launches: 0,
            runtime_starts: 0,
            anti_rollback_minimum_enforced: false,
            restart_budget_exercised: false,
            total_restarts: 0,
            log_checked_cycles: 0,
            log_proof: None,
        }
    }

    fn record_cycle(&mut self, observation: CycleObservation) {
        self.completed_cycles = self.completed_cycles.saturating_add(1);
        self.guardian_pids.insert(observation.guardian_pid);
        self.runtime_instance_ids
            .extend(observation.runtime_instance_ids);
        self.guardian_launches = self.guardian_launches.saturating_add(1);
        self.runtime_starts = self
            .runtime_starts
            .saturating_add(observation.runtime_starts);
        self.anti_rollback_minimum_enforced |= observation.anti_rollback_minimum_enforced;
        self.restart_budget_exercised |= observation.restart_budget_exercised;
        self.total_restarts = self.total_restarts.saturating_add(observation.restarts);
        self.log_checked_cycles = self.log_checked_cycles.saturating_add(1);
        self.log_proof = Some(observation.log_proof);
    }
}

struct CycleObservation {
    guardian_pid: u32,
    runtime_instance_ids: Vec<String>,
    runtime_starts: u64,
    anti_rollback_minimum_enforced: bool,
    restart_budget_exercised: bool,
    restarts: u64,
    log_proof: LogProof,
}

struct CapturedOutput {
    stdout: tokio::task::JoinHandle<std::io::Result<Vec<u8>>>,
    stderr: tokio::task::JoinHandle<std::io::Result<Vec<u8>>>,
}

impl CapturedOutput {
    fn take(child: &mut Child) -> Result<Self, String> {
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Guardian stdout capture was unavailable".to_owned())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "Guardian stderr capture was unavailable".to_owned())?;
        Ok(Self {
            stdout: tokio::spawn(read_stdout(stdout)),
            stderr: tokio::spawn(read_stderr(stderr)),
        })
    }

    async fn collect(self) -> Result<(Vec<u8>, Vec<u8>), String> {
        let stdout = self
            .stdout
            .await
            .map_err(|error| format!("Guardian stdout task failed: {error}"))?
            .map_err(|error| format!("Guardian stdout read failed: {error}"))?;
        let stderr = self
            .stderr
            .await
            .map_err(|error| format!("Guardian stderr task failed: {error}"))?
            .map_err(|error| format!("Guardian stderr read failed: {error}"))?;
        Ok((stdout, stderr))
    }
}

async fn read_stdout(mut stream: ChildStdout) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).await?;
    Ok(bytes)
}

async fn read_stderr(mut stream: ChildStderr) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).await?;
    Ok(bytes)
}

async fn finish_guardian(
    guardian: &mut Child,
    captured: CapturedOutput,
    shutdown_wait: Duration,
    runtime_process_id: Option<u32>,
) -> Result<std::process::Output, String> {
    let status = match tokio::time::timeout(shutdown_wait, guardian.wait()).await {
        Ok(result) => result.map_err(|error| format!("Guardian process wait failed: {error}"))?,
        Err(_) => {
            if let Some(pid) = runtime_process_id {
                let _ = force_runtime_exit(pid);
            }
            let _ = guardian.start_kill();
            let _ = tokio::time::timeout(shutdown_wait, guardian.wait()).await;
            let (stdout, stderr) = captured.collect().await?;
            return Err(format!(
                "Guardian did not complete production shutdown; guardian_stdout={}; guardian_stderr={}",
                diagnostic_tail(&String::from_utf8_lossy(&stdout), Path::new(".")),
                diagnostic_tail(&String::from_utf8_lossy(&stderr), Path::new("."))
            ));
        }
    };
    let (stdout, stderr) = captured.collect().await?;
    Ok(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

struct LogProof {
    master_log_ref: String,
    master_log_sha256: String,
    master_log_records: u64,
    log_audit_ref: String,
    log_audit_sha256: String,
}

struct Failure {
    run: u64,
    cycle: u64,
    completed_runs: u64,
    completed_cycles: u64,
    error: String,
}

async fn execute_suite(
    args: &Args,
    fixture: &ProductionFixture,
    started: Instant,
) -> Result<Execution, Failure> {
    match args.suite {
        Suite::Preflight => {
            let observation = execute_cycle(args, fixture, 1, 1, 1, true, true)
                .await
                .map_err(|error| Failure {
                    run: 1,
                    cycle: 1,
                    completed_runs: 0,
                    completed_cycles: 0,
                    error,
                })?;
            let mut execution = Execution::new(1, 1, 1);
            execution.record_cycle(observation);
            Ok(execution)
        }
        Suite::Lifecycle { cycles } => {
            let mut execution = Execution::new(1, cycles, cycles);
            for cycle in 1..=cycles {
                let observation =
                    execute_cycle(args, fixture, 1, cycle, cycle, cycle == cycles, cycle == 1)
                        .await
                        .map_err(|error| Failure {
                            run: 1,
                            cycle,
                            completed_runs: 0,
                            completed_cycles: cycle.saturating_sub(1),
                            error,
                        })?;
                execution.record_cycle(observation);
                if cycle % 1_000 == 0 {
                    eprintln!("guardian_runtime_lifecycle_progress={cycle}/{cycles}");
                }
            }
            verify_continuity_chain(
                &fixture.continuity_root,
                cycles,
                &fixture.continuity_verifying_key,
            )
            .await
            .map_err(|error| Failure {
                run: 1,
                cycle: cycles,
                completed_runs: 0,
                completed_cycles: cycles,
                error,
            })?;
            Ok(execution)
        }
        Suite::Stress { runs, seconds } | Suite::Endurance { runs, seconds } => {
            let mut total_cycles = 0_u64;
            let mut minimum_cycles_per_run = u64::MAX;
            let mut execution = Execution::new(runs, 0, 0);
            for run in 1..=runs {
                if run > 1 {
                    discard_checked_observability(&fixture.observability_root).map_err(
                        |error| Failure {
                            run,
                            cycle: 1,
                            completed_runs: run.saturating_sub(1),
                            completed_cycles: total_cycles,
                            error,
                        },
                    )?;
                }
                let deadline = Instant::now() + Duration::from_secs(seconds);
                let mut run_cycles = 0_u64;
                while run_cycles == 0 || Instant::now() < deadline {
                    run_cycles = run_cycles.saturating_add(1);
                    let expected_generation = total_cycles.saturating_add(run_cycles);
                    let observation = execute_cycle(
                        args,
                        fixture,
                        run,
                        run_cycles,
                        expected_generation,
                        false,
                        run == 1 && run_cycles == 1,
                    )
                    .await
                    .map_err(|error| Failure {
                        run,
                        cycle: run_cycles,
                        completed_runs: run.saturating_sub(1),
                        completed_cycles: total_cycles + run_cycles.saturating_sub(1),
                        error,
                    })?;
                    execution.record_cycle(observation);
                }
                run_cycles = run_cycles.saturating_add(1);
                let expected_generation = total_cycles.saturating_add(run_cycles);
                let observation = execute_cycle(
                    args,
                    fixture,
                    run,
                    run_cycles,
                    expected_generation,
                    true,
                    false,
                )
                .await
                .map_err(|error| Failure {
                    run,
                    cycle: run_cycles,
                    completed_runs: run.saturating_sub(1),
                    completed_cycles: total_cycles + run_cycles.saturating_sub(1),
                    error,
                })?;
                execution.record_cycle(observation);
                total_cycles = total_cycles.saturating_add(run_cycles);
                minimum_cycles_per_run = minimum_cycles_per_run.min(run_cycles);
                execution.continuity_generation =
                    execution.continuity_generation.saturating_add(run_cycles);
                execution.minimum_cycles_per_run = minimum_cycles_per_run;
                eprintln!(
                    "guardian_runtime_window_progress={run}/{runs} run_cycles={run_cycles} total_cycles={total_cycles} elapsed_millis={}",
                    started.elapsed().as_millis()
                );
            }
            verify_continuity_chain(
                &fixture.continuity_root,
                total_cycles,
                &fixture.continuity_verifying_key,
            )
            .await
            .map_err(|error| Failure {
                run: runs,
                cycle: minimum_cycles_per_run,
                completed_runs: runs,
                completed_cycles: total_cycles,
                error,
            })?;
            Ok(execution)
        }
    }
}

async fn execute_cycle(
    args: &Args,
    fixture: &ProductionFixture,
    run: u64,
    cycle: u64,
    expected_generation: u64,
    retain_log: bool,
    require_restart_proof: bool,
) -> Result<CycleObservation, String> {
    fixture.configure_cycle(args, run, cycle, expected_generation.saturating_sub(1))?;
    std::fs::create_dir_all(&fixture.continuity_root)
        .map_err(|error| format!("could not create continuity root: {error}"))?;
    let mut guardian_command = Command::new(&args.guardian);
    guardian_command
        .arg("--init")
        .arg(&fixture.init)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(windows)]
    guardian_command.creation_flags(CREATE_NEW_PROCESS_GROUP);
    let mut guardian = guardian_command
        .spawn()
        .map_err(|error| format!("Guardian binary launch failed: {error}"))?;
    let captured = CapturedOutput::take(&mut guardian)?;
    let guardian_process_id = guardian
        .id()
        .ok_or_else(|| "Guardian binary did not expose a process id".to_owned())?;
    let first_ready = match wait_for_authenticated_observatory(fixture, &mut guardian).await {
        Ok(ready) => ready,
        Err(readiness_error) => {
            if matches!(guardian.try_wait(), Ok(None)) {
                let _ = request_native_shutdown(&mut guardian).await;
            }
            let output = finish_guardian(&mut guardian, captured, fixture.shutdown_wait, None)
                .await
                .map_err(|error| format!("{readiness_error}; {error}"))?;
            return Err(format!(
                "{readiness_error}; guardian_status={}; guardian_stdout={}; guardian_stderr={}",
                output.status,
                diagnostic_tail(&String::from_utf8_lossy(&output.stdout), &args.state_root),
                diagnostic_tail(&String::from_utf8_lossy(&output.stderr), &args.state_root)
            ));
        }
    };
    let first_runtime_instance_id = runtime_instance_id(&first_ready)?.to_owned();
    let first_runtime_process_id = runtime_process_id(&first_ready)?;
    let mut runtime_instance_ids = vec![first_runtime_instance_id.clone()];
    if require_restart_proof {
        if let Err(error) = force_runtime_exit(first_runtime_process_id) {
            let _ = request_native_shutdown(&mut guardian).await;
            let _ = finish_guardian(
                &mut guardian,
                captured,
                fixture.shutdown_wait,
                Some(first_runtime_process_id),
            )
            .await;
            return Err(error);
        }
        let restarted = match wait_for_restarted_observatory(
            fixture,
            &mut guardian,
            &first_runtime_instance_id,
            first_runtime_process_id,
        )
        .await
        {
            Ok(restarted) => restarted,
            Err(error) => {
                let _ = request_native_shutdown(&mut guardian).await;
                let diagnostic = finish_guardian(
                    &mut guardian,
                    captured,
                    fixture.shutdown_wait,
                    Some(first_runtime_process_id),
                )
                .await
                .map(|output| {
                    format!(
                        "{error}; guardian_status={}; guardian_stdout={}; guardian_stderr={}",
                        output.status,
                        diagnostic_tail(&String::from_utf8_lossy(&output.stdout), &args.state_root),
                        diagnostic_tail(&String::from_utf8_lossy(&output.stderr), &args.state_root)
                    )
                })
                .unwrap_or_else(|diagnostic_error| {
                    format!("{error}; guardian_diagnostic_failed={diagnostic_error}")
                });
                return Err(diagnostic);
            }
        };
        runtime_instance_ids.push(runtime_instance_id(&restarted)?.to_owned());
    }
    let latest_runtime_process_id = authenticated_observatory(fixture)
        .await
        .ok()
        .and_then(|observatory| runtime_process_id(&observatory).ok());
    if let Err(error) = request_native_shutdown(&mut guardian).await {
        let _ = finish_guardian(
            &mut guardian,
            captured,
            fixture.shutdown_wait,
            latest_runtime_process_id,
        )
        .await;
        return Err(error);
    }
    let output = finish_guardian(
        &mut guardian,
        captured,
        fixture.shutdown_wait,
        latest_runtime_process_id,
    )
    .await?;
    if !output.status.success() {
        let outcome_diagnostic = guardian_failure_diagnostic(&output.stdout, &args.state_root);
        return Err(format!(
            "Guardian process exited with {}; guardian_outcome={}; stdout={}; stderr={}",
            output.status,
            outcome_diagnostic,
            diagnostic_tail(&String::from_utf8_lossy(&output.stdout), &args.state_root),
            diagnostic_tail(&String::from_utf8_lossy(&output.stderr), &args.state_root)
        ));
    }
    reject_fatal_process_output(&output.stdout, &output.stderr)?;
    let outcome = guardian_outcome_from_stdout(&output.stdout)?;
    validate_guardian_outcome(&outcome, require_restart_proof)?;
    verify_generation(&fixture.continuity_root, expected_generation).map_err(|error| {
        format!(
            "{error}; guardian_stderr={}",
            diagnostic_tail(&outcome.attempts_detail[0].stderr, &args.state_root)
        )
    })?;
    verify_writer_lock_released(&fixture.local_state_root)?;
    let log_proof = verify_master_log(args, fixture, run, cycle)?;
    if !retain_log {
        discard_checked_observability(&fixture.observability_root)?;
    }
    Ok(CycleObservation {
        guardian_pid: guardian_process_id,
        runtime_starts: u64::try_from(runtime_instance_ids.len())
            .map_err(|_| "runtime start count overflowed".to_owned())?,
        runtime_instance_ids,
        anti_rollback_minimum_enforced: expected_generation > 1,
        restart_budget_exercised: outcome.restarts > 0,
        restarts: u64::from(outcome.restarts),
        log_proof,
    })
}

fn discard_checked_observability(observability_root: &Path) -> Result<(), String> {
    match std::fs::remove_dir_all(observability_root) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "checked Vector log could not be discarded from {}: {error}",
            observability_root.display()
        )),
    }
}

fn diagnostic_tail(output: &str, state_root: &Path) -> String {
    diagnostic_suffix(output, state_root, 4_096)
}

fn diagnostic_suffix(output: &str, state_root: &Path, max_chars: usize) -> String {
    let redacted = output.replace(&state_root.to_string_lossy().to_string(), "<state-root>");
    let tail = redacted
        .char_indices()
        .rev()
        .nth(max_chars.saturating_sub(1))
        .map_or(redacted.as_str(), |(index, _)| &redacted[index..]);
    tail.replace(['\n', '\r'], " | ")
}

fn guardian_failure_diagnostic(stdout: &[u8], state_root: &Path) -> String {
    let Ok(outcome) = guardian_outcome_from_stdout(stdout) else {
        return "guardian_outcome_unparseable".to_owned();
    };
    let Some(attempt) = outcome.attempts_detail.last() else {
        return format!(
            "terminal_state={:?};attempts={};restarts={};last_attempt=missing",
            outcome.terminal_state, outcome.attempts, outcome.restarts
        );
    };
    format!(
        "terminal_state={:?};attempts={};restarts={};attempt={};pid={:?};exit_code={:?};exit_status={:?};unix_signal={:?};windows_ctrl_event={:?};forced_shutdown={};clean_checkpointed_shutdown={};reason_code={};child_stdout_tail={};child_stderr_tail={}",
        outcome.terminal_state,
        outcome.attempts,
        outcome.restarts,
        attempt.attempt,
        attempt.pid,
        attempt.exit_code,
        attempt.exit_status,
        attempt.unix_signal,
        attempt.windows_ctrl_event,
        attempt.forced_shutdown,
        attempt.clean_checkpointed_shutdown,
        attempt.reason_code,
        diagnostic_suffix(&attempt.stdout, state_root, 1_024),
        diagnostic_suffix(&attempt.stderr, state_root, 1_024),
    )
}

async fn wait_for_authenticated_observatory(
    fixture: &ProductionFixture,
    guardian: &mut Child,
) -> Result<serde_json::Value, String> {
    let deadline = Instant::now() + fixture.readiness_timeout;
    loop {
        match guardian.try_wait() {
            Ok(Some(status)) => {
                return Err(format!(
                    "Runtime v3 exited before its authenticated API became ready: {status}"
                ));
            }
            Ok(None) => {}
            Err(error) => {
                return Err(format!("Guardian process readiness check failed: {error}"));
            }
        }
        match authenticated_observatory(fixture).await {
            Ok(observatory) => match validate_observatory(&observatory) {
                Ok(()) => return Ok(observatory),
                Err(_) if Instant::now() < deadline => {
                    tokio::time::sleep(fixture.readiness_poll).await;
                }
                Err(error) => return Err(error),
            },
            Err(error) if Instant::now() < deadline => {
                let _ = error;
                tokio::time::sleep(fixture.readiness_poll).await;
            }
            Err(error) => {
                return Err(format!(
                    "Runtime v3 authenticated API was not ready on {}: {error}",
                    fixture.address
                ))
            }
        }
    }
}

async fn wait_for_restarted_observatory(
    fixture: &ProductionFixture,
    guardian: &mut Child,
    previous_instance_id: &str,
    previous_process_id: u32,
) -> Result<serde_json::Value, String> {
    let deadline = Instant::now() + fixture.readiness_timeout;
    loop {
        match guardian.try_wait() {
            Ok(Some(status)) => {
                return Err(format!(
                    "Guardian exited instead of restarting the killed kernel: {status}"
                ));
            }
            Ok(None) => {}
            Err(error) => return Err(format!("Guardian restart check failed: {error}")),
        }
        let observation = match authenticated_observatory(fixture).await {
            Ok(observatory) => match validate_observatory(&observatory) {
                Ok(()) => {
                    let instance_id = runtime_instance_id(&observatory)?;
                    let process_id = runtime_process_id(&observatory)?;
                    if instance_id != previous_instance_id && process_id != previous_process_id {
                        return Ok(observatory);
                    }
                    format!(
                            "authenticated Observatory still reported prior runtime instance {instance_id} process {process_id}"
                        )
                }
                Err(error) => error,
            },
            Err(error) if Instant::now() < deadline => error,
            Err(error) => {
                return Err(format!(
                    "Guardian did not restore the kernel after external termination: {error}"
                ))
            }
        };
        if Instant::now() >= deadline {
            return Err(format!(
                "Guardian did not expose a distinct restarted kernel before deadline: {observation}"
            ));
        }
        tokio::time::sleep(fixture.readiness_poll).await;
    }
}

fn runtime_instance_id(observatory: &serde_json::Value) -> Result<&str, String> {
    observatory["runtime_instance_id"]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "Runtime v3 Observatory did not expose runtime_instance_id".to_owned())
}

fn runtime_process_id(observatory: &serde_json::Value) -> Result<u32, String> {
    observatory["runtime_process_id"]
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .filter(|value| *value > 0)
        .ok_or_else(|| "Runtime v3 Observatory did not expose runtime_process_id".to_owned())
}

async fn request_native_shutdown(guardian: &mut Child) -> Result<(), String> {
    let pid = guardian
        .id()
        .ok_or_else(|| "Guardian process id disappeared before shutdown".to_owned())?;
    send_native_shutdown(pid, guardian).await
}

#[cfg(unix)]
fn force_runtime_exit(pid: u32) -> Result<(), String> {
    if unsafe { libc::kill(pid as i32, libc::SIGKILL) } == 0 {
        Ok(())
    } else {
        Err(format!(
            "external kernel SIGKILL fault failed: {}",
            std::io::Error::last_os_error()
        ))
    }
}

#[cfg(windows)]
fn force_runtime_exit(pid: u32) -> Result<(), String> {
    struct Handle(HANDLE);
    impl Drop for Handle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    CloseHandle(self.0);
                }
            }
        }
    }
    let handle = Handle(unsafe {
        OpenProcess(
            PROCESS_TERMINATE | PROCESS_QUERY_LIMITED_INFORMATION,
            0,
            pid,
        )
    });
    if handle.0.is_null() {
        return Err(format!(
            "external kernel process open failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    if unsafe { TerminateProcess(handle.0, 86) } == 0 {
        return Err(format!(
            "external kernel termination failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(not(any(unix, windows)))]
fn force_runtime_exit(_pid: u32) -> Result<(), String> {
    Err("external kernel termination is unsupported on this platform".to_owned())
}

fn reject_fatal_process_output(stdout: &[u8], stderr: &[u8]) -> Result<(), String> {
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(stdout),
        String::from_utf8_lossy(stderr)
    )
    .to_ascii_lowercase();
    for forbidden in ["panicked at", "fatal:", "fatal error", "stack backtrace:"] {
        if combined.contains(forbidden) {
            return Err(format!(
                "Guardian or kernel emitted forbidden fatal output marker: {forbidden}"
            ));
        }
    }
    Ok(())
}

#[cfg(unix)]
async fn send_native_shutdown(pid: u32, _guardian: &mut Child) -> Result<(), String> {
    let rc = unsafe { libc::kill(pid as i32, libc::SIGTERM) };
    if rc == 0 {
        Ok(())
    } else {
        Err(format!(
            "native Guardian SIGTERM failed: {}",
            std::io::Error::last_os_error()
        ))
    }
}

#[cfg(windows)]
async fn send_native_shutdown(pid: u32, _guardian: &mut Child) -> Result<(), String> {
    if unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid) } != 0 {
        Ok(())
    } else {
        Err(format!(
            "native Guardian CTRL_BREAK failed: {}",
            std::io::Error::last_os_error()
        ))
    }
}

fn guardian_outcome_from_stdout(stdout: &[u8]) -> Result<GuardianOutcome, String> {
    let text = String::from_utf8(stdout.to_vec())
        .map_err(|error| format!("Guardian stdout was not UTF-8 JSON: {error}"))?;
    let payload = text
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| "Guardian stdout did not include its JSON outcome".to_owned())?;
    serde_json::from_str(payload).map_err(|error| format!("Guardian outcome JSON invalid: {error}"))
}

async fn authenticated_observatory(
    fixture: &ProductionFixture,
) -> Result<serde_json::Value, String> {
    let stream = tokio::net::TcpStream::connect(fixture.address)
        .await
        .map_err(|error| error.to_string())?;
    let server_name = ServerName::try_from("localhost").map_err(|error| error.to_string())?;
    let mut stream = fixture
        .tls_connector
        .connect(server_name, stream)
        .await
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {}\r\nConnection: close\r\n\r\n",
        fixture.observatory_token
    );
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|error| error.to_string())?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .await
        .map_err(|error| error.to_string())?;
    let response = String::from_utf8(response).map_err(|error| error.to_string())?;
    if !response.starts_with("HTTP/1.1 200 OK") {
        return Err("authenticated Observatory request did not return HTTP 200".to_owned());
    }
    let body = response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .ok_or_else(|| "authenticated Observatory response had no body".to_owned())?;
    serde_json::from_str(body).map_err(|error| error.to_string())
}

fn validate_observatory(observatory: &serde_json::Value) -> Result<(), String> {
    let runtime_instance_id = runtime_instance_id(observatory)?;
    let _runtime_process_id = runtime_process_id(observatory)?;
    let snapshot = &observatory["health"]["snapshot"];
    let components_ready = snapshot["components"]
        .as_object()
        .filter(|components| !components.is_empty())
        .map(|components| {
            components.values().all(|state| {
                state
                    .as_str()
                    .is_some_and(|state| matches!(state, "ready" | "running"))
            })
        })
        .unwrap_or(false);
    if observatory["schema"] != "adl.runtime_v3.observatory_feed.v2"
        || runtime_instance_id.is_empty()
        || observatory["runtime_selection"] != "runtime_v3_explicit_opt_in"
        || observatory["control"]["websocket_full_duplex"] != true
        || observatory["health"]["observability_ready"] != true
        || snapshot["schema"] != "adl.runtime.control_snapshot.v1"
        || snapshot["lifecycle"] != "running"
        || snapshot["clock"]["status"] != "authoritative"
        || snapshot["observability"]["status"] != "ready"
        || snapshot["observability_pipeline"]["health"]["status"] != "ready"
        || !components_ready
        || observatory["proof"]["sidecar_required"] != false
    {
        return Err(format!(
            "Runtime v3 Observatory did not expose typed ready production health: {}",
            serde_json::to_string(&observatory["health"])
                .unwrap_or_else(|_| "<invalid-health>".to_owned())
        ));
    }
    Ok(())
}

fn verify_writer_lock_released(local_state_root: &Path) -> Result<(), String> {
    let writer_lock = local_state_root.join("writer.lock");
    if writer_lock.exists() {
        return Err(format!(
            "production adapter writer lock survived clean shutdown: {}",
            writer_lock.display()
        ));
    }
    Ok(())
}

fn verify_master_log(
    args: &Args,
    fixture: &ProductionFixture,
    run: u64,
    cycle: u64,
) -> Result<LogProof, String> {
    let master_log = &fixture.master_log;
    let audit = &fixture.log_audit;
    let master_log_sha256 =
        file_sha256(master_log).map_err(|error| format!("master log unavailable: {error}"))?;
    let master_log_bytes =
        std::fs::read(master_log).map_err(|error| format!("master log unreadable: {error}"))?;
    let master_log_text = String::from_utf8(master_log_bytes)
        .map_err(|_| "master log is not UTF-8 JSONL".to_owned())?;
    let expected_run = format!("{}:run-{run}", args.revision);
    let expected_cycle = format!("{}:run-{run}:cycle-{cycle}", args.suite.name());
    let mut records_by_sequence = BTreeMap::new();
    for (index, line) in master_log_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
    {
        let record: serde_json::Value = serde_json::from_str(line)
            .map_err(|error| format!("master log record {} is invalid: {error}", index + 1))?;
        if record["lifecycle_run"] != expected_run
            || record["lifecycle_cycle"] != expected_cycle
            || record["revision"] != args.revision
        {
            return Err(format!(
                "master log record {} is not correlated to run {run} cycle {cycle}",
                index + 1
            ));
        }
        let sequence = record["sequence"].as_u64().ok_or_else(|| {
            format!(
                "master log record {} omitted its numeric sequence",
                index + 1
            )
        })?;
        if let Some(previous) = records_by_sequence.get(&sequence) {
            if previous != &record {
                return Err(format!(
                    "master log sequence {sequence} was reused with conflicting content"
                ));
            }
            continue;
        }
        let searchable = format!(
            "{} {} {} {}",
            record["severity"], record["reason"], record["error_chain"], record["fields"]
        )
        .to_ascii_lowercase();
        if ["panicked at", "fatal:", "fatal error", "stack backtrace:"]
            .iter()
            .any(|marker| searchable.contains(marker))
        {
            return Err(format!(
                "master log record {} contains a forbidden fatal marker",
                index + 1
            ));
        }
        records_by_sequence.insert(sequence, record);
    }
    let master_log_records = u64::try_from(records_by_sequence.len())
        .map_err(|_| "master log unique record count overflowed".to_owned())?;
    if master_log_records == 0 {
        return Err("master log retained no records for this lifecycle cycle".to_owned());
    }
    let audit_bytes =
        std::fs::read(audit).map_err(|error| format!("master log audit unavailable: {error}"))?;
    let audit_value: serde_json::Value = serde_json::from_slice(&audit_bytes)
        .map_err(|error| format!("master log audit is invalid JSON: {error}"))?;
    let expected_platform = std::env::consts::OS;
    let expected_suite = args.suite.name();
    let zero_counters = [
        "malformed_records",
        "missing_required_fields",
        "sequence_gaps",
        "error_events",
        "degraded_events",
        "unexplained_restarts",
        "incomplete_drains",
    ]
    .iter()
    .all(|field| audit_value[*field].as_u64() == Some(0));
    if audit_value["schema"] != "adl.runtime.master_log_audit.v1"
        || audit_value["status"] != "pass"
        || audit_value["platform"] != expected_platform
        || audit_value["suite"] != expected_suite
        || audit_value["revision"] != args.revision
        || audit_value["record_count"].as_u64() != Some(master_log_records)
        || !zero_counters
    {
        return Err(format!(
            "Vector master log audit did not prove a clean {expected_platform}/{expected_suite} lifecycle"
        ));
    }
    Ok(LogProof {
        master_log_ref: repo_relative(master_log)?,
        master_log_sha256,
        master_log_records,
        log_audit_ref: repo_relative(audit)?,
        log_audit_sha256: file_sha256(audit)
            .map_err(|error| format!("master log audit hash failed: {error}"))?,
    })
}

fn aggregate_platform(args: &AggregateArgs) -> ExitCode {
    match build_platform_proof(args) {
        Ok(proof) => {
            if let Err(error) = write_report(&args.output, &proof) {
                eprintln!("failed writing platform proof: {error}");
                return ExitCode::from(66);
            }
            println!("{proof}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

struct SoakReport {
    value: serde_json::Value,
    revision: String,
    kernel_sha256: String,
    platform: String,
    architecture: String,
}

fn build_platform_proof(args: &AggregateArgs) -> Result<serde_json::Value, String> {
    let preflight = read_soak_report(&args.preflight_report, "preflight_1x")?;
    let lifecycle = read_soak_report(&args.lifecycle_report, "lifecycle_10000")?;
    let stress = read_soak_report(&args.stress_report, "stress_100x10s")?;
    let endurance = read_soak_report(&args.endurance_report, "endurance_10x600s")?;
    let reports = [&preflight, &lifecycle, &stress, &endurance];
    let first = reports[0];
    for report in reports {
        if report.revision != first.revision {
            return Err("platform reports do not share one exact Git revision".to_owned());
        }
        if report.kernel_sha256 != first.kernel_sha256 {
            return Err("platform reports do not share one Runtime v3 kernel digest".to_owned());
        }
        if report.platform != first.platform || report.architecture != first.architecture {
            return Err(
                "platform reports mix native platform or architecture identities".to_owned(),
            );
        }
    }
    let platform_id = platform_proof_id(&first.platform, &first.architecture)?;
    Ok(serde_json::json!({
        "schema": PLATFORM_PROOF_SCHEMA,
        "issue": 5344,
        "platform": platform_id,
        "native_os": first.platform,
        "architecture": first.architecture,
        "status": "pass",
        "guardian_process_zero": true,
        "native_execution": true,
        "wsl_used": false,
        "docker_used": false,
        "lifecycle_acceptance": {
            "revision": first.revision,
            "kernel_sha256": first.kernel_sha256,
            "all_logs_clean": true,
            "preflight": suite_summary(&preflight.value),
            "lifecycle_10000": suite_summary(&lifecycle.value),
            "stress_100x10s": suite_summary(&stress.value),
            "endurance_10x600s": suite_summary(&endurance.value),
        },
    }))
}

fn read_soak_report(path: &Path, expected_suite: &str) -> Result<SoakReport, String> {
    let value: serde_json::Value = serde_json::from_slice(
        &std::fs::read(path).map_err(|error| format!("{} unreadable: {error}", path.display()))?,
    )
    .map_err(|error| format!("{} is invalid JSON: {error}", path.display()))?;
    require_string(&value, "schema", REPORT_SCHEMA)?;
    require_string(&value, "status", "pass")?;
    require_string(&value, "suite", expected_suite)?;
    let platform = string_field(&value, "platform")?;
    let architecture = string_field(&value, "architecture")?;
    let native_platform = std::env::consts::OS;
    let native_architecture = std::env::consts::ARCH;
    if platform != native_platform || architecture != native_architecture {
        return Err(format!(
            "{expected_suite} was collected for {platform}/{architecture}, not native {native_platform}/{native_architecture}"
        ));
    }
    let revision = string_field(&value, "revision")?;
    if !is_lower_hex(&revision, 40) {
        return Err(format!("{expected_suite} has invalid revision identity"));
    }
    let kernel_sha256 = string_field(&value, "kernel_sha256")?;
    if !is_lower_hex(&kernel_sha256, 64) {
        return Err(format!("{expected_suite} has invalid kernel digest"));
    }
    require_bool(&value, "logging_complete", true)?;
    require_string(&value, "master_log_status", "clean")?;
    if u64_field(&value, "log_checked_cycles")? != u64_field(&value, "completed_cycles")? {
        return Err(format!(
            "{expected_suite} did not validate every completed cycle's Vector log"
        ));
    }
    if u64_field(&value, "guardian_launch_count")? != u64_field(&value, "completed_cycles")? {
        return Err(format!(
            "{expected_suite} guardian launch count does not match completed cycles"
        ));
    }
    if u64_field(&value, "runtime_start_count")? < u64_field(&value, "completed_cycles")? {
        return Err(format!(
            "{expected_suite} runtime start count is below completed cycles"
        ));
    }
    if u64_field(&value, "runtime_start_count")?
        != u64_field(&value, "completed_cycles")?
            .saturating_add(u64_field(&value, "total_restarts")?)
    {
        return Err(format!(
            "{expected_suite} runtime start count does not reconcile with restarts"
        ));
    }
    if u64_field(&value, "runtime_instance_count")? != u64_field(&value, "runtime_start_count")? {
        return Err(format!(
            "{expected_suite} reused a runtime instance identity"
        ));
    }
    require_bool(&value, "restart_budget_exercised", true)?;
    if u64_field(&value, "master_log_records")? == 0 {
        return Err(format!("{expected_suite} retained no master log records"));
    }
    validate_suite_counts(&value, expected_suite)?;
    if expected_suite != "preflight_1x" {
        require_bool(&value, "anti_rollback_minimum_enforced", true)?;
    }
    Ok(SoakReport {
        value,
        revision,
        kernel_sha256,
        platform,
        architecture,
    })
}

fn validate_suite_counts(value: &serde_json::Value, suite: &str) -> Result<(), String> {
    match suite {
        "preflight_1x" => {
            require_bool(value, "acceptance_eligible", false)?;
            require_u64(value, "requested_cycles", 1)?;
            require_u64(value, "requested_runs", 1)?;
            require_u64(value, "completed_runs", 1)?;
            require_u64(value, "completed_cycles", 1)?;
        }
        "lifecycle_10000" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_cycles", REQUIRED_CYCLES)?;
            require_u64(value, "requested_runs", 1)?;
            require_u64(value, "completed_runs", 1)?;
            require_u64(value, "completed_cycles", REQUIRED_CYCLES)?;
        }
        "stress_100x10s" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_runs", STRESS_RUNS)?;
            require_u64(value, "duration_seconds_per_run", STRESS_SECONDS)?;
            require_u64(value, "completed_runs", STRESS_RUNS)?;
            require_positive(value, "completed_cycles")?;
            require_positive(value, "minimum_cycles_per_run")?;
        }
        "endurance_10x600s" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_runs", ENDURANCE_RUNS)?;
            require_u64(value, "duration_seconds_per_run", ENDURANCE_SECONDS)?;
            require_u64(value, "completed_runs", ENDURANCE_RUNS)?;
            require_positive(value, "completed_cycles")?;
            require_positive(value, "minimum_cycles_per_run")?;
        }
        _ => return Err(format!("unsupported suite identity: {suite}")),
    }
    Ok(())
}

fn suite_summary(value: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "status": value["status"],
        "suite": value["suite"],
        "requested_cycles": value["requested_cycles"],
        "requested_runs": value["requested_runs"],
        "duration_seconds_per_run": value["duration_seconds_per_run"],
        "completed_runs": value["completed_runs"],
        "completed_cycles": value["completed_cycles"],
        "failed_cycles": 0,
        "degraded_cycles": 0,
        "minimum_cycles_per_run": value["minimum_cycles_per_run"],
        "guardian_process_count": value["guardian_process_count"],
        "guardian_launch_count": value["guardian_launch_count"],
        "runtime_instance_count": value["runtime_instance_count"],
        "runtime_start_count": value["runtime_start_count"],
        "total_restarts": value["total_restarts"],
        "restart_budget_exercised": value["restart_budget_exercised"],
        "anti_rollback_minimum_enforced": value["anti_rollback_minimum_enforced"],
        "acceptance_eligible": value["acceptance_eligible"],
        "logging_complete": value["logging_complete"],
        "log_checked_cycles": value["log_checked_cycles"],
        "master_log_status": value["master_log_status"],
        "master_log_ref": value["master_log_ref"],
        "master_log_sha256": value["master_log_sha256"],
        "master_log_records": value["master_log_records"],
        "log_audit_ref": value["log_audit_ref"],
        "log_audit_sha256": value["log_audit_sha256"],
    })
}

fn platform_proof_id(platform: &str, architecture: &str) -> Result<&'static str, String> {
    match (platform, architecture) {
        ("macos", "aarch64") => Ok("macos-arm64"),
        ("linux", "x86_64") => Ok("linux-x86_64"),
        ("windows", "x86_64") => Ok("windows-x86_64-msvc"),
        _ => Err(format!(
            "unsupported native WP-12 platform identity: {platform}/{architecture}"
        )),
    }
}

fn require_string(value: &serde_json::Value, field: &str, expected: &str) -> Result<(), String> {
    let actual = string_field(value, field)?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_bool(value: &serde_json::Value, field: &str, expected: bool) -> Result<(), String> {
    let actual = value[field]
        .as_bool()
        .ok_or_else(|| format!("{field} must be boolean"))?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_u64(value: &serde_json::Value, field: &str, expected: u64) -> Result<(), String> {
    let actual = u64_field(value, field)?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_positive(value: &serde_json::Value, field: &str) -> Result<(), String> {
    let actual = u64_field(value, field)?;
    if actual == 0 {
        return Err(format!("{field} must be greater than zero"));
    }
    Ok(())
}

fn string_field(value: &serde_json::Value, field: &str) -> Result<String, String> {
    value[field]
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("{field} must be string"))
}

fn u64_field(value: &serde_json::Value, field: &str) -> Result<u64, String> {
    value[field]
        .as_u64()
        .ok_or_else(|| format!("{field} must be unsigned integer"))
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn repo_relative(path: &Path) -> Result<String, String> {
    let root = std::env::current_dir()
        .map_err(|error| format!("current checkout unavailable: {error}"))?
        .canonicalize()
        .map_err(|error| format!("current checkout cannot be canonicalized: {error}"))?;
    let path = path
        .canonicalize()
        .map_err(|error| format!("evidence path cannot be canonicalized: {error}"))?;
    let relative = path
        .strip_prefix(&root)
        .map_err(|_| "lifecycle evidence escaped the repository checkout".to_owned())?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn verify_generation(continuity_root: &Path, expected: u64) -> Result<(), String> {
    let manifest = continuity_root
        .join(format!("generation-{expected}"))
        .join("manifest.json");
    let generation = continuity_generation(&manifest)
        .map_err(|error| format!("continuity verification failed: {error}"))?;
    if generation != expected {
        return Err(format!(
            "continuity generation {generation} did not equal completed cycles {expected}"
        ));
    }
    Ok(())
}

async fn verify_continuity_chain(
    continuity_root: &Path,
    expected_generation: u64,
    verifying_key: &VerifyingKey,
) -> Result<(), String> {
    verify_live_continuity_lineage(
        continuity_root,
        "runtime-continuity",
        verifying_key.to_owned(),
        expected_generation,
    )
    .await
    .map_err(|error| format!("runtime continuity verification failed: {error}"))
}

fn prepare_state_root(path: &Path) -> Result<(), String> {
    if path.exists() {
        let mut entries =
            std::fs::read_dir(path).map_err(|error| format!("state root unreadable: {error}"))?;
        if entries.next().is_some() {
            return Err("state root must be empty for an exact lifecycle soak".to_owned());
        }
    } else {
        std::fs::create_dir_all(path)
            .map_err(|error| format!("state root could not be created: {error}"))?;
    }
    Ok(())
}

fn validate_guardian_outcome(
    outcome: &GuardianOutcome,
    restart_required: bool,
) -> Result<(), String> {
    let expected_attempts = if restart_required { 2 } else { 1 };
    let expected_restarts = if restart_required { 1 } else { 0 };
    if outcome.terminal_state != GuardianTerminalState::ShutdownCheckpointed
        || outcome.attempts != expected_attempts
        || outcome.restarts != expected_restarts
        || outcome.attempts_detail.len() != expected_attempts as usize
    {
        return Err(format!("unexpected guardian outcome: {outcome:?}"));
    }
    let attempt = outcome
        .attempts_detail
        .last()
        .ok_or_else(|| "Guardian outcome omitted its final attempt".to_owned())?;
    if attempt.reason_code != "shutdown_clean_checkpointed"
        || attempt.pid.is_none()
        || !attempt.clean_checkpointed_shutdown
        || attempt.forced_shutdown
        || attempt.exit_status.is_none()
    {
        return Err(format!("unexpected guardian attempt: {attempt:?}"));
    }
    Ok(())
}

fn toml_path(path: &Path) -> Result<String, String> {
    let value = path
        .to_str()
        .ok_or_else(|| "runtime configuration path is not UTF-8".to_owned())?;
    if value.contains(['\n', '\r']) {
        return Err("runtime configuration path contains a line break".to_owned());
    }
    Ok(value.to_owned())
}

fn continuity_generation(path: &Path) -> Result<u64, String> {
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    value["generation"]
        .as_u64()
        .ok_or_else(|| "continuity generation is missing".to_owned())
}

fn file_sha256(path: &Path) -> std::io::Result<String> {
    let mut hash = Sha256::new();
    hash.update(std::fs::read(path)?);
    Ok(format!("{:x}", hash.finalize()))
}

fn fail(args: &Args, kernel_sha256: &str, started: Instant, failure: Failure) -> ExitCode {
    let execution = Execution {
        completed_runs: failure.completed_runs,
        completed_cycles: failure.completed_cycles,
        continuity_generation: 0,
        minimum_cycles_per_run: 0,
        guardian_pids: BTreeSet::new(),
        runtime_instance_ids: BTreeSet::new(),
        guardian_launches: 0,
        runtime_starts: 0,
        anti_rollback_minimum_enforced: false,
        restart_budget_exercised: false,
        total_restarts: 0,
        log_checked_cycles: 0,
        log_proof: None,
    };
    let report = report(
        args,
        kernel_sha256,
        started,
        "fail",
        &execution,
        Some(failure),
    );
    let _ = write_report(&args.report, &report);
    eprintln!("{report}");
    ExitCode::from(1)
}

fn report(
    args: &Args,
    kernel_sha256: &str,
    started: Instant,
    status: &str,
    execution: &Execution,
    failure: Option<Failure>,
) -> serde_json::Value {
    let (requested_cycles, requested_runs, duration_seconds) = match args.suite {
        Suite::Preflight => (Some(1), Some(1), None),
        Suite::Lifecycle { cycles } => (Some(cycles), Some(1), None),
        Suite::Stress { runs, seconds } | Suite::Endurance { runs, seconds } => {
            (None, Some(runs), Some(seconds))
        }
    };
    let logging_complete = execution.log_proof.is_some();
    let master_log_ref = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_ref.as_str());
    let master_log_sha256 = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_sha256.as_str());
    let master_log_records = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_records);
    let log_audit_ref = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.log_audit_ref.as_str());
    let log_audit_sha256 = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.log_audit_sha256.as_str());
    serde_json::json!({
        "schema": REPORT_SCHEMA,
        "status": status,
        "suite": args.suite.name(),
        "platform": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "revision": args.revision,
        "requested_cycles": requested_cycles,
        "requested_runs": requested_runs,
        "duration_seconds_per_run": duration_seconds,
        "completed_runs": execution.completed_runs,
        "completed_cycles": execution.completed_cycles,
        "minimum_cycles_per_run": execution.minimum_cycles_per_run,
        "guardian_process_count": execution.guardian_pids.len(),
        "guardian_launch_count": execution.guardian_launches,
        "runtime_instance_count": execution.runtime_instance_ids.len(),
        "runtime_start_count": execution.runtime_starts,
        "anti_rollback_minimum_enforced": execution.anti_rollback_minimum_enforced,
        "restart_budget_exercised": execution.restart_budget_exercised,
        "total_restarts": execution.total_restarts,
        "acceptance_eligible": !matches!(args.suite, Suite::Preflight),
        "logging_complete": logging_complete,
        "log_checked_cycles": if logging_complete {
            Some(execution.log_checked_cycles)
        } else {
            None
        },
        "master_log_status": if logging_complete { "clean" } else { "incomplete" },
        "master_log_ref": master_log_ref,
        "master_log_sha256": master_log_sha256,
        "master_log_records": master_log_records,
        "log_audit_ref": log_audit_ref,
        "log_audit_sha256": log_audit_sha256,
        "continuity_generation": execution.continuity_generation,
        "kernel_sha256": kernel_sha256,
        "duration_millis": u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        "failure": failure.map(|failure| serde_json::json!({
            "run": failure.run,
            "cycle": failure.cycle,
            "error": failure.error,
        })),
    })
}

fn write_report(path: &Path, report: &serde_json::Value) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, serde_json::to_vec_pretty(report)?)?;
    std::fs::rename(temporary, path)
}

fn write_secret(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    std::io::Write::write_all(&mut file, bytes)?;
    file.sync_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualification_lock_serializes_the_configured_api_address() {
        let current_dir = std::env::current_dir().expect("current directory");
        let directory = tempfile::tempdir_in(current_dir).expect("repo-local temporary directory");
        let lock_path = directory.path().join("api.lock");
        let address = "127.0.0.1:20997".parse().expect("test address");

        let first =
            QualificationLock::acquire_at(&lock_path, address).expect("first qualification lock");
        let contention = QualificationLock::acquire_at(&lock_path, address)
            .expect_err("second qualification must be rejected");
        assert!(contention.contains("another lifecycle qualification owns"));

        drop(first);
        QualificationLock::acquire_at(&lock_path, address)
            .expect("qualification lock should release with its owner");
    }

    #[test]
    fn checked_observability_is_discarded_between_timed_runs() {
        let current_dir = std::env::current_dir().expect("current directory");
        let directory = tempfile::tempdir_in(current_dir).expect("repo-local temporary directory");
        let observability_root = directory.path().join("observability");
        std::fs::create_dir_all(&observability_root).expect("observability root");
        std::fs::write(observability_root.join("master.log.jsonl"), b"checked")
            .expect("checked log");

        discard_checked_observability(&observability_root)
            .expect("retained prior-run log should be discarded");
        assert!(!observability_root.exists());
        discard_checked_observability(&observability_root)
            .expect("already absent observability root is idempotent");
    }

    #[test]
    fn toml_path_preserves_windows_paths_through_serializer_round_trip() {
        let original = PathBuf::from(r#"C:\adl-wp-5344\state\quoted"name"#);
        let document = toml::Value::Table(toml::map::Map::from_iter([(
            "path".to_owned(),
            toml::Value::String(toml_path(&original).expect("portable path")),
        )]));
        let serialized = toml::to_string(&document).expect("serialize path");
        let parsed = toml::from_str::<toml::Value>(&serialized).expect("parse path");
        assert_eq!(
            parsed.get("path").and_then(toml::Value::as_str),
            original.to_str()
        );
    }

    fn arguments(mode: &[&str]) -> Vec<String> {
        let root = std::env::current_dir().expect("current directory");
        let executable = std::env::current_exe()
            .expect("current executable")
            .to_string_lossy()
            .into_owned();
        let init_template = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("infra")
            .join("runtime-v3")
            .join("runtime-init.toml");
        let mut values = vec![
            "--guardian".to_owned(),
            executable.clone(),
            "--kernel".to_owned(),
            executable.clone(),
            "--vector".to_owned(),
            executable,
            "--init-template".to_owned(),
            init_template.to_string_lossy().into_owned(),
            "--state-root".to_owned(),
            root.join("state").to_string_lossy().into_owned(),
            "--report".to_owned(),
            root.join("report.json").to_string_lossy().into_owned(),
            "--revision".to_owned(),
            "0123456789abcdef0123456789abcdef01234567".to_owned(),
        ];
        values.extend(mode.iter().map(|value| (*value).to_owned()));
        values
    }

    #[test]
    fn accepts_only_the_three_exact_acceptance_suites() {
        let lifecycle = Args::parse(arguments(&["--suite", "lifecycle_10000"]).into_iter())
            .expect("10k lifecycle suite");
        assert!(matches!(
            lifecycle.suite,
            Suite::Lifecycle {
                cycles: REQUIRED_CYCLES
            }
        ));

        let stress = Args::parse(arguments(&["--suite", "stress_100x10s"]).into_iter())
            .expect("100x10s stress suite");
        assert!(matches!(
            stress.suite,
            Suite::Stress {
                runs: STRESS_RUNS,
                seconds: STRESS_SECONDS
            }
        ));

        let endurance = Args::parse(arguments(&["--suite", "endurance_10x600s"]).into_iter())
            .expect("10x600s endurance suite");
        assert!(matches!(
            endurance.suite,
            Suite::Endurance {
                runs: ENDURANCE_RUNS,
                seconds: ENDURANCE_SECONDS
            }
        ));
    }

    #[test]
    fn preflight_is_real_but_never_acceptance_eligible() {
        let preflight = Args::parse(arguments(&["--suite", "preflight_1x"]).into_iter())
            .expect("one-cycle preflight");
        assert!(matches!(preflight.suite, Suite::Preflight));
        let execution = Execution {
            completed_runs: 1,
            completed_cycles: 1,
            continuity_generation: 1,
            minimum_cycles_per_run: 1,
            guardian_pids: BTreeSet::from([1234]),
            runtime_instance_ids: BTreeSet::from(["runtime-test-instance".to_owned()]),
            guardian_launches: 1,
            runtime_starts: 1,
            anti_rollback_minimum_enforced: false,
            restart_budget_exercised: false,
            total_restarts: 0,
            log_checked_cycles: 1,
            log_proof: Some(LogProof {
                master_log_ref: ".csdlc/evidence/5344/work/master.jsonl".to_owned(),
                master_log_sha256: "b".repeat(64),
                master_log_records: 2,
                log_audit_ref: ".csdlc/evidence/5344/work/audit.json".to_owned(),
                log_audit_sha256: "c".repeat(64),
            }),
        };
        let value = report(
            &preflight,
            &"a".repeat(64),
            Instant::now(),
            "pass",
            &execution,
            None,
        );
        assert_eq!(value["acceptance_eligible"], false);
        assert_eq!(value["logging_complete"], true);
        assert_eq!(value["master_log_status"], "clean");
        assert_eq!(value["master_log_records"], 2);
    }

    #[test]
    fn nonzero_guardian_diagnostic_preserves_child_exit_cause() {
        let root = std::env::current_dir().expect("current directory");
        let outcome = GuardianOutcome {
            schema: "adl.runtime_v3.guardian.v1".to_owned(),
            terminal_state: GuardianTerminalState::ShutdownForced,
            attempts: 1,
            restarts: 0,
            attempts_detail: vec![adl_runtime::guardian::GuardianAttempt {
                attempt: 1,
                pid: Some(42),
                exit_code: Some(70),
                exit_status: Some("exit code: 70".to_owned()),
                unix_signal: None,
                windows_ctrl_event: Some(1),
                forced_shutdown: false,
                clean_checkpointed_shutdown: false,
                stdout: format!("stopped {}", root.display()),
                stderr: "runtime shutdown failed: component".to_owned(),
                reason_code: "shutdown_child_failed".to_owned(),
            }],
        };
        let stdout = serde_json::to_vec(&outcome).expect("Guardian outcome");

        let diagnostic = guardian_failure_diagnostic(&stdout, &root);

        assert!(diagnostic.contains("terminal_state=ShutdownForced"));
        assert!(diagnostic.contains("exit_code=Some(70)"));
        assert!(diagnostic.contains("windows_ctrl_event=Some(1)"));
        assert!(diagnostic.contains("reason_code=shutdown_child_failed"));
        assert!(diagnostic.contains("runtime shutdown failed: component"));
        assert!(!diagnostic.contains(&root.to_string_lossy().to_string()));
    }

    #[test]
    fn rejects_partial_or_mixed_acceptance_suites() {
        for mode in [
            vec!["--suite", "lifecycle_9999"],
            vec!["--suite", "stress_100x9s"],
            vec!["--suite", "endurance_9x600s"],
            vec!["--suite", "lifecycle_10000", "--suite", "stress_100x10s"],
            vec!["--preflight", "--suite", "lifecycle_10000"],
        ] {
            assert!(
                Args::parse(arguments(&mode).into_iter()).is_err(),
                "unexpectedly accepted {mode:?}"
            );
        }
    }

    #[test]
    fn aggregates_four_exact_lifecycle_reports_with_compact_clean_logs() {
        let root = std::env::current_dir().expect("current directory");
        let temp = tempfile::tempdir_in(&root).expect("repo-local temp evidence");
        let revision = "0123456789abcdef0123456789abcdef01234567";
        let kernel_sha256 = &"a".repeat(64);
        let preflight = write_sample_report(
            &root,
            temp.path(),
            "preflight_1x",
            revision,
            kernel_sha256,
            1,
        );
        let lifecycle = write_sample_report(
            &root,
            temp.path(),
            "lifecycle_10000",
            revision,
            kernel_sha256,
            REQUIRED_CYCLES,
        );
        let stress = write_sample_report(
            &root,
            temp.path(),
            "stress_100x10s",
            revision,
            kernel_sha256,
            42,
        );
        let endurance = write_sample_report(
            &root,
            temp.path(),
            "endurance_10x600s",
            revision,
            kernel_sha256,
            24,
        );
        let output = temp.path().join("platform-proof.json");
        let args = AggregateArgs {
            preflight_report: preflight,
            lifecycle_report: lifecycle,
            stress_report: stress,
            endurance_report: endurance,
            output: output.clone(),
        };

        let proof = build_platform_proof(&args).expect("platform proof");
        write_report(&output, &proof).expect("atomic proof write");
        let written: serde_json::Value =
            serde_json::from_slice(&std::fs::read(output).expect("written proof"))
                .expect("proof JSON");

        assert_eq!(written["schema"], PLATFORM_PROOF_SCHEMA);
        assert_eq!(written["status"], "pass");
        assert_eq!(written["guardian_process_zero"], true);
        assert_eq!(written["native_execution"], true);
        assert_eq!(written["wsl_used"], false);
        assert_eq!(written["docker_used"], false);
        assert_eq!(written["lifecycle_acceptance"]["all_logs_clean"], true);
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["completed_cycles"],
            REQUIRED_CYCLES
        );
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["failed_cycles"],
            0
        );
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["degraded_cycles"],
            0
        );
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["master_log_records"],
            3
        );
        assert_eq!(
            written["lifecycle_acceptance"]["stress_100x10s"]["master_log_records"],
            3
        );
    }

    fn write_sample_report(
        root: &Path,
        temp: &Path,
        suite: &str,
        revision: &str,
        kernel_sha256: &str,
        completed_cycles: u64,
    ) -> PathBuf {
        let suite_dir = temp.join(suite);
        std::fs::create_dir_all(&suite_dir).expect("suite dir");
        let log = suite_dir.join("master.log.jsonl");
        std::fs::write(
            &log,
            b"{\"sequence\":1,\"level\":\"info\"}\n{\"sequence\":2,\"level\":\"info\"}\n{\"sequence\":3,\"level\":\"info\"}\n",
        )
        .expect("master log");
        let log_sha256 = file_sha256(&log).expect("log sha");
        let audit = suite_dir.join("master-log-audit.json");
        let audit_value = serde_json::json!({
            "schema": "adl.runtime.master_log_audit.v1",
            "status": "pass",
            "platform": std::env::consts::OS,
            "suite": suite,
            "revision": revision,
            "master_log_sha256": log_sha256,
            "record_count": 3,
            "malformed_records": 0,
            "missing_required_fields": 0,
            "sequence_gaps": 0,
            "error_events": 0,
            "degraded_events": 0,
            "unexplained_restarts": 0,
            "incomplete_drains": 0,
        });
        std::fs::write(
            &audit,
            serde_json::to_vec_pretty(&audit_value).expect("audit bytes"),
        )
        .expect("audit");
        let audit_sha256 = file_sha256(&audit).expect("audit sha");
        let (requested_runs, requested_cycles, duration_seconds) = match suite {
            "preflight_1x" => (1, Some(1), None),
            "lifecycle_10000" => (1, Some(REQUIRED_CYCLES), None),
            "stress_100x10s" => (STRESS_RUNS, None, Some(STRESS_SECONDS)),
            "endurance_10x600s" => (ENDURANCE_RUNS, None, Some(ENDURANCE_SECONDS)),
            _ => panic!("unsupported sample suite"),
        };
        let report = serde_json::json!({
            "schema": REPORT_SCHEMA,
            "status": "pass",
            "suite": suite,
            "platform": std::env::consts::OS,
            "architecture": std::env::consts::ARCH,
            "revision": revision,
            "requested_cycles": requested_cycles,
            "requested_runs": requested_runs,
            "duration_seconds_per_run": duration_seconds,
            "completed_runs": requested_runs,
            "completed_cycles": completed_cycles,
            "failed_cycles": 0,
            "degraded_cycles": 0,
            "minimum_cycles_per_run": completed_cycles.max(1),
            "guardian_process_count": 1,
            "guardian_launch_count": completed_cycles,
            "runtime_instance_count": completed_cycles + 1,
            "runtime_start_count": completed_cycles + 1,
            "anti_rollback_minimum_enforced": suite != "preflight_1x",
            "restart_budget_exercised": true,
            "total_restarts": 1,
            "acceptance_eligible": suite != "preflight_1x",
            "logging_complete": true,
            "log_checked_cycles": completed_cycles,
            "master_log_status": "clean",
            "master_log_ref": rel(root, &log),
            "master_log_sha256": log_sha256,
            "master_log_records": 3,
            "log_audit_ref": rel(root, &audit),
            "log_audit_sha256": audit_sha256,
            "continuity_generation": completed_cycles,
            "kernel_sha256": kernel_sha256,
            "duration_millis": 1,
            "failure": null,
        });
        let report_path = suite_dir.join("report.json");
        std::fs::write(
            &report_path,
            serde_json::to_vec_pretty(&report).expect("report bytes"),
        )
        .expect("report");
        report_path
    }

    fn rel(root: &Path, path: &Path) -> String {
        path.strip_prefix(root)
            .expect("repo-relative test path")
            .to_string_lossy()
            .replace('\\', "/")
    }
}
