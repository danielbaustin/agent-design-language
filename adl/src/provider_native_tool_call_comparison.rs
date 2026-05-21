use crate::adl::ProviderSpec;
use crate::provider_substrate::{
    provider_invocation_target_v1, CapabilityModeV1, CapabilitySupportV1,
    ProviderInvocationTargetV1,
};
use crate::uts_acc_multi_model_benchmark::{
    UTS_ACC_MULTI_MODEL_BENCHMARK_PROMPT_VERSION,
    UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, io, path::Path};

pub const PROVIDER_NATIVE_TOOL_CALL_COMPARISON_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.2/review/provider_native_tool_call_comparison_report.json";
pub const PROVIDER_NATIVE_TOOL_CALL_COMPARISON_SCHEMA_VERSION: &str =
    "provider_native_tool_call_comparison.v1";
pub const PROVIDER_NATIVE_TOOL_CALL_COMPARISON_PROMPT_VERSION: &str =
    "wp03.provider_native_tool_call_comparison.v1";
#[cfg(test)]
const HOST_PATH_MARKER: &str = "/Users/daniel/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderNativeComparisonPromptRecord {
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub comparison_boundary_summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderNativeJsonProposalBaseline {
    pub interface_mode: &'static str,
    pub prompt_version: &'static str,
    pub tracked_report_artifact_path: &'static str,
    pub evaluated_fixture_panel_count: usize,
    pub task_count: usize,
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderNativeSurfaceClass {
    DirectNativeVendorAdapter,
    CompatibilityHttpProfile,
    LocalNativeCandidate,
    SemanticFallbackOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderNativeComparisonRunStatus {
    FixtureBaselineReferenced,
    SkippedMissingCredentials,
    SkippedUnconfiguredEndpoint,
    SkippedLocalRuntimeUnavailable,
    UnsupportedSurface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderNativeCapabilityRecord {
    pub supported: bool,
    pub mode: CapabilityModeV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderNativeComparisonEntry {
    pub candidate_id: &'static str,
    pub provider_family: &'static str,
    pub surface_class: ProviderNativeSurfaceClass,
    pub run_status: ProviderNativeComparisonRunStatus,
    pub skip_reason: Option<String>,
    pub required_env_vars: Vec<&'static str>,
    pub invocation_target: ProviderInvocationTargetV1,
    pub tool_calling: ProviderNativeCapabilityRecord,
    pub structured_json: ProviderNativeCapabilityRecord,
    pub semantic_tool_fallback: ProviderNativeCapabilityRecord,
    pub limitations: Vec<String>,
    pub comparison_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProviderNativeToolCallComparisonReport {
    pub schema_version: &'static str,
    pub prompt_record: ProviderNativeComparisonPromptRecord,
    pub compared_provider_count: usize,
    pub direct_native_supported_count: usize,
    pub compatibility_supported_count: usize,
    pub skipped_provider_count: usize,
    pub unsupported_provider_count: usize,
    pub json_proposal_baseline: ProviderNativeJsonProposalBaseline,
    pub providers: Vec<ProviderNativeComparisonEntry>,
    pub non_claims: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TrackedWp02BaselineArtifact {
    prompt_record: TrackedWp02PromptRecord,
    task_count: usize,
    evaluated_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TrackedWp02PromptRecord {
    prompt_version: String,
}

#[derive(Debug, Clone)]
struct ProviderComparisonFixture {
    candidate_id: &'static str,
    provider_family: &'static str,
    surface_class: ProviderNativeSurfaceClass,
    run_status: ProviderNativeComparisonRunStatus,
    provider_id: &'static str,
    skip_reason: Option<&'static str>,
    required_env_vars: &'static [&'static str],
    limitations: &'static [&'static str],
    comparison_notes: &'static [&'static str],
    spec: fn() -> ProviderSpec,
    model_override: Option<&'static str>,
}

fn prompt_record() -> ProviderNativeComparisonPromptRecord {
    ProviderNativeComparisonPromptRecord {
        prompt_version: PROVIDER_NATIVE_TOOL_CALL_COMPARISON_PROMPT_VERSION,
        issue_number: 3002,
        comparison_boundary_summary:
            "WP-03 compares ADL JSON proposal mode against provider-native or provider-adjacent tool-call surfaces without treating native success as ACC authority.",
    }
}

fn json_proposal_baseline() -> ProviderNativeJsonProposalBaseline {
    let report: TrackedWp02BaselineArtifact =
        serde_json::from_str(&read_tracked_wp02_baseline_artifact())
            .expect("tracked WP-02 benchmark artifact should remain valid json");
    ProviderNativeJsonProposalBaseline {
        interface_mode: "adl_json_proposal",
        prompt_version: UTS_ACC_MULTI_MODEL_BENCHMARK_PROMPT_VERSION,
        tracked_report_artifact_path: UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH,
        evaluated_fixture_panel_count: report.evaluated_count,
        task_count: report.task_count,
        summary:
            "WP-02 already proves the bounded ADL JSON proposal harness against deterministic fixture candidates; WP-03 compares that interface boundary to provider-native or provider-adjacent tool-call surfaces.",
    }
}

fn read_tracked_wp02_baseline_artifact() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH);
    fs::read_to_string(path).expect("tracked WP-02 benchmark artifact should exist")
}

fn capability_record(value: &CapabilitySupportV1) -> ProviderNativeCapabilityRecord {
    ProviderNativeCapabilityRecord {
        supported: value.supported,
        mode: value.mode.clone(),
    }
}

fn openai_native_spec() -> ProviderSpec {
    let mut config = HashMap::new();
    config.insert(
        "provider_model_id".to_string(),
        serde_json::json!("gpt-4.1-mini"),
    );
    config.insert(
        "model_ref".to_string(),
        serde_json::json!("reasoning/default"),
    );
    ProviderSpec {
        id: Some("openai_primary".to_string()),
        profile: None,
        kind: "openai".to_string(),
        base_url: None,
        default_model: Some("reasoning/default".to_string()),
        config,
    }
}

fn anthropic_native_spec() -> ProviderSpec {
    let mut config = HashMap::new();
    config.insert(
        "provider_model_id".to_string(),
        serde_json::json!("claude-haiku-4-5-20251001"),
    );
    config.insert(
        "model_ref".to_string(),
        serde_json::json!("reasoning/default"),
    );
    ProviderSpec {
        id: Some("anthropic_primary".to_string()),
        profile: None,
        kind: "anthropic".to_string(),
        base_url: None,
        default_model: Some("reasoning/default".to_string()),
        config,
    }
}

fn gemini_http_profile_spec() -> ProviderSpec {
    let mut config = HashMap::new();
    config.insert(
        "endpoint".to_string(),
        serde_json::json!("https://api.example.invalid/v1/complete"),
    );
    config.insert(
        "provider_model_id".to_string(),
        serde_json::json!("gemini-2.0-flash"),
    );
    config.insert(
        "model_ref".to_string(),
        serde_json::json!("reasoning/default"),
    );
    ProviderSpec {
        id: Some("gemini_primary".to_string()),
        profile: Some("http:gemini-2.0-flash".to_string()),
        kind: "http".to_string(),
        base_url: None,
        default_model: Some("reasoning/default".to_string()),
        config,
    }
}

fn deepseek_http_profile_spec() -> ProviderSpec {
    let mut config = HashMap::new();
    config.insert(
        "endpoint".to_string(),
        serde_json::json!("https://api.example.invalid/v1/complete"),
    );
    config.insert(
        "provider_model_id".to_string(),
        serde_json::json!("deepseek-chat"),
    );
    config.insert(
        "model_ref".to_string(),
        serde_json::json!("reasoning/default"),
    );
    ProviderSpec {
        id: Some("deepseek_primary".to_string()),
        profile: Some("http:deepseek-chat".to_string()),
        kind: "http".to_string(),
        base_url: None,
        default_model: Some("reasoning/default".to_string()),
        config,
    }
}

fn ollama_gpt_oss_spec() -> ProviderSpec {
    ProviderSpec {
        id: Some("local_ollama".to_string()),
        profile: None,
        kind: "local_ollama".to_string(),
        base_url: None,
        default_model: Some("gpt-oss:latest".to_string()),
        config: HashMap::new(),
    }
}

fn ollama_deepseek_spec() -> ProviderSpec {
    ProviderSpec {
        id: Some("local_ollama".to_string()),
        profile: None,
        kind: "local_ollama".to_string(),
        base_url: None,
        default_model: Some("deepseek-r1:latest".to_string()),
        config: HashMap::new(),
    }
}

fn comparison_fixtures() -> Vec<ProviderComparisonFixture> {
    vec![
        ProviderComparisonFixture {
            candidate_id: "openai_native_gpt_4_1_mini",
            provider_family: "openai",
            surface_class: ProviderNativeSurfaceClass::DirectNativeVendorAdapter,
            run_status: ProviderNativeComparisonRunStatus::SkippedMissingCredentials,
            provider_id: "openai_primary",
            skip_reason: Some(
                "bounded tracked artifact does not assume OPENAI_API_KEY; direct native OpenAI tool-call execution remains skipped until credentials are supplied",
            ),
            required_env_vars: &["OPENAI_API_KEY"],
            limitations: &[
                "This entry records the native OpenAI adapter surface, not a completed live provider run.",
                "Provider-native tool-call success would still feed proposal output into governed ADL review surfaces; it would not replace ACC authority.",
            ],
            comparison_notes: &[
                "OpenAI's first-class adapter is the clearest direct native tool-call comparison target for WP-03.",
                "Native structured output support is interface capability evidence, not execution authority evidence.",
            ],
            spec: openai_native_spec,
            model_override: None,
        },
        ProviderComparisonFixture {
            candidate_id: "anthropic_native_claude_haiku_4_5",
            provider_family: "anthropic",
            surface_class: ProviderNativeSurfaceClass::DirectNativeVendorAdapter,
            run_status: ProviderNativeComparisonRunStatus::SkippedMissingCredentials,
            provider_id: "anthropic_primary",
            skip_reason: Some(
                "bounded tracked artifact does not assume ANTHROPIC_API_KEY; direct native Anthropic tool-call execution remains skipped until credentials are supplied",
            ),
            required_env_vars: &["ANTHROPIC_API_KEY"],
            limitations: &[
                "This entry records the native Anthropic adapter surface, not a completed live provider run.",
                "Anthropic native tool-call behavior would still require downstream ACC and Freedom Gate review before any side effect.",
            ],
            comparison_notes: &[
                "Anthropic is tracked separately from generic HTTP profiles because WP-03 needs a truthful native-vendor comparison route.",
            ],
            spec: anthropic_native_spec,
            model_override: None,
        },
        ProviderComparisonFixture {
            candidate_id: "gemini_http_compat_flash",
            provider_family: "gemini",
            surface_class: ProviderNativeSurfaceClass::CompatibilityHttpProfile,
            run_status: ProviderNativeComparisonRunStatus::SkippedUnconfiguredEndpoint,
            provider_id: "gemini_primary",
            skip_reason: Some(
                "the canonical Gemini profile in this bounded report still uses a placeholder compatibility endpoint; no raw vendor-native tool-call run is claimed",
            ),
            required_env_vars: &["GEMINI_API_KEY"],
            limitations: &[
                "The `http:gemini-2.0-flash` profile is a compatibility HTTP surface, not a proof of raw vendor-native Gemini tool-call semantics.",
                "A real run would require a trusted ADL-compatible endpoint and explicit credential injection.",
            ],
            comparison_notes: &[
                "This entry exists to make provider-adjacent compatibility surfaces visible instead of letting them silently masquerade as native-vendor coverage.",
            ],
            spec: gemini_http_profile_spec,
            model_override: None,
        },
        ProviderComparisonFixture {
            candidate_id: "deepseek_http_compat_chat",
            provider_family: "deepseek",
            surface_class: ProviderNativeSurfaceClass::CompatibilityHttpProfile,
            run_status: ProviderNativeComparisonRunStatus::SkippedUnconfiguredEndpoint,
            provider_id: "deepseek_primary",
            skip_reason: Some(
                "the canonical DeepSeek profile in this bounded report still uses a placeholder compatibility endpoint; no raw vendor-native tool-call run is claimed",
            ),
            required_env_vars: &["DEEPSEEK_API_KEY"],
            limitations: &[
                "The `http:deepseek-chat` profile is a compatibility HTTP surface, not a proof of raw vendor-native DeepSeek tool-call semantics.",
                "A real run would require a trusted ADL-compatible endpoint and explicit credential injection.",
            ],
            comparison_notes: &[
                "DeepSeek remains visible in the panel because the milestone docs name provider-native comparison, not just OpenAI/Anthropic.",
            ],
            spec: deepseek_http_profile_spec,
            model_override: None,
        },
        ProviderComparisonFixture {
            candidate_id: "ollama_local_gpt_oss_native_candidate",
            provider_family: "ollama",
            surface_class: ProviderNativeSurfaceClass::LocalNativeCandidate,
            run_status: ProviderNativeComparisonRunStatus::SkippedLocalRuntimeUnavailable,
            provider_id: "local_ollama",
            skip_reason: Some(
                "tracked artifact does not assume a local Ollama runtime or model pull; native-capable local validation remains an operator-run follow-on",
            ),
            required_env_vars: &[],
            limitations: &[
                "This entry captures the local native-candidate path only; the tracked report does not claim a live local Ollama run.",
                "Local native capability still does not bypass UTS/ACC proposal and authority boundaries.",
            ],
            comparison_notes: &[
                "The provider substrate marks GPT-OSS-style Ollama models as native tool-capable, which makes this the strongest local native candidate surface for later live runs.",
            ],
            spec: ollama_gpt_oss_spec,
            model_override: Some("gpt-oss"),
        },
        ProviderComparisonFixture {
            candidate_id: "ollama_local_deepseek_semantic_fallback",
            provider_family: "ollama",
            surface_class: ProviderNativeSurfaceClass::SemanticFallbackOnly,
            run_status: ProviderNativeComparisonRunStatus::UnsupportedSurface,
            provider_id: "local_ollama",
            skip_reason: Some(
                "the DeepSeek local Ollama path is semantic-fallback only in the current provider substrate and is not a native tool-call comparison surface",
            ),
            required_env_vars: &[],
            limitations: &[
                "Semantic fallback can remain useful for proposal generation, but it is not equivalent to a native tool/function-call interface.",
                "This path must not be counted as provider-native success in release or milestone claims.",
            ],
            comparison_notes: &[
                "Including one explicit unsupported local surface prevents the report from overstating local-provider coverage.",
            ],
            spec: ollama_deepseek_spec,
            model_override: Some("deepseek-r1"),
        },
    ]
}

fn comparison_entry(fixture: &ProviderComparisonFixture) -> ProviderNativeComparisonEntry {
    let target = provider_invocation_target_v1(
        fixture.provider_id,
        &(fixture.spec)(),
        fixture.model_override,
    )
    .expect("provider-native comparison fixtures should remain normalizable");
    ProviderNativeComparisonEntry {
        candidate_id: fixture.candidate_id,
        provider_family: fixture.provider_family,
        surface_class: fixture.surface_class.clone(),
        run_status: fixture.run_status.clone(),
        skip_reason: fixture.skip_reason.map(ToString::to_string),
        required_env_vars: fixture.required_env_vars.to_vec(),
        tool_calling: capability_record(&target.capabilities.tool_calling),
        structured_json: capability_record(&target.capabilities.structured_json),
        semantic_tool_fallback: capability_record(&target.capabilities.semantic_tool_fallback),
        invocation_target: target,
        limitations: fixture
            .limitations
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        comparison_notes: fixture
            .comparison_notes
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    }
}

pub fn run_provider_native_tool_call_comparison() -> ProviderNativeToolCallComparisonReport {
    let providers = comparison_fixtures()
        .iter()
        .map(comparison_entry)
        .collect::<Vec<_>>();
    let direct_native_supported_count = providers
        .iter()
        .filter(|entry| {
            entry.tool_calling.supported
                && matches!(
                    entry.surface_class,
                    ProviderNativeSurfaceClass::DirectNativeVendorAdapter
                        | ProviderNativeSurfaceClass::LocalNativeCandidate
                )
        })
        .count();
    let compatibility_supported_count = providers
        .iter()
        .filter(|entry| {
            entry.tool_calling.supported
                && entry.surface_class == ProviderNativeSurfaceClass::CompatibilityHttpProfile
        })
        .count();
    let skipped_provider_count = providers
        .iter()
        .filter(|entry| {
            matches!(
                entry.run_status,
                ProviderNativeComparisonRunStatus::SkippedMissingCredentials
                    | ProviderNativeComparisonRunStatus::SkippedUnconfiguredEndpoint
                    | ProviderNativeComparisonRunStatus::SkippedLocalRuntimeUnavailable
            )
        })
        .count();
    let unsupported_provider_count = providers
        .iter()
        .filter(|entry| entry.run_status == ProviderNativeComparisonRunStatus::UnsupportedSurface)
        .count();

    ProviderNativeToolCallComparisonReport {
        schema_version: PROVIDER_NATIVE_TOOL_CALL_COMPARISON_SCHEMA_VERSION,
        prompt_record: prompt_record(),
        compared_provider_count: providers.len(),
        direct_native_supported_count,
        compatibility_supported_count,
        skipped_provider_count,
        unsupported_provider_count,
        json_proposal_baseline: json_proposal_baseline(),
        providers,
        non_claims: vec![
            "Provider-native or provider-adjacent interface support does not grant execution authority; ACC, policy, and Freedom Gate remain authoritative.",
            "This tracked report compares interface surfaces and skip states; it does not claim a complete live hosted-provider ranking.",
            "Compatibility HTTP profiles must not be mistaken for raw vendor-native tool/function-call proof.",
        ],
    }
}

pub fn write_provider_native_tool_call_comparison_report(
    path: impl AsRef<Path>,
) -> io::Result<ProviderNativeToolCallComparisonReport> {
    let report = run_provider_native_tool_call_comparison();
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let body = serde_json::to_string_pretty(&report).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("serialize provider-native tool-call comparison report: {error}"),
        )
    })?;
    fs::write(path, body + "\n")?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn provider_native_tool_call_comparison_covers_expected_provider_panel() {
        let report = run_provider_native_tool_call_comparison();
        let ids = report
            .providers
            .iter()
            .map(|entry| entry.candidate_id)
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                "openai_native_gpt_4_1_mini",
                "anthropic_native_claude_haiku_4_5",
                "gemini_http_compat_flash",
                "deepseek_http_compat_chat",
                "ollama_local_gpt_oss_native_candidate",
                "ollama_local_deepseek_semantic_fallback",
            ]
        );
    }

    #[test]
    fn provider_native_tool_call_comparison_preserves_native_vs_unsupported_truth() {
        let report = run_provider_native_tool_call_comparison();
        let openai = report
            .providers
            .iter()
            .find(|entry| entry.candidate_id == "openai_native_gpt_4_1_mini")
            .expect("openai entry");
        assert!(openai.tool_calling.supported);
        assert_eq!(openai.tool_calling.mode, CapabilityModeV1::Native);
        assert_eq!(
            openai.run_status,
            ProviderNativeComparisonRunStatus::SkippedMissingCredentials
        );

        let deepseek = report
            .providers
            .iter()
            .find(|entry| entry.candidate_id == "ollama_local_deepseek_semantic_fallback")
            .expect("deepseek local entry");
        assert!(!deepseek.tool_calling.supported);
        assert_eq!(deepseek.tool_calling.mode, CapabilityModeV1::None);
        assert_eq!(
            deepseek.run_status,
            ProviderNativeComparisonRunStatus::UnsupportedSurface
        );
        assert_eq!(
            deepseek.semantic_tool_fallback.mode,
            CapabilityModeV1::SemanticFallback
        );
    }

    #[test]
    fn provider_native_tool_call_comparison_records_json_baseline_reference() {
        let report = run_provider_native_tool_call_comparison();
        let tracked: TrackedWp02BaselineArtifact =
            serde_json::from_str(&read_tracked_wp02_baseline_artifact())
                .expect("tracked WP-02 artifact should remain parseable");
        assert_eq!(
            report.json_proposal_baseline.prompt_version,
            tracked.prompt_record.prompt_version
        );
        assert_eq!(
            report.json_proposal_baseline.tracked_report_artifact_path,
            UTS_ACC_MULTI_MODEL_BENCHMARK_REPORT_ARTIFACT_PATH
        );
        assert_eq!(
            report.json_proposal_baseline.evaluated_fixture_panel_count,
            tracked.evaluated_count
        );
        assert_eq!(report.json_proposal_baseline.task_count, tracked.task_count);
    }

    #[test]
    fn provider_native_tool_call_comparison_splits_direct_native_from_compatibility_counts() {
        let report = run_provider_native_tool_call_comparison();
        assert_eq!(report.direct_native_supported_count, 3);
        assert_eq!(report.compatibility_supported_count, 0);
    }

    #[test]
    fn provider_native_tool_call_comparison_compatibility_profiles_do_not_report_native_tool_support(
    ) {
        let report = run_provider_native_tool_call_comparison();
        for candidate_id in ["gemini_http_compat_flash", "deepseek_http_compat_chat"] {
            let entry = report
                .providers
                .iter()
                .find(|entry| entry.candidate_id == candidate_id)
                .expect("compatibility entry");
            assert_eq!(
                entry.surface_class,
                ProviderNativeSurfaceClass::CompatibilityHttpProfile
            );
            assert!(!entry.tool_calling.supported);
            assert_eq!(entry.tool_calling.mode, CapabilityModeV1::None);
            assert!(entry.structured_json.supported);
            assert_eq!(entry.structured_json.mode, CapabilityModeV1::PromptBased);
        }
    }

    #[test]
    fn provider_native_tool_call_comparison_serializes_deterministically_without_host_paths() {
        let first = serde_json::to_string_pretty(&run_provider_native_tool_call_comparison())
            .expect("serialize first report");
        let second = serde_json::to_string_pretty(&run_provider_native_tool_call_comparison())
            .expect("serialize second report");
        assert_eq!(first, second);
        assert!(!first.contains(HOST_PATH_MARKER));
        assert!(!first.contains("/Users/daniel/"));
    }

    #[test]
    fn provider_native_tool_call_comparison_report_writer_emits_portable_json() {
        let report_path = unique_temp_path("provider-native-tool-call-comparison");
        let report = write_provider_native_tool_call_comparison_report(&report_path)
            .expect("write provider-native comparison report");
        let body = fs::read_to_string(&report_path).expect("read report");

        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&body).expect("valid json"),
            serde_json::to_value(report).expect("report should serialize")
        );

        fs::remove_file(&report_path).expect("remove report");
    }

    #[test]
    fn provider_native_tool_call_comparison_artifact_path_is_repo_relative_and_bounded() {
        assert!(
            !Path::new(PROVIDER_NATIVE_TOOL_CALL_COMPARISON_REPORT_ARTIFACT_PATH).is_absolute()
        );
        assert!(!PROVIDER_NATIVE_TOOL_CALL_COMPARISON_REPORT_ARTIFACT_PATH.contains(".."));
    }

    #[test]
    fn provider_native_tool_call_comparison_tracked_report_matches_generated_report() {
        let expected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join(PROVIDER_NATIVE_TOOL_CALL_COMPARISON_REPORT_ARTIFACT_PATH);
        let actual = fs::read_to_string(expected).expect("read tracked report");
        let generated = serde_json::to_string_pretty(&run_provider_native_tool_call_comparison())
            .expect("serialize generated report")
            + "\n";
        assert_eq!(actual, generated);
    }
}
