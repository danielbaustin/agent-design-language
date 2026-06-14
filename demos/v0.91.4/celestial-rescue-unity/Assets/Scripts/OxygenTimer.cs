using UnityEngine;

namespace ADL.Demos.CelestialRescue
{
    public sealed class OxygenTimer : MonoBehaviour
    {
        [SerializeField] private CelestialRescueGame game;
        [SerializeField] private float remainingSeconds = 90f;
        private bool running;

        public float RemainingSeconds => Mathf.Max(0f, remainingSeconds);

        public void Configure(CelestialRescueGame configuredGame)
        {
            game = configuredGame;
        }

        public void ResetTimer(float seconds)
        {
            remainingSeconds = Mathf.Max(0f, seconds);
            running = false;
        }

        public void StartTimer()
        {
            running = true;
        }

        public void StopTimer()
        {
            running = false;
        }

        private void Update()
        {
            if (!running)
            {
                return;
            }

            remainingSeconds = Mathf.Max(0f, remainingSeconds - Time.deltaTime);
            if (remainingSeconds <= 0f && game != null)
            {
                game.FailMission();
            }
        }
    }
}
