using System;
using UnityEngine;
using UnityEngine.UIElements;

namespace ADL.Demos.UnityObservatory
{
    public sealed class UnityObservatoryShellController : MonoBehaviour
    {
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
            root.AddToClassList("observatory-screen");
            root.style.flexGrow = 1f;
            root.style.paddingLeft = 18f;
            root.style.paddingRight = 18f;
            root.style.paddingTop = 18f;
            root.style.paddingBottom = 18f;
            root.style.backgroundColor = new Color(0.02f, 0.03f, 0.07f, 1f);

            VisualElement shell = new();
            shell.AddToClassList("observatory-shell");
            shell.style.flexGrow = 1f;
            shell.style.flexDirection = FlexDirection.Column;
            shell.style.backgroundColor = new Color(0.05f, 0.07f, 0.14f, 0.96f);

            shell.Add(BuildHeader());
            shell.Add(BuildBody());
            shell.Add(BuildFooter());

            root.Add(shell);
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
            header.style.paddingBottom = 16f;
            header.Add(CreateLabel(title, "title", 26, FontStyle.Bold));
            header.Add(CreateLabel(subtitle, "subtitle", 14));
            return header;
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

        private VisualElement BuildNavigation()
        {
            VisualElement nav = new();
            nav.AddToClassList("navigation");
            nav.style.width = 220f;
            nav.style.paddingRight = 12f;
            nav.Add(CreateLabel("Rooms", null, 16, FontStyle.Bold));
            foreach (string label in roomLabels)
            {
                nav.Add(CreateLabel(DefaultIfBlank(label, "Unnamed room")));
            }
            nav.Add(CreateLabel("Lenses", null, 16, FontStyle.Bold));
            foreach (string label in lensLabels)
            {
                nav.Add(CreateLabel(DefaultIfBlank(label, "Unnamed lens")));
            }
            return nav;
        }

        private VisualElement BuildSummaryCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Observed summary", "summary-title", 18, FontStyle.Bold));
            card.Add(CreateLabel($"Citizens: {citizenCount}", "citizen-count"));
            card.Add(CreateLabel($"Episodes: {episodeCount}", "episode-count"));
            card.Add(CreateLabel($"Default room: {defaultRoomLabel}", "default-room"));
            card.Add(CreateLabel($"Default lens: {defaultLensLabel}", "default-lens"));
            card.Add(CreateLabel($"Current tick: {currentTick}", "current-tick"));
            card.Add(CreateLabel(proposalModeStatement, "proposal-mode"));
            return card;
        }

