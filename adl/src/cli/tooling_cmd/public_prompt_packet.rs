use anyhow::{bail, ensure, Result};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

use super::common::{
    contains_absolute_host_path_in_text, contains_secret_like_token, is_normalized_slug,
    repo_relative_display, repo_root, valid_github_issue_url, valid_version,
};
use super::structured_prompt::{
    validate_sip_text, validate_sor_text, validate_spp_text, validate_srp_text, validate_stp_text,
};
use super::tooling_usage;

const CARD_KINDS: [&str; 5] = ["sip", "stp", "spp", "srp", "sor"];

pub(crate) fn real_public_prompt_packet(args: &[String]) -> Result<()> {
    let Some(subcommand) = args.first().map(|arg| arg.as_str()) else {
        bail!("public-prompt-packet requires a subcommand: export");
    };

    match subcommand {
        "export" => export_public_prompt_packet(&args[1..]),
        "--help" | "-h" | "help" => {
            println!("{}", tooling_usage());
            Ok(())
        }
        other => bail!("unknown public-prompt-packet subcommand '{other}' (expected export)"),
    }
}

fn export_public_prompt_packet(args: &[String]) -> Result<()> {
    if has_help_arg(args) {
        println!("{}", tooling_usage());
        return Ok(());
    }

    let mut issue: Option<u32> = None;
    let mut slug: Option<String> = None;
    let mut version: Option<String> = None;
    let mut source: Option<PathBuf> = None;
    let mut out_root: Option<PathBuf> = None;
    let mut tracker_url: Option<String> = None;
    let mut root_override: Option<PathBuf> = None;

    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--issue" => {
                idx += 1;
                issue = Some(value_arg(args, idx, "--issue")?.parse()?);
            }
            "--slug" => {
                idx += 1;
                slug = Some(value_arg(args, idx, "--slug")?.to_string());
            }
            "--version" => {
                idx += 1;
                version = Some(value_arg(args, idx, "--version")?.to_string());
            }
            "--source" => {
                idx += 1;
                source = Some(PathBuf::from(value_arg(args, idx, "--source")?));
            }
            "--out-root" => {
                idx += 1;
                out_root = Some(PathBuf::from(value_arg(args, idx, "--out-root")?));
            }
            "--tracker-url" => {
                idx += 1;
                tracker_url = Some(value_arg(args, idx, "--tracker-url")?.to_string());
            }
            "--repo-root" => {
                idx += 1;
                root_override = Some(PathBuf::from(value_arg(args, idx, "--repo-root")?));
            }
            other => bail!("unknown arg for tooling public-prompt-packet export: {other}"),
        }
        idx += 1;
    }

    let root = root_override.unwrap_or(repo_root()?);
    let issue = issue.ok_or_else(|| anyhow::anyhow!("export requires --issue"))?;
    let slug = slug.ok_or_else(|| anyhow::anyhow!("export requires --slug"))?;
    let version = version.ok_or_else(|| anyhow::anyhow!("export requires --version"))?;
    ensure!(issue > 0, "--issue must be positive");
    ensure!(
        is_normalized_slug(&slug),
        "--slug must be lowercase kebab-case with no leading/trailing hyphen"
    );
    ensure!(
        valid_version(&version),
        "--version must use milestone form like v0.91.5"
    );
    if let Some(url) = tracker_url.as_deref() {
        ensure!(
            valid_github_issue_url(url),
            "--tracker-url must be a GitHub issue URL when provided"
        );
    }

    let source = source
        .map(|path| absolutize_against(&root, &path))
        .unwrap_or_else(|| {
            root.join(".adl")
                .join(&version)
                .join("tasks")
                .join(format!("issue-{issue}__{slug}"))
        });
    ensure!(
        source.is_dir(),
        "source card bundle not found: {}",
        source.display()
    );

    let out_root = out_root
        .map(|path| absolutize_against(&root, &path))
        .unwrap_or_else(|| {
            root.join("docs")
                .join("milestones")
                .join(&version)
                .join("review")
                .join("evidence")
                .join("csdlc")
                .join("issues")
        });
    let packet_dir = out_root.join(format!("issue-{issue}-{slug}"));
    let mut source_cards = Vec::new();
    for kind in CARD_KINDS {
        let source_path = source.join(format!("{kind}.md"));
        ensure!(
            source_path.is_file(),
            "missing required {kind} card: {}",
            source_path.display()
        );
        let text = fs::read_to_string(&source_path)?;
        validate_public_card_text(kind, &source_path, &text)?;
        source_cards.push((kind, source_path, text));
    }

    if packet_dir.exists() {
        ensure!(
            packet_dir.is_dir(),
            "public packet output path exists but is not a directory: {}",
            packet_dir.display()
        );
        fs::remove_dir_all(&packet_dir)?;
    }
    let cards_dir = packet_dir.join("cards");
    fs::create_dir_all(&cards_dir)?;

    let mut cards = Vec::new();
    let mut checks = Vec::new();
    for (kind, source_path, text) in source_cards {
        let public_path = cards_dir.join(format!("{kind}.md"));
        fs::write(&public_path, text.as_bytes())?;

        cards.push(json!({
            "kind": kind,
            "source_rel_path": repo_relative_display(&root, &source_path)?,
            "public_rel_path": repo_relative_display(&root, &public_path)?,
            "template_version": extract_template_version(&text),
            "card_status": extract_card_status(&text),
            "validation_state": "source_present_export_hygiene_passed"
        }));
        checks.push(format!("{kind}: no host paths, secret-like tokens, private key markers, or local scratch paths"));
    }

    let source_rel = repo_relative_display(&root, &source)?;
    let packet_rel = repo_relative_display(&root, &packet_dir)?;
    let manifest = json!({
        "schema": "adl.public_prompt_packet.v1",
        "version": &version,
        "issue_number": issue,
        "slug": &slug,
        "template_registry": "docs/templates/prompts/current.json",
        "source_bundle": &source_rel,
        "output_dir": &packet_rel,
        "tracker": {
            "provider": "github",
            "url": tracker_url.as_deref(),
            "issue_number": issue
        },
        "work_item": {
            "kind": "issue",
            "id": format!("issue-{issue}"),
            "slug": &slug
        },
        "cards": cards,
        "redaction": {
            "status": "passed",
            "mode": "refuse_not_rewrite",
            "checks": checks
        },
        "generated_by": "adl tooling public-prompt-packet export",
        "non_claims": [
            "This packet does not make local .adl state canonical public truth.",
            "This packet does not claim runtime validation was executed.",
            "This packet preserves source card text after export hygiene checks."
        ]
    });
    fs::write(
        packet_dir.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)? + "\n",
    )?;
    fs::write(
        packet_dir.join("README.md"),
        packet_readme(
            issue,
            &version,
            &slug,
            &source_rel,
            &packet_rel,
            tracker_url.as_deref(),
        ),
    )?;

    println!("PASS: public prompt packet exported to {packet_rel}");
    Ok(())
}

