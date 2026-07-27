using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;
using UnityEngine.UIElements;

namespace ADL.Demos.UnityObservatory
{
    public sealed class UnityObservatoryShellController : MonoBehaviour
    {
        private const string RuntimeStyleSheetResourcePath = "ObservatoryShellRuntime";
        private const string RuntimeApiEnvironmentVariable = "ADL_CSM_API_BASE";
        private const string RuntimeApiArgumentPrefix = "--csm-api-base=";
        private const string RuntimeV3ApiEnvironmentVariable =
            "ADL_RUNTIME_OBSERVATORY_URL";
        private const string RuntimeV3TokenEnvironmentVariable =
            "ADL_RUNTIME_OBSERVATORY_TOKEN";
        private const string RuntimeV3ApiArgumentPrefix = "--runtime-observatory-base=";
        private const string RuntimeV3ObservatoryPath = "/v1/observatory";
        private const string RuntimeV3ControlPath = "/v1/control";
        private const string RuntimeV3ObservatorySchema =
            "adl.runtime_v3.observatory_feed.v2";
        private const float RuntimeProbeIntervalSeconds = 3f;

        private enum RuntimeTransportKind
        {
            None,
            LegacyCsm,
            RuntimeV3,
        }

        private enum RuntimeTruthMode
        {
            Demo,
            Connecting,
            Live,
            Degraded,
            Disconnected,
        }

        [Serializable]
        private sealed class RuntimeStatusDocument
        {
            public string schema;
            public string status;
            public string ready;
            public string runtime_owner;
            public string agent_instance_id;
            public RuntimeAgentStatus agent_status;
            public RuntimeUptime uptime;
        }

        [Serializable]
        private sealed class RuntimeAgentStatus
        {
            public long completed_cycle_count;
            public string last_cycle_id;
            public string last_cycle_status;
            public string state;
            public string updated_at;
        }

        [Serializable]
        private sealed class RuntimeUptime
        {
            public long uptime_secs;
        }

        [Serializable]
        private sealed class RuntimeReadyDocument
        {
            public string schema;
            public string ready;
            public string runtime_owner;
            public string agent_instance_id;
            public string[] blocking_reasons;
        }

        [Serializable]
        private sealed class RuntimeHealthDocument
        {
            public string schema;
            public string status;
            public string runtime_owner;
            public string agent_instance_id;
        }

        [Serializable]
        private sealed class RuntimeMetricsDocument
        {
            public string schema;
            public string runtime_owner;
            public string agent_instance_id;
            public RuntimeMetricGauges gauges;
            public RuntimeMetricStates states;
        }

        [Serializable]
        private sealed class RuntimeMetricGauges
        {
            public long completed_cycle_count;
            public long consecutive_failure_count;
            public long operator_event_count_observed;
            public long restart_count;
        }

        [Serializable]
        private sealed class RuntimeMetricStates
        {
            public string agent_state;
            public string health;
            public string ready;
        }

        [Serializable]
        private sealed class RuntimeEventsDocument
        {
            public string schema;
            public string runtime_owner;
            public string agent_instance_id;
            public RuntimeEventCollection events;
        }

        [Serializable]
        private sealed class RuntimeEventCollection
        {
            public string status;
            public int tail_limit;
            public int unreadable_lines;
            public RuntimeEventEntry[] entries;
        }

        [Serializable]
        private sealed class RuntimeEventEntry
        {
            public string agent_instance_id;
            public string at;
            public string @event;
            public string @operator;
            public string schema;
        }

        [Serializable]
        private sealed class RuntimeV3ObservatoryFeed
        {
            public string schema;
            public string runtime_instance_id;
            public string runtime_selection;
            public RuntimeV3ControlFeed control;
            public RuntimeV3HealthFeed health;
            public RuntimeV3AgentPopulation agents;
            public RuntimeV3ProofFeed proof;
            public RuntimeV3ContinuityFeed continuity;
            public RuntimeV3Event[] events;
        }

        [Serializable]
        private sealed class RuntimeV3ControlFeed
        {
            public string read_endpoint;
            public string websocket_endpoint;
            public string signed_command_endpoint;
            public bool signed_commands_required_for_mutation;
            public bool bearer_token_required_for_read;
            public bool browser_mutation_authority;
        }

        [Serializable]
        private sealed class RuntimeV3HealthFeed
        {
            public RuntimeV3Snapshot snapshot;
            public bool observability_ready;
        }

        [Serializable]
        private sealed class RuntimeV3Snapshot
        {
            public long revision;
            public long topology_generation;
            public long event_count;
            public string lifecycle;
            public string observability;
            public bool observability_ready;
        }

        [Serializable]
        private sealed class RuntimeV3AgentPopulation
        {
            public long total_count;
            public long rendered_sample_count;
            public RuntimeV3Agent[] sample;
        }

        [Serializable]
        private sealed class RuntimeV3Agent
        {
            public string id;
            public string label;
            public string role;
            public string state;
            public string detail;
        }

        [Serializable]
        private sealed class RuntimeV3ProofFeed
        {
            public bool default_runtime_switch_authorized;
            public bool runtime_v2_decommission_authorized;
            public bool sidecar_required;
            public string vector_cloudwatch_route;
        }

        [Serializable]
        private sealed class RuntimeV3ContinuityFeed
        {
            public RuntimeV3ContinuityHead checkpoint;
        }

        [Serializable]
        private sealed class RuntimeV3ContinuityHead
        {
            public long generation;
            public long accepted_through;
            public string integrity;
        }

        [Serializable]
        private sealed class RuntimeV3Event
        {
            public long sequence;
            public long monotonic_millis;
            public string component;
            public string @event;
            public string correlation_id;
        }

        [Serializable]
        private sealed class UnityObservatoryContractDocument
        {
            public string schema;
            public string contract_id;
            public string packet_schema;
            public string source_packet_ref;
            public string runtime_artifact_root;
            public string claim_boundary;
            public string evidence_level;
            public ManifoldSection manifold;
            public WorldSection world;
            public SummarySection summary;
            public StatusSection status;
            public InhabitantReadinessSection inhabitant_readiness;
            public FreedomGateSection freedom_gate;
            public ObservabilitySection observability;
            public ReviewSection review;
            public LabelEntry[] rooms;
            public LabelEntry[] lenses;
            public InhabitantProjection[] inhabitants;
        }

        [Serializable]
        private sealed class ManifoldSection
        {
            public string display_name;
            public string state;
            public string health_summary;
            public int current_tick;
        }

        [Serializable]
        private sealed class SummarySection
        {
            public int citizen_count;
            public int episode_count;
            public string default_room_label;
            public string default_lens_label;
            public string proposal_mode_statement;
        }

        [Serializable]
        private sealed class WorldSection
        {
            public string default_room_label;
            public string default_room_question;
            public string default_room_note;
            public string default_lens_label;
            public string default_lens_summary;
            public string corporate_investor_fallback_label;
            public string corporate_investor_boundary;
        }

        [Serializable]
        private sealed class StatusSection
        {
            public string health_summary;
            public string snapshot_state;
            public string snapshot_note;
            public string kernel_pulse_status;
            public string resource_state;
            public string[] attention_items;
        }

        [Serializable]
        private sealed class InhabitantReadinessSection
        {
            public string identity_boundary;
            public string security_floor_ref;
            public ReadinessCheck[] checklist;
        }

        [Serializable]
        private sealed class FreedomGateSection
        {
            public int allow_count;
            public int defer_count;
            public int refuse_count;
        }

        [Serializable]
        private sealed class ObservabilitySection
        {
            public string consumption_status;
            public string otel_boundary_ref;
            public string event_stream_example_ref;
            public string logging_validation_ref;
            public string security_review_ref;
            public string proof_packet_ref;
            public string claim_boundary;
            public string private_state_posture;
            public string findings_disposition;
        }

        [Serializable]
        private sealed class ReviewSection
        {
            public string demo_classification;
            public string operator_report_ref;
            public string[] caveats;
        }

        [Serializable]
        private sealed class LabelEntry
        {
            public string label;
        }

        [Serializable]
        private sealed class ReadinessCheck
        {
            public string check_id;
            public string label;
            public string state;
            public string note;
        }

        [Serializable]
        private sealed class InhabitantProjection
        {
            public string projection_label;
            public string activity_posture;
            public string capability_summary;
            public string alert_summary;
            public string identity_visibility;
            public string identity_note;
        }

        private string title = "Unity Observatory";
        private string subtitle = "Fixture-backed governed shell";
        private string packetSchema = "adl.csm_visibility_packet.v1";
        private string packetRef =
            "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json";
        private string runtimeArtifactRoot = "adl/tests/fixtures/runtime_v2/observatory";
        private string evidenceLevel = "fixture_backed";
        private int citizenCount = 3;
        private int episodeCount = 2;
        private int currentTick = 0;
        private int allowCount = 0;
        private int deferCount = 0;
        private int refuseCount = 0;
        private string defaultRoomLabel = "World / Reality";
        private string defaultLensLabel = "Operator lens";
        private string proposalModeStatement =
            "Every active-looking control is a governed request proposal only. No direct runtime mutation is performed from this surface.";
        private string claimBoundary =
            "Fixture-backed governed Observatory prototype. This is not a live Runtime v2 capture and it does not grant direct mutation authority.";
        private string operatorReportRef = "runtime_v2/observatory/operator_report.md";
        private string caveat = "This is not a live mutation console.";
        private string worldQuestion = "What exists, where is it, and what is moving?";
        private string worldNote = "Default inhabited polis view.";
        private string lensSummary = "Operational state, disabled reasons, and review links.";
        private string corporateInvestorLabel = "Corporate Investor UI";
        private string corporateInvestorBoundary =
            "Presentation mode only; evidence, authority, and trace boundaries do not change.";
        private string healthSummary =
            "Bounded polis state is inspectable, trace-backed, and still explicitly governed.";
        private string snapshotState = "deferred";
        private string snapshotNote = "Snapshot refresh remains governed follow-on work.";
        private string kernelPulseStatus = "stable";
        private string resourceState = "bounded";
        private string identityBoundary =
            "Identity and profile surfaces stay bounded to fixture aliases and readiness placeholders until WP-08 lands reviewed proof.";
        private string securityFloorRef =
            "docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md";
        private string observabilityStatus = "reviewed_floor_not_live_export";
        private string otelBoundaryRef =
            "docs/milestones/v0.91.6/review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md";
        private string eventStreamExampleRef =
            "docs/milestones/v0.91.6/review/logging_observability/observatory_event_stream_example_3999.jsonl";
        private string loggingValidationRef =
            "docs/milestones/v0.91.6/review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md";
        private string observabilitySecurityReviewRef =
            "docs/milestones/v0.91.6/review/security/UNITY_OBSERVATORY_INHABITANT_READINESS_SECURITY_REVIEW_4023.md";
        private string observabilityProofPacketRef =
            "docs/milestones/v0.91.6/review/observatory/UNITY_OBSERVATORY_LOGGING_OTEL_SECURITY_CONSUMPTION_4034.md";
        private string observabilityClaimBoundary =
            "Observatory consumers may reuse the redacted event-stream vocabulary and operator-report surfaces, but this contract does not claim a live OpenTelemetry collector or exporter integration.";
        private string privateStatePosture =
            "No private paths, secrets, raw logs, or identity-sensitive state are required by this Unity surface.";
        private string findingsDisposition =
            "Accepted WP-07 security findings remain explicit, while identity-safe display and final closeout stay routed to their owning issues.";
        private bool hasObservabilityContract;
        private RuntimeTruthMode runtimeTruthMode = RuntimeTruthMode.Demo;
        private RuntimeTransportKind runtimeTransportKind = RuntimeTransportKind.None;
        private string runtimeApiBase = string.Empty;
        private string runtimeReadToken = string.Empty;
        private string runtimeEndpointRejection = string.Empty;
        private string runtimeDetail = "No runtime endpoint configured; contract fixture is displayed.";
        private string runtimeAgentId = "unavailable";
        private string runtimeAgentState = "unavailable";
        private string runtimeAgentDetail = "unavailable";
        private string runtimeHealthState = "unavailable";
        private string runtimeReadyState = "unavailable";
        private string runtimeProofState = "unavailable";
        private string runtimeContinuityState = "unavailable";
        private string runtimeCloudWatchRoute = "unavailable";
        private string runtimeLatestEvent = "unavailable";
        private long runtimeCycleCount;
        private long runtimeEventCount;
        private long runtimeAgentCount;
        private int runtimeSuccessfulEndpointCount;
        private Coroutine runtimeProbeRoutine;
        private Label runtimeModeLabel;
        private Label runtimeMirrorLabel;
        private Label footerStateLabel;
        private Label navigationTruthLabel;
        private Label readinessMetricValueLabel;
        private Label readinessMetricNoteLabel;
        private Label inhabitantsMetricValueLabel;
        private Label inhabitantsMetricNoteLabel;
        private Label episodesMetricValueLabel;
        private Label episodesMetricNoteLabel;
        private Label topologyModeLabel;
        private Label eventStreamModeLabel;
        private readonly Label[] eventSeverityLabels = new Label[3];
        private readonly Label[] eventSourceLabels = new Label[3];
        private readonly Label[] eventNameLabels = new Label[3];
        private readonly Label[] eventDetailLabels = new Label[3];
        private readonly Dictionary<string, Button> navigationButtons =
            new(StringComparer.Ordinal);
        private RuntimeEventEntry[] runtimeEventEntries = Array.Empty<RuntimeEventEntry>();
        private Label inspectorTitleLabel;
        private Label inspectorModeLabel;
        private Label inspectorStateLabel;
        private Label inspectorEvidenceLabel;
        private Label inspectorRoomLabel;
        private Label inspectorLensLabel;
        private Label inspectorTickLabel;
        private Label communicationStateLabel;
        private string[] roomLabels =
        {
            "World / Reality",
            "Operator / Governance",
            "Cognition / Internal State",
        };
        private string[] lensLabels = { "Public lens", "Operator lens", "Reviewer lens" };
        private string[] attentionItems =
        {
            "Snapshot evidence remains governed and intentionally not treated as live capture.",
        };
        private ReadinessCheck[] readinessChecks =
        {
            new ReadinessCheck
            {
                check_id = "world-space",
                label = "World and lens surfaces are visible from governed packet evidence.",
                state = "ready",
                note = "Rooms, lenses, and proposal-mode boundaries come from the bounded Unity Observatory contract.",
            },
            new ReadinessCheck
            {
                check_id = "identity-boundary",
                label = "Identity and profile display remains bounded to placeholder-safe projections.",
                state = "routed",
                note = "Do not treat fixture aliases as approved profile exposure before WP-08 proof lands.",
            },
        };
        private InhabitantProjection[] inhabitants =
        {
            new InhabitantProjection
            {
                projection_label = "Inhabitant lane 1",
                activity_posture = "bounded work lane",
                capability_summary = "episode execution allowed; 2 allowed lanes, 3 forbidden lanes.",
                alert_summary = "1 routed operator alert remains visible under governed review.",
                identity_visibility = "withheld_pending_wp08",
                identity_note = "Citizen identity, profile, memory, and continuity-sensitive details remain withheld until WP-08 proof lands.",
            },
            new InhabitantProjection
            {
                projection_label = "Inhabitant lane 2",
                activity_posture = "review-only lane",
                capability_summary = "episode execution disabled; 2 allowed lanes, 3 forbidden lanes.",
                alert_summary = "1 routed operator alert remains visible under governed review.",
                identity_visibility = "withheld_pending_wp08",
                identity_note = "Standing and continuity details remain withheld until WP-08 proof lands.",
            },
        };

