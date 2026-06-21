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
                return;
            }

            UnityObservatoryContractDocument contract =
                JsonUtility.FromJson<UnityObservatoryContractDocument>(rawContractJson);
            if (contract == null)
            {
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
            root.Clear();
            root.AddToClassList("observatory-screen");

            VisualElement shell = new();
            shell.AddToClassList("observatory-shell");

            shell.Add(BuildHeader());
            shell.Add(BuildBody());
            shell.Add(BuildFooter());

            root.Add(shell);
        }

        private VisualElement BuildHeader()
        {
            VisualElement header = new();
            header.AddToClassList("header");
            header.Add(new Label(title) { name = "title" });
            header.Add(new Label(subtitle) { name = "subtitle" });
            return header;
        }

        private VisualElement BuildBody()
        {
            VisualElement body = new();
            body.AddToClassList("body");

            body.Add(BuildNavigation());

            VisualElement content = new();
            content.AddToClassList("content");
            content.Add(BuildSummaryCard());
            content.Add(BuildWorldCard());
            content.Add(BuildStatusCard());
            content.Add(BuildInhabitantReadinessCard());
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
            nav.Add(new Label("Rooms"));
            foreach (string label in roomLabels)
            {
                nav.Add(new Label(label));
            }
            nav.Add(new Label("Lenses"));
            foreach (string label in lensLabels)
            {
                nav.Add(new Label(label));
            }
            return nav;
        }

        private VisualElement BuildSummaryCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Observed summary") { name = "summary-title" });
            card.Add(new Label($"Citizens: {citizenCount}") { name = "citizen-count" });
            card.Add(new Label($"Episodes: {episodeCount}") { name = "episode-count" });
            card.Add(new Label($"Default room: {defaultRoomLabel}") { name = "default-room" });
            card.Add(new Label($"Default lens: {defaultLensLabel}") { name = "default-lens" });
            card.Add(new Label($"Current tick: {currentTick}") { name = "current-tick" });
            card.Add(new Label(proposalModeStatement) { name = "proposal-mode" });
            return card;
        }

        private VisualElement BuildWorldCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Inhabited world") { name = "world-title" });
            card.Add(new Label(worldQuestion) { name = "world-question" });
            card.Add(new Label(worldNote) { name = "world-note" });
            card.Add(new Label($"Default lens: {defaultLensLabel}") { name = "world-lens" });
            card.Add(new Label(lensSummary) { name = "world-lens-summary" });
            card.Add(new Label($"{corporateInvestorLabel}: {corporateInvestorBoundary}") { name = "world-investor-boundary" });
            return card;
        }

        private VisualElement BuildStatusCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Runtime status") { name = "status-title" });
            card.Add(new Label(healthSummary) { name = "status-health" });
            card.Add(new Label($"Kernel pulse: {kernelPulseStatus}") { name = "status-pulse" });
            card.Add(new Label($"Resources: {resourceState}") { name = "status-resources" });
            card.Add(new Label($"Snapshot: {snapshotState}") { name = "status-snapshot" });
            card.Add(new Label(snapshotNote) { name = "status-snapshot-note" });
            foreach (string item in attentionItems)
            {
                card.Add(new Label($"Attention: {item}") { name = "status-attention" });
            }
            return card;
        }

        private VisualElement BuildInhabitantReadinessCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Inhabitant readiness") { name = "readiness-title" });
            card.Add(new Label(identityBoundary) { name = "readiness-boundary" });
            card.Add(new Label($"Security floor: {securityFloorRef}") { name = "readiness-security-floor" });
            foreach (ReadinessCheck check in readinessChecks)
            {
                card.Add(
                    new Label($"{check.state}: {check.label}") { name = "readiness-check" }
                );
                card.Add(new Label(check.note) { name = "readiness-note" });
            }
            return card;
        }

        private VisualElement BuildInhabitantsCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Citizen explorer") { name = "inhabitants-title" });
            foreach (InhabitantProjection inhabitant in inhabitants)
            {
                card.Add(
                    new Label(
                        inhabitant.projection_label
                    ) { name = "inhabitant-label" }
                );
                card.Add(
                    new Label(
                        inhabitant.activity_posture
                    ) { name = "inhabitant-state" }
                );
                card.Add(
                    new Label(inhabitant.capability_summary) { name = "inhabitant-capability" }
                );
                card.Add(
                    new Label(inhabitant.alert_summary) { name = "inhabitant-alert-summary" }
                );
                card.Add(
                    new Label(
                        $"{inhabitant.identity_visibility}: {inhabitant.identity_note}"
                    ) { name = "inhabitant-identity-boundary" }
                );
            }
            return card;
        }

        private VisualElement BuildBoundaryCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Governed boundary") { name = "boundary-title" });
            card.Add(new Label(claimBoundary) { name = "boundary-body" });
            card.Add(
                new Label(
                    $"Freedom Gate counts: allow {allowCount}, defer {deferCount}, refuse {refuseCount}."
                ) { name = "boundary-followon" }
            );
            card.Add(new Label(caveat) { name = "boundary-caveat" });
            return card;
        }

        private VisualElement BuildPacketCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Packet contract") { name = "packet-title" });
            card.Add(new Label(packetSchema) { name = "packet-schema" });
            card.Add(new Label(packetRef) { name = "packet-ref" });
            card.Add(new Label(runtimeArtifactRoot) { name = "artifact-root" });
            card.Add(new Label(operatorReportRef) { name = "report-ref" });
            card.Add(
                new Label(
                    $"This shell is reading a deterministic Unity-facing contract derived from {evidenceLevel} Observatory evidence."
                ) { name = "packet-note" }
            );
            return card;
        }

        private VisualElement BuildFooter()
        {
            VisualElement footer = new();
            footer.AddToClassList("footer");
            footer.Add(new Label("Deterministic Unity Observatory inhabitant-readiness projection for WP-09 O-03."));
            return footer;
        }

        private static string DefaultIfBlank(string observed, string fallback)
        {
            return string.IsNullOrWhiteSpace(observed) ? fallback : observed;
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
                labels[index] = string.IsNullOrWhiteSpace(entries[index]?.label)
                    ? fallback[Mathf.Min(index, fallback.Length - 1)]
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
