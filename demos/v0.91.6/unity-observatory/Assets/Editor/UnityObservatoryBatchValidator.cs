using System;
using System.Collections;
using System.IO;
using System.Reflection;
using ADL.Demos.UnityObservatory;
using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.UIElements;
using UnityEngine.SceneManagement;

namespace ADL.Demos.UnityObservatory.Editor
{
    public static class UnityObservatoryBatchValidator
    {
        private const string ScenePath = "Assets/Scenes/UnityObservatory.unity";
        private const string FlagshipScenePath = "Assets/Scenes/FlagshipObservatoryStage.unity";
        private const string ContractResourcePath = "observatory_contract";
        private const string ThemeResourcePath = "UnityDefaultRuntimeTheme";
        private const string RuntimeStyleSheetResourcePath = "ObservatoryShellRuntime";
        private const string ShellObjectName = "Unity Observatory Shell";
        private const string ExpectedTitleEnvVar = "ADL_UNITY_EXPECTED_TITLE";
        private const string ExpectedPacketRefEnvVar = "ADL_UNITY_EXPECTED_PACKET_REF";
        private const string ExpectedArtifactRootEnvVar = "ADL_UNITY_EXPECTED_ARTIFACT_ROOT";
        private const string ExpectedReportRefEnvVar = "ADL_UNITY_EXPECTED_REPORT_REF";
        private const string ExpectedEvidenceLevelEnvVar = "ADL_UNITY_EXPECTED_EVIDENCE_LEVEL";

        public static void ValidateScene()
        {
            Scene scene = EditorSceneManager.OpenScene(ScenePath);

            UnityObservatoryBootstrap bootstrap = FindBootstrap(scene);

            TextAsset contractAsset = Resources.Load<TextAsset>(ContractResourcePath);
            if (contractAsset == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation could not load Resources/observatory_contract.json."
                );
            }

            ThemeStyleSheet theme = Resources.Load<ThemeStyleSheet>(ThemeResourcePath);
            if (theme == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation could not load Resources/UnityDefaultRuntimeTheme.tss."
                );
            }

