use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const VISIBILITY_PACKET_SCHEMA: &str = "adl.csm_visibility_packet.v1";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ObservatoryFormat {
    Bundle,
    Json,
    Report,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ObservatoryOutput {
    pub packet_path: Option<PathBuf>,
    pub report_path: Option<PathBuf>,
    pub console_reference_path: Option<PathBuf>,
    pub manifest_path: Option<PathBuf>,
}

impl ObservatoryFormat {
    pub fn parse(raw: &str) -> Result<Self> {
        match raw {
            "bundle" => Ok(Self::Bundle),
            "json" => Ok(Self::Json),
            "report" => Ok(Self::Report),
            other => {
                bail!("unsupported CSM Observatory format '{other}' (expected bundle|json|report)")
            }
        }
    }
}

pub fn load_visibility_packet(path: &Path) -> Result<Value> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("read CSM Observatory packet '{}'", path.display()))?;
    let packet: Value = serde_json::from_str(&raw)
        .with_context(|| format!("parse CSM Observatory packet '{}'", path.display()))?;
    validate_visibility_packet(&packet)?;
    Ok(packet)
}

pub fn validate_visibility_packet(packet: &Value) -> Result<()> {
    let Some(object) = packet.as_object() else {
        bail!("CSM Observatory packet root must be an object");
    };
    let required = [
        "schema",
        "packet_id",
        "generated_at",
        "source",
        "manifold",
        "kernel",
        "citizens",
        "episodes",
        "freedom_gate",
        "invariants",
        "resources",
        "trace",
        "operator_actions",
        "review",
    ];
    for field in required {
        if !object.contains_key(field) {
            bail!("CSM Observatory packet missing required field '{field}'");
        }
    }
    if packet.pointer("/schema").and_then(Value::as_str) != Some(VISIBILITY_PACKET_SCHEMA) {
        bail!("CSM Observatory packet schema must be {VISIBILITY_PACKET_SCHEMA}");
    }
    require_section_fields(
        packet,
        "/source",
        "source",
        &[
            "mode",
            "evidence_level",
            "fixture",
            "runtime_artifact_root",
            "claim_boundary",
        ],
    )?;
    require_section_fields(
        packet,
        "/manifold",
        "manifold",
        &[
            "manifold_id",
            "display_name",
            "state",
            "lifecycle",
            "current_tick",
            "uptime",
            "policy_profile",
            "snapshot_status",
            "health",
            "evidence_refs",
        ],
    )?;
    require_section_fields(
        packet,
        "/kernel",
        "kernel",
        &[
            "scheduler_state",
            "trace_state",
            "invariant_state",
            "resource_state",
            "service_states",
            "active_guardrails",
            "pulse",
        ],
    )?;
    require_section_fields(
        packet,
        "/freedom_gate",
        "freedom_gate",
        &[
            "recent_docket",
            "allow_count",
            "defer_count",
            "refuse_count",
            "open_questions",
            "rejected_actions",
        ],
    )?;
    require_section_fields(
        packet,
        "/operator_actions",
        "operator_actions",
        &[
            "available_actions",
            "disabled_actions",
            "required_confirmations",
            "safety_notes",
        ],
    )?;
    require_section_fields(
        packet,
        "/review",
        "review",
        &[
            "primary_artifacts",
            "missing_artifacts",
            "demo_classification",
            "caveats",
            "next_consumers",
        ],
    )?;
    validate_source(packet)?;
    validate_citizens(packet)?;
    validate_episodes(packet)?;
    validate_freedom_gate(packet)?;
    validate_invariants(packet)?;
    validate_operator_actions(packet)?;
    validate_review(packet)?;
    validate_refs_and_leakage(packet)?;
    Ok(())
}

fn require_section_fields(
    packet: &Value,
    pointer: &str,
    label: &str,
    fields: &[&str],
) -> Result<()> {
    let Some(section) = packet.pointer(pointer).and_then(Value::as_object) else {
        bail!("CSM Observatory packet {label} must be an object");
    };
    for field in fields {
        if !section.contains_key(*field) {
            bail!("CSM Observatory packet {label}.{field} is required");
        }
    }
    Ok(())
}