        public void ConfigureFallback(
            string configuredPacketSchema,
            string configuredPacketRef,
            int configuredCitizenCount,
            int configuredEpisodeCount,
            string configuredRoomLabel,
            string configuredLensLabel
        )
        {
            packetSchema = configuredPacketSchema;
            packetRef = configuredPacketRef;
            citizenCount = configuredCitizenCount;
            episodeCount = configuredEpisodeCount;
            defaultRoomLabel = configuredRoomLabel;
            defaultLensLabel = configuredLensLabel;
        }

        public void ConfigureFromContract(string rawContractJson)
        {
            if (string.IsNullOrWhiteSpace(rawContractJson))
            {
                Debug.LogWarning("Unity Observatory contract resource is empty; using fallback state.");
                return;
            }

            UnityObservatoryContractDocument contract;
            try
            {
                contract = JsonUtility.FromJson<UnityObservatoryContractDocument>(rawContractJson);
            }
            catch (ArgumentException error)
            {
                Debug.LogWarning(
                    $"Unity Observatory contract resource could not be parsed; using fallback state. {error.Message}"
                );
                return;
            }

            if (contract == null)
            {
                Debug.LogWarning("Unity Observatory contract parsed to null; using fallback state.");
                return;
            }

            title = string.IsNullOrWhiteSpace(contract.manifold?.display_name)
                ? "Unity Observatory"
                : contract.manifold.display_name;
            subtitle = string.IsNullOrWhiteSpace(contract.evidence_level)
                ? "Fixture-backed governed shell"
                : contract.evidence_level.Replace("_", " ") + " governed shell";
            packetSchema = DefaultIfBlank(contract.packet_schema, packetSchema);
            packetRef = DefaultIfBlank(contract.source_packet_ref, packetRef);
            runtimeArtifactRoot = DefaultIfBlank(contract.runtime_artifact_root, runtimeArtifactRoot);
            evidenceLevel = DefaultIfBlank(contract.evidence_level, evidenceLevel);
            citizenCount = contract.summary != null ? contract.summary.citizen_count : citizenCount;
            episodeCount = contract.summary != null ? contract.summary.episode_count : episodeCount;
            currentTick = contract.manifold != null ? contract.manifold.current_tick : currentTick;
            allowCount = contract.freedom_gate != null ? contract.freedom_gate.allow_count : allowCount;
            deferCount = contract.freedom_gate != null ? contract.freedom_gate.defer_count : deferCount;
            refuseCount = contract.freedom_gate != null ? contract.freedom_gate.refuse_count : refuseCount;
            defaultRoomLabel = DefaultIfBlank(
                contract.summary?.default_room_label,
                defaultRoomLabel
            );
            defaultLensLabel = DefaultIfBlank(
                contract.summary?.default_lens_label,
                defaultLensLabel
            );
            proposalModeStatement = DefaultIfBlank(
                contract.summary?.proposal_mode_statement,
                proposalModeStatement
            );
            claimBoundary = DefaultIfBlank(contract.claim_boundary, claimBoundary);
            operatorReportRef = DefaultIfBlank(
                contract.review?.operator_report_ref,
                operatorReportRef
            );
            worldQuestion = DefaultIfBlank(
                contract.world?.default_room_question,
                worldQuestion
            );
            worldNote = DefaultIfBlank(contract.world?.default_room_note, worldNote);
            lensSummary = DefaultIfBlank(contract.world?.default_lens_summary, lensSummary);
            corporateInvestorLabel = DefaultIfBlank(
                contract.world?.corporate_investor_fallback_label,
                corporateInvestorLabel
            );
            corporateInvestorBoundary = DefaultIfBlank(
                contract.world?.corporate_investor_boundary,
                corporateInvestorBoundary
            );
            healthSummary = DefaultIfBlank(contract.status?.health_summary, healthSummary);
            snapshotState = DefaultIfBlank(contract.status?.snapshot_state, snapshotState);
            snapshotNote = DefaultIfBlank(contract.status?.snapshot_note, snapshotNote);
            kernelPulseStatus = DefaultIfBlank(
                contract.status?.kernel_pulse_status,
                kernelPulseStatus
            );
            resourceState = DefaultIfBlank(contract.status?.resource_state, resourceState);
            attentionItems = ExtractStringArray(contract.status?.attention_items, attentionItems);
            identityBoundary = DefaultIfBlank(
                contract.inhabitant_readiness?.identity_boundary,
                identityBoundary
            );
            securityFloorRef = DefaultIfBlank(
                contract.inhabitant_readiness?.security_floor_ref,
                securityFloorRef
            );
            hasObservabilityContract = HasObservabilitySection(contract.observability);
            if (hasObservabilityContract)
            {
                observabilityStatus = DefaultIfBlank(
                    contract.observability.consumption_status,
                    observabilityStatus
                );
                otelBoundaryRef = DefaultIfBlank(
                    contract.observability.otel_boundary_ref,
                    otelBoundaryRef
                );
                eventStreamExampleRef = DefaultIfBlank(
                    contract.observability.event_stream_example_ref,
                    eventStreamExampleRef
                );
                loggingValidationRef = DefaultIfBlank(
                    contract.observability.logging_validation_ref,
                    loggingValidationRef
                );
                observabilitySecurityReviewRef = DefaultIfBlank(
                    contract.observability.security_review_ref,
                    observabilitySecurityReviewRef
                );
                observabilityProofPacketRef = DefaultIfBlank(
                    contract.observability.proof_packet_ref,
                    observabilityProofPacketRef
                );
                observabilityClaimBoundary = DefaultIfBlank(
                    contract.observability.claim_boundary,
                    observabilityClaimBoundary
                );
                privateStatePosture = DefaultIfBlank(
                    contract.observability.private_state_posture,
                    privateStatePosture
                );
                findingsDisposition = DefaultIfBlank(
                    contract.observability.findings_disposition,
                    findingsDisposition
                );
            }
            caveat =
                contract.review?.caveats != null && contract.review.caveats.Length > 0
                    ? contract.review.caveats[0]
                    : caveat;
            roomLabels = ExtractLabels(contract.rooms, roomLabels);
            lensLabels = ExtractLabels(contract.lenses, lensLabels);
            readinessChecks = contract.inhabitant_readiness?.checklist != null &&
                    contract.inhabitant_readiness.checklist.Length > 0
                ? contract.inhabitant_readiness.checklist
                : readinessChecks;
            inhabitants = contract.inhabitants != null && contract.inhabitants.Length > 0
                ? contract.inhabitants
                : inhabitants;
        }

        public void Build(VisualElement root)
        {
            if (root == null)
            {
                Debug.LogError("Unity Observatory cannot build without a root visual element.");
                return;
            }

            root.Clear();
            StyleSheet runtimeStyleSheet = Resources.Load<StyleSheet>(
                RuntimeStyleSheetResourcePath
            );
            if (runtimeStyleSheet != null && !root.styleSheets.Contains(runtimeStyleSheet))
            {
                root.styleSheets.Add(runtimeStyleSheet);
            }
            root.AddToClassList("observatory-screen");
            root.style.flexGrow = 1f;
            root.style.paddingLeft = 10f;
            root.style.paddingRight = 10f;
            root.style.paddingTop = 10f;
            root.style.paddingBottom = 8f;
            root.style.backgroundColor = Color.clear;

            VisualElement shell = new();
            shell.AddToClassList("observatory-shell");
            shell.style.flexGrow = 1f;
            shell.style.flexDirection = FlexDirection.Column;
            shell.style.backgroundColor = Color.clear;

            shell.Add(BuildCommandTopBar());
            shell.Add(BuildCommandBody());
            shell.Add(BuildCommandFooter());

            root.Add(shell);

            runtimeApiBase = ResolveRuntimeApiBase();
            if (string.IsNullOrWhiteSpace(runtimeApiBase))
            {
                bool rejected = !string.IsNullOrWhiteSpace(runtimeEndpointRejection);
                ApplyRuntimeTruth(
                    rejected ? RuntimeTruthMode.Disconnected : RuntimeTruthMode.Demo,
                    rejected
                        ? runtimeEndpointRejection
                        : "No runtime endpoint configured; contract fixture is displayed."
                );
                return;
            }

            ApplyRuntimeTruth(RuntimeTruthMode.Connecting, $"Probing {runtimeApiBase}");
            if (runtimeProbeRoutine != null)
            {
                StopCoroutine(runtimeProbeRoutine);
            }
            runtimeProbeRoutine = StartCoroutine(ProbeRuntimeLoop());
        }

        public void ConfigureRuntimeEndpoint(string endpoint)
        {
            runtimeEndpointRejection = string.Empty;
            runtimeReadToken = Environment.GetEnvironmentVariable(
                RuntimeV3TokenEnvironmentVariable
            ) ?? string.Empty;
            if (
                !string.IsNullOrWhiteSpace(endpoint)
                && !TryNormalizeRuntimeEndpoint(
                    endpoint,
                    out runtimeApiBase,
                    out runtimeTransportKind,
                    out runtimeEndpointRejection
                )
            )
            {
                runtimeApiBase = string.Empty;
                runtimeTransportKind = RuntimeTransportKind.None;
            }
            else if (string.IsNullOrWhiteSpace(endpoint))
            {
                runtimeApiBase = string.Empty;
                runtimeTransportKind = RuntimeTransportKind.None;
            }
            else if (
                runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                && string.IsNullOrWhiteSpace(runtimeReadToken)
            )
            {
                runtimeApiBase = string.Empty;
                runtimeTransportKind = RuntimeTransportKind.None;
                runtimeEndpointRejection =
                    "Runtime v3 endpoint rejected: ADL_RUNTIME_OBSERVATORY_TOKEN is not configured.";
            }
            if (runtimeProbeRoutine != null)
            {
                StopCoroutine(runtimeProbeRoutine);
                runtimeProbeRoutine = null;
            }

            if (string.IsNullOrWhiteSpace(runtimeApiBase))
            {
                bool rejected = !string.IsNullOrWhiteSpace(runtimeEndpointRejection);
                ApplyRuntimeTruth(
                    rejected ? RuntimeTruthMode.Disconnected : RuntimeTruthMode.Demo,
                    rejected
                        ? runtimeEndpointRejection
                        : "No runtime endpoint configured; contract fixture is displayed."
                );
                return;
            }

            ApplyRuntimeTruth(RuntimeTruthMode.Connecting, $"Probing {runtimeApiBase}");
            runtimeProbeRoutine = StartCoroutine(ProbeRuntimeLoop());
        }

