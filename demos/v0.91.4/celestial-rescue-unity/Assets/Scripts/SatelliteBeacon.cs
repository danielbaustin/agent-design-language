using UnityEngine;

namespace ADL.Demos.CelestialRescue
{
    [RequireComponent(typeof(Collider2D))]
    public sealed class SatelliteBeacon : MonoBehaviour
    {
        [SerializeField] private CelestialRescueGame game;
        [SerializeField] private SpriteRenderer beaconRenderer;
        [SerializeField] private Color activeColor = new(0.7f, 0.95f, 1f, 1f);
        [SerializeField] private Color rescuedColor = new(0.4f, 1f, 0.62f, 1f);

        public bool IsRescued { get; private set; }

        private void Awake()
        {
            Collider2D rescueCollider = GetComponent<Collider2D>();
            if (rescueCollider != null)
            {
                rescueCollider.isTrigger = true;
            }
        }

        public void Configure(CelestialRescueGame configuredGame)
        {
            game = configuredGame;
        }

        public void ResetBeacon()
        {
            IsRescued = false;
            SetColor(activeColor);
        }

        public void MarkRescued()
        {
            IsRescued = true;
            SetColor(rescuedColor);
        }

        private void OnTriggerEnter2D(Collider2D other)
        {
            if (IsRescued || game == null || other.GetComponent<ShipController>() == null)
            {
                return;
            }

            game.RegisterRescue(this);
        }

        private void SetColor(Color color)
        {
            if (beaconRenderer != null)
            {
                beaconRenderer.color = color;
            }
        }
    }
}