fn array_at_required<'a>(packet: &'a Value, pointer: &str, label: &str) -> Result<&'a Vec<Value>> {
    packet
        .pointer(pointer)
        .and_then(Value::as_array)
        .with_context(|| format!("CSM Observatory packet {label} must be a list"))
}

fn validate_source(packet: &Value) -> Result<()> {
    let source_mode = packet.pointer("/source/mode").and_then(Value::as_str);
    if !matches!(
        source_mode,
        Some("fixture" | "captured_artifacts" | "live_runtime")
    ) {
        bail!("CSM Observatory packet source.mode is invalid");
    }
    if source_mode == Some("fixture") {
        if packet.pointer("/source/fixture").and_then(Value::as_bool) != Some(true) {
            bail!("fixture CSM Observatory packets must set source.fixture to true");
        }
        let claim_boundary = packet
            .pointer("/source/claim_boundary")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !claim_boundary.contains("not a live") {
            bail!("fixture CSM Observatory packets must state that they are not live runtime captures");
        }
        if packet
            .pointer("/review/demo_classification")
            .and_then(Value::as_str)
            != Some("fixture_backed")
        {
            bail!("fixture CSM Observatory packets must be classified as fixture_backed");
        }
    }
    Ok(())
}

fn validate_citizens(packet: &Value) -> Result<()> {
    let citizens = array_at_required(packet, "/citizens", "citizens")?;
    if citizens.len() < 2 {
        bail!("CSM Observatory packet citizens must contain at least two items");
    }
    let mut seen = HashSet::new();
    for citizen in citizens {
        let citizen_object = citizen
            .as_object()
            .context("CSM Observatory packet citizen entries must be objects")?;
        for field in [
            "citizen_id",
            "display_name",
            "role",
            "lifecycle_state",
            "continuity_status",
            "current_episode",
            "resource_balance",
            "recent_decisions",
            "capability_envelope",
            "alerts",
            "evidence_refs",
        ] {
            if !citizen_object.contains_key(field) {
                bail!("CSM Observatory packet citizen.{field} is required");
            }
        }
        let citizen_id = citizen
            .pointer("/citizen_id")
            .and_then(Value::as_str)
            .context("CSM Observatory packet citizen_id must be a string")?;
        if !seen.insert(citizen_id.to_string()) {
            bail!("CSM Observatory packet duplicate citizen_id: {citizen_id}");
        }
        if !matches!(
            citizen.pointer("/lifecycle_state").and_then(Value::as_str),
            Some(
                "proposed"
                    | "active"
                    | "awake"
                    | "sleeping"
                    | "paused"
                    | "degraded"
                    | "blocked"
                    | "suspended"
                    | "migrating"
            )
        ) {
            bail!("CSM Observatory packet citizen lifecycle_state is invalid");
        }
    }
    Ok(())
}

fn validate_episodes(packet: &Value) -> Result<()> {
    let citizens: HashSet<String> = array_at_required(packet, "/citizens", "citizens")?
        .iter()
        .filter_map(|citizen| citizen.pointer("/citizen_id").and_then(Value::as_str))
        .map(ToOwned::to_owned)
        .collect();
    let episodes = array_at_required(packet, "/episodes", "episodes")?;
    if episodes.is_empty() {
        bail!("CSM Observatory packet episodes must contain at least one item");
    }
    for episode in episodes {
        let episode_object = episode
            .as_object()
            .context("CSM Observatory packet episode entries must be objects")?;
        for field in [
            "episode_id",
            "title",
            "state",
            "citizen_ids",
            "started_at",
            "last_event",
            "proof_surface",
            "blocked_reason",
        ] {
            if !episode_object.contains_key(field) {
                bail!("CSM Observatory packet episode.{field} is required");
            }
        }
        if !matches!(
            episode.pointer("/state").and_then(Value::as_str),
            Some("planned" | "active" | "completed" | "blocked" | "deferred" | "failed")
        ) {
            bail!("CSM Observatory packet episode state is invalid");
        }
        for citizen_id in array_at_required(episode, "/citizen_ids", "episode.citizen_ids")? {
            let Some(citizen_id) = citizen_id.as_str() else {
                bail!("CSM Observatory packet episode citizen_ids must be strings");
            };
            if !citizens.contains(citizen_id) {
                bail!("CSM Observatory packet episode references unknown citizen_id: {citizen_id}");
            }
        }
    }
    Ok(())
}

