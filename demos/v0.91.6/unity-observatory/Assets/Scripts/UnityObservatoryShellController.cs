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
            public SummarySection summary;
            public FreedomGateSection freedom_gate;
            public ReviewSection review;
            public LabelEntry[] rooms;
            public LabelEntry[] lenses;
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
        private string[] roomLabels =
        {
            "World / Reality",
            "Operator / Governance",
            "Cognition / Internal State",
        };
        private string[] lensLabels = { "Public lens", "Operator lens", "Reviewer lens" };

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
            caveat =
                contract.review?.caveats != null && contract.review.caveats.Length > 0
                    ? contract.review.caveats[0]
                    : caveat;
            roomLabels = ExtractLabels(contract.rooms, roomLabels);
            lensLabels = ExtractLabels(contract.lenses, lensLabels);
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
            footer.Add(new Label("Deterministic Unity Observatory contract loader for WP-09 O-02."));
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
    }
}
