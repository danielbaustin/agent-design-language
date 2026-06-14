using System;
using System.Collections.Generic;
using UnityEngine;

namespace ADL.Demos.CelestialRescue
{
    public enum RescueGameState
    {
        Menu,
        Running,
        Won,
        Failed
    }

    public sealed class CelestialRescueGame : MonoBehaviour
    {
        [SerializeField] private int satellitesRequired = 5;
        [SerializeField] private float startingOxygenSeconds = 90f;
        [SerializeField] private ShipController ship;
        [SerializeField] private OxygenTimer oxygenTimer;
        [SerializeField] private HudController hud;
        [SerializeField] private List<SatelliteBeacon> satellites = new();

        private int rescuedCount;
        private RescueGameState state = RescueGameState.Menu;

        public RescueGameState State => state;
        public int RescuedCount => rescuedCount;
        public int SatellitesRequired => satellitesRequired;
        public float OxygenRemaining => oxygenTimer != null ? oxygenTimer.RemainingSeconds : startingOxygenSeconds;

        public event Action<RescueGameState> StateChanged;

        private void Awake()
        {
            if (oxygenTimer == null)
            {
                oxygenTimer = GetComponent<OxygenTimer>();
            }
        }

        private void Start()
        {
            EnterMenu();
        }

        public void Configure(ShipController configuredShip, OxygenTimer configuredTimer, HudController configuredHud, List<SatelliteBeacon> configuredSatellites, int required, float oxygenSeconds)
        {
            ship = configuredShip;
            oxygenTimer = configuredTimer;
            hud = configuredHud;
            satellites = configuredSatellites ?? new List<SatelliteBeacon>();
            satellitesRequired = Mathf.Max(1, required);
            startingOxygenSeconds = Mathf.Max(1f, oxygenSeconds);

            if (oxygenTimer != null)
            {
                oxygenTimer.Configure(this);
            }

            if (hud != null)
            {
                hud.Configure(this);
            }

            foreach (SatelliteBeacon satellite in satellites)
            {
                if (satellite != null)
                {
                    satellite.Configure(this);
                }
            }
        }

        public void StartMission()
        {
            rescuedCount = 0;
            state = RescueGameState.Running;

            if (ship != null)
            {
                ship.ResetShip();
                ship.SetInputEnabled(true);
            }

            foreach (SatelliteBeacon satellite in satellites)
            {
                if (satellite != null)
                {
                    satellite.ResetBeacon();
                }
            }

            oxygenTimer.ResetTimer(startingOxygenSeconds);
            oxygenTimer.StartTimer();
            PushHud();
            StateChanged?.Invoke(state);
        }

        public void EnterMenu()
        {
            state = RescueGameState.Menu;
            rescuedCount = 0;
            if (oxygenTimer != null)
            {
                oxygenTimer.ResetTimer(startingOxygenSeconds);
            }

            if (ship != null)
            {
                ship.SetInputEnabled(false);
            }

            PushHud();
            StateChanged?.Invoke(state);
        }

        public void RegisterRescue(SatelliteBeacon satellite)
        {
            if (state != RescueGameState.Running || satellite == null || satellite.IsRescued)
            {
                return;
            }

            satellite.MarkRescued();
            rescuedCount += 1;

            if (rescuedCount >= satellitesRequired)
            {
                CompleteMission();
            }
            else
            {
                PushHud();
            }
        }

        public void FailMission()
        {
            if (state != RescueGameState.Running)
            {
                return;
            }

            state = RescueGameState.Failed;
            oxygenTimer.StopTimer();
            if (ship != null)
            {
                ship.SetInputEnabled(false);
            }
            PushHud();
            StateChanged?.Invoke(state);
        }

        private void CompleteMission()
        {
            state = RescueGameState.Won;
            oxygenTimer.StopTimer();
            if (ship != null)
            {
                ship.SetInputEnabled(false);
            }
            PushHud();
            StateChanged?.Invoke(state);
        }

        private void Update()
        {
            if (state != RescueGameState.Running || oxygenTimer == null)
            {
                return;
            }

            if (oxygenTimer.RemainingSeconds <= 0f)
            {
                FailMission();
                return;
            }

            PushHud();
        }

        private void PushHud()
        {
            if (hud != null)
            {
                hud.Render(state, rescuedCount, satellitesRequired, OxygenRemaining);
            }
        }
    }
}
