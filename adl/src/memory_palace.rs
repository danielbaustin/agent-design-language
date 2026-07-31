//! Deterministic Memory Palace context packets for long-lived agent handoff.
use crate::obsmem_contract::{MemoryCitation, MemoryRecord, MemoryTemporalAnchor};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

pub const MEMORY_PALACE_CONTEXT_SCHEMA: &str = "adl.memory_palace_context.v1";
pub const MEMORY_PALACE_INPUT_SCHEMA: &str = "adl.memory_palace_input.v1";
pub const MEMORY_PALACE_CONTEXT_REF: &str = "memory_palace_context.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceAgentConfig {
    pub input_ref: String,
    pub max_working_set_items: usize,
    pub stale_after_ms: u128,
    #[serde(default)]
    pub required_continuity_id: Option<String>,
    #[serde(default)]
    pub observed_epoch_ms: Option<u128>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceInput {
    pub schema: String,
    pub records: Vec<MemoryRecord>,
    #[serde(default)]
    pub expected_citation_hashes: BTreeMap<String, String>,
    #[serde(default)]
    pub required_continuity_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceContextPacket {
    pub schema: String,
    pub cycle_id: String,
    pub input_ref: String,
    pub canonical_input_sha256: String,
    pub topology: MemoryPalaceTopologyPacket,
    pub working_set: MemoryPalaceWorkingSetPacket,
    pub stale_context_report: MemoryPalaceStaleContextReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceTopologyPacket {
    pub rooms: Vec<MemoryPalaceRoom>,
    pub anchors: Vec<MemoryPalaceAnchor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceRoom {
    pub room_id: String,
    pub workflow_id: String,
    pub record_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceAnchor {
    pub anchor_id: String,
    pub record_id: String,
    pub run_id: String,
    pub continuity_id: Option<String>,
    pub effective_epoch_ms: u128,
    pub citations: Vec<MemoryCitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceWorkingSetPacket {
    pub max_items: usize,
    pub selected: Vec<MemoryPalaceWorkingSetItem>,
    pub excluded: Vec<MemoryPalaceExclusion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceWorkingSetItem {
    pub record_id: String,
    pub room_id: String,
    pub anchor_id: String,
    pub payload: String,
    pub provenance: Vec<MemoryCitation>,
    pub temporal_anchor: MemoryTemporalAnchor,
    pub inclusion_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceExclusion {
    pub record_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceStaleContextReport {
    pub observed_epoch_ms: u128,
    pub stale_after_ms: u128,
    pub fail_closed: bool,
    pub dispositions: Vec<MemoryPalaceDisposition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPalaceDisposition {
    pub record_id: String,
    pub status: String,
    pub reason: String,
}

pub fn build_context_from_agent_memory(
    memory: &Value,
    spec_dir: &Path,
    cycle_id: &str,
    observed_epoch_ms: u128,
) -> Result<Option<MemoryPalaceContextPacket>> {
    let Some(raw_config) = memory.get("memory_palace") else {
        return Ok(None);
    };
    if raw_config.is_null() {
        return Ok(None);
    }
    let config: MemoryPalaceAgentConfig = serde_json::from_value(raw_config.clone())
        .context("memory.memory_palace must be an object with input_ref and bounds")?;
    let observed = config.observed_epoch_ms.unwrap_or(observed_epoch_ms);
    let input_path = resolve_declared_input(spec_dir, &config.input_ref)?;
    let raw = fs::read(&input_path).with_context(|| {
        format!(
            "failed reading Memory Palace input {}",
            input_path.display()
        )
    })?;
    let input: MemoryPalaceInput = serde_json::from_slice(&raw).with_context(|| {
        format!(
            "failed parsing Memory Palace input {}",
            input_path.display()
        )
    })?;
    build_context_packet(cycle_id, &config, &input, observed).map(Some)
}

pub fn build_context_packet(
    cycle_id: &str,
    config: &MemoryPalaceAgentConfig,
    input: &MemoryPalaceInput,
    observed_epoch_ms: u128,
) -> Result<MemoryPalaceContextPacket> {
    validate_config(config)?;
    if cycle_id.trim().is_empty() {
        return Err(anyhow!("Memory Palace cycle_id must be non-empty"));
    }
    if input.schema != MEMORY_PALACE_INPUT_SCHEMA {
        return Err(anyhow!(
            "unsupported Memory Palace input schema '{}' (expected {MEMORY_PALACE_INPUT_SCHEMA})",
            input.schema
        ));
    }
    let required_continuity = config
        .required_continuity_id
        .as_ref()
        .or(input.required_continuity_id.as_ref());
    let mut records = input.records.clone();
    for record in &records {
        validate_record(
            record,
            input,
            required_continuity,
            observed_epoch_ms,
            config.stale_after_ms,
        )?;
    }
    records.sort_by(record_order);

    let canonical_input_sha256 = canonical_input_sha256(input, &records)?;
    let topology = build_topology(&records);
    let (working_set, dispositions) = build_working_set(
        &records,
        config.max_working_set_items,
        observed_epoch_ms,
        config.stale_after_ms,
    );
    Ok(MemoryPalaceContextPacket {
        schema: MEMORY_PALACE_CONTEXT_SCHEMA.to_string(),
        cycle_id: cycle_id.to_string(),
        input_ref: config.input_ref.clone(),
        canonical_input_sha256,
        topology,
        working_set,
        stale_context_report: MemoryPalaceStaleContextReport {
            observed_epoch_ms,
            stale_after_ms: config.stale_after_ms,
            fail_closed: false,
            dispositions,
        },
    })
}

pub fn context_packet_bytes(packet: &MemoryPalaceContextPacket) -> Result<Vec<u8>> {
    serde_json::to_vec(packet).context("serialize Memory Palace packet")
}

fn validate_config(config: &MemoryPalaceAgentConfig) -> Result<()> {
    validate_relative_ref(&config.input_ref)?;
    if config.max_working_set_items == 0 || config.max_working_set_items > 64 {
        return Err(anyhow!(
            "Memory Palace max_working_set_items must be between 1 and 64"
        ));
    }
    if config.stale_after_ms == 0 {
        return Err(anyhow!(
            "Memory Palace stale_after_ms must be greater than zero"
        ));
    }
    if config
        .required_continuity_id
        .as_ref()
        .is_some_and(|id| id.trim().is_empty())
    {
        return Err(anyhow!(
            "Memory Palace required_continuity_id must be non-empty when present"
        ));
    }
    Ok(())
}

fn validate_record(
    record: &MemoryRecord,
    input: &MemoryPalaceInput,
    required_continuity: Option<&String>,
    observed_epoch_ms: u128,
    stale_after_ms: u128,
) -> Result<()> {
    if record.id.trim().is_empty()
        || record.run_id.trim().is_empty()
        || record.workflow_id.trim().is_empty()
        || record.payload.trim().is_empty()
    {
        return Err(anyhow!(
            "Memory Palace records require non-empty id, run_id, workflow_id, and payload"
        ));
    }
    if contains_disallowed_content(&record.payload) {
        return Err(anyhow!(
            "Memory Palace record '{}' contains disallowed host-path or secret-like content",
            record.id
        ));
    }
    if record.citations.is_empty() {
        return Err(anyhow!(
            "Memory Palace record '{}' requires at least one citation",
            record.id
        ));
    }
    for citation in &record.citations {
        validate_citation(citation, input)?;
    }
    let anchor = record.temporal_anchor.as_ref().ok_or_else(|| {
        anyhow!(
            "Memory Palace record '{}' requires a temporal anchor",
            record.id
        )
    })?;
    anchor.validate().map_err(|err| {
        anyhow!(
            "Memory Palace record '{}' temporal anchor invalid: {err}",
            record.id
        )
    })?;
    if let Some(required) = required_continuity {
        if anchor.continuity_id.as_ref() != Some(required) {
            return Err(anyhow!(
                "Memory Palace record '{}' continuity_id does not match required continuity",
                record.id
            ));
        }
    }
    let effective_epoch_ms = anchor.effective_epoch_ms();
    if effective_epoch_ms > observed_epoch_ms {
        return Err(anyhow!(
            "Memory Palace record '{}' effective temporal anchor is after the observed handoff time",
            record.id
        ));
    }
    if observed_epoch_ms - effective_epoch_ms > stale_after_ms {
        return Err(anyhow!(
            "Memory Palace record '{}' is stale for the configured handoff window",
            record.id
        ));
    }
    Ok(())
}

fn validate_citation(citation: &MemoryCitation, input: &MemoryPalaceInput) -> Result<()> {
    validate_relative_ref(&citation.path)?;
    if !is_sha256_ref(&citation.hash) {
        return Err(anyhow!(
            "Memory Palace citation '{}' requires a sha256:<64-hex> hash",
            citation.path
        ));
    }
    if let Some(expected) = input.expected_citation_hashes.get(&citation.path) {
        if expected != &citation.hash {
            return Err(anyhow!(
                "Memory Palace citation '{}' hash does not match declared provenance",
                citation.path
            ));
        }
    }
    Ok(())
}

fn build_topology(records: &[MemoryRecord]) -> MemoryPalaceTopologyPacket {
    let mut rooms = BTreeMap::<String, BTreeSet<String>>::new();
    let mut anchors = Vec::new();
    for record in records {
        rooms
            .entry(record.workflow_id.clone())
            .or_default()
            .insert(record.id.clone());
        let temporal_anchor = record
            .temporal_anchor
            .as_ref()
            .expect("validated temporal anchor");
        anchors.push(MemoryPalaceAnchor {
            anchor_id: anchor_id(record),
            record_id: record.id.clone(),
            run_id: record.run_id.clone(),
            continuity_id: temporal_anchor.continuity_id.clone(),
            effective_epoch_ms: temporal_anchor.effective_epoch_ms(),
            citations: sorted_citations(&record.citations),
        });
    }
    MemoryPalaceTopologyPacket {
        rooms: rooms
            .into_iter()
            .map(|(workflow_id, ids)| MemoryPalaceRoom {
                room_id: room_id(&workflow_id),
                workflow_id,
                record_ids: ids.into_iter().collect(),
            })
            .collect(),
        anchors,
    }
}

fn build_working_set(
    records: &[MemoryRecord],
    max_items: usize,
    observed_epoch_ms: u128,
    stale_after_ms: u128,
) -> (MemoryPalaceWorkingSetPacket, Vec<MemoryPalaceDisposition>) {
    let mut selected = Vec::new();
    let mut excluded = Vec::new();
    let mut dispositions = Vec::new();
    for record in records {
        let temporal_anchor = record
            .temporal_anchor
            .as_ref()
            .expect("validated temporal anchor")
            .clone();
        let stale_age = observed_epoch_ms.saturating_sub(temporal_anchor.effective_epoch_ms());
        dispositions.push(MemoryPalaceDisposition {
            record_id: record.id.clone(),
            status: "current".to_string(),
            reason: format!("effective age {stale_age}ms within stale_after_ms {stale_after_ms}"),
        });
        if selected.len() < max_items {
            selected.push(MemoryPalaceWorkingSetItem {
                record_id: record.id.clone(),
                room_id: room_id(&record.workflow_id),
                anchor_id: anchor_id(record),
                payload: record.payload.clone(),
                provenance: sorted_citations(&record.citations),
                temporal_anchor,
                inclusion_reason: "canonical traversal within configured working-set bound"
                    .to_string(),
            });
        } else {
            excluded.push(MemoryPalaceExclusion {
                record_id: record.id.clone(),
                reason: format!("excluded after max_working_set_items={max_items}"),
            });
        }
    }
    (
        MemoryPalaceWorkingSetPacket {
            max_items,
            selected,
            excluded,
        },
        dispositions,
    )
}

fn record_order(left: &MemoryRecord, right: &MemoryRecord) -> std::cmp::Ordering {
    left.workflow_id
        .cmp(&right.workflow_id)
        .then_with(|| continuity(left).cmp(&continuity(right)))
        .then_with(|| left.run_id.cmp(&right.run_id))
        .then_with(|| left.id.cmp(&right.id))
}

fn continuity(record: &MemoryRecord) -> String {
    record
        .temporal_anchor
        .as_ref()
        .and_then(|anchor| anchor.continuity_id.clone())
        .unwrap_or_default()
}

fn sorted_citations(citations: &[MemoryCitation]) -> Vec<MemoryCitation> {
    let mut sorted = citations.to_vec();
    sorted.sort_by(|a, b| a.path.cmp(&b.path).then_with(|| a.hash.cmp(&b.hash)));
    sorted
}

fn canonical_input_sha256(input: &MemoryPalaceInput, records: &[MemoryRecord]) -> Result<String> {
    let mut canonical = input.clone();
    canonical.records = records.to_vec();
    for record in &mut canonical.records {
        record.tags.sort();
        record.tags.dedup();
        record.citations = sorted_citations(&record.citations);
        record.trace_event_refs.sort_by(|a, b| {
            a.event_sequence
                .cmp(&b.event_sequence)
                .then_with(|| a.event_kind.cmp(&b.event_kind))
                .then_with(|| a.step_id.cmp(&b.step_id))
                .then_with(|| a.delegation_id.cmp(&b.delegation_id))
        });
        record.review_findings.sort_by(|a, b| {
            a.id.cmp(&b.id)
                .then_with(|| a.severity.cmp(&b.severity))
                .then_with(|| a.disposition.cmp(&b.disposition))
                .then_with(|| a.summary.cmp(&b.summary))
        });
        record.residual_risks.sort();
        record.residual_risks.dedup();
        record.follow_on_refs.sort_by(|a, b| {
            a.issue_number
                .cmp(&b.issue_number)
                .then_with(|| a.title.cmp(&b.title))
                .then_with(|| a.status.cmp(&b.status))
        });
    }
    sha256_json(&canonical)
}

fn room_id(workflow_id: &str) -> String {
    format!("room:{}", stable_id(workflow_id))
}

fn anchor_id(record: &MemoryRecord) -> String {
    format!(
        "anchor:{}:{}",
        stable_id(&record.workflow_id),
        stable_id(&record.id)
    )
}

fn stable_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

fn resolve_declared_input(spec_dir: &Path, input_ref: &str) -> Result<PathBuf> {
    validate_relative_ref(input_ref)?;
    Ok(spec_dir.join(input_ref))
}

fn validate_relative_ref(value: &str) -> Result<()> {
    if value.trim().is_empty() || contains_disallowed_content(value) {
        return Err(anyhow!(
            "Memory Palace references must be non-empty and non-private"
        ));
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return Err(anyhow!("Memory Palace references must be relative"));
    }
    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        ) {
            return Err(anyhow!(
                "Memory Palace references must stay within the declared boundary"
            ));
        }
    }
    Ok(())
}

fn contains_disallowed_content(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "/users/",
        "\\users\\",
        "/private/",
        "bearer ",
        "api_key",
        "api key",
        "private key",
        "raw_chat_transcript",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn is_sha256_ref(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return false;
    };
    hex.len() == 64 && hex.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn sha256_json<T: Serialize>(value: &T) -> Result<String> {
    let bytes = serde_json::to_vec(value)?;
    let digest = Sha256::digest(bytes);
    Ok(format!("sha256:{digest:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::obsmem_contract::{MemoryCitation, MemoryTemporalAnchor, MemoryTraceRef};

    fn record(id: &str, effective_epoch_ms: u128) -> MemoryRecord {
        MemoryRecord {
            id: id.to_string(),
            run_id: format!("run-{id}"),
            workflow_id: "workflow-alpha".to_string(),
            tags: vec![],
            payload: format!("bounded handoff summary {id}"),
            score: "1.0".to_string(),
            citations: vec![MemoryCitation {
                path: format!("runs/{id}/trace.json"),
                hash: "sha256:1111111111111111111111111111111111111111111111111111111111111111"
                    .to_string(),
            }],
            trace_event_refs: vec![],
            temporal_anchor: Some(MemoryTemporalAnchor {
                t_created_epoch_ms: effective_epoch_ms,
                t_observed_epoch_ms: Some(effective_epoch_ms),
                t_effective_epoch_ms: Some(effective_epoch_ms),
                continuity_id: Some("continuity-alpha".to_string()),
                event_sequence: Some(1),
            }),
            review_findings: vec![],
            residual_risks: vec![],
            follow_on_refs: vec![],
        }
    }

    fn config(max: usize) -> MemoryPalaceAgentConfig {
        MemoryPalaceAgentConfig {
            input_ref: "memory_palace_input.json".to_string(),
            max_working_set_items: max,
            stale_after_ms: 1000,
            required_continuity_id: Some("continuity-alpha".to_string()),
            observed_epoch_ms: Some(2000),
        }
    }

    fn input(records: Vec<MemoryRecord>) -> MemoryPalaceInput {
        MemoryPalaceInput {
            schema: MEMORY_PALACE_INPUT_SCHEMA.to_string(),
            records,
            expected_citation_hashes: BTreeMap::new(),
            required_continuity_id: None,
        }
    }

    #[test]
    fn memory_palace_packet_is_deterministic_after_canonical_ordering() {
        let mut first = input(vec![record("b", 1500), record("a", 1500)]);
        let mut second = input(vec![record("a", 1500), record("b", 1500)]);
        first.records[0].citations.push(MemoryCitation {
            path: "runs/b/proof.json".to_string(),
            hash: "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                .to_string(),
        });
        second.records[1].citations.push(MemoryCitation {
            path: "runs/b/proof.json".to_string(),
            hash: "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                .to_string(),
        });
        first.records[0].trace_event_refs = vec![
            MemoryTraceRef {
                event_sequence: 1,
                event_kind: "step_finished".to_string(),
                step_id: Some("s2".to_string()),
                delegation_id: None,
            },
            MemoryTraceRef {
                event_sequence: 1,
                event_kind: "step_finished".to_string(),
                step_id: Some("s1".to_string()),
                delegation_id: None,
            },
        ];
        second.records[1].trace_event_refs = vec![
            MemoryTraceRef {
                event_sequence: 1,
                event_kind: "step_finished".to_string(),
                step_id: Some("s1".to_string()),
                delegation_id: None,
            },
            MemoryTraceRef {
                event_sequence: 1,
                event_kind: "step_finished".to_string(),
                step_id: Some("s2".to_string()),
                delegation_id: None,
            },
        ];

        let left = build_context_packet("cycle-000001", &config(16), &first, 2000).unwrap();
        let right = build_context_packet("cycle-000001", &config(16), &second, 2000).unwrap();

        assert_eq!(
            context_packet_bytes(&left).unwrap(),
            context_packet_bytes(&right).unwrap()
        );
        assert_eq!(left.working_set.selected[0].record_id, "a");
        assert_eq!(left.working_set.selected[1].record_id, "b");
    }

    #[test]
    fn memory_palace_rejects_stale_context() {
        let err = build_context_packet(
            "cycle-000001",
            &config(16),
            &input(vec![record("a", 999)]),
            2000,
        )
        .expect_err("stale context must fail closed");
        assert!(err.to_string().contains("stale"));
    }

    #[test]
    fn memory_palace_rejects_missing_citation_hash() {
        let mut bad = record("a", 1500);
        bad.citations[0].hash.clear();
        let err = build_context_packet("cycle-000001", &config(16), &input(vec![bad]), 2000)
            .expect_err("missing citation hash must fail");
        assert!(err.to_string().contains("sha256"));
    }

    #[test]
    fn memory_palace_rejects_private_paths_and_temporal_mismatch() {
        let mut private_path = record("a", 1500);
        private_path.citations[0].path = "/Users/daniel/private.json".to_string();
        let err = build_context_packet(
            "cycle-000001",
            &config(16),
            &input(vec![private_path]),
            2000,
        )
        .expect_err("private path must fail");
        assert!(err.to_string().contains("relative") || err.to_string().contains("private"));

        let mut wrong_continuity = record("b", 1500);
        wrong_continuity
            .temporal_anchor
            .as_mut()
            .unwrap()
            .continuity_id = Some("other".to_string());
        let err = build_context_packet(
            "cycle-000001",
            &config(16),
            &input(vec![wrong_continuity]),
            2000,
        )
        .expect_err("continuity mismatch must fail");
        assert!(err.to_string().contains("continuity_id"));

        let future_anchor = record("c", 2500);
        let err = build_context_packet(
            "cycle-000001",
            &config(16),
            &input(vec![future_anchor]),
            2000,
        )
        .expect_err("future effective anchor must fail");
        assert!(err.to_string().contains("observed handoff time"));
    }

    #[test]
    fn memory_palace_records_working_set_overflow_without_consuming_extra_items() {
        let packet = build_context_packet(
            "cycle-000001",
            &config(1),
            &input(vec![record("a", 1500), record("b", 1500)]),
            2000,
        )
        .unwrap();

        assert_eq!(packet.working_set.selected.len(), 1);
        assert_eq!(packet.working_set.excluded.len(), 1);
        assert_eq!(
            packet.working_set.excluded[0].reason,
            "excluded after max_working_set_items=1"
        );
    }
}