        private VisualElement BuildWorldCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Inhabited world", "world-title", 18, FontStyle.Bold));
            card.Add(CreateLabel(worldQuestion, "world-question"));
            card.Add(CreateLabel(worldNote, "world-note"));
            card.Add(CreateLabel($"Default lens: {defaultLensLabel}", "world-lens"));
            card.Add(CreateLabel(lensSummary, "world-lens-summary"));
            card.Add(CreateLabel($"{corporateInvestorLabel}: {corporateInvestorBoundary}", "world-investor-boundary"));
            return card;
        }

        private VisualElement BuildStatusCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Runtime status", "status-title", 18, FontStyle.Bold));
            card.Add(CreateLabel(healthSummary, "status-health"));
            card.Add(CreateLabel($"Kernel pulse: {kernelPulseStatus}", "status-pulse"));
            card.Add(CreateLabel($"Resources: {resourceState}", "status-resources"));
            card.Add(CreateLabel($"Snapshot: {snapshotState}", "status-snapshot"));
            card.Add(CreateLabel(snapshotNote, "status-snapshot-note"));
            foreach (string item in attentionItems)
            {
                card.Add(CreateLabel($"Attention: {DefaultIfBlank(item, "Review bounded state.")}", "status-attention"));
            }
            return card;
        }

        private VisualElement BuildObservabilityCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Observability and security", "observability-title", 18, FontStyle.Bold));
            if (!hasObservabilityContract)
            {
                card.Add(CreateLabel("Observability/security consumption proof is not bound in this contract.", "observability-unbound"));
                return card;
            }
            card.Add(CreateLabel($"Consumption status: {observabilityStatus}", "observability-status"));
            card.Add(CreateLabel(observabilityClaimBoundary, "observability-boundary"));
            card.Add(CreateLabel(privateStatePosture, "observability-private-state"));
            card.Add(CreateLabel($"OTel boundary: {otelBoundaryRef}", "observability-otel-ref"));
            card.Add(CreateLabel($"Event stream example: {eventStreamExampleRef}", "observability-stream-ref"));
            card.Add(CreateLabel($"Logging validation: {loggingValidationRef}", "observability-logging-ref"));
            card.Add(CreateLabel($"Security review: {observabilitySecurityReviewRef}", "observability-security-ref"));
            card.Add(CreateLabel($"Proof packet: {observabilityProofPacketRef}", "observability-proof-ref"));
            card.Add(CreateLabel(findingsDisposition, "observability-findings-disposition"));
            return card;
        }

        private VisualElement BuildInhabitantReadinessCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Inhabitant readiness", "readiness-title", 18, FontStyle.Bold));
            card.Add(CreateLabel(identityBoundary, "readiness-boundary"));
            card.Add(CreateLabel($"Security floor: {securityFloorRef}", "readiness-security-floor"));
            foreach (ReadinessCheck check in readinessChecks)
            {
                if (check == null)
                {
                    continue;
                }

                card.Add(CreateLabel($"{DefaultIfBlank(check.state, "unknown")}: {DefaultIfBlank(check.label, "Readiness check")}", "readiness-check"));
                card.Add(CreateLabel(DefaultIfBlank(check.note, "No readiness note supplied."), "readiness-note"));
            }
            return card;
        }

        private VisualElement BuildInhabitantsCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Citizen explorer", "inhabitants-title", 18, FontStyle.Bold));
            foreach (InhabitantProjection inhabitant in inhabitants)
            {
                if (inhabitant == null)
                {
                    continue;
                }

                card.Add(CreateLabel(DefaultIfBlank(inhabitant.projection_label, "Inhabitant lane"), "inhabitant-label", 15, FontStyle.Bold));
                card.Add(CreateLabel(DefaultIfBlank(inhabitant.activity_posture, "bounded"), "inhabitant-state"));
                card.Add(CreateLabel(DefaultIfBlank(inhabitant.capability_summary, "No capability summary supplied."), "inhabitant-capability"));
                card.Add(CreateLabel(DefaultIfBlank(inhabitant.alert_summary, "No alert summary supplied."), "inhabitant-alert-summary"));
                card.Add(CreateLabel($"{DefaultIfBlank(inhabitant.identity_visibility, "identity_bounded")}: {DefaultIfBlank(inhabitant.identity_note, "Identity details remain bounded.")}", "inhabitant-identity-boundary"));
            }
            return card;
        }

        private VisualElement BuildBoundaryCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Governed boundary", "boundary-title", 18, FontStyle.Bold));
            card.Add(CreateLabel(claimBoundary, "boundary-body"));
            card.Add(CreateLabel($"Freedom Gate counts: allow {allowCount}, defer {deferCount}, refuse {refuseCount}.", "boundary-followon"));
            card.Add(CreateLabel(caveat, "boundary-caveat"));
            return card;
        }

        private VisualElement BuildPacketCard()
        {
            VisualElement card = CreateCard();
            card.Add(CreateLabel("Packet contract", "packet-title", 18, FontStyle.Bold));
            card.Add(CreateLabel(packetSchema, "packet-schema"));
            card.Add(CreateLabel(packetRef, "packet-ref"));
            card.Add(CreateLabel(runtimeArtifactRoot, "artifact-root"));
            card.Add(CreateLabel(operatorReportRef, "report-ref"));
            card.Add(CreateLabel($"This shell is reading a deterministic Unity-facing contract derived from {evidenceLevel} Observatory evidence.", "packet-note"));
            return card;
        }

        private VisualElement BuildFooter()
        {
            VisualElement footer = new();
            footer.AddToClassList("footer");
            footer.style.paddingTop = 16f;
            footer.Add(CreateLabel("Deterministic Unity Observatory logging, OTel, and security consumption projection for WP-09 O-04.", null, 13));
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

        private static Label CreateLabel(
            string text,
            string name = null,
            int fontSize = 13,
            FontStyle fontStyle = FontStyle.Normal
        )
        {
            Label label = new(text);
            if (!string.IsNullOrWhiteSpace(name))
            {
                label.name = name;
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
