using UnityEngine;
using UnityEngine.UIElements;

namespace ADL.Demos.UnityObservatory
{
    public sealed class UnityObservatoryBootstrap : MonoBehaviour
    {
        private const string ContractResourcePath = "observatory_contract";
        [SerializeField] private int baselineCitizenCount = 3;
        [SerializeField] private int baselineEpisodeCount = 2;
        [SerializeField] private string packetSchema = "adl.csm_visibility_packet.v1";
        [SerializeField] private string packetRef =
            "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json";
        [SerializeField] private string defaultRoomLabel = "World / Reality";
        [SerializeField] private string defaultLensLabel = "Operator lens";

        private void Awake()
        {
            EnsureCamera();
            CreateObservatoryShell();
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
            camera.orthographicSize = 6f;
            camera.backgroundColor = new Color(0.015f, 0.025f, 0.055f, 1f);
        }

        private void CreateObservatoryShell()
        {
            GameObject shellObject = new("Unity Observatory Shell");
            UnityObservatoryShellController controller =
                shellObject.AddComponent<UnityObservatoryShellController>();
            TextAsset contractAsset = Resources.Load<TextAsset>(ContractResourcePath);
            if (contractAsset != null)
            {
                controller.ConfigureFromContract(contractAsset.text);
            }
            else
            {
                controller.ConfigureFallback(
                    packetSchema,
                    packetRef,
                    baselineCitizenCount,
                    baselineEpisodeCount,
                    defaultRoomLabel,
                    defaultLensLabel
                );
            }

            UIDocument document = shellObject.AddComponent<UIDocument>();
            VisualElement root = document.rootVisualElement;
            controller.Build(root);
        }
    }
}