        private VisualElement BuildCommandTopBar()
        {
            VisualElement topBar = CreateCommandPanel(0.88f);
            topBar.style.height = 54f;
            topBar.style.flexDirection = FlexDirection.Row;
            topBar.style.alignItems = Align.Center;
            topBar.style.paddingLeft = 12f;
            topBar.style.paddingRight = 12f;
            topBar.style.marginBottom = 6f;

            VisualElement mark = new();
            mark.style.width = 32f;
            mark.style.height = 32f;
            mark.style.marginRight = 10f;
            mark.style.backgroundColor = new Color(0.04f, 0.32f, 0.5f, 0.95f);
            SetRoundedBorder(mark, new Color(0.17f, 0.72f, 0.95f, 0.9f), 16f);
            Label markLabel = CreateLabel("ADL", null, 10, FontStyle.Bold);
            markLabel.style.unityTextAlign = TextAnchor.MiddleCenter;
            markLabel.style.flexGrow = 1f;
            markLabel.style.marginBottom = 0f;
            mark.Add(markLabel);
            topBar.Add(mark);

            VisualElement identity = new();
            identity.style.width = 240f;
            identity.Add(
                CreateLabel(
                    "ADL OBSERVATORY",
                    "command-product-title",
                    18,
                    FontStyle.Bold,
                    "command-title"
                )
            );
            Label version = CreateLabel("v0.91.8  /  POLIS CONTROL ROOM", null, 10, FontStyle.Bold);
            version.style.color = new Color(0.47f, 0.74f, 0.9f, 1f);
            version.style.marginBottom = 0f;
            identity.Add(version);
            topBar.Add(identity);

            VisualElement environment = CreateTopBarStatus(
                "ENVIRONMENT",
                "PROTOTYPE CSM 02",
                new Color(0.24f, 0.88f, 0.58f, 1f)
            );
            topBar.Add(environment);
            topBar.Add(
                CreateTopBarStatus(
                    "MODE",
                    "DEMO DATA",
                    new Color(1f, 0.72f, 0.24f, 1f),
                    "runtime-mode"
                )
            );
            topBar.Add(
                CreateTopBarStatus(
                    "RUNTIME MIRROR",
                    "CONTRACT ONLY",
                    new Color(0.33f, 0.72f, 1f, 1f),
                    "runtime-mirror"
                )
            );
            runtimeModeLabel = topBar.Q<Label>("runtime-mode");
            runtimeMirrorLabel = topBar.Q<Label>("runtime-mirror");

            VisualElement spacer = new();
            spacer.style.flexGrow = 1f;
            topBar.Add(spacer);

            Button evidence = CreateCommandButton("▣  PROOF");
            evidence.clicked += () => SelectProjection("Evidence");
            topBar.Add(evidence);
            Button operatorButton = CreateCommandButton("↗  OPERATOR");
            operatorButton.clicked += () => SelectProjection("Operator");
            topBar.Add(operatorButton);
            return topBar;
        }

        private VisualElement BuildCommandBody()
        {
            VisualElement body = new();
            body.style.flexGrow = 1f;
            body.style.flexDirection = FlexDirection.Row;
            body.style.minHeight = 0f;

            body.Add(BuildCommandNavigation());

            VisualElement center = new();
            center.style.flexGrow = 1f;
            center.style.flexDirection = FlexDirection.Column;
            center.style.minWidth = 0f;
            center.style.marginLeft = 6f;
            center.style.marginRight = 6f;
            center.Add(BuildCommandMetrics());
            center.Add(BuildPanopticonSurface());
            center.Add(BuildEventStream());
            body.Add(center);

            body.Add(BuildCommandInspector());
            return body;
        }

        private VisualElement BuildCommandNavigation()
        {
            VisualElement nav = CreateCommandPanel(0.86f);
            nav.style.width = 132f;
            nav.style.paddingTop = 10f;
            nav.style.paddingLeft = 8f;
            nav.style.paddingRight = 8f;

            nav.Add(CreateLabel("CONTROL", null, 10, FontStyle.Bold, "command-kicker"));
            (string destination, string icon)[] destinations =
            {
                ("Runtime", "∿"),
                ("Agents", "◎"),
                ("Events", "↻"),
                ("Governance", "◇"),
                ("Evidence", "▣"),
            };
            navigationButtons.Clear();
            foreach ((string destination, string icon) in destinations)
            {
                Button button = CreateNavigationButton(icon, destination);
                button.clicked += () => SelectProjection(destination);
                navigationButtons[destination] = button;
                nav.Add(button);
            }
            UpdateNavigationState("Runtime");

            VisualElement navSpacer = new();
            navSpacer.style.flexGrow = 1f;
            nav.Add(navSpacer);
            nav.Add(CreateLabel("TRUTH MODE", null, 9, FontStyle.Bold, "command-kicker"));
            Label truth = CreateLabel("DEMO / FIXTURE", null, 11, FontStyle.Bold);
            truth.style.color = new Color(1f, 0.72f, 0.24f, 1f);
            nav.Add(truth);
            navigationTruthLabel = truth;
            nav.Add(CreateLabel($"Tick {currentTick}", null, 10));
            return nav;
        }

        private VisualElement BuildCommandMetrics()
        {
            VisualElement rail = new();
            rail.style.height = 64f;
            rail.style.flexDirection = FlexDirection.Row;
            rail.style.marginBottom = 6f;
            VisualElement readiness = CreateCommandMetric(
                "READINESS",
                ShortRuntimeState(kernelPulseStatus),
                "contract projection",
                new Color(0.3f, 0.9f, 0.58f, 1f),
                "readiness-metric-value",
                "readiness-metric-note"
            );
            rail.Add(readiness);
            readinessMetricValueLabel = readiness.Q<Label>("readiness-metric-value");
            readinessMetricNoteLabel = readiness.Q<Label>("readiness-metric-note");
            VisualElement inhabitantsMetric = CreateCommandMetric(
                "INHABITANTS",
                citizenCount.ToString(),
                "fixture aliases",
                new Color(0.35f, 0.75f, 1f, 1f),
                "inhabitants-metric-value",
                "inhabitants-metric-note"
            );
            rail.Add(inhabitantsMetric);
            inhabitantsMetricValueLabel = inhabitantsMetric.Q<Label>(
                "inhabitants-metric-value"
            );
            inhabitantsMetricNoteLabel = inhabitantsMetric.Q<Label>(
                "inhabitants-metric-note"
            );
            VisualElement eventsMetric = CreateCommandMetric(
                "EVENTS",
                episodeCount.ToString(),
                "retained evidence",
                new Color(1f, 0.72f, 0.24f, 1f),
                "events-metric-value",
                "events-metric-note"
            );
            rail.Add(eventsMetric);
            episodesMetricValueLabel = eventsMetric.Q<Label>("events-metric-value");
            episodesMetricNoteLabel = eventsMetric.Q<Label>("events-metric-note");
            rail.Add(CreateCommandMetric("FREEDOM GATE", $"{allowCount}/{deferCount}/{refuseCount}", "allow / defer / refuse", new Color(0.58f, 0.87f, 0.72f, 1f)));
            return rail;
        }

        private VisualElement BuildPanopticonSurface()
        {
            VisualElement panel = CreateCommandPanel(0.38f);
            panel.style.flexGrow = 1f;
            panel.style.minHeight = 150f;
            panel.style.paddingLeft = 12f;
            panel.style.paddingRight = 12f;
            panel.style.paddingTop = 8f;
            panel.style.paddingBottom = 8f;

            VisualElement header = new();
            header.style.height = 32f;
            header.style.flexDirection = FlexDirection.Row;
            header.Add(CreateLabel("PANOPTICON", null, 13, FontStyle.Bold));
            Label topology = CreateLabel("CSM POLIS TOPOLOGY", null, 9, FontStyle.Bold);
            topology.style.color = new Color(0.4f, 0.72f, 0.86f, 1f);
            topology.style.marginLeft = 10f;
            header.Add(topology);
            VisualElement headerSpacer = new();
            headerSpacer.style.flexGrow = 1f;
            header.Add(headerSpacer);
            topologyModeLabel = CreatePill("FIXTURE PROJECTION", "command-demo-pill");
            header.Add(topologyModeLabel);
            panel.Add(header);

            VisualElement topologyBody = new();
            topologyBody.style.flexGrow = 1f;
            topologyBody.style.flexDirection = FlexDirection.Column;
            topologyBody.style.justifyContent = Justify.Center;

            VisualElement upperAgents = new();
            upperAgents.style.flexDirection = FlexDirection.Row;
            upperAgents.style.justifyContent = Justify.SpaceAround;
            upperAgents.Add(CreateAgentNode("OWNER", "bounded lane", new Color(0.67f, 0.42f, 1f, 1f)));
            upperAgents.Add(CreateAgentNode("SCHEDULER", "proposal only", new Color(0.33f, 0.72f, 1f, 1f)));
            topologyBody.Add(upperAgents);

            VisualElement kernelRow = new();
            kernelRow.style.flexDirection = FlexDirection.Row;
            kernelRow.style.justifyContent = Justify.Center;
            VisualElement kernel = CreateCommandPanel(0.86f);
            kernel.style.width = 180f;
            kernel.style.height = 58f;
            kernel.style.alignItems = Align.Center;
            kernel.style.justifyContent = Justify.Center;
            Label kernelTitle = CreateLabel("CSM RUNTIME KERNEL", null, 12, FontStyle.Bold);
            kernelTitle.style.marginBottom = 1f;
            kernel.Add(kernelTitle);
            Label kernelState = CreateLabel("CONTRACT-BOUND / DEMO", null, 9, FontStyle.Bold);
            kernelState.style.color = new Color(0.3f, 0.9f, 0.58f, 1f);
            kernel.Add(kernelState);
            kernelRow.Add(kernel);
            topologyBody.Add(kernelRow);

            VisualElement lowerAgents = new();
            lowerAgents.style.flexDirection = FlexDirection.Row;
            lowerAgents.style.justifyContent = Justify.SpaceAround;
            lowerAgents.Add(CreateAgentNode("EVIDENCE", evidenceLevel.Replace("_", " "), new Color(0.24f, 0.9f, 0.83f, 1f)));
            lowerAgents.Add(CreateAgentNode("GOVERNANCE", defaultLensLabel, new Color(1f, 0.72f, 0.24f, 1f)));
            topologyBody.Add(lowerAgents);
            panel.Add(topologyBody);
            return panel;
        }

        private VisualElement BuildEventStream()
        {
            VisualElement panel = CreateCommandPanel(0.86f);
            panel.style.height = 116f;
            panel.style.marginTop = 6f;
            panel.style.paddingLeft = 10f;
            panel.style.paddingRight = 10f;
            panel.style.paddingTop = 6f;

            VisualElement titleRow = new();
            titleRow.style.height = 24f;
            titleRow.style.flexDirection = FlexDirection.Row;
            titleRow.Add(CreateLabel("EVENT STREAM", null, 12, FontStyle.Bold));
            eventStreamModeLabel = CreatePill("RETAINED DEMO", "command-demo-pill");
            titleRow.Add(eventStreamModeLabel);
            panel.Add(titleRow);
            ScrollView rows = new(ScrollViewMode.Vertical)
            {
                name = "event-stream-scroll",
                verticalScrollerVisibility = ScrollerVisibility.Auto,
                horizontalScrollerVisibility = ScrollerVisibility.Hidden,
            };
            rows.style.flexGrow = 1f;
            rows.Add(
                CreateEventRow(
                    "INFO",
                    "kernel",
                    "contract.loaded",
                    $"tick {currentTick}",
                    "event-row-0"
                )
            );
            rows.Add(
                CreateEventRow(
                    "INFO",
                    "evidence",
                    "packet.projected",
                    episodeCount.ToString(),
                    "event-row-1"
                )
            );
            rows.Add(
                CreateEventRow(
                    "WARN",
                    "snapshot",
                    snapshotState,
                    "governed follow-on",
                    "event-row-2"
                )
            );
            panel.Add(rows);
            for (int index = 0; index < 3; index++)
            {
                string prefix = $"event-row-{index}";
                eventSeverityLabels[index] = panel.Q<Label>($"{prefix}-severity");
                eventSourceLabels[index] = panel.Q<Label>($"{prefix}-source");
                eventNameLabels[index] = panel.Q<Label>($"{prefix}-name");
                eventDetailLabels[index] = panel.Q<Label>($"{prefix}-detail");
            }
            return panel;
        }

