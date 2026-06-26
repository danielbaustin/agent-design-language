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
        private const string ContractResourcePath = "observatory_contract";
        private const string ThemeResourcePath = "UnityDefaultRuntimeTheme";
        private const string RuntimeStyleSheetResourcePath = "ObservatoryShellRuntime";
        private const string ShellObjectName = "Unity Observatory Shell";

        public static void ValidateScene()
        {
            Scene scene = EditorSceneManager.OpenScene(ScenePath);

            UnityObservatoryBootstrap bootstrap = null;
            foreach (GameObject rootObject in scene.GetRootGameObjects())
            {
                bootstrap = rootObject.GetComponentInChildren<UnityObservatoryBootstrap>(true);
                if (bootstrap != null)
                {
                    break;
                }
            }

            if (bootstrap == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory validation could not find UnityObservatoryBootstrap in the scene."
                );
            }

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

                if (!root.styleSheets.Contains(runtimeStyleSheet))
                {
                    throw new InvalidOperationException(
                        "Unity Observatory validation did not attach the runtime stylesheet to the root visual element."
                    );
                }

                string title = root.Q<Label>("title")?.text ?? "unknown";
                string packetSchema = root.Q<Label>("packet-schema")?.text ?? "unknown";
                Debug.Log(
                    $"Unity Observatory compatibility verification passed. rootChildren={root.childCount}; title={title}; packetSchema={packetSchema}"
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
