using UnityEngine;
using UnityEngine.UIElements;

namespace ADL.Demos.CelestialRescue
{
    public sealed class HudController : MonoBehaviour
    {
        [SerializeField] private CelestialRescueGame game;

        private Label missionState;
        private Label rescueCount;
        private Label oxygen;
        private VisualElement oxygenFill;
        private Button startButton;
        private Button retryButton;
        private VisualElement resultPanel;
        private Label resultTitle;
        private Label resultBody;

        public void Configure(CelestialRescueGame configuredGame)
        {
            game = configuredGame;
        }

        public void Bind(VisualElement root)
        {
            missionState = root.Q<Label>("mission-state");
            rescueCount = root.Q<Label>("rescue-count");
            oxygen = root.Q<Label>("oxygen-value");
            oxygenFill = root.Q<VisualElement>("oxygen-fill");
            startButton = root.Q<Button>("start-button");
            retryButton = root.Q<Button>("retry-button");
            resultPanel = root.Q<VisualElement>("result-panel");
            resultTitle = root.Q<Label>("result-title");
            resultBody = root.Q<Label>("result-body");

            if (startButton != null && game != null)
            {
                startButton.clicked += game.StartMission;
            }

            if (retryButton != null && game != null)
            {
                retryButton.clicked += game.StartMission;
            }
        }

        public void Render(RescueGameState state, int rescued, int required, float oxygenSeconds)
        {
            if (missionState == null)
            {
                return;
            }

            missionState.text = state switch
            {
                RescueGameState.Menu => "Mission briefing",
                RescueGameState.Running => "Rescue in progress",
                RescueGameState.Won => "Orbit stabilized",
                RescueGameState.Failed => "Oxygen depleted",
                _ => "Unknown state"
            };

            rescueCount.text = $"Satellites {rescued}/{required}";
            oxygen.text = $"O2 {Mathf.CeilToInt(oxygenSeconds)}s";

            float oxygenPercent = Mathf.Clamp01(oxygenSeconds / 90f);
            oxygenFill.style.width = Length.Percent(oxygenPercent * 100f);

            bool showResult = state == RescueGameState.Won || state == RescueGameState.Failed || state == RescueGameState.Menu;
            resultPanel.style.display = showResult ? DisplayStyle.Flex : DisplayStyle.None;
            startButton.style.display = state == RescueGameState.Menu ? DisplayStyle.Flex : DisplayStyle.None;
            retryButton.style.display = state == RescueGameState.Won || state == RescueGameState.Failed ? DisplayStyle.Flex : DisplayStyle.None;

            resultTitle.text = state switch
            {
                RescueGameState.Menu => "Celestial Rescue",
                RescueGameState.Won => "All satellites recovered",
                RescueGameState.Failed => "Rescue window closed",
                _ => string.Empty
            };

            resultBody.text = state switch
            {
                RescueGameState.Menu => "Pilot a small rescue craft through a dark orbital field. Recover five stranded satellites before oxygen runs out.",
                RescueGameState.Won => "The constellation is online. The C-SDLC demo now has a Unity game surface.",
                RescueGameState.Failed => "Return to dock, refill oxygen, and try the rescue route again.",
                _ => string.Empty
            };
        }
    }
}