        private VisualElement BuildCommandInspector()
        {
            VisualElement inspector = CreateCommandPanel(0.9f);
            inspector.style.width = 250f;
            inspector.style.paddingLeft = 10f;
            inspector.style.paddingRight = 10f;
            inspector.style.paddingTop = 8f;

            inspector.Add(CreateLabel("INSPECTOR", null, 10, FontStyle.Bold, "command-kicker"));
            inspectorTitleLabel = CreateLabel("Runtime", "inspector-title", 16, FontStyle.Bold);
            inspector.Add(inspectorTitleLabel);
            inspectorModeLabel = CreateLabel("Runtime projection selected", "inspector-mode", 10);
            inspectorModeLabel.style.color = new Color(0.47f, 0.74f, 0.9f, 1f);
            inspector.Add(inspectorModeLabel);

            ScrollView inspectorScroll = new(ScrollViewMode.Vertical)
            {
                name = "inspector-scroll",
                verticalScrollerVisibility = ScrollerVisibility.Auto,
                horizontalScrollerVisibility = ScrollerVisibility.Hidden,
            };
            inspectorScroll.style.flexGrow = 1f;

            VisualElement details = CreateCommandPanel(0.48f);
            details.style.paddingLeft = 8f;
            details.style.paddingRight = 8f;
            details.style.paddingTop = 8f;
            details.style.paddingBottom = 6f;
            details.Add(CreateInspectorRow("State", ShortRuntimeState(kernelPulseStatus), "inspector-state"));
            details.Add(CreateInspectorRow("Evidence", evidenceLevel.Replace("_", " "), "inspector-evidence"));
            details.Add(CreateInspectorRow("Room", defaultRoomLabel, "inspector-room"));
            details.Add(CreateInspectorRow("Lens", defaultLensLabel, "inspector-lens"));
            details.Add(CreateInspectorRow("Tick", currentTick.ToString(), "inspector-tick"));
            inspectorScroll.Add(details);
            inspectorStateLabel = details.Q<Label>("inspector-state");
            inspectorEvidenceLabel = details.Q<Label>("inspector-evidence");
            inspectorRoomLabel = details.Q<Label>("inspector-room");
            inspectorLensLabel = details.Q<Label>("inspector-lens");
            inspectorTickLabel = details.Q<Label>("inspector-tick");

            inspectorScroll.Add(
                CreateLabel(
                    "OPERATOR COMMUNICATION",
                    null,
                    10,
                    FontStyle.Bold,
                    "command-kicker"
                )
            );
            TextField message = new();
            message.multiline = true;
            message.value = "Request current readiness and evidence tail.";
            message.style.height = 58f;
            message.style.fontSize = 10f;
            message.style.color = new Color(0.82f, 0.88f, 0.95f, 1f);
            message.style.backgroundColor = new Color(0.02f, 0.05f, 0.075f, 0.88f);
            VisualElement messageInput = message.Q<VisualElement>(
                className: "unity-text-field__input"
            );
            if (messageInput != null)
            {
                messageInput.style.backgroundColor = new Color(0.02f, 0.05f, 0.075f, 0.96f);
                messageInput.style.color = new Color(0.82f, 0.88f, 0.95f, 1f);
                SetRoundedBorder(messageInput, new Color(0.12f, 0.32f, 0.42f, 1f), 3f);
            }
            inspectorScroll.Add(message);

            communicationStateLabel = CreateLabel(
                "Fixture contract has no send authority.",
                "communication-state",
                9
            );
            communicationStateLabel.style.color = new Color(1f, 0.72f, 0.24f, 1f);
            inspectorScroll.Add(communicationStateLabel);

            Button send = CreateCommandButton("↗  SEND PROPOSAL");
            send.style.height = 34f;
            send.clicked += () =>
            {
                communicationStateLabel.text = DescribeCommunicationAttempt(
                    runtimeTruthMode.ToString()
                );
            };
            inspectorScroll.Add(send);

            VisualElement inspectorSpacer = new();
            inspectorSpacer.style.flexGrow = 1f;
            inspectorScroll.Add(inspectorSpacer);
            Label boundary = CreateLabel(
                "No live runtime, cloud mutation, or direct authority claimed.",
                null,
                9
            );
            boundary.style.color = new Color(0.58f, 0.66f, 0.76f, 1f);
            inspectorScroll.Add(boundary);
            inspector.Add(inspectorScroll);
            return inspector;
        }

        private VisualElement BuildCommandFooter()
        {
            VisualElement footer = CreateCommandPanel(0.84f);
            footer.style.height = 26f;
            footer.style.marginTop = 6f;
            footer.style.flexDirection = FlexDirection.Row;
            footer.style.alignItems = Align.Center;
            footer.style.paddingLeft = 10f;
            footer.style.paddingRight = 10f;
            Label state = CreateLabel("LOCAL UNITY  /  CONTRACT PROJECTION", null, 9, FontStyle.Bold);
            state.style.color = new Color(0.3f, 0.9f, 0.58f, 1f);
            state.style.marginBottom = 0f;
            footer.Add(state);
            footerStateLabel = state;
            VisualElement spacer = new();
            spacer.style.flexGrow = 1f;
            footer.Add(spacer);
            Label source = CreateLabel("DATA SOURCE: REPOSITORY OBSERVATORY CONTRACT", null, 9);
            source.style.marginBottom = 0f;
            footer.Add(source);
            return footer;
        }

        private static VisualElement CreateCommandPanel(float opacity)
        {
            VisualElement panel = new();
            panel.style.backgroundColor = new Color(0.018f, 0.045f, 0.068f, opacity);
            SetRoundedBorder(panel, new Color(0.13f, 0.28f, 0.38f, 0.95f), 5f);
            return panel;
        }

        private static VisualElement CreateTopBarStatus(
            string label,
            string value,
            Color accent,
            string valueElementName = null
        )
        {
            VisualElement status = new();
            status.style.width = 142f;
            status.style.marginLeft = 8f;
            status.style.paddingLeft = 8f;
            status.Add(CreateLabel(label, null, 8, FontStyle.Bold, "command-kicker"));
            Label valueLabel = CreateLabel(value, valueElementName, 10, FontStyle.Bold);
            valueLabel.style.color = accent;
            valueLabel.style.marginBottom = 0f;
            status.Add(valueLabel);
            return status;
        }

        private static VisualElement CreateCommandMetric(
            string label,
            string value,
            string note,
            Color accent,
            string valueElementName = null,
            string noteElementName = null
        )
        {
            VisualElement tile = CreateCommandPanel(0.9f);
            tile.style.flexGrow = 1f;
            tile.style.marginRight = 5f;
            tile.style.paddingLeft = 9f;
            tile.style.paddingTop = 7f;
            tile.Add(CreateLabel(label, null, 8, FontStyle.Bold, "command-kicker"));
            Label valueLabel = CreateLabel(value, valueElementName, 16, FontStyle.Bold);
            valueLabel.style.color = accent;
            valueLabel.style.marginBottom = 0f;
            tile.Add(valueLabel);
            Label noteLabel = CreateLabel(note, noteElementName, 8);
            noteLabel.style.color = new Color(0.58f, 0.68f, 0.78f, 1f);
            noteLabel.style.marginBottom = 0f;
            tile.Add(noteLabel);
            return tile;
        }

        private VisualElement CreateAgentNode(string role, string state, Color accent)
        {
            Button node = new(() => SelectProjection(role));
            node.text = string.Empty;
            node.style.backgroundColor = new Color(0.018f, 0.045f, 0.068f, 0.82f);
            SetRoundedBorder(node, new Color(0.13f, 0.28f, 0.38f, 0.95f), 5f);
            node.style.width = 150f;
            node.style.height = 43f;
            node.style.paddingLeft = 8f;
            node.style.paddingTop = 5f;
            Label roleLabel = CreateLabel(role, null, 10, FontStyle.Bold);
            roleLabel.style.color = accent;
            roleLabel.style.marginBottom = 0f;
            node.Add(roleLabel);
            Label stateLabel = CreateLabel(state, null, 8);
            stateLabel.style.marginBottom = 0f;
            node.Add(stateLabel);
            return node;
        }

        private static VisualElement CreateEventRow(
            string severity,
            string source,
            string eventName,
            string detail,
            string elementPrefix = null
        )
        {
            VisualElement row = new();
            row.style.height = 23f;
            row.style.flexDirection = FlexDirection.Row;
            row.style.alignItems = Align.Center;
            Label severityLabel = CreateLabel(severity, null, 8, FontStyle.Bold);
            severityLabel.name = string.IsNullOrWhiteSpace(elementPrefix)
                ? string.Empty
                : $"{elementPrefix}-severity";
            severityLabel.style.width = 42f;
            severityLabel.style.color = severity == "WARN"
                ? new Color(1f, 0.72f, 0.24f, 1f)
                : new Color(0.3f, 0.9f, 0.58f, 1f);
            severityLabel.style.marginBottom = 0f;
            row.Add(severityLabel);
            Label sourceLabel = CreateLabel(source, null, 9);
            sourceLabel.name = string.IsNullOrWhiteSpace(elementPrefix)
                ? string.Empty
                : $"{elementPrefix}-source";
            sourceLabel.style.width = 70f;
            sourceLabel.style.marginBottom = 0f;
            row.Add(sourceLabel);
            Label eventLabel = CreateLabel(eventName, null, 9, FontStyle.Bold);
            eventLabel.name = string.IsNullOrWhiteSpace(elementPrefix)
                ? string.Empty
                : $"{elementPrefix}-name";
            eventLabel.style.flexGrow = 1f;
            eventLabel.style.marginBottom = 0f;
            row.Add(eventLabel);
            Label detailLabel = CreateLabel(detail, null, 8);
            detailLabel.name = string.IsNullOrWhiteSpace(elementPrefix)
                ? string.Empty
                : $"{elementPrefix}-detail";
            detailLabel.style.width = 105f;
            detailLabel.style.unityTextAlign = TextAnchor.MiddleRight;
            detailLabel.style.marginBottom = 0f;
            row.Add(detailLabel);
            return row;
        }

        private static VisualElement CreateInspectorRow(
            string label,
            string value,
            string valueElementName
        )
        {
            VisualElement row = new();
            row.style.height = 21f;
            row.style.flexDirection = FlexDirection.Row;
            Label key = CreateLabel(label, null, 9, FontStyle.Bold);
            key.style.width = 70f;
            key.style.color = new Color(0.53f, 0.66f, 0.78f, 1f);
            key.style.marginBottom = 0f;
            row.Add(key);
            Label observed = CreateLabel(value, valueElementName, 9);
            observed.style.flexGrow = 1f;
            observed.style.unityTextAlign = TextAnchor.MiddleRight;
            observed.style.marginBottom = 0f;
            row.Add(observed);
            return row;
        }

        private void SelectProjection(string projection)
        {
            if (inspectorTitleLabel == null || inspectorModeLabel == null)
            {
                return;
            }

            UpdateNavigationState(projection);
            inspectorTitleLabel.text = projection;
            inspectorModeLabel.text = $"{projection} projection selected";

            switch (projection)
            {
                case "Runtime":
                    SetInspectorValues(
                        runtimeTruthMode.ToString().ToUpperInvariant(),
                        runtimeDetail,
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeHealthState
                            : defaultRoomLabel,
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeReadyState
                            : defaultLensLabel,
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeCycleCount.ToString()
                            : currentTick.ToString()
                    );
                    break;
                case "Agents":
                case "OWNER":
                case "SCHEDULER":
                    SetInspectorValues(
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? $"{runtimeAgentId} / {runtimeAgentState}"
                            : $"{citizenCount} FIXTURE ALIASES",
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                                ? runtimeAgentDetail
                                : "Observed from the CSM /status and /metrics contracts."
                            : identityBoundary,
                        "Polis inhabitants",
                        "Identity-safe projection",
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeCycleCount.ToString()
                            : currentTick.ToString()
                    );
                    break;
                case "Events":
                    SetInspectorValues(
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? $"{runtimeEventCount} OBSERVED EVENTS"
                            : $"{episodeCount} RETAINED EPISODES",
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? $"Latest observed event: {runtimeLatestEvent}"
                            : eventStreamExampleRef,
                        "Event stream",
                        "Evidence lens",
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeCycleCount.ToString()
                            : currentTick.ToString()
                    );
                    break;
                case "Governance":
                    SetInspectorValues(
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                                ? "SIGNED CONTROL REQUIRED"
                                : "UNAVAILABLE VIA LEGACY CSM API"
                            : $"{allowCount}/{deferCount}/{refuseCount}",
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                                ? "Runtime v3 exposes signed control; this read surface has no mutation authority."
                                : "No governance decision endpoint is present in the observed legacy CSM contract."
                            : proposalModeStatement,
                        "Freedom Gate",
                        defaultLensLabel,
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeCycleCount.ToString()
                            : currentTick.ToString()
                    );
                    break;
                case "Evidence":
                case "EVIDENCE":
                    SetInspectorValues(
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                                ? $"{runtimeSuccessfulEndpointCount}/1 RUNTIME V3 FEED"
                                : $"{runtimeSuccessfulEndpointCount}/5 CSM ENDPOINTS"
                            : evidenceLevel.Replace("_", " ").ToUpperInvariant(),
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                                ? $"{runtimeProofState}; health {runtimeHealthState}; event {runtimeLatestEvent}."
                                : $"Health {runtimeHealthState}; event tail {runtimeLatestEvent}."
                            : operatorReportRef,
                        runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                            ? runtimeContinuityState
                            : "Retained packet",
                        runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                            ? runtimeCloudWatchRoute
                            : "Reviewer lens",
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeCycleCount.ToString()
                            : currentTick.ToString()
                    );
                    break;
                case "Operator":
                    SetInspectorValues(
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? "SIGNED PROPOSAL MAPPING UNAVAILABLE"
                            : "NO SEND AUTHORITY",
                        DescribeCommunicationAttempt(runtimeTruthMode.ToString()),
                        "Operator channel",
                        "Proposal only",
                        runtimeTruthMode == RuntimeTruthMode.Live
                            || runtimeTruthMode == RuntimeTruthMode.Degraded
                            ? runtimeCycleCount.ToString()
                            : currentTick.ToString()
                    );
                    break;
                default:
                    SetInspectorValues(
                        resourceState.ToUpperInvariant(),
                        claimBoundary,
                        defaultRoomLabel,
                        defaultLensLabel,
                        currentTick.ToString()
                    );
                    break;
            }
        }

