using UnityEngine;

namespace ADL.Demos.CelestialRescue
{
    [RequireComponent(typeof(Rigidbody2D))]
    public sealed class ShipController : MonoBehaviour
    {
        [SerializeField] private float acceleration = 9.5f;
        [SerializeField] private float maxSpeed = 8f;
        [SerializeField] private float turnSpeed = 180f;
        [SerializeField] private Vector2 startPosition = Vector2.zero;

        private Rigidbody2D body;
        private bool inputEnabled;

        private void Awake()
        {
            body = GetComponent<Rigidbody2D>();
            body.gravityScale = 0f;
            body.drag = 0.8f;
        }

        public void SetInputEnabled(bool enabled)
        {
            inputEnabled = enabled;
        }

        public void ResetShip()
        {
            transform.position = startPosition;
            transform.rotation = Quaternion.identity;
            if (body != null)
            {
                body.velocity = Vector2.zero;
                body.angularVelocity = 0f;
            }
        }

        private void FixedUpdate()
        {
            if (!inputEnabled)
            {
                return;
            }

            float thrust = Input.GetAxisRaw("Vertical");
            float turn = -Input.GetAxisRaw("Horizontal");

            body.AddForce(transform.up * thrust * acceleration, ForceMode2D.Force);
            body.MoveRotation(body.rotation + turn * turnSpeed * Time.fixedDeltaTime);

            if (body.velocity.magnitude > maxSpeed)
            {
                body.velocity = body.velocity.normalized * maxSpeed;
            }
        }
    }
}