fn validate_freedom_gate(packet: &Value) -> Result<()> {
    let docket = array_at_required(
        packet,
        "/freedom_gate/recent_docket",
        "freedom_gate.recent_docket",
    )?;
    let mut allow = 0;
    let mut defer = 0;
    let mut refuse = 0;
    for entry in docket {
        for field in [
            "decision_id",
            "actor",
            "action",
            "decision",
            "rationale",
            "evidence_ref",
        ] {
            if entry.pointer(&format!("/{field}")).is_none() {
                bail!("CSM Observatory packet freedom_gate.recent_docket.{field} is required");
            }
        }
        match entry.pointer("/decision").and_then(Value::as_str) {
            Some("allow") => allow += 1,
            Some("defer") => defer += 1,
            Some("refuse") => refuse += 1,
            _ => bail!("CSM Observatory packet Freedom Gate decision is invalid"),
        }
        if let Some(reference) = entry.pointer("/evidence_ref").and_then(Value::as_str) {
            validate_repo_relative_ref(reference, "freedom_gate.recent_docket.evidence_ref")?;
        }
    }
    if packet
        .pointer("/freedom_gate/allow_count")
        .and_then(Value::as_i64)
        != Some(allow)
    {
        bail!("CSM Observatory packet freedom_gate.allow_count does not match recent_docket");
    }
    if packet
        .pointer("/freedom_gate/defer_count")
        .and_then(Value::as_i64)
        != Some(defer)
    {
        bail!("CSM Observatory packet freedom_gate.defer_count does not match recent_docket");
    }
    if packet
        .pointer("/freedom_gate/refuse_count")
        .and_then(Value::as_i64)
        != Some(refuse)
    {
        bail!("CSM Observatory packet freedom_gate.refuse_count does not match recent_docket");
    }
    Ok(())
}

fn validate_invariants(packet: &Value) -> Result<()> {
    let invariants = array_at_required(packet, "/invariants", "invariants")?;
    if invariants.is_empty() {
        bail!("CSM Observatory packet invariants must contain at least one item");
    }
    for invariant in invariants {
        for field in [
            "invariant_id",
            "name",
            "state",
            "severity",
            "last_checked",
            "evidence_ref",
        ] {
            if invariant.pointer(&format!("/{field}")).is_none() {
                bail!("CSM Observatory packet invariant.{field} is required");
            }
        }
        if !matches!(
            invariant.pointer("/state").and_then(Value::as_str),
            Some("healthy" | "warning" | "violated" | "blocked" | "missing" | "deferred")
        ) {
            bail!("CSM Observatory packet invariant state is invalid");
        }
        if !matches!(
            invariant.pointer("/severity").and_then(Value::as_str),
            Some("info" | "low" | "medium" | "high" | "critical")
        ) {
            bail!("CSM Observatory packet invariant severity is invalid");
        }
        if let Some(reference) = invariant.pointer("/evidence_ref").and_then(Value::as_str) {
            validate_repo_relative_ref(reference, "invariant.evidence_ref")?;
        }
    }
    Ok(())
}

fn validate_operator_actions(packet: &Value) -> Result<()> {
    let source_mode = packet.pointer("/source/mode").and_then(Value::as_str);
    let available = array_at_required(
        packet,
        "/operator_actions/available_actions",
        "operator_actions.available_actions",
    )?;
    if available.is_empty() {
        bail!("CSM Observatory packet operator_actions.available_actions must contain at least one item");
    }
    for action in available {
        let action_object = action
            .as_object()
            .context("CSM Observatory packet available actions must be objects")?;
        for field in ["action", "mode", "status"] {
            if !action_object.contains_key(field) {
                bail!(
                    "CSM Observatory packet operator_actions.available_actions.{field} is required"
                );
            }
        }
        if source_mode == Some("fixture")
            && action.pointer("/mode").and_then(Value::as_str) != Some("read_only")
        {
            bail!("fixture CSM Observatory packet available actions must be read_only");
        }
    }
    array_at_required(
        packet,
        "/operator_actions/disabled_actions",
        "operator_actions.disabled_actions",
    )?;
    Ok(())
}