        private void SetInspectorValues(
            string state,
            string evidence,
            string room,
            string lens,
            string tick
        )
        {
            if (inspectorStateLabel != null)
            {
                inspectorStateLabel.text = state;
            }
            if (inspectorEvidenceLabel != null)
            {
                inspectorEvidenceLabel.text = TruncateForInspector(evidence);
            }
            if (inspectorRoomLabel != null)
            {
                inspectorRoomLabel.text = TruncateForInspector(room);
            }
            if (inspectorLensLabel != null)
            {
                inspectorLensLabel.text = TruncateForInspector(lens);
            }
            if (inspectorTickLabel != null)
            {
                inspectorTickLabel.text = tick;
            }
        }

        private IEnumerator ProbeRuntimeLoop()
        {
            while (enabled && !string.IsNullOrWhiteSpace(runtimeApiBase))
            {
                yield return runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                    ? ProbeRuntimeV3Once()
                    : ProbeLegacyCsmOnce();
                yield return new WaitForSecondsRealtime(RuntimeProbeIntervalSeconds);
            }
        }

        private IEnumerator ProbeRuntimeV3Once()
        {
            UnityWebRequest request = UnityWebRequest.Get(
                $"{runtimeApiBase.TrimEnd('/')}{RuntimeV3ObservatoryPath}"
            );
            request.timeout = 3;
            ConfigureRuntimeV3Request(request, runtimeReadToken);
            yield return request.SendWebRequest();

            bool successful = IsSuccessfulRuntimeResponse(request);
            runtimeSuccessfulEndpointCount = successful ? 1 : 0;
            string body = successful ? request.downloadHandler?.text ?? string.Empty : string.Empty;
            string classification = ClassifyRuntimeV3Feed(
                request.responseCode,
                body,
                IsRuntimeTransportFailure(request.result),
                !string.IsNullOrWhiteSpace(runtimeReadToken)
            );
            RuntimeTruthMode mode = Enum.TryParse(classification, out RuntimeTruthMode parsed)
                ? parsed
                : RuntimeTruthMode.Disconnected;
            bool parsedFeed = successful && IngestRuntimeV3Feed(body);
            if (!parsedFeed && mode != RuntimeTruthMode.Disconnected)
            {
                mode = RuntimeTruthMode.Degraded;
            }

            string detail = mode == RuntimeTruthMode.Live
                ? $"Runtime v3 Observatory feed authenticated and ready at {runtimeApiBase}"
                : mode == RuntimeTruthMode.Degraded
                    ? $"Runtime v3 Observatory feed responded but was unauthorized, invalid, or not ready at {runtimeApiBase}"
                    : $"Runtime v3 Observatory feed did not respond at {runtimeApiBase}";
            ApplyRuntimeTruth(mode, detail);
            request.Dispose();
        }

        private IEnumerator ProbeLegacyCsmOnce()
        {
            string[] endpointPaths = { "/status", "/health", "/ready", "/metrics", "/events" };
            UnityWebRequest[] requests = new UnityWebRequest[endpointPaths.Length];
            UnityWebRequestAsyncOperation[] operations =
                new UnityWebRequestAsyncOperation[endpointPaths.Length];
            for (int index = 0; index < endpointPaths.Length; index++)
            {
                requests[index] = UnityWebRequest.Get(
                    $"{runtimeApiBase.TrimEnd('/')}{endpointPaths[index]}"
                );
                requests[index].timeout = 3;
                operations[index] = requests[index].SendWebRequest();
            }

            for (int index = 0; index < operations.Length; index++)
            {
                yield return operations[index];
            }

            int successCount = 0;
            for (int index = 0; index < requests.Length; index++)
            {
                if (IsSuccessfulRuntimeResponse(requests[index]))
                {
                    successCount++;
                }
            }

            runtimeSuccessfulEndpointCount = successCount;
            string statusBody = ResponseBody(requests[0]);
            string healthBody = ResponseBody(requests[1]);
            string readyBody = ResponseBody(requests[2]);
            string metricsBody = ResponseBody(requests[3]);
            string eventsBody = ResponseBody(requests[4]);
            string classification = ClassifyRuntimeSnapshot(
                successCount,
                statusBody,
                healthBody,
                readyBody,
                metricsBody,
                eventsBody
            );
            RuntimeTruthMode mode = Enum.TryParse(classification, out RuntimeTruthMode parsed)
                ? parsed
                : RuntimeTruthMode.Disconnected;
            bool documentsParsed = IngestRuntimeDocuments(
                statusBody,
                healthBody,
                readyBody,
                metricsBody,
                eventsBody
            );
            if (!documentsParsed && mode != RuntimeTruthMode.Disconnected)
            {
                mode = RuntimeTruthMode.Degraded;
            }
            string detail = mode == RuntimeTruthMode.Live
                ? $"CSM healthy and ready; 5/5 validated runtime contracts at {runtimeApiBase}"
                : mode == RuntimeTruthMode.Degraded
                    ? $"CSM contract incomplete, invalid, or unhealthy; {successCount}/5 endpoints responded at {runtimeApiBase}"
                    : $"No CSM contract endpoint responded at {runtimeApiBase}";
            ApplyRuntimeTruth(mode, detail);

            for (int index = 0; index < requests.Length; index++)
            {
                requests[index].Dispose();
            }
        }

        private static bool IsSuccessfulRuntimeResponse(UnityWebRequest request)
        {
            return request != null
                && request.result == UnityWebRequest.Result.Success
                && request.responseCode >= 200
                && request.responseCode < 300;
        }

        private static string ResponseBody(UnityWebRequest request)
        {
            return IsSuccessfulRuntimeResponse(request)
                ? request.downloadHandler?.text ?? string.Empty
                : string.Empty;
        }

        public static string ClassifyRuntimeSnapshot(
            int successfulEndpointCount,
            string statusBody,
            string healthBody,
            string readyBody,
            string metricsBody,
            string eventsBody
        )
        {
            if (successfulEndpointCount <= 0)
            {
                return RuntimeTruthMode.Disconnected.ToString();
            }

            if (successfulEndpointCount != 5)
            {
                return RuntimeTruthMode.Degraded.ToString();
            }

            try
            {
                RuntimeStatusDocument status = ParseDocument<RuntimeStatusDocument>(
                    statusBody
                );
                RuntimeHealthDocument health = ParseDocument<RuntimeHealthDocument>(
                    healthBody
                );
                RuntimeReadyDocument ready = ParseDocument<RuntimeReadyDocument>(
                    readyBody
                );
                RuntimeMetricsDocument metrics = ParseDocument<RuntimeMetricsDocument>(
                    metricsBody
                );
                RuntimeEventsDocument events = ParseDocument<RuntimeEventsDocument>(
                    eventsBody
                );

                bool schemasValid =
                    HasRuntimeSchema(status?.schema, "status")
                    && HasRuntimeSchema(health?.schema, "health")
                    && HasRuntimeSchema(ready?.schema, "ready")
                    && HasRuntimeSchema(metrics?.schema, "metrics")
                    && HasRuntimeSchema(events?.schema, "events");
                bool statesHealthy =
                    IsObservedState(status?.status, "healthy")
                    && IsObservedState(status?.ready, "ready")
                    && IsObservedState(health?.status, "healthy")
                    && IsObservedState(ready?.ready, "ready")
                    && IsObservedState(metrics?.states?.health, "healthy")
                    && IsObservedState(metrics?.states?.ready, "ready")
                    && IsObservedState(events?.events?.status, "serialized");

                return schemasValid && statesHealthy
                    ? RuntimeTruthMode.Live.ToString()
                    : RuntimeTruthMode.Degraded.ToString();
            }
            catch
            {
                return RuntimeTruthMode.Degraded.ToString();
            }
        }

        public static string ClassifyRuntimeProbe(
            long responseCode,
            string responseBody,
            bool transportError
        )
        {
            if (transportError || responseCode < 200 || responseCode >= 300)
            {
                return RuntimeTruthMode.Disconnected.ToString();
            }

            return RuntimeTruthMode.Degraded.ToString();
        }

        public static string ClassifyRuntimeV3Feed(
            long responseCode,
            string responseBody,
            bool transportError,
            bool tokenConfigured
        )
        {
            if (transportError || responseCode <= 0)
            {
                return RuntimeTruthMode.Disconnected.ToString();
            }
            if (
                !tokenConfigured
                || responseCode < 200
                || responseCode >= 300
                || string.IsNullOrWhiteSpace(responseBody)
            )
            {
                return RuntimeTruthMode.Degraded.ToString();
            }

            try
            {
                RuntimeV3ObservatoryFeed feed = ParseDocument<RuntimeV3ObservatoryFeed>(
                    responseBody
                );
                bool validContract =
                    feed != null
                    && string.Equals(
                        feed.schema,
                        RuntimeV3ObservatorySchema,
                        StringComparison.Ordinal
                    )
                    && string.Equals(
                        feed.runtime_selection,
                        "runtime_v3_explicit_opt_in",
                        StringComparison.Ordinal
                    )
                    && feed.control != null
                    && string.Equals(
                        feed.control.read_endpoint,
                        RuntimeV3ObservatoryPath,
                        StringComparison.Ordinal
                    )
                    && string.Equals(
                        feed.control.signed_command_endpoint,
                        RuntimeV3ControlPath,
                        StringComparison.Ordinal
                    )
                    && feed.control.bearer_token_required_for_read
                    && feed.control.signed_commands_required_for_mutation
                    && !feed.control.browser_mutation_authority;
                bool ready =
                    feed?.health != null
                    && feed.health.observability_ready
                    && feed.health.snapshot != null
                    && feed.health.snapshot.observability_ready;
                return validContract && ready
                    ? RuntimeTruthMode.Live.ToString()
                    : RuntimeTruthMode.Degraded.ToString();
            }
            catch
            {
                return RuntimeTruthMode.Degraded.ToString();
            }
        }

        private static bool HasRuntimeSchema(string schema, string endpoint)
        {
            return string.Equals(
                schema,
                $"adl.csm.runtime_api.{endpoint}.v1",
                StringComparison.Ordinal
            );
        }

        private static bool IsObservedState(string observed, string expected)
        {
            return string.Equals(observed, expected, StringComparison.OrdinalIgnoreCase);
        }

        private bool IngestRuntimeDocuments(
            string statusBody,
            string healthBody,
            string readyBody,
            string metricsBody,
            string eventsBody
        )
        {
            ResetRuntimeDocuments();
            try
            {
                RuntimeStatusDocument status = ParseDocument<RuntimeStatusDocument>(
                    statusBody
                );
                RuntimeHealthDocument health = ParseDocument<RuntimeHealthDocument>(
                    healthBody
                );
                RuntimeReadyDocument ready = ParseDocument<RuntimeReadyDocument>(
                    readyBody
                );
                RuntimeMetricsDocument metrics = ParseDocument<RuntimeMetricsDocument>(
                    metricsBody
                );
                RuntimeEventsDocument events = ParseDocument<RuntimeEventsDocument>(
                    eventsBody
                );

                runtimeAgentId = FirstObserved(
                    status?.agent_instance_id,
                    metrics?.agent_instance_id,
                    ready?.agent_instance_id,
                    health?.agent_instance_id
                );
                runtimeAgentState = FirstObserved(
                    status?.agent_status?.state,
                    metrics?.states?.agent_state
                );
                runtimeAgentCount = runtimeAgentId == "unavailable" ? 0 : 1;
                runtimeHealthState = FirstObserved(health?.status, status?.status);
                runtimeReadyState = FirstObserved(ready?.ready, status?.ready);
                runtimeCycleCount = Math.Max(
                    status?.agent_status?.completed_cycle_count ?? 0,
                    metrics?.gauges?.completed_cycle_count ?? 0
                );
                runtimeEventCount = Math.Max(
                    metrics?.gauges?.operator_event_count_observed ?? 0,
                    events?.events?.entries?.Length ?? 0
                );
                RuntimeEventEntry[] entries = events?.events?.entries;
                runtimeEventEntries = entries ?? Array.Empty<RuntimeEventEntry>();
                runtimeLatestEvent =
                    entries != null && entries.Length > 0
                        ? FirstObserved(entries[entries.Length - 1].@event, "observed")
                        : "unavailable";
                return true;
            }
            catch
            {
                ResetRuntimeDocuments();
                return false;
            }
        }

