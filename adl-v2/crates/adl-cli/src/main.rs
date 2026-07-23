use adl_compiler::compile;
use adl_language::{json_schema, parse_and_validate_json, parse_and_validate_yaml};
use adl_records::{
    decode_envelope, sign_record, verify_envelope, InMemoryReplayGuard, Limits, Record, TrustEntry,
    TrustPolicy,
};
use clap::{Parser, Subcommand};
use ed25519_dalek::{SigningKey, VerifyingKey};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const SELECTOR_SCHEMA: &str = "adl.selector.v1";
const RECEIPT_SCHEMA: &str = "adl.selector.receipt.v1";

#[derive(Parser)]
#[command(name = "adl-v2", version, about = "Thin ADL v2 owner CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Validate {
        input: PathBuf,
        #[arg(long)]
        yaml: bool,
    },
    Schema,
    Plan {
        input: PathBuf,
        #[arg(long)]
        yaml: bool,
    },
    Run {
        input: PathBuf,
        #[arg(long)]
        yaml: bool,
    },
    Inspect {
        #[arg(long)]
        root: Option<PathBuf>,
    },
    Sign {
        input: PathBuf,
        #[arg(long)]
        key_id: String,
        #[arg(long, env = "ADL_SIGNING_KEY_HEX")]
        key_hex: String,
    },
    Verify {
        input: PathBuf,
        #[arg(long, env = "ADL_VERIFY_KEY_HEX")]
        public_key_hex: String,
        #[arg(long, default_value_t = 0)]
        logical_time: u64,
    },
    Select {
        generation: String,
        #[arg(long)]
        expected_current_digest: Option<String>,
        #[arg(long)]
        root: Option<PathBuf>,
    },
    Rollback {
        #[arg(long)]
        root: Option<PathBuf>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct Selector {
    schema: String,
    current: Option<Selection>,
    previous: Option<Selection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Selection {
    generation: String,
    executable: String,
    digest: String,
    receipt: String,
}

#[derive(Debug, Serialize)]
struct Envelope<T: Serialize> {
    schema: &'static str,
    ok: bool,
    result: T,
}

fn main() {
    if let Err(error) = dispatch(Cli::parse().command) {
        let _ = print_json(&serde_json::json!({"schema":"adl.error.v1","ok":false,"error":error}));
        std::process::exit(2);
    }
}

fn dispatch(command: Command) -> Result<(), String> {
    match command {
        Command::Validate { input, yaml } => {
            let document = read_document(&input, yaml)?;
            print_json(&Envelope {
                schema: "adl.validate.v1",
                ok: true,
                result: document,
            })
        }
        Command::Schema => print_json(&Envelope {
            schema: "adl.schema.v1",
            ok: true,
            result: json_schema(),
        }),
        Command::Plan { input, yaml } => {
            let document = read_document(&input, yaml)?;
            let plan = compile(&document).map_err(|errors| {
                errors
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            })?;
            print_json(&Envelope {
                schema: "adl.plan.v1",
                ok: true,
                result: plan,
            })
        }
        Command::Run { input, yaml } => {
            let document = read_document(&input, yaml)?;
            let plan = compile(&document).map_err(|errors| {
                errors
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            })?;
            let policy = adl_engine::EnginePolicy::provider_for(&plan, 1);
            let engine =
                adl_engine::Engine::new(plan.clone(), policy, adl_engine::EngineLimits::default())
                    .map_err(|error| format!("{error:?}"))?;
            let result = serde_json::json!({"contract": adl_engine::ENGINE_CONTRACT_VERSION, "status": "ready", "plan": plan, "snapshot": engine.snapshot()});
            print_json(&Envelope {
                schema: "adl.run.v1",
                ok: true,
                result,
            })
        }
        Command::Inspect { root } => print_json(&Envelope {
            schema: SELECTOR_SCHEMA,
            ok: true,
            result: load_selector(root.as_deref())?,
        }),
        Command::Sign {
            input,
            key_id,
            key_hex,
        } => {
            let record: Record =
                serde_json::from_slice(&fs::read(&input).map_err(|_| "read record failed")?)
                    .map_err(|_| "record JSON rejected")?;
            let key_bytes = hex::decode(key_hex).map_err(|_| "signing key encoding rejected")?;
            let key_array: [u8; 32] = key_bytes
                .try_into()
                .map_err(|_| "signing key length rejected")?;
            let envelope = sign_record(
                record,
                &key_id,
                &SigningKey::from_bytes(&key_array),
                &Limits::default(),
            )
            .map_err(|error| error.to_string())?;
            print_json(&Envelope {
                schema: "adl.sign.v1",
                ok: true,
                result: envelope,
            })
        }
        Command::Verify {
            input,
            public_key_hex,
            logical_time,
        } => {
            let bytes = fs::read(&input).map_err(|_| "read envelope failed")?;
            let envelope =
                decode_envelope(&bytes, &Limits::default()).map_err(|error| error.to_string())?;
            let key_bytes =
                hex::decode(public_key_hex).map_err(|_| "verification key encoding rejected")?;
            let key_array: [u8; 32] = key_bytes
                .try_into()
                .map_err(|_| "verification key length rejected")?;
            let mut allowed_kinds = BTreeSet::new();
            allowed_kinds.insert(envelope.record_kind);
            let mut entries = BTreeMap::new();
            entries.insert(
                envelope.key_id.clone(),
                TrustEntry {
                    verifying_key: VerifyingKey::from_bytes(&key_array)
                        .map_err(|_| "verification key rejected")?,
                    profile_version: envelope.profile_version,
                    allowed_kinds,
                    not_before: 0,
                    not_after: u64::MAX,
                    revoked: false,
                },
            );
            let policy =
                TrustPolicy::new(entries, &Limits::default()).map_err(|error| error.to_string())?;
            let mut guard = InMemoryReplayGuard::new(&Limits::default());
            let record = verify_envelope(
                &envelope,
                &policy,
                &mut guard,
                logical_time,
                &Limits::default(),
            )
            .map_err(|error| error.to_string())?;
            print_json(&Envelope {
                schema: "adl.verify.v1",
                ok: true,
                result: serde_json::json!({"record_kind": record.kind(), "payload_digest": envelope.payload_digest}),
            })
        }
        Command::Select {
            generation,
            expected_current_digest,
            root,
        } => mutate_selector(root.as_deref(), generation, expected_current_digest, false),
        Command::Rollback { root } => mutate_selector(root.as_deref(), String::new(), None, true),
    }
}

fn read_document(path: &Path, yaml: bool) -> Result<adl_language::AdlDocument, String> {
    let source = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if yaml {
        parse_and_validate_yaml(&source).map_err(format_diagnostics)
    } else {
        parse_and_validate_json(&source).map_err(format_diagnostics)
    }
}

fn format_diagnostics(diagnostics: Vec<adl_language::Diagnostic>) -> String {
    serde_json::to_string(&diagnostics).unwrap_or_else(|_| "validation failed".into())
}

fn print_json<T: Serialize>(value: &T) -> Result<(), String> {
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    serde_json::to_writer(&mut stdout, value).map_err(|e| e.to_string())?;
    stdout.write_all(b"\n").map_err(|e| e.to_string())
}

fn selector_root(root: Option<&Path>) -> PathBuf {
    root.map(Path::to_path_buf)
        .or_else(|| std::env::var_os("ADL_DATA_ROOT").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from(".adl-v2"))
}

fn selector_path(root: Option<&Path>) -> PathBuf {
    selector_root(root).join("selector.json")
}

fn load_selector(root: Option<&Path>) -> Result<Selector, String> {
    let path = selector_path(root);
    if !path.exists() {
        return Ok(Selector {
            schema: SELECTOR_SCHEMA.into(),
            current: None,
            previous: None,
        });
    }
    let selector: Selector =
        serde_json::from_slice(&fs::read(path).map_err(|_| "read selector failed")?)
            .map_err(|_| "selector schema rejected")?;
    if selector.schema != SELECTOR_SCHEMA {
        return Err("selector schema rejected".into());
    }
    if let Some(previous) = selector.previous.as_ref() {
        validate_generation(&previous.generation)?;
    }
    if let Some(current) = selector.current.as_ref() {
        validate_generation(&current.generation)?;
    }
    Ok(selector)
}

fn digest_file(path: &Path) -> Result<serde_json::Value, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
    Ok(
        serde_json::json!({"path": path.file_name().and_then(|n| n.to_str()).unwrap_or("input"), "sha256": format!("{:x}", Sha256::digest(bytes))}),
    )
}

fn mutate_selector(
    root: Option<&Path>,
    generation: String,
    expected_current_digest: Option<String>,
    rollback: bool,
) -> Result<(), String> {
    let root = selector_root(root);
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let lock = File::create(root.join("selector.lock")).map_err(|e| e.to_string())?;
    lock.lock_exclusive().map_err(|e| e.to_string())?;
    let mut selector = load_selector(Some(&root))?;
    if let Some(expected) = expected_current_digest.as_deref() {
        let observed = selector
            .current
            .as_ref()
            .map(|selection| selection.digest.as_str())
            .unwrap_or("");
        if expected != observed {
            return Err("compare-and-swap current digest mismatch".into());
        }
    }
    let next = if rollback {
        selector
            .previous
            .clone()
            .ok_or_else(|| "no verified previous generation".to_string())?
    } else {
        validate_generation(&generation)?;
        let executable = root.join("bin").join(&generation);
        if !executable.is_file() {
            return Err(format!(
                "generation executable is missing: {}",
                executable.display()
            ));
        }
        let digest = digest_file(&executable)?["sha256"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        verify_install_receipt(&root, &generation, &digest)?;
        Selection {
            generation: generation.clone(),
            executable: executable.display().to_string(),
            digest: digest.clone(),
            receipt: format!("sha256:{digest}"),
        }
    };
    verify_selection(&root, &next)?;
    selector.previous = selector.current.take();
    selector.current = Some(next.clone());
    let bytes = serde_json::to_vec_pretty(&selector).map_err(|e| e.to_string())?;
    let mut temp = tempfile::NamedTempFile::new_in(&root).map_err(|e| e.to_string())?;
    temp.write_all(&bytes).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    temp.persist(selector_path(Some(&root)))
        .map_err(|e| e.error.to_string())?;
    if load_selector(Some(&root))?.current.as_ref() != Some(&next) {
        return Err("selector re-read mismatch".into());
    }
    print_json(&Envelope {
        schema: RECEIPT_SCHEMA,
        ok: true,
        result: serde_json::json!({"selection": next, "rollback": rollback}),
    })
}

fn verify_install_receipt(root: &Path, generation: &str, digest: &str) -> Result<(), String> {
    validate_generation(generation)?;
    let path = root.join("receipts").join(format!("{generation}.json"));
    let value: serde_json::Value = serde_json::from_slice(
        &fs::read(&path).map_err(|e| format!("read receipt {}: {e}", path.display()))?,
    )
    .map_err(|e| format!("parse receipt {}: {e}", path.display()))?;
    if value["schema"] != "adl.install.receipt.v1"
        || value["binary"] != generation
        || value["sha256"].as_str() != Some(digest)
    {
        return Err(format!("receipt identity mismatch: {}", path.display()));
    }
    Ok(())
}

fn validate_generation(generation: &str) -> Result<(), String> {
    if generation.is_empty()
        || generation == "."
        || generation == ".."
        || generation.contains('/')
        || generation.contains('\\')
    {
        return Err("invalid generation name".into());
    }
    Ok(())
}

fn verify_selection(root: &Path, selection: &Selection) -> Result<(), String> {
    let executable = Path::new(&selection.executable);
    let root = root
        .canonicalize()
        .map_err(|_| "selector root unavailable")?;
    let executable_root = executable
        .canonicalize()
        .map_err(|_| "selected executable unavailable")?;
    if !executable_root.starts_with(&root) {
        return Err("selected executable escapes selector root".into());
    }
    let observed = digest_file(&executable_root)?["sha256"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    if observed != selection.digest {
        return Err(format!(
            "selection digest mismatch: expected {}, observed {}",
            selection.digest, observed
        ));
    }
    verify_install_receipt(&root, &selection.generation, &selection.digest)
}
