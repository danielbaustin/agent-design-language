using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UIElements;

namespace ADL.Demos.CelestialRescue
{
    public sealed class CelestialRescueBootstrap : MonoBehaviour
    {
        [SerializeField] private int satellitesRequired = 5;
        [SerializeField] private float oxygenSeconds = 90f;

        private void Awake()
        {
            EnsureCamera();

            GameObject gameObject = new("Celestial Rescue Game");
            CelestialRescueGame game = gameObject.AddComponent<CelestialRescueGame>();
            OxygenTimer oxygen = gameObject.AddComponent<OxygenTimer>();

            ShipController ship = CreateShip();
            List<SatelliteBeacon> satellites = CreateSatellites();
            HudController hud = CreateHud(game);

            game.Configure(ship, oxygen, hud, satellites, satellitesRequired, oxygenSeconds);
        }

        private static void EnsureCamera()
        {
            if (Camera.main != null)
            {
                return;
            }

            GameObject cameraObject = new("Main Camera");
            cameraObject.tag = "MainCamera";
            Camera camera = cameraObject.AddComponent<Camera>();
            camera.orthographic = true;
            camera.orthographicSize = 7f;
            camera.backgroundColor = new Color(0.015f, 0.025f, 0.055f, 1f);
        }

        private static ShipController CreateShip()
        {
            GameObject shipObject = new("Rescue Ship");
            shipObject.transform.position = Vector3.zero;
            Rigidbody2D body = shipObject.AddComponent<Rigidbody2D>();
            body.gravityScale = 0f;
            body.drag = 0.8f;
            CircleCollider2D collider = shipObject.AddComponent<CircleCollider2D>();
            collider.radius = 0.42f;
            SpriteRenderer renderer = shipObject.AddComponent<SpriteRenderer>();
            renderer.color = new Color(0.45f, 0.9f, 1f, 1f);
            return shipObject.AddComponent<ShipController>();
        }

        private List<SatelliteBeacon> CreateSatellites()
        {
            List<SatelliteBeacon> satellites = new();
            Vector2[] positions =
            {
                new(-5.0f, 3.1f),
                new(-2.7f, -3.4f),
                new(0.9f, 4.0f),
                new(3.8f, -2.6f),
                new(5.4f, 2.1f)
            };

            for (int index = 0; index < satellitesRequired; index += 1)
            {
                GameObject satelliteObject = new($"Satellite Beacon {index + 1}");
                satelliteObject.transform.position = positions[index % positions.Length];
                CircleCollider2D collider = satelliteObject.AddComponent<CircleCollider2D>();
                collider.isTrigger = true;
                collider.radius = 0.8f;
                SpriteRenderer renderer = satelliteObject.AddComponent<SpriteRenderer>();
                renderer.color = new Color(0.7f, 0.95f, 1f, 1f);
                SatelliteBeacon beacon = satelliteObject.AddComponent<SatelliteBeacon>();
                satellites.Add(beacon);
            }

            return satellites;
        }

        private static HudController CreateHud(CelestialRescueGame game)
        {
            GameObject hudObject = new("Celestial Rescue HUD");
            HudController hud = hudObject.AddComponent<HudController>();
            hud.Configure(game);

            UIDocument document = hudObject.AddComponent<UIDocument>();
            VisualElement root = document.rootVisualElement;
            BuildHudTree(root);
            hud.Bind(root);
            return hud;
        }

        private static void BuildHudTree(VisualElement root)
        {
            root.AddToClassList("screen");

            VisualElement left = new();
            left.AddToClassList("hud-card");
            left.AddToClassList("top-left");
            left.Add(new Label("Mission briefing") { name = "mission-state" });
            left.Add(new Label("Satellites 0/5") { name = "rescue-count" });
            root.Add(left);

            VisualElement right = new();
            right.AddToClassList("hud-card");
            right.AddToClassList("top-right");
            right.Add(new Label("O2 90s") { name = "oxygen-value" });
            VisualElement oxygenTrack = new();
            oxygenTrack.AddToClassList("oxygen-track");
            VisualElement oxygenFill = new() { name = "oxygen-fill" };
            oxygenFill.AddToClassList("oxygen-fill");
            oxygenTrack.Add(oxygenFill);
            right.Add(oxygenTrack);
            root.Add(right);

            VisualElement panel = new() { name = "result-panel" };
            panel.AddToClassList("mission-panel");
            panel.Add(new Label("Celestial Rescue") { name = "result-title" });
            panel.Add(new Label("Recover five satellites before oxygen runs out.") { name = "result-body" });
            panel.Add(new Button { name = "start-button", text = "Begin rescue" });
            panel.Add(new Button { name = "retry-button", text = "Try again" });
            root.Add(panel);
        }
    }
}