fn validate_review(packet: &Value) -> Result<()> {
    let consumers = array_at_required(packet, "/review/next_consumers", "review.next_consumers")?;
    let observed: HashSet<i64> = consumers
        .iter()
        .filter_map(|consumer| consumer.pointer("/issue").and_then(Value::as_i64))
        .collect();
    for issue in [2189, 2190, 2191, 2192] {
        if !observed.contains(&issue) {
            bail!("CSM Observatory packet review.next_consumers must include issue #{issue}");
        }
    }
    Ok(())
}

fn validate_refs_and_leakage(packet: &Value) -> Result<()> {
    walk_packet_strings(packet, "packet", &mut |path, value| {
        if path.ends_with("_ref")
            || path.contains("_refs[")
            || path.contains("primary_artifacts[")
            || path.contains("missing_artifacts[")
        {
            validate_repo_relative_ref(value, path)?;
        }
        validate_no_private_or_secret_text(value)?;
        Ok(())
    })
}

fn walk_packet_strings<F>(value: &Value, path: &str, visit: &mut F) -> Result<()>
where
    F: FnMut(&str, &str) -> Result<()>,
{
    match value {
        Value::String(text) => visit(path, text),
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                walk_packet_strings(item, &format!("{path}[{index}]"), visit)?;
            }
            Ok(())
        }
        Value::Object(map) => {
            for (key, child) in map {
                walk_packet_strings(child, &format!("{path}.{key}"), visit)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn validate_repo_relative_ref(reference: &str, path: &str) -> Result<()> {
    if reference.starts_with("http://") || reference.starts_with("https://") {
        bail!("CSM Observatory packet {path} must not be a URL");
    }
    let ref_path = Path::new(reference);
    if ref_path.is_absolute()
        || reference.split('/').any(|part| part == "..")
        || reference.contains('\\')
    {
        bail!("CSM Observatory packet {path} must be repository-relative");
    }
    Ok(())
}

fn validate_no_private_or_secret_text(text: &str) -> Result<()> {
    let lower = text.to_ascii_lowercase();
    if text.contains("/Users/")
        || text.contains("/private/var/")
        || lower.contains("localhost:")
        || lower.contains("192.168.")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("apikey")
        || lower.contains("secret=")
        || lower.contains("secret:")
        || lower.contains("token=")
        || lower.contains("token:")
    {
        bail!("CSM Observatory packet leaked private path, endpoint, or secret-like value");
    }
    Ok(())
}

pub fn write_observatory_outputs(
    packet_path: &Path,
    out_dir: &Path,
    format: ObservatoryFormat,
) -> Result<ObservatoryOutput> {
    let packet = load_visibility_packet(packet_path)?;
    fs::create_dir_all(out_dir)
        .with_context(|| format!("create CSM Observatory output dir '{}'", out_dir.display()))?;

    let mut output = ObservatoryOutput {
        packet_path: None,
        report_path: None,
        console_reference_path: None,
        manifest_path: None,
    };

    if matches!(format, ObservatoryFormat::Bundle | ObservatoryFormat::Json) {
        let path = out_dir.join("visibility_packet.json");
        fs::write(&path, serde_json::to_string_pretty(&packet)? + "\n")
            .with_context(|| format!("write '{}'", path.display()))?;
        output.packet_path = Some(path);
    }

    if matches!(
        format,
        ObservatoryFormat::Bundle | ObservatoryFormat::Report
    ) {
        let path = out_dir.join("operator_report.md");
        fs::write(&path, render_operator_report(&packet))
            .with_context(|| format!("write '{}'", path.display()))?;
        output.report_path = Some(path);
    }

    if matches!(format, ObservatoryFormat::Bundle) {
        let path = out_dir.join("console_reference.md");
        fs::write(&path, render_console_reference(packet_path))
            .with_context(|| format!("write '{}'", path.display()))?;
        output.console_reference_path = Some(path);

        let manifest_path = out_dir.join("demo_manifest.json");
        fs::write(
            &manifest_path,
            serde_json::to_string_pretty(&render_manifest(&packet, &output))? + "\n",
        )
        .with_context(|| format!("write '{}'", manifest_path.display()))?;
        output.manifest_path = Some(manifest_path);
    }

    Ok(output)
}

fn render_manifest(packet: &Value, output: &ObservatoryOutput) -> Value {
    json!({
        "schema": "adl.csm_observatory.demo_manifest.v1",
        "demo_id": "demo-v0901-csm-observatory",
        "classification": packet.pointer("/review/demo_classification").and_then(Value::as_str).unwrap_or("unknown"),
        "packet_id": packet.pointer("/packet_id").and_then(Value::as_str).unwrap_or("unknown"),
        "proof_surfaces": {
            "visibility_packet": output.packet_path.as_deref().map(display_path),
            "operator_report": output.report_path.as_deref().map(display_path),
            "console_reference": output.console_reference_path.as_deref().map(display_path),
        },
        "truth_boundary": packet.pointer("/source/claim_boundary").and_then(Value::as_str).unwrap_or("not recorded"),
        "read_only": true,
        "live_runtime_capture": packet.pointer("/source/mode").and_then(Value::as_str) == Some("live_runtime"),
    })
}

fn display_path(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string())
}

fn render_console_reference(packet_path: &Path) -> String {
    let packet_ref = packet_path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| packet_path.display().to_string());
    format!(
        concat!(
            "# CSM Observatory Console Reference\n\n",
            "The CLI bundle is read-only. To inspect the visual console, open:\n\n",
            "- demos/v0.90.1/csm_observatory_static_console.html\n\n",
            "The command input packet was:\n\n",
            "- {}\n\n",
            "The static console and this CLI bundle both consume the same ",
            "adl.csm_visibility_packet.v1 contract. No live mutation is performed.\n"
        ),
        packet_ref
    )
}