        private bool IngestRuntimeV3Feed(string body)
        {
            ResetRuntimeDocuments();
            try
            {
                RuntimeV3ObservatoryFeed feed = ParseDocument<RuntimeV3ObservatoryFeed>(
                    body
                );
                if (
                    feed == null
                    || !string.Equals(
                        feed.schema,
                        RuntimeV3ObservatorySchema,
                        StringComparison.Ordinal
                    )
                )
                {
                    return false;
                }

                RuntimeV3Snapshot snapshot = feed.health?.snapshot;
                RuntimeV3Agent firstAgent =
                    feed.agents?.sample != null && feed.agents.sample.Length > 0
                        ? feed.agents.sample[0]
                        : null;
                runtimeAgentId = FirstObserved(firstAgent?.id, feed.runtime_instance_id);
                runtimeAgentState = FirstObserved(firstAgent?.state, snapshot?.lifecycle);
                runtimeAgentDetail = firstAgent == null
                    ? $"Runtime instance {FirstObserved(feed.runtime_instance_id)}; no agent sample was rendered."
                    : $"{FirstObserved(firstAgent.label, firstAgent.id)} / {FirstObserved(firstAgent.role, "role unavailable")}: {FirstObserved(firstAgent.detail, firstAgent.state)}";
                runtimeAgentCount = Math.Max(feed.agents?.total_count ?? 0, 0);
                runtimeHealthState = feed.health?.observability_ready == true
                    ? "healthy"
                    : "pending";
                runtimeReadyState =
                    feed.health?.observability_ready == true
                    && snapshot?.observability_ready == true
                        ? "ready"
                        : "pending";
                runtimeCycleCount = Math.Max(snapshot?.revision ?? 0, 0);
                runtimeProofState = feed.proof == null
                    ? "proof unavailable"
                    : $"switch {(feed.proof.default_runtime_switch_authorized ? "authorized" : "not authorized")}, sidecar {(feed.proof.sidecar_required ? "required" : "not required")}";
                runtimeCloudWatchRoute = FirstObserved(
                    feed.proof?.vector_cloudwatch_route
                );
                runtimeContinuityState = feed.continuity?.checkpoint == null
                    ? "continuity unavailable"
                    : $"checkpoint generation {feed.continuity.checkpoint.generation}, accepted through {feed.continuity.checkpoint.accepted_through}, integrity {FirstObserved(feed.continuity.checkpoint.integrity)}";
                RuntimeV3Event[] events = feed.events ?? Array.Empty<RuntimeV3Event>();
                runtimeEventCount = Math.Max(snapshot?.event_count ?? 0, events.Length);
                runtimeLatestEvent =
                    events.Length > 0
                        ? FirstObserved(events[events.Length - 1].@event, "observed")
                        : "unavailable";
                runtimeEventEntries = new RuntimeEventEntry[Math.Min(events.Length, 3)];
                int sourceOffset = Math.Max(0, events.Length - runtimeEventEntries.Length);
                for (int index = 0; index < runtimeEventEntries.Length; index++)
                {
                    RuntimeV3Event source = events[sourceOffset + index];
                    runtimeEventEntries[index] = new RuntimeEventEntry
                    {
                        agent_instance_id = FirstObserved(
                            source.component,
                            feed.runtime_instance_id
                        ),
                        at = $"t+{source.monotonic_millis}ms",
                        @event = FirstObserved(source.@event, "runtime_event"),
                        @operator = FirstObserved(
                            source.correlation_id,
                            $"sequence {source.sequence}"
                        ),
                        schema = RuntimeV3ObservatorySchema,
                    };
                }
                return true;
            }
            catch
            {
                ResetRuntimeDocuments();
                return false;
            }
        }

        private void ResetRuntimeDocuments()
        {
            runtimeAgentId = "unavailable";
            runtimeAgentState = "unavailable";
            runtimeAgentDetail = "unavailable";
            runtimeHealthState = "unavailable";
            runtimeReadyState = "unavailable";
            runtimeProofState = "unavailable";
            runtimeContinuityState = "unavailable";
            runtimeCloudWatchRoute = "unavailable";
            runtimeLatestEvent = "unavailable";
            runtimeCycleCount = 0;
            runtimeEventCount = 0;
            runtimeAgentCount = 0;
            runtimeEventEntries = Array.Empty<RuntimeEventEntry>();
        }

        private static T ParseDocument<T>(string body)
            where T : class
        {
            return string.IsNullOrWhiteSpace(body) ? null : JsonUtility.FromJson<T>(body);
        }

        public static void ConfigureRuntimeV3Request(
            UnityWebRequest request,
            string bearerToken
        )
        {
            if (request == null)
            {
                throw new ArgumentNullException(nameof(request));
            }
            if (string.IsNullOrWhiteSpace(bearerToken))
            {
                throw new ArgumentException(
                    "Runtime v3 read token is required.",
                    nameof(bearerToken)
                );
            }

            request.SetRequestHeader("Authorization", $"Bearer {bearerToken}");
        }

        public static bool IsRuntimeTransportFailureForProof(
            UnityWebRequest.Result result
        )
        {
            return IsRuntimeTransportFailure(result);
        }

        private static bool IsRuntimeTransportFailure(UnityWebRequest.Result result)
        {
            return result == UnityWebRequest.Result.ConnectionError
                || result == UnityWebRequest.Result.DataProcessingError;
        }

        private static string FirstObserved(params string[] candidates)
        {
            foreach (string candidate in candidates)
            {
                if (!string.IsNullOrWhiteSpace(candidate))
                {
                    return candidate;
                }
            }
            return "unavailable";
        }

        public static string DescribeCommunicationAttempt(string runtimeMode)
        {
            return string.Equals(runtimeMode, RuntimeTruthMode.Live.ToString(), StringComparison.Ordinal)
                || string.Equals(
                    runtimeMode,
                    RuntimeTruthMode.Degraded.ToString(),
                    StringComparison.Ordinal
                )
                ? "NOT SENT: runtime control exists, but Unity has no governed signed operator-proposal mapping."
                : "NOT SENT: fixture mode exposes no operator transport.";
        }

        private void ApplyRuntimeTruth(RuntimeTruthMode mode, string detail)
        {
            runtimeTruthMode = mode;
            runtimeDetail = detail;

            string modeText;
            string mirrorText;
            string footerText;
            Color accent;
            switch (mode)
            {
                case RuntimeTruthMode.Live:
                    modeText = "LIVE";
                    mirrorText = runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                        ? "RUNTIME V3"
                        : "CSM RUNTIME";
                    footerText = runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                        ? "AUTHENTICATED HTTPS  /  RUNTIME V3 READY"
                        : "LIVE LOOPBACK  /  CSM READY";
                    accent = new Color(0.3f, 0.9f, 0.58f, 1f);
                    break;
                case RuntimeTruthMode.Degraded:
                    modeText = "DEGRADED";
                    mirrorText = "HEALTH DEGRADED";
                    footerText = runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                        ? "AUTHENTICATED HTTPS  /  DEGRADED"
                        : "LOOPBACK  /  DEGRADED";
                    accent = new Color(1f, 0.72f, 0.24f, 1f);
                    break;
                case RuntimeTruthMode.Connecting:
                    modeText = "CONNECTING";
                    mirrorText = "PROBING RUNTIME";
                    footerText = runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                        ? "HTTPS  /  AUTHENTICATING"
                        : "LOOPBACK  /  CONNECTING";
                    accent = new Color(0.33f, 0.72f, 1f, 1f);
                    break;
                case RuntimeTruthMode.Disconnected:
                    modeText = "DISCONNECTED";
                    mirrorText = "NO RUNTIME";
                    footerText = runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                        ? "HTTPS  /  DISCONNECTED"
                        : "LOOPBACK  /  DISCONNECTED";
                    accent = new Color(1f, 0.35f, 0.34f, 1f);
                    break;
                default:
                    modeText = "DEMO DATA";
                    mirrorText = "CONTRACT ONLY";
                    footerText = "LOCAL UNITY  /  CONTRACT PROJECTION";
                    accent = new Color(1f, 0.72f, 0.24f, 1f);
                    break;
            }

            SetRuntimeLabel(runtimeModeLabel, modeText, accent);
            SetRuntimeLabel(runtimeMirrorLabel, mirrorText, accent);
            SetRuntimeLabel(footerStateLabel, footerText, accent);
            SetRuntimeLabel(
                navigationTruthLabel,
                mode == RuntimeTruthMode.Demo ? "DEMO / FIXTURE" : modeText,
                accent
            );
            SetRuntimeLabel(
                readinessMetricValueLabel,
                mode == RuntimeTruthMode.Demo ? ShortRuntimeState(kernelPulseStatus) : modeText,
                accent
            );
            if (readinessMetricNoteLabel != null)
            {
                readinessMetricNoteLabel.text =
                    mode == RuntimeTruthMode.Demo
                        ? "contract projection"
                        : runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                            ? $"{runtimeSuccessfulEndpointCount}/1 Runtime v3 feed"
                            : $"{runtimeSuccessfulEndpointCount}/5 CSM endpoints";
            }
            if (inhabitantsMetricValueLabel != null)
            {
                inhabitantsMetricValueLabel.text =
                    mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded
                        ? runtimeAgentCount.ToString()
                        : citizenCount.ToString();
            }
            if (inhabitantsMetricNoteLabel != null)
            {
                inhabitantsMetricNoteLabel.text =
                    mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded
                        ? TruncateForInspector(runtimeAgentState)
                        : "fixture aliases";
            }
            if (episodesMetricValueLabel != null)
            {
                episodesMetricValueLabel.text =
                    mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded
                        ? runtimeEventCount.ToString()
                        : episodeCount.ToString();
            }
            if (episodesMetricNoteLabel != null)
            {
                episodesMetricNoteLabel.text =
                    mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded
                        ? TruncateForInspector(runtimeLatestEvent)
                        : "retained evidence";
            }
            if (topologyModeLabel != null)
            {
                topologyModeLabel.text = mode switch
                {
                    RuntimeTruthMode.Live
                        => runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                            ? "RUNTIME V3 OBSERVED"
                            : "CSM OBSERVED",
                    RuntimeTruthMode.Degraded
                        => runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                            ? "RUNTIME V3 DEGRADED"
                            : "CSM PARTIAL",
                    RuntimeTruthMode.Connecting => "PROBING RUNTIME",
                    RuntimeTruthMode.Disconnected => "NO RUNTIME",
                    _ => "FIXTURE PROJECTION",
                };
                topologyModeLabel.style.color = accent;
            }
            UpdateEventRows(mode, accent);
            if (communicationStateLabel != null)
            {
                communicationStateLabel.text = mode switch
                {
                    RuntimeTruthMode.Live =>
                        "Runtime control exists; no signed Unity proposal mapping is configured.",
                    RuntimeTruthMode.Degraded =>
                        "Runtime control exists; no signed Unity proposal mapping is configured.",
                    RuntimeTruthMode.Connecting => "Waiting for runtime capability truth.",
                    RuntimeTruthMode.Disconnected => "No runtime transport is available.",
                    _ => "Fixture contract has no send authority.",
                };
            }
            if (inspectorTitleLabel != null && inspectorTitleLabel.text == "Runtime")
            {
                SetInspectorValues(
                    mode.ToString().ToUpperInvariant(),
                    detail,
                    mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded
                        ? runtimeHealthState
                        : defaultRoomLabel,
                    mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded
                        ? runtimeReadyState
                        : defaultLensLabel,
                    mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded
                        ? runtimeCycleCount.ToString()
                        : currentTick.ToString()
                );
            }
        }

        private void UpdateEventRows(RuntimeTruthMode mode, Color accent)
        {
            if (eventStreamModeLabel == null)
            {
                return;
            }

            eventStreamModeLabel.text = mode switch
            {
                RuntimeTruthMode.Live => "RUNTIME TAIL",
                RuntimeTruthMode.Degraded => "PARTIAL RUNTIME TAIL",
                RuntimeTruthMode.Connecting => "CONNECTING",
                RuntimeTruthMode.Disconnected => "NO RUNTIME EVENTS",
                _ => "RETAINED DEMO",
            };
            eventStreamModeLabel.style.color = accent;

            if (mode == RuntimeTruthMode.Demo)
            {
                SetEventRow(0, "INFO", "kernel", "contract.loaded", $"tick {currentTick}");
                SetEventRow(1, "INFO", "evidence", "packet.projected", episodeCount.ToString());
                SetEventRow(2, "WARN", "snapshot", snapshotState, "governed follow-on");
                return;
            }

            if (mode == RuntimeTruthMode.Live || mode == RuntimeTruthMode.Degraded)
            {
                for (int row = 0; row < 3; row++)
                {
                    int sourceIndex = runtimeEventEntries.Length - 1 - row;
                    if (sourceIndex >= 0)
                    {
                        RuntimeEventEntry entry = runtimeEventEntries[sourceIndex];
                        SetEventRow(
                            row,
                            "INFO",
                            FirstObserved(entry.agent_instance_id, runtimeAgentId),
                            FirstObserved(entry.@event, "event unavailable"),
                            FirstObserved(entry.at, "time unavailable")
                        );
                    }
                    else
                    {
                        SetEventRow(
                            row,
                            "INFO",
                            "runtime",
                            "no additional observed event",
                            "unavailable"
                        );
                    }
                }
                return;
            }

            string eventName =
                mode == RuntimeTruthMode.Connecting
                    ? "probing runtime event contract"
                    : "runtime event stream unavailable";
            for (int row = 0; row < 3; row++)
            {
                SetEventRow(
                    row,
                    mode == RuntimeTruthMode.Connecting ? "WAIT" : "OFF",
                    "transport",
                    row == 0 ? eventName : "no observed runtime event",
                    "unavailable"
                );
            }
        }

