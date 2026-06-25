using System;
using System.Reflection;
using ADL.Demos.UnityObservatory;
using UnityEditor;
using UnityEngine;
using UnityEngine.UI;

namespace ADL.Demos.UnityObservatory.Editor
{
    public static class UnityObservatoryCompatibilityVerifier
    {
        private const string ContractResourcePath = "observatory_contract";
        private const int ExpectedSortingOrder = 10;

        [MenuItem("ADL/Observatory/Verify Compatibility Canvas")]
        public static void Run()
        {
            GameObject shellObject = new("Unity Observatory Compatibility Verification Shell");
            UnityObservatoryShellController controller =
                shellObject.AddComponent<UnityObservatoryShellController>();

            try
            {
                TextAsset contractAsset = Resources.Load<TextAsset>(ContractResourcePath);
                if (contractAsset != null)
                {
                    controller.ConfigureFromContract(contractAsset.text);
                }
                else
                {
                    controller.ConfigureFallback(
                        "adl.csm_visibility_packet.v1",
                        "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json",
                        3,
                        2,
                        "World / Reality",
                        "Operator lens"
                    );
                }

                bool shouldUseCompatibilityCanvas = InvokeCompatibilityDecision();
                if (!shouldUseCompatibilityCanvas)
                {
                    throw new InvalidOperationException(
                        "Compatibility verifier expected the 2022.3.x editor path to require the uGUI compatibility canvas."
                    );
                }

                InvokeCompatibilityCanvasBuild(shellObject, controller);

                Canvas canvas = shellObject.GetComponent<Canvas>();
                if (canvas == null)
                {
                    throw new InvalidOperationException(
                        "Compatibility verifier did not create a Canvas component."
                    );
                }

                Transform panelTransform = shellObject.transform.Find(
                    "Observatory Compatibility Panel"
                );
                if (panelTransform == null)
                {
                    throw new InvalidOperationException(
                        "Compatibility verifier did not create the compatibility panel object."
                    );
                }

                Text compatibilityText = panelTransform.GetComponentInChildren<Text>();
                if (compatibilityText == null)
                {
                    throw new InvalidOperationException(
                        "Compatibility verifier did not create a Text component."
                    );
                }

                if (string.IsNullOrWhiteSpace(compatibilityText.text))
                {
                    throw new InvalidOperationException(
                        "Compatibility verifier created an empty compatibility text payload."
                    );
                }

                if (canvas.sortingOrder != ExpectedSortingOrder)
                {
                    throw new InvalidOperationException(
                        $"Compatibility verifier expected Canvas.sortingOrder={ExpectedSortingOrder}, found {canvas.sortingOrder}."
                    );
                }

                Debug.Log(
                    $"Unity Observatory compatibility verification passed. shouldUseCompatibilityCanvas={shouldUseCompatibilityCanvas}; textLength={compatibilityText.text.Length}; sortingOrder={canvas.sortingOrder}"
                );
                ExitIfBatchMode(0);
            }
            catch (Exception error)
            {
                Debug.LogError(
                    $"Unity Observatory compatibility verification failed: {error}"
                );
                ExitIfBatchMode(1);
                throw;
            }
            finally
            {
                UnityEngine.Object.DestroyImmediate(shellObject);
            }
        }

        private static bool InvokeCompatibilityDecision()
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

        private static void InvokeCompatibilityCanvasBuild(
            GameObject shellObject,
            UnityObservatoryShellController controller
        )
        {
            MethodInfo method = typeof(UnityObservatoryBootstrap).GetMethod(
                "CreateCompatibilityCanvas",
                BindingFlags.NonPublic | BindingFlags.Static
            );
            if (method == null)
            {
                throw new MissingMethodException(
                    typeof(UnityObservatoryBootstrap).FullName,
                    "CreateCompatibilityCanvas"
                );
            }

            method.Invoke(null, new object[] { shellObject, controller });
        }

        private static void ExitIfBatchMode(int code)
        {
            if (Application.isBatchMode)
            {
                EditorApplication.Exit(code);
            }
        }
    }
}
