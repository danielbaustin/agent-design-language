using System;
using System.Collections;
using System.Reflection;
using ADL.Demos.UnityObservatory;
using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;
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
                string packetSchema = root.Q<Label>("packet-schema")?.text ?? "unknown";
                string packetRef = root.Q<Label>("packet-ref")?.text ?? "unknown";
                string artifactRoot = root.Q<Label>("artifact-root")?.text ?? "unknown";
                string reportRef = root.Q<Label>("report-ref")?.text ?? "unknown";
                string packetNote = root.Q<Label>("packet-note")?.text ?? "unknown";
                string demoOperatorBoundary =
                    root.Q<Label>("demo-operator-boundary")?.text ?? "unknown";
                string demoNextStep = root.Q<Label>("demo-next-step")?.text ?? "unknown";

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

                Debug.Log(
                    $"Unity Observatory compatibility verification passed. rootChildren={root.childCount}; title={title}; packetSchema={packetSchema}; packetRef={packetRef}; artifactRoot={artifactRoot}; reportRef={reportRef}; demoBoundary={demoOperatorBoundary}; demoNextStep={demoNextStep}"
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
            if (!System.IO.File.Exists(FlagshipScenePath))
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

                UIDocument document = shellObject.GetComponent<UIDocument>();
                if (document == null || document.panelSettings == null)
                {
                    throw new InvalidOperationException(
                        "Unity Observatory flagship validation did not create a UIDocument with PanelSettings."
                    );
                }

                VisualElement root = document.rootVisualElement;
                if (root == null || root.childCount == 0)
                {
                    throw new InvalidOperationException(
                        "Unity Observatory flagship validation built an empty root visual tree."
                    );
                }

                Label title = root.Q<Label>("title");
                Label packetRef = root.Q<Label>("packet-ref");
                Label observabilityTitle = root.Q<Label>("observability-title");
                Label demoSurfaceTitle = root.Q<Label>("demo-surface-title");
                Label demoPolisState = root.Q<Label>("demo-polis-state");
                Label demoOperatorBoundary = root.Q<Label>("demo-operator-boundary");
                Label demoNextStep = root.Q<Label>("demo-next-step");

                if (title == null ||
                    packetRef == null ||
                    observabilityTitle == null ||
                    demoSurfaceTitle == null ||
                    demoPolisState == null ||
                    demoOperatorBoundary == null ||
                    demoNextStep == null)
                {
                    throw new InvalidOperationException(
                        "Unity Observatory flagship validation did not find the runtime shell, packet, observability, and polis handoff labels."
                    );
                }

                if (!root.styleSheets.Contains(runtimeStyleSheet))
                {
                    throw new InvalidOperationException(
                        "Unity Observatory flagship validation did not attach the runtime stylesheet to the root visual element."
                    );
                }

                if (!demoOperatorBoundary.text.Contains("proposal-only", StringComparison.Ordinal))
                {
                    throw new InvalidOperationException(
                        $"Unity Observatory flagship validation expected proposal-only demo boundary but observed '{demoOperatorBoundary.text}'."
                    );
                }

                if (!demoNextStep.text.Contains("#4704", StringComparison.Ordinal))
                {
                    throw new InvalidOperationException(
                        $"Unity Observatory flagship validation expected #4704 handoff marker but observed '{demoNextStep.text}'."
                    );
                }

                Debug.Log(
                    $"Unity Observatory flagship shell verification passed. scene={scene.name}; rootChildren={root.childCount}; title={title.text}; packetRef={packetRef.text}; observabilityTitle={observabilityTitle.text}; demoSurfaceTitle={demoSurfaceTitle.text}; polisState={demoPolisState.text}; demoBoundary={demoOperatorBoundary.text}; demoNextStep={demoNextStep.text}"
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
    }
}