pub fn render_operator_report(packet: &Value) -> String {
    let mut lines = Vec::new();
    let manifold = packet.get("manifold").unwrap_or(&Value::Null);
    let kernel = packet.get("kernel").unwrap_or(&Value::Null);
    let source = packet.get("source").unwrap_or(&Value::Null);
    let review = packet.get("review").unwrap_or(&Value::Null);

    lines.push(format!(
        "# CSM Observatory Operator Report: {}",
        str_at(manifold, "/display_name")
    ));
    lines.push(String::new());
    lines.push("## Report Identity".to_string());
    lines.push(table_row(&["Field", "Value"]));
    lines.push(table_row(&["---", "---"]));
    lines.push(table_row(&["Packet", &str_at(packet, "/packet_id")]));
    lines.push(table_row(&["Schema", &str_at(packet, "/schema")]));
    lines.push(table_row(&["Generated", &str_at(packet, "/generated_at")]));
    lines.push(table_row(&["Source mode", &str_at(source, "/mode")]));
    lines.push(table_row(&[
        "Evidence level",
        &str_at(source, "/evidence_level"),
    ]));
    lines.push(table_row(&[
        "Demo classification",
        &str_at(review, "/demo_classification"),
    ]));
    lines.push(String::new());
    lines.push("## Operator Summary".to_string());
    lines.push(format!(
        "The manifold is {} at tick {}. The kernel pulse is {} through event sequence {}. Current evidence is {}; claim boundary: {}",
        str_at(manifold, "/state"),
        str_at(manifold, "/current_tick"),
        str_at(kernel, "/pulse/status"),
        str_at(kernel, "/pulse/completed_through_event_sequence"),
        str_at(source, "/evidence_level"),
        str_at(source, "/claim_boundary")
    ));
    lines.push(String::new());
    lines.push("## Attention Items".to_string());
    for item in attention_items(packet) {
        lines.push(format!("- {item}"));
    }
    lines.push(String::new());
    lines.push("## Citizens".to_string());
    lines.push(table_row(&[
        "Citizen",
        "State",
        "Continuity",
        "Episode",
        "Capability",
    ]));
    lines.push(table_row(&["---", "---", "---", "---", "---"]));
    for citizen in array_at(packet, "/citizens") {
        let capability = if bool_at(citizen, "/capability_envelope/can_execute_episodes") {
            "episode execution allowed"
        } else {
            "episode execution disabled"
        };
        lines.push(table_row(&[
            &str_at(citizen, "/display_name"),
            &str_at(citizen, "/lifecycle_state"),
            &str_at(citizen, "/continuity_status"),
            &str_at(citizen, "/current_episode"),
            capability,
        ]));
    }
    lines.push(String::new());
    lines.push("## Freedom Gate Docket".to_string());
    lines.push(format!(
        "Counts: allow {}, defer {}, refuse {}.",
        str_at(packet, "/freedom_gate/allow_count"),
        str_at(packet, "/freedom_gate/defer_count"),
        str_at(packet, "/freedom_gate/refuse_count")
    ));
    lines.push(String::new());
    lines.push(table_row(&[
        "Decision",
        "Actor",
        "Action",
        "Rationale",
        "Evidence",
    ]));
    lines.push(table_row(&["---", "---", "---", "---", "---"]));
    for entry in array_at(packet, "/freedom_gate/recent_docket") {
        lines.push(table_row(&[
            &str_at(entry, "/decision"),
            &str_at(entry, "/actor"),
            &str_at(entry, "/action"),
            &str_at(entry, "/rationale"),
            &str_at(entry, "/evidence_ref"),
        ]));
    }
    lines.push(String::new());
    lines.push("## Invariant Review".to_string());
    lines.push(table_row(&["Invariant", "State", "Severity", "Evidence"]));
    lines.push(table_row(&["---", "---", "---", "---"]));
    for invariant in sorted_invariants(packet) {
        lines.push(table_row(&[
            &str_at(invariant, "/name"),
            &str_at(invariant, "/state"),
            &str_at(invariant, "/severity"),
            &str_at(invariant, "/evidence_ref"),
        ]));
    }
    lines.push(String::new());
    lines.push("## Operator Action Boundary".to_string());
    lines.push("Available read-only actions:".to_string());
    for action in array_at(packet, "/operator_actions/available_actions") {
        lines.push(format!(
            "- {}: {}",
            str_at(action, "/action"),
            str_at(action, "/status")
        ));
    }
    lines.push(String::new());
    lines.push("Disabled mutation actions:".to_string());
    for action in array_at(packet, "/operator_actions/disabled_actions") {
        lines.push(format!(
            "- {}: {}",
            str_at(action, "/action"),
            str_at(action, "/reason")
        ));
    }
    lines.push(String::new());
    lines.push("## Evidence And Caveats".to_string());
    lines.push("Primary evidence references:".to_string());
    for reference in evidence_refs(packet) {
        lines.push(format!("- {reference}"));
    }
    lines.push(String::new());
    lines.push("Caveats:".to_string());
    for caveat in array_at(packet, "/review/caveats") {
        lines.push(format!("- {}", value_text(caveat)));
    }
    lines.push(String::new());
    lines.push("## Reviewer Use".to_string());
    lines.push("This report is a proof surface for the packet-to-operator-report path. It is useful for reviewing visibility semantics, attention routing, claim boundaries, and evidence coverage without opening the HTML console.".to_string());
    lines.push(String::new());
    lines.join("\n")
}