            StyleSheet runtimeStyleSheet = Resources.Load<StyleSheet>(RuntimeStyleSheetResourcePath);
            if (runtimeStyleSheet == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation could not load Resources/ObservatoryShellRuntime.uss."
                );
            }

            try
            {
                RunBootstrapPath(bootstrap);

                GameObject shellObject = GameObject.Find(ShellObjectName);
                if (shellObject == null)
                {
                    throw new InvalidOperationException(
                        "Unity Observatory validation did not create the runtime shell object."
                    );
                }

                if (UsesCompatibilityCanvas())
                {
                    ValidateCompatibilityCanvas(shellObject);
                    Debug.Log(
                        "Unity Observatory compatibility validation passed for the runtime shell."
                    );
                }
                else
                {
                    ValidateUiToolkitShell(shellObject, runtimeStyleSheet);
                }

                ValidateFlagshipScene();

                Debug.Log(
                    "Unity Observatory batch validation passed for the shell and flagship environment."
                );
            }
            finally
            {
                GameObject shellObject = GameObject.Find(ShellObjectName);
                if (shellObject != null)
                {
                    UnityEngine.Object.DestroyImmediate(shellObject);
                }
            }
        }

        public static void ValidateFlagshipShellScene()
        {
            if (!File.Exists(FlagshipScenePath))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory flagship validation requires '{FlagshipScenePath}'. Run this validator only in a staged flagship project."
                );
            }

            Scene scene = EditorSceneManager.OpenScene(FlagshipScenePath);
            UnityObservatoryBootstrap bootstrap = FindBootstrap(scene);

            StyleSheet runtimeStyleSheet = Resources.Load<StyleSheet>(RuntimeStyleSheetResourcePath);
            if (runtimeStyleSheet == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory flagship validation could not load Resources/ObservatoryShellRuntime.uss."
                );
            }

            try
            {
                RunBootstrapPath(bootstrap);

                GameObject shellObject = GameObject.Find(ShellObjectName);
                if (shellObject == null)
                {
                    throw new InvalidOperationException(
                        "Unity Observatory flagship validation did not create the runtime shell object."
                    );
                }

                if (UsesCompatibilityCanvas())
                {
                    ValidateCompatibilityCanvas(shellObject);
                    Debug.Log(
                        "Unity Observatory flagship compatibility validation passed for the runtime shell."
                    );
                }
                else
                {
                    ValidateUiToolkitShell(shellObject, runtimeStyleSheet);
                }

                UnityObservatoryFlagshipStageBuilder.ValidateFlagshipStage(scene);
                Debug.Log(
                    $"Unity Observatory flagship shell verification passed. scene={scene.name}"
                );
            }
            finally
            {
                GameObject shellObject = GameObject.Find(ShellObjectName);
                if (shellObject != null)
                {
                    UnityEngine.Object.DestroyImmediate(shellObject);
                }
            }
        }

        private static void ValidateUiToolkitShell(
            GameObject shellObject,
            StyleSheet runtimeStyleSheet
        )
        {
            UIDocument document = shellObject.GetComponent<UIDocument>();
            if (document == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not attach UIDocument to the runtime shell."
                );
            }

            if (document.panelSettings == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not create runtime PanelSettings."
                );
            }

            if (document.panelSettings.themeStyleSheet == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not attach a runtime theme stylesheet."
                );
            }

            VisualElement root = document.rootVisualElement;

            if (root == null || root.childCount == 0)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation built an empty root visual tree."
                );
            }

            if (root.Q<Label>("title") == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not find the title label in the built shell."
                );
            }

            if (root.Q<Label>("packet-schema") == null || root.Q<Label>("packet-ref") == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not find the packet contract labels in the built shell."
                );
            }

            if (root.Q<Label>("observability-title") == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not find the observability card in the built shell."
                );
            }

            if (root.Q<Label>("demo-surface-title") == null ||
                root.Q<Label>("demo-polis-state") == null ||
                root.Q<Label>("demo-operator-boundary") == null ||
                root.Q<Label>("demo-next-step") == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not find the runtime polis demo-surface strip."
                );
            }

            if (!root.styleSheets.Contains(runtimeStyleSheet))
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not attach the runtime stylesheet to the root visual element."
                );
            }

            string title = root.Q<Label>("title")?.text ?? "unknown";
            string packetRef = root.Q<Label>("packet-ref")?.text ?? "unknown";
            string artifactRoot = root.Q<Label>("artifact-root")?.text ?? "unknown";
            string reportRef = root.Q<Label>("report-ref")?.text ?? "unknown";
            string packetNote = root.Q<Label>("packet-note")?.text ?? "unknown";
            string demoOperatorBoundary =
                root.Q<Label>("demo-operator-boundary")?.text ?? "unknown";
            string demoNextStep = root.Q<Label>("demo-next-step")?.text ?? "unknown";

            AssertContractExpectations(title, packetRef, artifactRoot, reportRef, packetNote);

            if (!demoOperatorBoundary.Contains("proposal-only", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory validation expected proposal-only demo boundary but observed '{demoOperatorBoundary}'."
                );
            }

            if (!demoNextStep.Contains("#4704", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory validation expected #4704 handoff marker but observed '{demoNextStep}'."
                );
            }
        }

        private static void ValidateCompatibilityCanvas(GameObject shellObject)
        {
            Canvas canvas = shellObject.GetComponent<Canvas>();
            if (canvas == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation expected the compatibility path to create a Canvas."
                );
            }

            Transform panelTransform = shellObject.transform.Find("Observatory Compatibility Panel");
            if (panelTransform == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not create the compatibility panel."
                );
            }

            UnityEngine.UI.Text text = panelTransform.GetComponentInChildren<UnityEngine.UI.Text>();
            if (text == null || string.IsNullOrWhiteSpace(text.text))
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation did not create populated compatibility text."
                );
            }

            string expectedTitle = Environment.GetEnvironmentVariable(ExpectedTitleEnvVar);
            if (!string.IsNullOrWhiteSpace(expectedTitle) &&
                !text.text.Contains(expectedTitle, StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory validation expected compatibility text to contain title '{expectedTitle}'."
                );
            }

            string expectedPacketRef = Environment.GetEnvironmentVariable(
                ExpectedPacketRefEnvVar
            );
            if (!string.IsNullOrWhiteSpace(expectedPacketRef) &&
                !text.text.Contains(expectedPacketRef, StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory validation expected compatibility text to contain packet ref '{expectedPacketRef}'."
                );
            }
        }

        private static void AssertContractExpectations(
            string title,
            string packetRef,
            string artifactRoot,
            string reportRef,
            string packetNote
        )
        {
            AssertMatchesExpectation(
                "title",
                title,
                Environment.GetEnvironmentVariable(ExpectedTitleEnvVar)
            );
            AssertMatchesExpectation(
                "packet-ref",
                packetRef,
                Environment.GetEnvironmentVariable(ExpectedPacketRefEnvVar)
            );
            AssertMatchesExpectation(
                "artifact-root",
                artifactRoot,
                Environment.GetEnvironmentVariable(ExpectedArtifactRootEnvVar)
            );
            AssertMatchesExpectation(
                "report-ref",
                reportRef,
                Environment.GetEnvironmentVariable(ExpectedReportRefEnvVar)
            );

            string expectedEvidenceLevel = Environment.GetEnvironmentVariable(
                ExpectedEvidenceLevelEnvVar
            );
            if (!string.IsNullOrWhiteSpace(expectedEvidenceLevel) &&
                !packetNote.Contains(expectedEvidenceLevel, StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory validation expected packet-note to contain '{expectedEvidenceLevel}' but observed '{packetNote}'."
                );
            }
        }

        private static void ValidateFlagshipScene()
        {
            if (!File.Exists(FlagshipScenePath))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory validation could not find submitted flagship scene at {FlagshipScenePath}."
                );
            }

            Scene scene = EditorSceneManager.OpenScene(FlagshipScenePath);
            UnityObservatoryFlagshipStageBuilder.ValidateFlagshipStage(scene);
        }

        private static UnityObservatoryBootstrap FindBootstrap(Scene scene)
        {
            foreach (GameObject rootObject in scene.GetRootGameObjects())
            {
                UnityObservatoryBootstrap bootstrap =
                    rootObject.GetComponentInChildren<UnityObservatoryBootstrap>(true);
                if (bootstrap != null)
                {
                    return bootstrap;
                }
            }

            throw new InvalidOperationException(
                $"Unity Observatory validation could not find UnityObservatoryBootstrap in scene '{scene.path}'."
            );
        }

        private static void AssertMatchesExpectation(
            string label,
            string observed,
            string expected
        )
        {
            if (!string.IsNullOrWhiteSpace(expected) &&
                !string.Equals(observed, expected, StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory validation expected {label} '{expected}' but observed '{observed}'."
                );
            }
        }

        private static void RunBootstrapPath(UnityObservatoryBootstrap bootstrap)
        {
            MethodInfo createShell = typeof(UnityObservatoryBootstrap).GetMethod(
                "CreateObservatoryShell",
                BindingFlags.Instance | BindingFlags.NonPublic
            );
            if (createShell == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation could not find CreateObservatoryShell on the bootstrap."
                );
            }

            IEnumerator routine = createShell.Invoke(bootstrap, null) as IEnumerator;
            if (routine == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation could not invoke the bootstrap coroutine."
                );
            }

            while (routine.MoveNext())
            {
                // Step the bounded coroutine to completion so the runtime shell is built
                // under the same code path used by Play mode.
            }
        }

        private static bool UsesCompatibilityCanvas()
        {
            MethodInfo method = typeof(UnityObservatoryBootstrap).GetMethod(
                "ShouldUseCompatibilityCanvas",
                BindingFlags.NonPublic | BindingFlags.Static
            );
            if (method == null)
            {
                throw new MissingMethodException(
                    typeof(UnityObservatoryBootstrap).FullName,
                    "ShouldUseCompatibilityCanvas"
                );
            }

            return (bool)method.Invoke(null, null);
        }
    }
}
