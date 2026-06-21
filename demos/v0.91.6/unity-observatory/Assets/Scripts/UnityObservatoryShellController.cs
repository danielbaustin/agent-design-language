using UnityEngine;
using UnityEngine.UIElements;

namespace ADL.Demos.UnityObservatory
{
    public sealed class UnityObservatoryShellController : MonoBehaviour
    {
        private string packetSchema = "adl.csm_visibility_packet.v1";
        private string packetRef =
            "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json";
        private int citizenCount = 3;
        private int episodeCount = 2;
        private string defaultRoomLabel = "World / Reality";
        private string defaultLensLabel = "Operator lens";

        public void Configure(
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
            header.Add(new Label("Unity Observatory") { name = "title" });
            header.Add(new Label("Launch-baseline governed shell") { name = "subtitle" });
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
            nav.Add(new Label("Observatory"));
            nav.Add(new Label("Citizens"));
            nav.Add(new Label("Contracts"));
            nav.Add(new Label("C-SDLC"));
            nav.Add(new Label("Traces"));
            nav.Add(new Label("Reviews"));
            return nav;
        }

        private VisualElement BuildSummaryCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Baseline summary") { name = "summary-title" });
            card.Add(new Label($"Citizens: {citizenCount}") { name = "citizen-count" });
            card.Add(new Label($"Episodes: {episodeCount}") { name = "episode-count" });
            card.Add(new Label($"Default room: {defaultRoomLabel}") { name = "default-room" });
            card.Add(new Label($"Default lens: {defaultLensLabel}") { name = "default-lens" });
            return card;
        }

        private VisualElement BuildBoundaryCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Governed boundary") { name = "boundary-title" });
            card.Add(
                new Label(
                    "This surface proposes reviewable observatory navigation only. No direct runtime mutation is performed here."
                ) { name = "boundary-body" }
            );
            card.Add(
                new Label(
                    "Packet parsing, inhabitant readiness, and observability/security closure remain issue-owned follow-on work."
                ) { name = "boundary-followon" }
            );
            return card;
        }

        private VisualElement BuildPacketCard()
        {
            VisualElement card = new();
            card.AddToClassList("card");
            card.Add(new Label("Packet contract") { name = "packet-title" });
            card.Add(new Label(packetSchema) { name = "packet-schema" });
            card.Add(new Label(packetRef) { name = "packet-ref" });
            card.Add(
                new Label(
                    "This launch baseline shows the bounded ingress seam that #4032 will deepen into a real Unity-facing loader."
                ) { name = "packet-note" }
            );
            return card;
        }

        private VisualElement BuildFooter()
        {
            VisualElement footer = new();
            footer.AddToClassList("footer");
            footer.Add(new Label("Deterministic launch-baseline scaffold for WP-09 O-01."));
            return footer;
        }
    }
}
