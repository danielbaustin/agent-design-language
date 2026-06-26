using System.Collections;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.UIElements;

namespace ADL.Demos.UnityObservatory
{
    public sealed class UnityObservatoryBootstrap : MonoBehaviour
    {
        private const string ContractResourcePath = "observatory_contract";
        private const string RuntimeThemeResourcePath = "UnityDefaultRuntimeTheme";
        private const string BootstrapObjectName = "Unity Observatory Bootstrap";
        private const string ShellObjectName = "Unity Observatory Shell";
        [SerializeField] private int baselineCitizenCount = 3;
        [SerializeField] private int baselineEpisodeCount = 2;
        [SerializeField] private string packetSchema = "adl.csm_visibility_packet.v1";
        [SerializeField] private string packetRef =
            "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json";
        [SerializeField] private string defaultRoomLabel = "World / Reality";
        [SerializeField] private string defaultLensLabel = "Operator lens";
        private bool bootstrapped;

        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.AfterSceneLoad)]
        private static void EnsureBootstrapOnSceneLoad()
        {
            UnityObservatoryBootstrap bootstrap = FindAnyObjectByType<UnityObservatoryBootstrap>();
            if (bootstrap == null)
            {
                GameObject bootstrapObject = new(BootstrapObjectName);
                bootstrap = bootstrapObject.AddComponent<UnityObservatoryBootstrap>();
            }

            bootstrap.Boot();
        }

        private void Awake()
        {
            Boot();
        }

        private void Boot()
        {
            if (bootstrapped)
            {
                return;
            }

            bootstrapped = true;
            EnsureCamera();
            StartCoroutine(CreateObservatoryShell());
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

        private IEnumerator CreateObservatoryShell()
        {
            GameObject existingShell = GameObject.Find(ShellObjectName);
            if (existingShell != null)
            {
                Destroy(existingShell);
                yield return null;
            }

            GameObject shellObject = new(ShellObjectName);
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

            if (ShouldUseCompatibilityCanvas())
            {
                Debug.LogWarning(
                    $"Unity Observatory is using the compatibility canvas for Unity {Application.unityVersion} to avoid runtime UI Toolkit theme incompatibilities in this editor path."
                );
                CreateCompatibilityCanvas(shellObject, controller);
                yield break;
            }

            PanelSettings panelSettings = CreatePanelSettings();
            if (panelSettings.themeStyleSheet == null)
            {
                Debug.LogWarning(
                    "Unity Observatory is falling back to the compatibility canvas because no ThemeStyleSheet was available for the runtime UI Toolkit panel."
                );
                CreateCompatibilityCanvas(shellObject, controller);
                yield break;
            }

            UIDocument document = shellObject.AddComponent<UIDocument>();
            document.panelSettings = panelSettings;
            document.sortingOrder = 10;
            yield return null;

            VisualElement root = document.rootVisualElement;
            if (root == null)
            {
                Debug.LogError(
                    "Unity Observatory could not create a UI Toolkit root visual element."
                );
                yield break;
            }

            controller.Build(root);
        }

        private static bool ShouldUseCompatibilityCanvas()
        {
            return Application.unityVersion.StartsWith("2022.3.");
        }

        private static void CreateCompatibilityCanvas(
            GameObject shellObject,
            UnityObservatoryShellController controller
        )
        {
            Canvas canvas = shellObject.AddComponent<Canvas>();
            canvas.renderMode = RenderMode.ScreenSpaceOverlay;
            canvas.sortingOrder = 10;
            shellObject.AddComponent<CanvasScaler>().uiScaleMode =
                CanvasScaler.ScaleMode.ScaleWithScreenSize;
            shellObject.AddComponent<GraphicRaycaster>();

            Font runtimeFont = ResolveRuntimeFont();

            GameObject panelObject = new("Observatory Compatibility Panel");
            panelObject.transform.SetParent(shellObject.transform, false);
            RectTransform panelTransform = panelObject.AddComponent<RectTransform>();
            panelTransform.anchorMin = new Vector2(0f, 0f);
            panelTransform.anchorMax = new Vector2(1f, 1f);
            panelTransform.offsetMin = new Vector2(24f, 24f);
            panelTransform.offsetMax = new Vector2(-24f, -24f);

            UnityEngine.UI.Image panelBackground =
                panelObject.AddComponent<UnityEngine.UI.Image>();
            panelBackground.color = new Color(0.05f, 0.07f, 0.14f, 0.94f);

            GameObject textObject = new("Observatory Compatibility Text");
            textObject.transform.SetParent(panelObject.transform, false);
            RectTransform textTransform = textObject.AddComponent<RectTransform>();
            textTransform.anchorMin = new Vector2(0f, 0f);
            textTransform.anchorMax = new Vector2(1f, 1f);
            textTransform.offsetMin = new Vector2(22f, 22f);
            textTransform.offsetMax = new Vector2(-22f, -22f);

            Text text = textObject.AddComponent<Text>();
            text.font = runtimeFont;
            text.fontSize = 18;
            text.alignment = TextAnchor.UpperLeft;
            text.horizontalOverflow = HorizontalWrapMode.Wrap;
            text.verticalOverflow = VerticalWrapMode.Overflow;
            text.color = new Color(0.92f, 0.95f, 0.99f, 1f);
            text.text = controller.BuildCompatibilityFallbackText();
        }

        private static PanelSettings CreatePanelSettings()
        {
            PanelSettings settings = ScriptableObject.CreateInstance<PanelSettings>();
            settings.name = "Unity Observatory Runtime Panel Settings";
            settings.themeStyleSheet =
                Resources.Load<ThemeStyleSheet>(RuntimeThemeResourcePath)
                ?? ResolveThemeStyleSheet();
            settings.scaleMode = PanelScaleMode.ScaleWithScreenSize;
            settings.referenceResolution = new Vector2Int(1440, 900);
            settings.screenMatchMode = PanelScreenMatchMode.MatchWidthOrHeight;
            settings.match = 0.5f;
            settings.sortingOrder = 10;
            return settings;
        }

        private static ThemeStyleSheet ResolveThemeStyleSheet()
        {
            Object[] loadedThemes = Resources.FindObjectsOfTypeAll(typeof(ThemeStyleSheet));
            if (loadedThemes.Length == 0)
            {
                loadedThemes = Resources.LoadAll("", typeof(ThemeStyleSheet));
            }

            if (loadedThemes.Length > 0)
            {
                return loadedThemes[0] as ThemeStyleSheet;
            }

            Debug.LogWarning(
                "Unity Observatory could not find a ThemeStyleSheet; UI Toolkit rendering may remain degraded."
            );
            return null;
        }

        private static Font ResolveRuntimeFont()
        {
            Font runtimeFont = Resources.GetBuiltinResource<Font>("LegacyRuntime.ttf");
            if (runtimeFont != null)
            {
                return runtimeFont;
            }

            runtimeFont = Resources.GetBuiltinResource<Font>("Arial.ttf");
            if (runtimeFont != null)
            {
                Debug.LogWarning(
                    "Unity Observatory is using Arial.ttf because LegacyRuntime.ttf was not available in this editor path."
                );
                return runtimeFont;
            }

            Debug.LogWarning(
                "Unity Observatory could not resolve a built-in runtime font for the compatibility canvas."
            );
            return null;
        }
    }
}