fn attention_items(packet: &Value) -> Vec<String> {
    let mut items = Vec::new();
    for item in array_at(packet, "/manifold/health/attention_items") {
        let text = capitalize(&value_text(item));
        if !text.to_ascii_lowercase().contains("snapshot evidence")
            && !text.to_ascii_lowercase().contains("proposed, not active")
        {
            items.push((2, text));
        }
    }
    if matches!(
        packet
            .pointer("/manifold/snapshot_status/state")
            .and_then(Value::as_str),
        Some("deferred" | "missing" | "blocked")
    ) {
        items.push((
            1,
            format!(
                "Snapshot evidence is {}: {}",
                str_at(packet, "/manifold/snapshot_status/state"),
                str_at(packet, "/manifold/snapshot_status/note")
            ),
        ));
    }
    for citizen in array_at(packet, "/citizens") {
        if str_at(citizen, "/lifecycle_state") != "active" {
            items.push((
                1,
                format!(
                    "{} is {}, not active; continuity is {}.",
                    str_at(citizen, "/display_name"),
                    str_at(citizen, "/lifecycle_state"),
                    str_at(citizen, "/continuity_status")
                ),
            ));
        }
    }
    for invariant in array_at(packet, "/invariants") {
        if str_at(invariant, "/state") != "healthy" {
            items.push((
                severity_rank(&str_at(invariant, "/severity")),
                format!(
                    "{} is {} ({}); evidence: {}.",
                    str_at(invariant, "/name"),
                    str_at(invariant, "/state"),
                    str_at(invariant, "/severity"),
                    str_at(invariant, "/evidence_ref")
                ),
            ));
        }
    }
    for gap in array_at(packet, "/trace/causal_gaps") {
        items.push((
            severity_rank(&str_at(gap, "/severity")),
            format!(
                "{}: {}",
                capitalize(&str_at(gap, "/severity")),
                str_at(gap, "/summary")
            ),
        ));
    }
    for question in array_at(packet, "/freedom_gate/open_questions") {
        items.push((
            2,
            format!("Open Freedom Gate question: {}", value_text(question)),
        ));
    }
    for action in array_at(packet, "/operator_actions/disabled_actions") {
        let issue_suffix = action
            .pointer("/future_issue")
            .and_then(Value::as_i64)
            .map(|issue| format!(" Future issue: #{issue}."))
            .unwrap_or_default();
        items.push((
            2,
            format!(
                "Operator action {} remains disabled: {}.{}",
                str_at(action, "/action"),
                str_at(action, "/reason").trim_end_matches('.'),
                issue_suffix
            ),
        ));
    }
    items.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    items.dedup_by(|left, right| left.1.eq_ignore_ascii_case(&right.1));
    items.into_iter().map(|(_, item)| item).collect()
}