        private void SetEventRow(
            int row,
            string severity,
            string source,
            string eventName,
            string detail
        )
        {
            if (row < 0 || row >= eventNameLabels.Length)
            {
                return;
            }

            if (eventSeverityLabels[row] != null)
            {
                eventSeverityLabels[row].text = severity;
                eventSeverityLabels[row].style.color = severity == "WARN"
                    ? new Color(1f, 0.72f, 0.24f, 1f)
                    : severity == "OFF"
                        ? new Color(1f, 0.35f, 0.34f, 1f)
                        : new Color(0.3f, 0.9f, 0.58f, 1f);
            }
            if (eventSourceLabels[row] != null)
            {
                eventSourceLabels[row].text = TruncateForInspector(source);
            }
            if (eventNameLabels[row] != null)
            {
                eventNameLabels[row].text = TruncateForInspector(eventName);
            }
            if (eventDetailLabels[row] != null)
            {
                eventDetailLabels[row].text = TruncateForInspector(detail);
            }
        }

        private static void SetRuntimeLabel(Label label, string text, Color accent)
        {
            if (label == null)
            {
                return;
            }
            label.text = text;
            label.style.color = accent;
        }

        public static bool TryNormalizeLoopbackEndpoint(
            string endpoint,
            out string normalized,
            out string rejection
        )
        {
            bool accepted = TryNormalizeRuntimeEndpoint(
                endpoint,
                out normalized,
                out RuntimeTransportKind kind,
                out rejection
            );
            if (accepted && kind == RuntimeTransportKind.LegacyCsm)
            {
                return true;
            }

            normalized = string.Empty;
            rejection =
                "Runtime endpoint rejected: only a root loopback HTTP origin is allowed.";
            return false;
        }

        public static bool TryNormalizeRuntimeEndpointForProof(
            string endpoint,
            out string normalized,
            out string transportKind,
            out string rejection
        )
        {
            bool accepted = TryNormalizeRuntimeEndpoint(
                endpoint,
                out normalized,
                out RuntimeTransportKind kind,
                out rejection
            );
            transportKind = kind.ToString();
            return accepted;
        }

        private static bool TryNormalizeRuntimeEndpoint(
            string endpoint,
            out string normalized,
            out RuntimeTransportKind kind,
            out string rejection
        )
        {
            normalized = string.Empty;
            kind = RuntimeTransportKind.None;
            rejection = string.Empty;
            if (
                string.IsNullOrWhiteSpace(endpoint)
                || !Uri.TryCreate(endpoint.Trim(), UriKind.Absolute, out Uri uri)
            )
            {
                rejection =
                    "Runtime endpoint rejected: expected an absolute HTTP loopback or HTTPS origin.";
                return false;
            }

            if (
                !string.IsNullOrEmpty(uri.UserInfo)
                || !string.IsNullOrEmpty(uri.Query)
                || !string.IsNullOrEmpty(uri.Fragment)
                || (uri.AbsolutePath != "/" && !string.IsNullOrEmpty(uri.AbsolutePath))
            )
            {
                rejection =
                    "Runtime endpoint rejected: credentials, paths, queries, and fragments are not allowed in the origin.";
                return false;
            }

            if (
                string.Equals(uri.Scheme, Uri.UriSchemeHttp, StringComparison.OrdinalIgnoreCase)
                && uri.IsLoopback
            )
            {
                kind = RuntimeTransportKind.LegacyCsm;
            }
            else if (
                string.Equals(uri.Scheme, Uri.UriSchemeHttps, StringComparison.OrdinalIgnoreCase)
            )
            {
                kind = RuntimeTransportKind.RuntimeV3;
            }
            else
            {
                rejection =
                    "Runtime endpoint rejected: legacy CSM requires loopback HTTP; Runtime v3 requires HTTPS.";
                return false;
            }

            normalized = uri.GetLeftPart(UriPartial.Authority);
            return true;
        }

        private string ResolveRuntimeApiBase()
        {
            runtimeEndpointRejection = string.Empty;
            runtimeTransportKind = RuntimeTransportKind.None;
            runtimeReadToken = Environment.GetEnvironmentVariable(
                RuntimeV3TokenEnvironmentVariable
            ) ?? string.Empty;
            string candidate = string.Empty;
            string environmentValue = Environment.GetEnvironmentVariable(
                RuntimeV3ApiEnvironmentVariable
            );
            if (!string.IsNullOrWhiteSpace(environmentValue))
            {
                candidate = environmentValue.Trim();
            }
            else
            {
                environmentValue = Environment.GetEnvironmentVariable(
                    RuntimeApiEnvironmentVariable
                );
                if (!string.IsNullOrWhiteSpace(environmentValue))
                {
                    candidate = environmentValue.Trim();
                }
            }

            if (string.IsNullOrWhiteSpace(candidate))
            {
                foreach (string argument in Environment.GetCommandLineArgs())
                {
                    if (
                        argument.StartsWith(
                            RuntimeV3ApiArgumentPrefix,
                            StringComparison.OrdinalIgnoreCase
                        )
                    )
                    {
                        candidate = argument.Substring(RuntimeV3ApiArgumentPrefix.Length).Trim();
                        break;
                    }
                    if (
                        argument.StartsWith(
                            RuntimeApiArgumentPrefix,
                            StringComparison.OrdinalIgnoreCase
                        )
                    )
                    {
                        candidate = argument.Substring(RuntimeApiArgumentPrefix.Length).Trim();
                    }
                }
            }

            if (string.IsNullOrWhiteSpace(candidate))
            {
                return string.Empty;
            }

            if (
                !TryNormalizeRuntimeEndpoint(
                    candidate,
                    out string normalized,
                    out runtimeTransportKind,
                    out runtimeEndpointRejection
                )
            )
            {
                return string.Empty;
            }

            if (
                runtimeTransportKind == RuntimeTransportKind.RuntimeV3
                && string.IsNullOrWhiteSpace(runtimeReadToken)
            )
            {
                runtimeTransportKind = RuntimeTransportKind.None;
                runtimeEndpointRejection =
                    "Runtime v3 endpoint rejected: ADL_RUNTIME_OBSERVATORY_TOKEN is not configured.";
                return string.Empty;
            }

            return normalized;
        }

        private static string TruncateForInspector(string value)
        {
            string normalized = string.IsNullOrWhiteSpace(value) ? "Unavailable" : value.Trim();
            return normalized.Length <= 34 ? normalized : normalized.Substring(0, 31) + "...";
        }

        private static Button CreateCommandButton(string text)
        {
            Button button = new()
            {
                text = text,
            };
            button.style.height = 30f;
            button.style.minWidth = 82f;
            button.style.marginLeft = 6f;
            button.style.fontSize = 9f;
            button.style.color = new Color(0.78f, 0.88f, 0.96f, 1f);
            button.style.backgroundColor = new Color(0.035f, 0.1f, 0.145f, 0.92f);
            SetRoundedBorder(button, new Color(0.13f, 0.36f, 0.5f, 1f), 4f);
            return button;
        }

        private static Button CreateNavigationButton(string icon, string destination)
        {
            Button button = new()
            {
                text = string.Empty,
                name = $"nav-{destination.ToLowerInvariant()}",
            };
            button.style.height = 40f;
            button.style.marginBottom = 5f;
            button.style.flexDirection = FlexDirection.Row;
            button.style.alignItems = Align.Center;
            button.style.paddingLeft = 8f;
            button.style.paddingRight = 6f;

            Label iconLabel = CreateLabel(icon, null, 17, FontStyle.Bold);
            iconLabel.name = $"{button.name}-icon";
            iconLabel.style.width = 27f;
            iconLabel.style.marginBottom = 0f;
            iconLabel.style.unityTextAlign = TextAnchor.MiddleCenter;
            button.Add(iconLabel);

            Label textLabel = CreateLabel(destination, null, 12, FontStyle.Normal);
            textLabel.name = $"{button.name}-label";
            textLabel.style.marginBottom = 0f;
            textLabel.style.flexGrow = 1f;
            button.Add(textLabel);
            return button;
        }

        private void UpdateNavigationState(string selected)
        {
            foreach (KeyValuePair<string, Button> entry in navigationButtons)
            {
                bool active = string.Equals(
                    entry.Key,
                    selected,
                    StringComparison.Ordinal
                );
                Color foreground = active
                    ? new Color(0.45f, 0.91f, 1f, 1f)
                    : new Color(0.75f, 0.82f, 0.91f, 1f);
                entry.Value.style.backgroundColor = active
                    ? new Color(0.03f, 0.24f, 0.34f, 0.88f)
                    : new Color(0.03f, 0.07f, 0.1f, 0.35f);
                SetRoundedBorder(
                    entry.Value,
                    active
                        ? new Color(0.19f, 0.67f, 0.82f, 1f)
                        : new Color(0.1f, 0.32f, 0.42f, 0.9f),
                    4f
                );
                Label icon = entry.Value.Q<Label>($"{entry.Value.name}-icon");
                Label label = entry.Value.Q<Label>($"{entry.Value.name}-label");
                if (icon != null)
                {
                    icon.style.color = foreground;
                }
                if (label != null)
                {
                    label.style.color = foreground;
                }
            }
        }

        private static void SetRoundedBorder(VisualElement element, Color color, float radius)
        {
            element.style.borderLeftColor = color;
            element.style.borderRightColor = color;
            element.style.borderTopColor = color;
            element.style.borderBottomColor = color;
            element.style.borderLeftWidth = 1f;
            element.style.borderRightWidth = 1f;
            element.style.borderTopWidth = 1f;
            element.style.borderBottomWidth = 1f;
            element.style.borderTopLeftRadius = radius;
            element.style.borderTopRightRadius = radius;
            element.style.borderBottomLeftRadius = radius;
            element.style.borderBottomRightRadius = radius;
        }

        public string BuildCompatibilityFallbackText()
        {
            string rooms = roomLabels == null || roomLabels.Length == 0
                ? "World / Reality"
                : string.Join(", ", roomLabels);
            string lenses = lensLabels == null || lensLabels.Length == 0
                ? "Operator lens"
                : string.Join(", ", lensLabels);

            return string.Join(
                "\n\n",
                $"{title}\n{subtitle}",
                $"Citizens: {citizenCount}\nEpisodes: {episodeCount}\nCurrent tick: {currentTick}",
                $"Default room: {defaultRoomLabel}\nDefault lens: {defaultLensLabel}",
                $"Rooms: {rooms}\nLenses: {lenses}",
                $"Runtime status: {healthSummary}\nKernel pulse: {kernelPulseStatus}\nResources: {resourceState}\nSnapshot: {snapshotState}",
                $"Governed boundary: {claimBoundary}",
                $"Packet: {packetSchema}\nRef: {packetRef}",
                $"Observability: {(hasObservabilityContract ? observabilityStatus : "contract section not bound")}",
                "Compatibility fallback active: rendering through uGUI for the governed Unity 2022.3.x compatibility path in this editor/runtime profile."
            );
        }

        private VisualElement BuildHeader()
        {
            VisualElement header = new();
            header.AddToClassList("header");
            header.Add(CreateLabel("Milestone proof demo", "eyebrow", 12, FontStyle.Bold));
            header.Add(CreateLabel(title, "title", 30, FontStyle.Bold));
            header.Add(CreateLabel(subtitle, "subtitle", 14));
            header.Add(CreateLabel(healthSummary, "headline-summary", 15));
            header.Add(BuildMetricRail());
            return header;
        }

        private VisualElement BuildMetricRail()
        {
            VisualElement rail = new();
            rail.AddToClassList("metric-rail");
            rail.Add(CreateMetricTile("Citizens", citizenCount.ToString(), "Metric"));
            rail.Add(CreateMetricTile("Episodes", episodeCount.ToString(), "Completed"));
            rail.Add(CreateMetricTile("Current tick", currentTick.ToString(), kernelPulseStatus));
            rail.Add(
                CreateMetricTile(
                    "Freedom Gate",
                    $"{allowCount}/{deferCount}/{refuseCount}",
                    "allow/defer/refuse"
                )
            );
            return rail;
        }

        private VisualElement BuildBody()
        {
            VisualElement body = new();
            body.AddToClassList("body");
            body.style.flexGrow = 1f;
            body.style.flexDirection = FlexDirection.Row;

            body.Add(BuildNavigation());

            VisualElement content = new();
            content.AddToClassList("content");
            content.style.flexGrow = 1f;
            content.style.flexDirection = FlexDirection.Column;
            content.Add(BuildDemoSurfaceCard());
            content.Add(BuildSummaryCard());
            content.Add(BuildWorldCard());
            content.Add(BuildStatusCard());
            content.Add(BuildInhabitantReadinessCard());
            content.Add(BuildObservabilityCard());
            content.Add(BuildInhabitantsCard());
            content.Add(BuildBoundaryCard());
            content.Add(BuildPacketCard());
            body.Add(content);

            return body;
        }

        private VisualElement BuildDemoSurfaceCard()
        {
            VisualElement card = CreateCard();
            card.AddToClassList("demo-surface-card");
            card.Add(CreateLabel("Runtime polis surface", "demo-surface-title", 20, FontStyle.Bold, "demo-title"));

            VisualElement strip = new();
            strip.AddToClassList("demo-strip");
            strip.Add(CreateDemoSignal("Polis state", healthSummary, "demo-polis-state"));
            strip.Add(CreateDemoSignal("Evidence", evidenceLevel.Replace("_", " "), "demo-evidence-level"));
            strip.Add(CreateDemoSignal("Operator guardrail", "proposal-only controls", "demo-operator-boundary"));
            strip.Add(CreateDemoSignal("Next proof", "#4704 walkthrough capture", "demo-next-step"));
            card.Add(strip);

            card.Add(
                CreateLabel(
                    "This surface is the bounded shell handoff between the staged observatory environment and reproducible proof capture.",
                    "demo-surface-boundary",
                    13
                )
            );
            return card;
        }

