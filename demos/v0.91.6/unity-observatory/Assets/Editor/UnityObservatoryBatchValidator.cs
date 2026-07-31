using System;
using System.Collections;
using System.IO;
using System.Reflection;
using ADL.Demos.UnityObservatory;
using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;
using UnityEngine.Networking;
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

            if (root.Q<Label>("runtime-mode") != null)
            {
                ValidateCommandRoomShell(shellObject, root, runtimeStyleSheet);
                return;
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

        private static void ValidateCommandRoomShell(
            GameObject shellObject,
            VisualElement root,
            StyleSheet runtimeStyleSheet
        )
        {
            string[] requiredLabels =
            {
                "command-product-title",
                "runtime-mode",
                "runtime-mirror",
                "readiness-metric-value",
                "inspector-title",
                "inspector-state",
                "communication-state",
            };
            foreach (string labelName in requiredLabels)
            {
                if (root.Q<Label>(labelName) == null)
                {
                    throw new InvalidOperationException(
                        $"Unity Observatory command-room validation did not find label '{labelName}'."
                    );
                }
            }

            string[] requiredButtons =
            {
                "nav-runtime",
                "nav-agents",
                "nav-events",
                "nav-governance",
                "nav-evidence",
            };
            foreach (string buttonName in requiredButtons)
            {
                UnityEngine.UIElements.Button button =
                    root.Q<UnityEngine.UIElements.Button>(buttonName);
                if (button == null || button.Q<Label>($"{buttonName}-icon") == null)
                {
                    throw new InvalidOperationException(
                        $"Unity Observatory command-room validation did not find icon button '{buttonName}'."
                    );
                }
            }
            if (
                root.Q<ScrollView>("event-stream-scroll") == null
                || root.Q<ScrollView>("inspector-scroll") == null
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory command-room validation expected bounded event and inspector scroll surfaces."
                );
            }

            if (!root.styleSheets.Contains(runtimeStyleSheet))
            {
                throw new InvalidOperationException(
                    "Unity Observatory command-room validation did not attach the runtime stylesheet."
                );
            }

            string productTitle = root.Q<Label>("command-product-title")?.text ?? "unknown";
            if (!string.Equals(productTitle, "ADL OBSERVATORY", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory command-room validation observed unexpected product title '{productTitle}'."
                );
            }

            string mode = root.Q<Label>("runtime-mode")?.text ?? "unknown";
            string mirror = root.Q<Label>("runtime-mirror")?.text ?? "unknown";
            if (
                !string.Equals(mode, "DEMO DATA", StringComparison.Ordinal)
                || !string.Equals(mirror, "CONTRACT ONLY", StringComparison.Ordinal)
            )
            {
                throw new InvalidOperationException(
                    $"Unity Observatory command-room validation expected truthful fixture mode but observed '{mode}' and '{mirror}'."
                );
            }

            string communication =
                root.Q<Label>("communication-state")?.text ?? "unknown";
            if (!communication.Contains("no send authority", StringComparison.OrdinalIgnoreCase))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory command-room validation expected fail-closed communication state but observed '{communication}'."
                );
            }

            UnityObservatoryShellController controller =
                shellObject.GetComponent<UnityObservatoryShellController>();
            if (controller == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory command-room validation did not find its shell controller."
                );
            }

            MethodInfo selectProjection =
                typeof(UnityObservatoryShellController).GetMethod(
                    "SelectProjection",
                    BindingFlags.Instance | BindingFlags.NonPublic
                );
            if (selectProjection == null)
            {
                throw new InvalidOperationException(
                    "Unity Observatory command-room validation could not resolve projection navigation."
                );
            }
            selectProjection.Invoke(controller, new object[] { "Agents" });
            UnityEngine.UIElements.Button activeAgentsButton =
                root.Q<UnityEngine.UIElements.Button>("nav-agents");
            if (
                root.Q<Label>("inspector-title")?.text != "Agents"
                || activeAgentsButton == null
                || activeAgentsButton.style.backgroundColor.value
                    != new Color(0.03f, 0.24f, 0.34f, 0.88f)
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory command-room validation expected navigation selection and active icon state to update together."
                );
            }
            selectProjection.Invoke(controller, new object[] { "Runtime" });

            AssertContractExpectations(
                ReadPrivateString(controller, "title"),
                ReadPrivateString(controller, "packetRef"),
                ReadPrivateString(controller, "runtimeArtifactRoot"),
                ReadPrivateString(controller, "operatorReportRef"),
                ReadPrivateString(controller, "evidenceLevel")
            );
            ValidateRuntimeAdapterBehavior(controller, root);

            string sendBoundary =
                UnityObservatoryShellController.DescribeCommunicationAttempt("Demo");
            if (!sendBoundary.Contains("NOT SENT", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    "Unity Observatory command-room validation expected the operator action to fail closed."
                );
            }
        }

        private static void ValidateRuntimeAdapterBehavior(
            UnityObservatoryShellController controller,
            VisualElement root
        )
        {
            const string status =
                "{\"schema\":\"adl.csm.runtime_api.status.v1\",\"status\":\"healthy\",\"ready\":\"ready\",\"runtime_owner\":\"csm\",\"agent_instance_id\":\"main-csm-runtime\",\"agent_status\":{\"state\":\"idle\",\"completed_cycle_count\":32417}}";
            const string health =
                "{\"schema\":\"adl.csm.runtime_api.health.v1\",\"status\":\"healthy\",\"runtime_owner\":\"csm\",\"agent_instance_id\":\"main-csm-runtime\"}";
            const string unhealthyHealth =
                "{\"schema\":\"adl.csm.runtime_api.health.v1\",\"status\":\"degraded\",\"runtime_owner\":\"csm\",\"agent_instance_id\":\"main-csm-runtime\"}";
            const string ready =
                "{\"schema\":\"adl.csm.runtime_api.ready.v1\",\"ready\":\"ready\",\"runtime_owner\":\"csm\",\"agent_instance_id\":\"main-csm-runtime\",\"blocking_reasons\":[]}";
            const string metrics =
                "{\"schema\":\"adl.csm.runtime_api.metrics.v1\",\"runtime_owner\":\"csm\",\"agent_instance_id\":\"main-csm-runtime\",\"gauges\":{\"completed_cycle_count\":32417,\"operator_event_count_observed\":138488},\"states\":{\"agent_state\":\"idle\",\"health\":\"healthy\",\"ready\":\"ready\"}}";
            const string events =
                "{\"schema\":\"adl.csm.runtime_api.events.v1\",\"runtime_owner\":\"csm\",\"agent_instance_id\":\"main-csm-runtime\",\"events\":{\"status\":\"serialized\",\"tail_limit\":40,\"unreadable_lines\":0,\"entries\":[{\"agent_instance_id\":\"main-csm-runtime\",\"at\":\"2026-07-10T10:14:50Z\",\"event\":\"checkpoint_write\",\"operator\":\"local\",\"schema\":\"adl.long_lived_agent_operator_event.v1\"}]}}";
            const string runtimeV3Feed =
                "{\"schema\":\"adl.runtime_v3.observatory_feed.v2\",\"runtime_instance_id\":\"runtime-v3-main\",\"runtime_selection\":\"runtime_v3_explicit_opt_in\",\"control\":{\"read_endpoint\":\"/v1/observatory\",\"websocket_endpoint\":\"/v1/observatory/ws\",\"signed_command_endpoint\":\"/v1/control\",\"signed_commands_required_for_mutation\":true,\"bearer_token_required_for_read\":true,\"browser_mutation_authority\":false},\"health\":{\"observability_ready\":true,\"snapshot\":{\"revision\":72,\"topology_generation\":9,\"event_count\":2,\"lifecycle\":\"Running\",\"observability\":\"Healthy\",\"observability_ready\":true}},\"agents\":{\"total_count\":6,\"rendered_sample_count\":1,\"sample\":[{\"id\":\"agent-0001\",\"label\":\"Owner Agent\",\"role\":\"runtime owner\",\"state\":\"running\",\"detail\":\"sample 1 of 6\"}]},\"proof\":{\"default_runtime_switch_authorized\":false,\"runtime_v2_decommission_authorized\":false,\"sidecar_required\":false,\"vector_cloudwatch_route\":\"vector.runtime_v3_cloudwatch_emf\"},\"continuity\":{\"checkpoint\":{\"generation\":12,\"accepted_through\":71,\"integrity\":\"verified\"}},\"events\":[{\"sequence\":71,\"monotonic_millis\":9001,\"component\":\"owner\",\"event\":\"checkpoint_write\",\"correlation_id\":\"cycle-71\"},{\"sequence\":72,\"monotonic_millis\":9015,\"component\":\"scheduler\",\"event\":\"cycle_completed\",\"correlation_id\":\"cycle-72\"}]}";

            string live = UnityObservatoryShellController.ClassifyRuntimeSnapshot(
                5,
                status,
                health,
                ready,
                metrics,
                events
            );
            if (!string.Equals(live, "Live", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    $"Unity Observatory runtime adapter expected a complete healthy contract to be Live but observed '{live}'."
                );
            }

            string unhealthy = UnityObservatoryShellController.ClassifyRuntimeSnapshot(
                5,
                status,
                unhealthyHealth,
                ready,
                metrics,
                events
            );
            if (!string.Equals(unhealthy, "Degraded", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter unhealthy CSM contract must degrade."
                );
            }

            string incomplete = UnityObservatoryShellController.ClassifyRuntimeSnapshot(
                4,
                status,
                health,
                ready,
                metrics,
                events
            );
            if (!string.Equals(incomplete, "Degraded", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter incomplete endpoint set must degrade."
                );
            }

            string malformed = UnityObservatoryShellController.ClassifyRuntimeSnapshot(
                5,
                status,
                health,
                ready,
                metrics,
                "{\"schema\":\"unexpected\"}"
            );
            if (!string.Equals(malformed, "Degraded", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter malformed schema must degrade."
                );
            }

            string wrongVersion = UnityObservatoryShellController.ClassifyRuntimeSnapshot(
                5,
                status,
                health,
                ready,
                metrics,
                events.Replace(
                    "adl.csm.runtime_api.events.v1",
                    "adl.csm.runtime_api.events.vgarbage"
                )
            );
            if (!string.Equals(wrongVersion, "Degraded", StringComparison.Ordinal))
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter must reject non-exact CSM schema versions."
                );
            }

            string runtimeV3Live =
                UnityObservatoryShellController.ClassifyRuntimeV3Feed(
                    200,
                    runtimeV3Feed,
                    false,
                    true
                );
            string runtimeV3Unauthorized =
                UnityObservatoryShellController.ClassifyRuntimeV3Feed(
                    401,
                    string.Empty,
                    false,
                    true
                );
            string runtimeV3MissingToken =
                UnityObservatoryShellController.ClassifyRuntimeV3Feed(
                    200,
                    runtimeV3Feed,
                    false,
                    false
                );
            string runtimeV3WrongControlEndpoint =
                UnityObservatoryShellController.ClassifyRuntimeV3Feed(
                    200,
                    runtimeV3Feed.Replace("/v1/control", "/v1/hostile-control"),
                    false,
                    true
                );
            if (
                runtimeV3Live != "Live"
                || runtimeV3Unauthorized != "Degraded"
                || runtimeV3MissingToken != "Degraded"
                || runtimeV3WrongControlEndpoint != "Degraded"
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory Runtime v3 feed classifier did not enforce exact schema, bearer authentication, and readiness."
                );
            }

            using (
                UnityWebRequest authRequest = UnityWebRequest.Get(
                    "https://runtime.example.test/v1/observatory"
                )
            )
            {
                UnityObservatoryShellController.ConfigureRuntimeV3Request(
                    authRequest,
                    "validator-token"
                );
                if (
                    authRequest.GetRequestHeader("Authorization")
                    != "Bearer validator-token"
                )
                {
                    throw new InvalidOperationException(
                        "Unity Observatory Runtime v3 probe did not construct its bearer authorization header."
                    );
                }
            }
            if (
                UnityObservatoryShellController.IsRuntimeTransportFailureForProof(
                    UnityWebRequest.Result.ProtocolError
                )
                || !UnityObservatoryShellController.IsRuntimeTransportFailureForProof(
                    UnityWebRequest.Result.ConnectionError
                )
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory Runtime v3 probe must distinguish responding HTTP protocol errors from transport disconnection."
                );
            }

            if (
                !UnityObservatoryShellController.TryNormalizeLoopbackEndpoint(
                    "http://127.0.0.1:19997",
                    out string normalized,
                    out _
                )
                || !string.Equals(
                    normalized,
                    "http://127.0.0.1:19997",
                    StringComparison.Ordinal
                )
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter expected loopback endpoint acceptance."
                );
            }

            if (
                UnityObservatoryShellController.TryNormalizeLoopbackEndpoint(
                    "https://example.com",
                    out _,
                    out _
                )
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory legacy CSM adapter external endpoint must be rejected."
                );
            }

            if (
                !UnityObservatoryShellController.TryNormalizeRuntimeEndpointForProof(
                    "https://runtime.example.test",
                    out string runtimeV3Origin,
                    out string transportKind,
                    out _
                )
                || runtimeV3Origin != "https://runtime.example.test"
                || transportKind != "RuntimeV3"
                || UnityObservatoryShellController.TryNormalizeRuntimeEndpointForProof(
                    "http://runtime.example.test",
                    out _,
                    out _,
                    out _
                )
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime endpoint policy must accept HTTPS Runtime v3 origins and reject external plaintext origins."
                );
            }

            MethodInfo ingest = typeof(UnityObservatoryShellController).GetMethod(
                "IngestRuntimeDocuments",
                BindingFlags.Instance | BindingFlags.NonPublic
            );
            MethodInfo applyTruth = typeof(UnityObservatoryShellController).GetMethod(
                "ApplyRuntimeTruth",
                BindingFlags.Instance | BindingFlags.NonPublic
            );
            MethodInfo ingestRuntimeV3 =
                typeof(UnityObservatoryShellController).GetMethod(
                    "IngestRuntimeV3Feed",
                    BindingFlags.Instance | BindingFlags.NonPublic
                );
            Type truthMode = typeof(UnityObservatoryShellController).GetNestedType(
                "RuntimeTruthMode",
                BindingFlags.NonPublic
            );
            FieldInfo endpointCount = typeof(UnityObservatoryShellController).GetField(
                "runtimeSuccessfulEndpointCount",
                BindingFlags.Instance | BindingFlags.NonPublic
            );
            FieldInfo runtimeTransportKindField =
                typeof(UnityObservatoryShellController).GetField(
                "runtimeTransportKind",
                BindingFlags.Instance | BindingFlags.NonPublic
            );
            Type transportKindType = typeof(UnityObservatoryShellController).GetNestedType(
                "RuntimeTransportKind",
                BindingFlags.NonPublic
            );
            MethodInfo selectRuntimeProjection =
                typeof(UnityObservatoryShellController).GetMethod(
                    "SelectProjection",
                    BindingFlags.Instance | BindingFlags.NonPublic
                );
            if (
                ingest == null
                || ingestRuntimeV3 == null
                || applyTruth == null
                || truthMode == null
                || endpointCount == null
                || runtimeTransportKindField == null
                || transportKindType == null
                || selectRuntimeProjection == null
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter behavior verifier could not resolve final adapter members."
                );
            }

            bool parsed =
                ingest.Invoke(
                    controller,
                    new object[] { status, health, ready, metrics, events }
                ) is true;
            if (!parsed)
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter could not parse its published contract shapes."
                );
            }

            endpointCount.SetValue(controller, 5);
            applyTruth.Invoke(
                controller,
                new object[]
                {
                    Enum.Parse(truthMode, "Live"),
                    "CSM healthy and ready; 5/5 validated runtime contracts",
                }
            );
            if (
                root.Q<Label>("runtime-mode")?.text != "LIVE"
                || root.Q<Label>("event-row-0-name")?.text != "checkpoint_write"
                || root.Q<Label>("event-row-0-source")?.text != "main-csm-runtime"
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter did not replace fixture event rows with observed runtime events."
                );
            }

            bool parsedRuntimeV3 =
                ingestRuntimeV3.Invoke(controller, new object[] { runtimeV3Feed }) is true;
            if (!parsedRuntimeV3)
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter could not parse the Runtime v3 Observatory feed."
                );
            }

            runtimeTransportKindField.SetValue(
                controller,
                Enum.Parse(transportKindType, "RuntimeV3")
            );
            endpointCount.SetValue(controller, 1);
            applyTruth.Invoke(
                controller,
                new object[]
                {
                    Enum.Parse(truthMode, "Live"),
                    "Runtime v3 Observatory feed authenticated and ready",
                }
            );
            selectRuntimeProjection.Invoke(controller, new object[] { "Agents" });
            if (
                root.Q<Label>("runtime-mode")?.text != "LIVE"
                || root.Q<Label>("inspector-state")?.text
                    != "agent-0001 / running"
                || !(root.Q<Label>("inspector-evidence")?.text ?? string.Empty).Contains(
                    "Owner Agent",
                    StringComparison.Ordinal
                )
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory Runtime v3 agent projection did not apply its parsed Live state."
                );
            }
            selectRuntimeProjection.Invoke(controller, new object[] { "Evidence" });
            string runtimeV3Proof =
                root.Q<Label>("inspector-evidence")?.text ?? string.Empty;
            if (
                !runtimeV3Proof.Contains(
                    "switch not authorized",
                    StringComparison.Ordinal
                )
                || !(root.Q<Label>("inspector-room")?.text ?? string.Empty).Contains(
                    "checkpoint generation 12",
                    StringComparison.Ordinal
                )
                || root.Q<Label>("inspector-lens")?.text
                    != "vector.runtime_v3_cloudwatch_emf"
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory Runtime v3 proof, continuity, and CloudWatch-route projection was not observable."
                );
            }

            applyTruth.Invoke(
                controller,
                new object[]
                {
                    Enum.Parse(truthMode, "Demo"),
                    "No runtime endpoint configured; contract fixture is displayed.",
                }
            );
            if (
                root.Q<Label>("runtime-mode")?.text != "DEMO DATA"
                || root.Q<Label>("event-row-0-name")?.text != "contract.loaded"
            )
            {
                throw new InvalidOperationException(
                    "Unity Observatory runtime adapter did not restore explicit fixture truth."
                );
            }
        }

        private static string ReadPrivateString(
            UnityObservatoryShellController controller,
            string fieldName
        )
        {
            FieldInfo field = typeof(UnityObservatoryShellController).GetField(
                fieldName,
                BindingFlags.Instance | BindingFlags.NonPublic
            );
            if (field?.GetValue(controller) is string value)
            {
                return value;
            }

            throw new InvalidOperationException(
                $"Unity Observatory command-room validation could not read contract field '{fieldName}'."
            );
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