fn validate_public_card_text(kind: &str, source_path: &Path, text: &str) -> Result<()> {
    if contains_absolute_host_path_in_text(text)
        || contains_secret_like_token(text)
        || contains_private_key_marker(text)
        || contains_local_scratch_marker(text)
    {
        bail!("{kind} card contains disallowed public-packet content");
    }
    ensure!(
        !text.contains("{{") && !text.contains("}}"),
        "{kind} card contains unresolved template markers"
    );
    match kind {
        "sip" => validate_sip_text(text, source_path, Some("bootstrap"))?,
        "stp" => validate_stp_text(text)?,
        "spp" => validate_spp_text(text)?,
        "srp" => validate_srp_text(text)?,
        "sor" => validate_sor_text(text, Some("bootstrap"))?,
        _ => bail!("unsupported public prompt card kind: {kind}"),
    }
    Ok(())
}

fn contains_private_key_marker(text: &str) -> bool {
    [
        "BEGIN OPENSSH PRIVATE KEY",
        "BEGIN RSA PRIVATE KEY",
        "BEGIN EC PRIVATE KEY",
        "id_rsa",
        "id_ed25519",
        ".ssh/",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn contains_local_scratch_marker(text: &str) -> bool {
    [
        "$HOME/temp/",
        "$HOME/tmp/",
        "/private/tmp/",
        ".worktrees/",
        ".codex/attachments/",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn extract_template_version(text: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (_, rest) = line.split_once("docs/templates/prompts/")?;
        let version = rest.split('/').next()?;
        if valid_version_like_template(version) {
            Some(version.to_string())
        } else {
            None
        }
    })
}

fn valid_version_like_template(value: &str) -> bool {
    value
        .split('.')
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
}

fn extract_card_status(text: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        for prefix in ["card_status:", "Card Status:"] {
            if let Some(value) = trimmed.strip_prefix(prefix) {
                return Some(value.trim().trim_matches('"').trim_matches('`').to_string());
            }
        }
    }
    None
}

fn packet_readme(
    issue: u32,
    version: &str,
    slug: &str,
    source_rel: &str,
    packet_rel: &str,
    tracker_url: Option<&str>,
) -> String {
    let tracker = tracker_url.unwrap_or("not provided");
    format!(
        "# Public C-SDLC Prompt Packet: issue-{issue}\n\n\
## Summary\n\n\
This packet exports the public prompt-card record for `{slug}` in `{version}`.\n\n\
## Source\n\n\
- Source bundle: `{source_rel}`\n\
- Output packet: `{packet_rel}`\n\
- Tracker URL: `{tracker}`\n\n\
## Contents\n\n\
- `cards/sip.md`\n\
- `cards/stp.md`\n\
- `cards/spp.md`\n\
- `cards/srp.md`\n\
- `cards/sor.md`\n\
- `manifest.json`\n\n\
## Safety Boundary\n\n\
The exporter refuses obvious host-local paths, secret-like tokens, private key \
markers, local scratch paths, and unresolved template markers. It does not \
rewrite card content during export.\n\n\
## Non-Claims\n\n\
- This packet does not make local `.adl` state canonical public truth.\n\
- This packet does not claim runtime validation was executed.\n\
- This packet is a reviewable prompt-record surface only.\n"
    )
}

fn absolutize_against(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn value_arg<'a>(args: &'a [String], idx: usize, flag: &str) -> Result<&'a str> {
    args.get(idx)
        .map(String::as_str)
        .ok_or_else(|| anyhow::anyhow!("missing value for {flag}"))
}

fn has_help_arg(args: &[String]) -> bool {
    args.iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h" | "help"))
}