        private VisualElement BuildNavigation()
        {
            VisualElement nav = new();
            nav.AddToClassList("navigation");
            nav.Add(CreateLabel("Observatory map", "nav-title", 16, FontStyle.Bold));
            nav.Add(CreateLabel("Rooms", "nav-section", 12, FontStyle.Bold));
            foreach (string label in roomLabels)
            {
                nav.Add(CreateNavItem(DefaultIfBlank(label, "Unnamed room"), "room"));
            }
            nav.Add(CreateLabel("Lenses", "nav-section", 12, FontStyle.Bold));
            foreach (string label in lensLabels)
            {
                nav.Add(CreateNavItem(DefaultIfBlank(label, "Unnamed lens"), "lens"));
            }
            return nav;
        }

        private VisualElement BuildSummaryCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Observed summary"));
            card.Add(CreateStatRow("Citizens", citizenCount.ToString(), "citizen-count"));
            card.Add(CreateStatRow("Episodes", episodeCount.ToString(), "episode-count"));
            card.Add(CreateStatRow("Default room", defaultRoomLabel, "default-room"));
            card.Add(CreateStatRow("Default lens", defaultLensLabel, "default-lens"));
            card.Add(CreateStatRow("Current tick", currentTick.ToString(), "current-tick"));
            card.Add(
                CreateStatRow(
                    "Freedom Gate counts:",
                    $"{allowCount} allow / {deferCount} defer / {refuseCount} refuse",
                    "freedom-gate-counts"
                )
            );
            card.Add(CreateLabel(proposalModeStatement, "proposal-mode"));
            return card;
        }

        private VisualElement BuildWorldCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Inhabited world"));
            card.Add(CreateLabel(worldQuestion, "world-question", 16, FontStyle.Bold));
            card.Add(CreateLabel(worldNote, "world-note"));
            card.Add(CreateStatRow("Default lens", defaultLensLabel, "world-lens"));
            card.Add(CreateLabel(lensSummary, "world-lens-summary"));
            card.Add(CreatePill(corporateInvestorLabel, "investor-pill"));
            card.Add(CreateLabel(corporateInvestorBoundary, "world-investor-boundary"));
            return card;
        }

        private VisualElement BuildStatusCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Runtime status"));
            card.Add(CreatePill(kernelPulseStatus, "status-pill"));
            card.Add(CreateLabel(healthSummary, "status-health"));
            card.Add(CreateStatRow("Resources", resourceState, "status-resources"));
            card.Add(CreateStatRow("Snapshot", snapshotState, "status-snapshot"));
            card.Add(CreateLabel(snapshotNote, "status-snapshot-note"));
            foreach (string item in attentionItems)
            {
                card.Add(
                    CreateAttentionRow(
                        DefaultIfBlank(item, "Review bounded state."),
                        "status-attention"
                    )
                );
            }
            return card;
        }

        private VisualElement BuildObservabilityCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Observability and security", "observability-title"));
            if (!hasObservabilityContract)
            {
                card.Add(CreateLabel("Observability/security consumption proof is not bound in this contract.", "observability-unbound"));
                return card;
            }
            card.Add(CreatePill(observabilityStatus, "observability-pill"));
            card.Add(CreateLabel(observabilityClaimBoundary, "observability-boundary"));
            card.Add(CreateLabel(privateStatePosture, "observability-private-state"));
            card.Add(CreateStatRow("OTel boundary", otelBoundaryRef, "observability-otel-ref"));
            card.Add(CreateStatRow("Event stream", eventStreamExampleRef, "observability-stream-ref"));
            card.Add(CreateStatRow("Logging validation", loggingValidationRef, "observability-logging-ref"));
            card.Add(CreateStatRow("Security review", observabilitySecurityReviewRef, "observability-security-ref"));
            card.Add(CreateStatRow("Proof packet", observabilityProofPacketRef, "observability-proof-ref"));
            card.Add(CreateLabel(findingsDisposition, "observability-findings-disposition"));
            return card;
        }

        private VisualElement BuildInhabitantReadinessCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Inhabitant readiness"));
            card.Add(CreateLabel(identityBoundary, "readiness-boundary"));
            card.Add(CreateStatRow("Security floor", securityFloorRef, "readiness-security-floor"));
            foreach (ReadinessCheck check in readinessChecks)
            {
                if (check == null)
                {
                    continue;
                }

                card.Add(CreatePill(DefaultIfBlank(check.state, "unknown"), "readiness-pill"));
                card.Add(CreateLabel(DefaultIfBlank(check.label, "Readiness check"), "readiness-check", 15, FontStyle.Bold));
                card.Add(CreateLabel(DefaultIfBlank(check.note, "No readiness note supplied."), "readiness-note"));
            }
            return card;
        }

        private VisualElement BuildInhabitantsCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Citizen explorer"));
            foreach (InhabitantProjection inhabitant in inhabitants)
            {
                if (inhabitant == null)
                {
                    continue;
                }

                card.Add(CreatePill(DefaultIfBlank(inhabitant.activity_posture, "bounded"), "inhabitant-pill"));
                card.Add(CreateLabel(DefaultIfBlank(inhabitant.projection_label, "Inhabitant lane"), "inhabitant-label", 15, FontStyle.Bold));
                card.Add(CreateLabel(DefaultIfBlank(inhabitant.capability_summary, "No capability summary supplied."), "inhabitant-capability"));
                card.Add(CreateLabel(DefaultIfBlank(inhabitant.alert_summary, "No alert summary supplied."), "inhabitant-alert-summary"));
                card.Add(CreateLabel($"{DefaultIfBlank(inhabitant.identity_visibility, "identity_bounded")}: {DefaultIfBlank(inhabitant.identity_note, "Identity details remain bounded.")}", "inhabitant-identity-boundary"));
            }
            return card;
        }

        private VisualElement BuildBoundaryCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Governed boundary"));
            card.Add(CreateLabel(claimBoundary, "boundary-body"));
            card.Add(CreatePill($"allow {allowCount} / defer {deferCount} / refuse {refuseCount}", "boundary-pill"));
            card.Add(CreateLabel(caveat, "boundary-caveat"));
            return card;
        }

        private VisualElement BuildPacketCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateSectionTitle("Packet contract"));
            card.Add(CreatePill(evidenceLevel.Replace("_", " "), "contract-pill"));
            card.Add(CreateStatRow("Schema", packetSchema, "packet-schema"));
            card.Add(CreateStatRow("Packet ref", packetRef, "packet-ref"));
            card.Add(CreateStatRow("Artifact root", runtimeArtifactRoot, "artifact-root"));
            card.Add(CreateStatRow("Report ref", operatorReportRef, "report-ref"));
            card.Add(CreateLabel($"This shell is reading a deterministic Unity-facing contract derived from {evidenceLevel} Observatory evidence.", "packet-note"));
            return card;
        }

        private VisualElement BuildFooter()
        {
            VisualElement footer = new();
            footer.AddToClassList("footer");
            footer.Add(CreateLabel("Deterministic Unity Observatory logging, OTel, and security consumption projection for WP-09 O-04.", "footer-line", 13));
            return footer;
        }

        private static VisualElement CreateCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.style.paddingLeft = 14f;
            card.style.paddingRight = 14f;
            card.style.paddingTop = 14f;
            card.style.paddingBottom = 14f;
            card.style.backgroundColor = new Color(0.09f, 0.12f, 0.21f, 0.96f);
            card.style.marginBottom = 6f;
            return card;
        }

        private static Label CreateSectionTitle(string text, string name = null)
        {
            return CreateLabel(text, name, 18, FontStyle.Bold, "section-title");
        }

        private static Label CreatePill(string text, string className)
        {
            return CreateLabel(text, null, 11, FontStyle.Bold, className);
        }

        private static Label CreateNavItem(string text, string className)
        {
            return CreateLabel(text, null, 13, FontStyle.Normal, className);
        }

        private static VisualElement CreateMetricTile(string label, string value, string note)
        {
            VisualElement tile = new();
            tile.AddToClassList("metric-tile");
            tile.Add(CreateLabel(label, null, 11, FontStyle.Bold, "metric-label"));
            tile.Add(CreateLabel(value, null, 22, FontStyle.Bold, "metric-value"));
            tile.Add(CreateLabel(note, null, 11, FontStyle.Normal, "metric-note"));
            return tile;
        }

        private static VisualElement CreateDemoSignal(string label, string value, string valueName)
        {
            VisualElement signal = new();
            signal.AddToClassList("demo-signal");
            signal.Add(CreateLabel(label, null, 11, FontStyle.Bold, "demo-signal-label"));
            signal.Add(CreateLabel(value, valueName, 13, FontStyle.Bold, "demo-signal-value"));
            return signal;
        }

        private static VisualElement CreateStatRow(string label, string value, string valueName)
        {
            VisualElement row = new();
            row.AddToClassList("stat-row");
            row.Add(CreateLabel(label, null, 12, FontStyle.Bold, "stat-label"));
            row.Add(CreateLabel(value, valueName, 12, FontStyle.Normal, "stat-value"));
            return row;
        }

        private static VisualElement CreateAttentionRow(string text, string labelName)
        {
            VisualElement row = new();
            row.AddToClassList("attention-row");
            row.Add(CreateLabel("Attention", null, 11, FontStyle.Bold, "attention-pill"));
            row.Add(CreateLabel(text, labelName, 12, FontStyle.Normal, "attention-text"));
            return row;
        }

        private static Label CreateLabel(
            string text,
            string name = null,
            int fontSize = 13,
            FontStyle fontStyle = FontStyle.Normal,
            string className = null
        )
        {
            Label label = new(text);
            if (!string.IsNullOrWhiteSpace(name))
            {
                label.name = name;
            }
            if (!string.IsNullOrWhiteSpace(className))
            {
                label.AddToClassList(className);
            }

            Font runtimeFont = ResolveRuntimeFont();
            if (runtimeFont != null)
            {
                label.style.unityFont = runtimeFont;
            }

            label.style.color = new Color(0.92f, 0.95f, 0.99f, 1f);
            label.style.fontSize = fontSize;
            label.style.unityFontStyleAndWeight = fontStyle;
            label.style.whiteSpace = WhiteSpace.Normal;
            label.style.marginBottom = 6f;
            return label;
        }

        private static Font ResolveRuntimeFont()
        {
            Font runtimeFont = Resources.GetBuiltinResource<Font>("LegacyRuntime.ttf");
            if (runtimeFont != null)
            {
                return runtimeFont;
            }

            return Resources.GetBuiltinResource<Font>("Arial.ttf");
        }

        private static string DefaultIfBlank(string observed, string fallback)
        {
            return string.IsNullOrWhiteSpace(observed) ? fallback : observed;
        }

        private static string ShortRuntimeState(string observed)
        {
            if (string.IsNullOrWhiteSpace(observed))
            {
                return "UNKNOWN";
            }

            string normalized = observed.Trim().ToLowerInvariant();
            if (normalized.Contains("complete") || normalized.Contains("ready"))
            {
                return "BOUNDED";
            }
            if (normalized.Contains("stable") || normalized.Contains("healthy"))
            {
                return "STABLE";
            }
            if (normalized.Contains("defer") || normalized.Contains("degrad"))
            {
                return "DEGRADED";
            }
            return observed.Replace("_", " ").ToUpperInvariant();
        }

        private static bool HasObservabilitySection(ObservabilitySection section)
        {
            return section != null
                && !string.IsNullOrWhiteSpace(section.consumption_status)
                && !string.IsNullOrWhiteSpace(section.otel_boundary_ref)
                && !string.IsNullOrWhiteSpace(section.event_stream_example_ref)
                && !string.IsNullOrWhiteSpace(section.logging_validation_ref)
                && !string.IsNullOrWhiteSpace(section.security_review_ref)
                && !string.IsNullOrWhiteSpace(section.proof_packet_ref)
                && !string.IsNullOrWhiteSpace(section.claim_boundary)
                && !string.IsNullOrWhiteSpace(section.private_state_posture)
                && !string.IsNullOrWhiteSpace(section.findings_disposition);
        }

        private static string[] ExtractLabels(LabelEntry[] entries, string[] fallback)
        {
            if (entries == null || entries.Length == 0)
            {
                return fallback;
            }

            string[] labels = new string[entries.Length];
            for (int index = 0; index < entries.Length; index++)
            {
                string fallbackLabel = fallback != null && fallback.Length > 0
                    ? fallback[Mathf.Min(index, fallback.Length - 1)]
                    : "Unnamed";
                labels[index] = string.IsNullOrWhiteSpace(entries[index]?.label)
                    ? fallbackLabel
                    : entries[index].label;
            }
            return labels;
        }

        private static string[] ExtractStringArray(string[] observed, string[] fallback)
        {
            return observed == null || observed.Length == 0 ? fallback : observed;
        }
    }
}