fn sorted_invariants(packet: &Value) -> Vec<&Value> {
    let mut invariants = array_at(packet, "/invariants");
    invariants.sort_by(|left, right| {
        severity_rank(&str_at(left, "/severity"))
            .cmp(&severity_rank(&str_at(right, "/severity")))
            .then_with(|| str_at(left, "/name").cmp(&str_at(right, "/name")))
    });
    invariants
}

fn evidence_refs(packet: &Value) -> Vec<String> {
    let mut refs = Vec::new();
    for pointer in [
        "/source/source_refs",
        "/manifold/evidence_refs",
        "/kernel/pulse/evidence_refs",
        "/review/primary_artifacts",
    ] {
        for item in array_at(packet, pointer) {
            let item = value_text(item);
            if !refs.contains(&item) {
                refs.push(item);
            }
        }
    }
    refs
}

fn table_row(values: &[&str]) -> String {
    format!("| {} |", values.join(" | "))
}

fn str_at(value: &Value, pointer: &str) -> String {
    value
        .pointer(pointer)
        .map(value_text)
        .unwrap_or_else(|| "not recorded".to_string())
}

fn value_text(value: &Value) -> String {
    match value {
        Value::Null => "not recorded".to_string(),
        Value::String(text) if text.trim().is_empty() => "not recorded".to_string(),
        Value::String(text) => text.clone(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        other => other.to_string(),
    }
}

fn array_at<'a>(value: &'a Value, pointer: &str) -> Vec<&'a Value> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .map(|items| items.iter().collect())
        .unwrap_or_default()
}

fn bool_at(value: &Value, pointer: &str) -> bool {
    value
        .pointer(pointer)
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn severity_rank(value: &str) -> usize {
    match value {
        "critical" => 0,
        "high" => 1,
        "medium" => 2,
        "low" => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_packet() -> Value {
        serde_json::from_str(include_str!(
            "../../demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json"
        ))
        .expect("fixture packet")
    }

    #[test]
    fn fixture_packet_validates() {
        validate_visibility_packet(&fixture_packet()).expect("valid fixture");
    }

    #[test]
    fn operator_report_highlights_attention_items() {
        let report = render_operator_report(&fixture_packet());
        assert!(report.contains("CSM Observatory Operator Report"));
        assert!(report.contains("Attention Items"));
        assert!(report.contains("Snapshot evidence is deferred"));
        assert!(report.contains("Operator action pause_citizen remains disabled"));
        assert!(report.contains(
            "This packet is a fixture-backed contract and does not prove a live CSM run."
        ));
    }
}
