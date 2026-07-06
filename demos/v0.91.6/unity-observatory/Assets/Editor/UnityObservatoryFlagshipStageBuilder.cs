using System;
using System.Collections.Generic;
using System.IO;
using ADL.Demos.UnityObservatory;
using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace ADL.Demos.UnityObservatory.Editor
{
    public static class UnityObservatoryFlagshipStageBuilder
    {
        private const string ScenePath = "Assets/Scenes/FlagshipObservatoryStage.unity";
        private const string BootstrapObjectName = "Unity Observatory Bootstrap";
        private const string ProofRigObjectName = "ADL Flagship Observatory Proof Rig";
        private const string MainCameraObjectName = "Main Camera";
        private const string DirectionalLightObjectName = "Directional Sun";
        private const string MetadataObjectName = "Proof Metadata";
        private const string ObservatoryDeckObjectName = "Observatory Deck";
        private const string BeaconObjectName = "Observatory Beacon";
        private const string HolographicGlobeObjectName = "Holographic Observatory Globe";
        private const string AssetBackedArchitectureName = "Asset Backed Observatory Architecture";
        private const string RuntimePolisProjectionName = "Runtime Polis Projection";
        private const string InvestorArrivalLayerName = "Investor Arrival And Depth Layer";
        private const string EnvironmentalBackdropName = "Imported Environment Backdrop Layer";
        private const string WideCameraObjectName = "Wide Observatory Camera";
        private const string RuntimeDetailCameraObjectName = "Runtime Detail Camera";
        private const string InvestorHeroCameraObjectName = "Investor Hero Camera";
        private const string ContractResourcePath = "observatory_contract";
        private const string HeroProofPath = "Proof/flagship-observatory-investor-hero.png";

        private static readonly string[] RequiredAssetRoots =
        {
            "Assets/Creepy_Cat",
            "Assets/Sci-Fi Styled Modular Pack",
            "Assets/ScifiOfficeLite",
        };

        private static readonly string[] RequiredPrefabPaths =
        {
            "Assets/Creepy_Cat/ShowRoom_Vol 32/Prefabs/P_Terrain_01.prefab",
            "Assets/Creepy_Cat/ShowRoom_Vol 32/Prefabs/P_Clouds_A.prefab",
            "Assets/Creepy_Cat/ShowRoom_Vol 32/Prefabs/P_Clouds_B.prefab",
            "Assets/Creepy_Cat/ShowRoom_Vol 32/Prefabs/SpaceShip_BubbleShip/P_BubbleShip_Mark_01_White_Light.prefab",
            "Assets/Creepy_Cat/ShowRoom_Vol 32/Prefabs/BigRoom/P_SkyTower_Full.prefab",
        };

        private static readonly string[] RequiredPresentationPrefabPaths =
        {
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Floors/floor_5.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Windows/Complete Windows/window_big.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Windows/Complete Windows/window_big_corner.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Walls/Half walls/decorative_half_wall_5.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/Column/column_middle.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Lights/light_celing_2.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/big_screen.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/console_screen.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/Shield Core.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/projector.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Lights/light_wall_2_blue.prefab",
            "Assets/ScifiOfficeLite/Prefabs/Tech Accessories/Server Rack.prefab",
            "Assets/ScifiOfficeLite/Prefabs/Tech Accessories/Mechanical arm 1.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Stairs/stairs_big_with_emmision.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Walls/glass_panel_1.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Doors/glass_panel_1_with_door.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/projector stars.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/generator.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/Tables/decorative_table_glass.prefab",
            "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/decorative_chair.prefab",
            "Assets/ScifiOfficeLite/Prefabs/Chairs and Stools/Office Chair Variant.prefab",
            "Assets/ScifiOfficeLite/Prefabs/Tables/Metal/Table Metal Variant.prefab",
        };

        private static readonly string[] RequiredArchitectureObjectNames =
        {
            "Presentation Floor Plate",
            "Rear Observatory Window",
            "Left Observatory Window Corner",
            "Right Observatory Window Corner",
            "Left Presentation Wall",
            "Right Presentation Wall",
            "Rear Focal Column Left",
            "Rear Focal Column Right",
            "Ceiling Light Truss Left",
            "Ceiling Light Truss Right",
            "Mission Big Screen",
            "Left Console Station",
            "Right Console Station",
            "Shield Core Hologram Base",
            "Orbital Projector",
            "Left Server Rack",
            "Right Server Rack",
            "Left Mechanical Telescope Arm",
            "Right Mechanical Telescope Arm",
            "Blue Perimeter Wall Light 1",
            "Blue Perimeter Wall Light 2",
            "Blue Perimeter Wall Light 3",
            "Blue Perimeter Wall Light 4",
        };

        private static readonly string[] RequiredInvestorArrivalObjectNames =
        {
            "Arrival Portal Glass Door",
            "Left Arrival Stair",
            "Right Arrival Stair",
            "Left Glass Catwalk",
            "Right Glass Catwalk",
            "Deep Space Star Projector",
            "Left Power Generator",
            "Right Power Generator",
            "Investor Witness Table",
            "Investor Witness Chair 1",
            "Investor Witness Chair 2",
            "Investor Witness Chair 3",
            "Investor Witness Chair 4",
            "Operator Review Desk",
            "Operator Review Chair",
            "Governance Walkway Spine",
            "Arrival Threshold Glow",
            "Investor Sightline Rails",
            "Hero Shot Floor Ribbon",
            "Executive Proof Caption",
        };

        private static readonly string[] RequiredEnvironmentalBackdropObjectNames =
        {
            "Atmospheric Horizon Dome",
            "Distant Observatory Ridge",
            "Left Structural Support Mast",
            "Right Structural Support Mast",
            "Central Structural Support Mast",
            "Lower Service Platform",
            "Observatory Foundation Shadow",
        };

        private static readonly string[] RequiredProofCameraNames =
        {
            MainCameraObjectName,
            WideCameraObjectName,
            RuntimeDetailCameraObjectName,
            InvestorHeroCameraObjectName,
        };

        private static readonly string[] RequiredPolisProjectionObjectNames =
        {
            "Investor Runtime Backplate",
            "Investor Runtime Banner",
            "Investor Runtime Claim",
            "Investor Runtime Proof Line",
            "Runtime Polis Ribbon",
            "Polis Projection Title",
            "Polis Projection Health",
            "Polis Projection Governance",
            "Polis Projection Metrics",
            "Runtime Contract Ref",
            "Runtime Artifact Root",
            "Runtime Operator Report",
            "Polis Evidence Wall",
            "Polis Evidence Header",
            "Polis Evidence Flow",
            "Polis Operator Guardrail",
        };

        [Serializable]
        private sealed class ObservatoryContractDocument
        {
            public string source_packet_ref;
            public string runtime_artifact_root;
            public ManifoldSection manifold;
            public SummarySection summary;
            public ReviewSection review;
        }

        [Serializable]
        private sealed class ManifoldSection
        {
            public string display_name;
        }

        [Serializable]
        private sealed class SummarySection
        {
            public int citizen_count;
            public int episode_count;
        }

        [Serializable]
        private sealed class ReviewSection
        {
            public string operator_report_ref;
        }

        private readonly struct RuntimeProjectionSnapshot
        {
            public RuntimeProjectionSnapshot(
                string displayName,
                string sourcePacketRef,
                string artifactRoot,
                string operatorReportRef,
                int citizenCount,
                int episodeCount
            )
            {
                DisplayName = displayName;
                SourcePacketRef = sourcePacketRef;
                ArtifactRoot = artifactRoot;
                OperatorReportRef = operatorReportRef;
                CitizenCount = citizenCount;
                EpisodeCount = episodeCount;
            }

            public string DisplayName { get; }
            public string SourcePacketRef { get; }
            public string ArtifactRoot { get; }
            public string OperatorReportRef { get; }
            public int CitizenCount { get; }
            public int EpisodeCount { get; }
        }

        [MenuItem("ADL/Observatory/Build Flagship Stage")]
        public static void EnsureFlagshipStage()
        {
            EnsureSceneDirectory();
            Scene scene = File.Exists(ScenePath)
                ? EditorSceneManager.OpenScene(ScenePath)
                : EditorSceneManager.NewScene(NewSceneSetup.EmptyScene, NewSceneMode.Single);

            EnsureProofRig(scene);
            EnsureBootstrap(scene);
            EnsureCamera(scene);
            EnsureKeyLight(scene);
            EnsureInvestorLighting(scene);
            EnsureAmbientLighting();
            EnsureMetadata(scene);
            RemoveLegacyImportedRoots(scene);
            EnsureImportedPrefabAnchors(scene);
            EnsureEnvironmentalBackdrop(scene);
            EnsureCompositionMarkers(scene);
            EnsureAssetBackedArchitecture(scene);
            EnsureInvestorPresentation(scene);
            EnsureInvestorArrivalAndDepth(scene);
            EnsureProofCameras(scene);

            EditorSceneManager.SaveScene(scene, ScenePath);
            AssetDatabase.Refresh();
        }

        [MenuItem("ADL/Observatory/Validate Flagship Stage")]
        public static void ValidateFlagshipStageMenu()
        {
            EnsureFlagshipStage();
            ValidateFlagshipStage(EditorSceneManager.OpenScene(ScenePath));
            Debug.Log("ADL flagship observatory stage validation passed.");
        }

        [MenuItem("ADL/Observatory/Capture Flagship Investor Hero Proof")]
        public static void CaptureInvestorHeroProof()
        {
            EnsureFlagshipStage();
            Scene scene = EditorSceneManager.OpenScene(ScenePath);
            ValidateFlagshipStage(scene);

            GameObject cameraObject = RequireObject(scene, InvestorHeroCameraObjectName);
            Camera camera = cameraObject.GetComponent<Camera>();
            if (camera == null)
            {
                throw new InvalidOperationException(
                    $"Flagship stage proof camera '{InvestorHeroCameraObjectName}' is missing a Camera component."
                );
            }

            string absolutePath = Path.Combine(Directory.GetCurrentDirectory(), HeroProofPath);
            Directory.CreateDirectory(Path.GetDirectoryName(absolutePath));
            RenderTexture previousTarget = camera.targetTexture;
            RenderTexture previousActive = RenderTexture.active;
            RenderTexture target = new RenderTexture(1920, 1080, 24)
            {
                antiAliasing = 4,
            };
            Texture2D image = new Texture2D(1920, 1080, TextureFormat.RGB24, false);
            try
            {
                camera.targetTexture = target;
                RenderTexture.active = target;
                camera.Render();
                image.ReadPixels(new Rect(0, 0, 1920, 1080), 0, 0);
                image.Apply();
                File.WriteAllBytes(absolutePath, image.EncodeToPNG());
            }
            finally
            {
                camera.targetTexture = previousTarget;
                RenderTexture.active = previousActive;
                UnityEngine.Object.DestroyImmediate(image);
                UnityEngine.Object.DestroyImmediate(target);
            }

            ValidateHeroProofArtifact(absolutePath);
            Debug.Log($"ADL flagship observatory hero proof captured. path={HeroProofPath}");
        }

        public static void ValidateFlagshipStage(Scene scene)
        {
            if (!scene.IsValid() || string.Equals(scene.path, string.Empty, StringComparison.Ordinal))
            {
                throw new InvalidOperationException("Flagship stage validation requires a saved scene.");
            }

            foreach (string cameraName in RequiredProofCameraNames)
            {
                RequireObject(scene, cameraName);
            }

            RequireObject(scene, DirectionalLightObjectName);
            RequireObject(scene, BootstrapObjectName);
            RequireObject(scene, ProofRigObjectName);
            RequireObject(scene, MetadataObjectName);
            RequireObject(scene, ObservatoryDeckObjectName);
            RequireObject(scene, BeaconObjectName);
            RequireObject(scene, HolographicGlobeObjectName);
            GameObject architecture = RequireObject(scene, AssetBackedArchitectureName);
            GameObject polisProjection = RequireObject(scene, RuntimePolisProjectionName);
            GameObject investorArrival = RequireObject(scene, InvestorArrivalLayerName);
            GameObject backdrop = RequireObject(scene, EnvironmentalBackdropName);

            foreach (string objectName in RequiredArchitectureObjectNames)
            {
                RequireChild(architecture, objectName);
            }

            foreach (string objectName in RequiredPolisProjectionObjectNames)
            {
                RequireChild(polisProjection, objectName);
            }

            foreach (string objectName in RequiredInvestorArrivalObjectNames)
            {
                RequireChild(investorArrival, objectName);
            }

            foreach (string objectName in RequiredEnvironmentalBackdropObjectNames)
            {
                RequireChild(backdrop, objectName);
            }

            int prefabInstances = CountSceneObjects(scene.path, "--- !u!1001");
            int gameObjects = CountSceneObjects(scene.path, "--- !u!1 ");
            int cameras = CountSceneObjects(scene.path, "--- !u!20 ");
            int lights = CountSceneObjects(scene.path, "--- !u!108 ");

            int requiredPrefabInstances = RequiredPrefabPaths.Length
                + RequiredArchitectureObjectNames.Length
                + RequiredInvestorArrivalObjectNames.Length
                - 5;
            if (prefabInstances < requiredPrefabInstances)
            {
                throw new InvalidOperationException(
                    $"Flagship stage expected at least {requiredPrefabInstances} prefab-backed instances, observed {prefabInstances}."
                );
            }

            if (gameObjects < 77 || cameras < RequiredProofCameraNames.Length || lights < 4)
            {
                throw new InvalidOperationException(
                    $"Flagship stage scene composition is too thin: gameObjects={gameObjects}, cameras={cameras}, lights={lights}."
                );
            }

            if (architecture.transform.childCount < RequiredArchitectureObjectNames.Length ||
                polisProjection.transform.childCount < RequiredPolisProjectionObjectNames.Length ||
                investorArrival.transform.childCount < RequiredInvestorArrivalObjectNames.Length)
            {
                throw new InvalidOperationException(
                    $"Flagship stage presentation layer is incomplete: architectureChildren={architecture.transform.childCount}, polisChildren={polisProjection.transform.childCount}, investorArrivalChildren={investorArrival.transform.childCount}."
                );
            }

            foreach (string root in RequiredAssetRoots)
            {
                if (!AssetDatabase.IsValidFolder(root))
                {
                    throw new InvalidOperationException(
                        $"Flagship stage is missing required imported asset root: {root}"
                    );
                }
            }

            foreach (string prefabPath in RequiredPrefabPaths)
            {
                if (AssetDatabase.LoadAssetAtPath<GameObject>(prefabPath) == null)
                {
                    throw new InvalidOperationException(
                        $"Flagship stage is missing required imported prefab: {prefabPath}"
                    );
                }
            }

            foreach (string prefabPath in RequiredPresentationPrefabPaths)
            {
                if (AssetDatabase.LoadAssetAtPath<GameObject>(prefabPath) == null)
                {
                    throw new InvalidOperationException(
                        $"Flagship stage is missing required presentation prefab: {prefabPath}"
                    );
                }
            }

            Debug.Log(
                $"ADL flagship observatory stage validation passed. scene={scene.path}; prefabInstances={prefabInstances}; gameObjects={gameObjects}; cameras={cameras}; lights={lights}"
            );
        }

        private static void ValidateHeroProofArtifact(string absolutePath)
        {
            FileInfo proofFile = new FileInfo(absolutePath);
            if (!proofFile.Exists || proofFile.Length < 512 * 1024)
            {
                throw new InvalidOperationException(
                    $"Flagship stage hero proof artifact is missing or too small: {HeroProofPath}"
                );
            }
        }

        private static void EnsureSceneDirectory()
        {
            if (!AssetDatabase.IsValidFolder("Assets/Scenes"))
            {
                AssetDatabase.CreateFolder("Assets", "Scenes");
            }
        }

        private static void EnsureProofRig(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);
            if (rig == null)
            {
                rig = new GameObject(ProofRigObjectName);
                SceneManager.MoveGameObjectToScene(rig, scene);
            }

            rig.transform.position = Vector3.zero;
            rig.transform.rotation = Quaternion.identity;
        }

        private static void EnsureBootstrap(Scene scene)
        {
            GameObject bootstrap = FindRoot(scene, BootstrapObjectName);
            if (bootstrap == null)
            {
                bootstrap = new GameObject(BootstrapObjectName);
                SceneManager.MoveGameObjectToScene(bootstrap, scene);
            }

            if (bootstrap.GetComponent<UnityObservatoryBootstrap>() == null)
            {
                bootstrap.AddComponent<UnityObservatoryBootstrap>();
            }
        }

        private static void EnsureCamera(Scene scene)
        {
            GameObject cameraObject = GameObject.FindWithTag("MainCamera") ?? FindRoot(scene, MainCameraObjectName);
            if (cameraObject == null)
            {
                cameraObject = new GameObject(MainCameraObjectName);
                SceneManager.MoveGameObjectToScene(cameraObject, scene);
                cameraObject.tag = "MainCamera";
            }

            cameraObject.name = MainCameraObjectName;
            cameraObject.tag = "MainCamera";
            ConfigureProofCamera(
                cameraObject,
                new Vector3(10.8f, 5.55f, -15.4f),
                new Vector3(3.8f, 3.05f, -4.9f),
                34f,
                0f
            );
        }

        private static void EnsureProofCameras(Scene scene)
        {
            EnsureNamedProofCamera(
                scene,
                WideCameraObjectName,
                new Vector3(15.8f, 7.4f, -18.8f),
                new Vector3(3.1f, 3.1f, -4.3f),
                36f,
                -20f
            );

            EnsureNamedProofCamera(
                scene,
                RuntimeDetailCameraObjectName,
                new Vector3(1.4f, 4.85f, -11.6f),
                new Vector3(2.3f, 2.25f, -4.45f),
                40f,
                -19f
            );

            EnsureNamedProofCamera(
                scene,
                InvestorHeroCameraObjectName,
                new Vector3(10.9f, 5.85f, -16.4f),
                new Vector3(2.55f, 2.8f, -2.75f),
                38f,
                -18f
            );
        }

        private static void EnsureNamedProofCamera(
            Scene scene,
            string cameraName,
            Vector3 position,
            Vector3 lookAt,
            float fieldOfView,
            float depth
        )
        {
            GameObject cameraObject = FindRoot(scene, cameraName);
            if (cameraObject == null)
            {
                cameraObject = new GameObject(cameraName);
                SceneManager.MoveGameObjectToScene(cameraObject, scene);
            }

            cameraObject.name = cameraName;
            cameraObject.tag = "Untagged";
            ConfigureProofCamera(cameraObject, position, lookAt, fieldOfView, depth);
        }

        private static void ConfigureProofCamera(
            GameObject cameraObject,
            Vector3 position,
            Vector3 lookAt,
            float fieldOfView,
            float depth
        )
        {
            Camera camera = cameraObject.GetComponent<Camera>();
            if (camera == null)
            {
                camera = cameraObject.AddComponent<Camera>();
            }

            cameraObject.transform.position = position;
            cameraObject.transform.LookAt(lookAt, Vector3.up);
            camera.fieldOfView = fieldOfView;
            camera.depth = depth;
            camera.nearClipPlane = 0.1f;
            camera.farClipPlane = 2000f;
            camera.backgroundColor = new Color(0.026f, 0.065f, 0.088f, 1f);
            camera.clearFlags = CameraClearFlags.SolidColor;
        }

        private static void EnsureKeyLight(Scene scene)
        {
            GameObject lightObject = FindRoot(scene, DirectionalLightObjectName);
            if (lightObject == null)
            {
                lightObject = new GameObject(DirectionalLightObjectName);
                SceneManager.MoveGameObjectToScene(lightObject, scene);
            }

            Light light = lightObject.GetComponent<Light>();
            if (light == null)
            {
                light = lightObject.AddComponent<Light>();
            }

            light.type = LightType.Directional;
            light.color = new Color(1f, 0.94f, 0.88f, 1f);
            light.intensity = 0.5f;
            light.shadows = LightShadows.Soft;
            lightObject.transform.rotation = Quaternion.Euler(38f, -28f, 0f);
        }

        private static void EnsureInvestorLighting(Scene scene)
        {
            EnsurePointLight(
                scene,
                "Investor Cyan Rim Light",
                new Vector3(-1.2f, 5.4f, -3.5f),
                new Color(0.08f, 0.74f, 1f, 1f),
                1.65f,
                8.5f
            );
            EnsurePointLight(
                scene,
                "Investor Amber Proof Wall Key",
                new Vector3(5.4f, 3.2f, -2.6f),
                new Color(1f, 0.72f, 0.34f, 1f),
                0.95f,
                5.5f
            );
            EnsurePointLight(
                scene,
                "Investor Runtime Floor Wash",
                new Vector3(3.8f, 1.25f, -7.6f),
                new Color(0.1f, 0.95f, 0.82f, 1f),
                0.8f,
                6.2f
            );
            EnsurePointLight(
                scene,
                "Investor Arrival Portal Glow",
                new Vector3(0f, 2.3f, -10.2f),
                new Color(0.34f, 0.78f, 1f, 1f),
                1.1f,
                7.4f
            );
            EnsurePointLight(
                scene,
                "Investor Witness Rail Warmth",
                new Vector3(-1.9f, 2.15f, -7.8f),
                new Color(1f, 0.66f, 0.36f, 1f),
                0.72f,
                5.4f
            );
        }

        private static void EnsurePointLight(
            Scene scene,
            string objectName,
            Vector3 position,
            Color color,
            float intensity,
            float range
        )
        {
            GameObject lightObject = FindRoot(scene, objectName);
            if (lightObject == null)
            {
                lightObject = new GameObject(objectName);
                SceneManager.MoveGameObjectToScene(lightObject, scene);
            }

            Light light = lightObject.GetComponent<Light>();
            if (light == null)
            {
                light = lightObject.AddComponent<Light>();
            }

            lightObject.transform.position = position;
            light.type = LightType.Point;
            light.color = color;
            light.intensity = intensity;
            light.range = range;
            light.shadows = LightShadows.Soft;
        }

        private static void EnsureMetadata(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);
            GameObject metadata = FindOrCreateSingleChild(
                scene,
                rig,
                MetadataObjectName,
                () => new GameObject(MetadataObjectName)
            );
            metadata.transform.localPosition = new Vector3(0f, 2f, 0f);
        }

        private static void EnsureImportedPrefabAnchors(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);
            List<Vector3> positions = new()
            {
                new Vector3(0f, -18f, 26f),
                new Vector3(-32f, 24f, 64f),
                new Vector3(38f, 28f, 70f),
                new Vector3(-7.8f, 7.4f, 5.6f),
                new Vector3(2.8f, -2.4f, 17.2f),
            };

            for (int index = 0; index < RequiredPrefabPaths.Length; index++)
            {
                string prefabPath = RequiredPrefabPaths[index];
                GameObject existing = FindOrCreateSingleChild(
                    scene,
                    rig,
                    AnchorName(index),
                    () =>
                    {
                        GameObject prefab = AssetDatabase.LoadAssetAtPath<GameObject>(prefabPath);
                        return prefab == null ? null : (GameObject)PrefabUtility.InstantiatePrefab(prefab, scene);
                    }
                );
                if (existing != null)
                {
                    existing.transform.SetParent(rig.transform, false);
                    existing.name = AnchorName(index);
                    ConfigureAnchor(index, existing, positions[index]);
                }
            }
        }

        private static void ConfigureAnchor(int index, GameObject instance, Vector3 position)
        {
            instance.transform.localPosition = position;
            instance.transform.localRotation = index switch
            {
                3 => Quaternion.Euler(0f, -38f, 0f),
                4 => Quaternion.Euler(0f, 180f, 0f),
                _ => Quaternion.identity,
            };
            instance.transform.localScale = index switch
            {
                1 => Vector3.one * 0.18f,
                2 => Vector3.one * 0.18f,
                3 => Vector3.one * 0.44f,
                4 => Vector3.one * 0.16f,
                _ => Vector3.one,
            };
        }

        private static void RemoveLegacyImportedRoots(Scene scene)
        {
            HashSet<string> prefabNames = new();
            foreach (string prefabPath in RequiredPrefabPaths)
            {
                prefabNames.Add(Path.GetFileNameWithoutExtension(prefabPath));
            }

            foreach (GameObject rootObject in scene.GetRootGameObjects())
            {
                if (prefabNames.Contains(rootObject.name)
                    || string.Equals(rootObject.name, "Camera Anim", StringComparison.Ordinal))
                {
                    UnityEngine.Object.DestroyImmediate(rootObject);
                }
            }
        }

        private static void EnsureCompositionMarkers(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);

            GameObject deck = EnsurePrimitive(scene, rig, ObservatoryDeckObjectName, PrimitiveType.Cylinder);
            deck.transform.localPosition = new Vector3(3.8f, 2.36f, -5.2f);
            deck.transform.localScale = new Vector3(2.35f, 0.08f, 2.35f);
            SetMaterial(deck, new Color(0.11f, 0.18f, 0.2f, 1f), 0.68f);

            GameObject beacon = EnsurePrimitive(scene, rig, BeaconObjectName, PrimitiveType.Sphere);
            beacon.transform.localPosition = new Vector3(3.8f, 4.12f, -5.2f);
            beacon.transform.localScale = Vector3.one * 0.36f;
            SetMaterial(beacon, new Color(0.12f, 0.82f, 1f, 1f), 0.9f);

            for (int index = 0; index < 3; index++)
            {
                float angle = 120f * index;
                Vector3 offset = Quaternion.Euler(0f, angle, 0f) * new Vector3(2.8f, 0f, 0f);
                GameObject pylon = EnsurePrimitive(scene, rig, $"Observatory Pylon {index + 1}", PrimitiveType.Cylinder);
                pylon.transform.localPosition = new Vector3(3.8f, 3.25f, -5.2f) + offset;
                pylon.transform.localScale = new Vector3(0.08f, 0.9f, 0.08f);
                SetMaterial(pylon, new Color(0.025f, 0.038f, 0.052f, 1f), 0.2f);
            }
        }

        private static void EnsureInvestorPresentation(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);
            Vector3 center = new Vector3(3.8f, 3.05f, -5.2f);

            GameObject globe = EnsurePrimitive(scene, rig, HolographicGlobeObjectName, PrimitiveType.Sphere);
            globe.transform.localPosition = center + new Vector3(0f, 1.95f, 0f);
            globe.transform.localScale = Vector3.one * 0.72f;
            SetMaterial(globe, new Color(0.03f, 0.58f, 0.72f, 1f), 0.95f, true);

            Light globeLight = globe.GetComponent<Light>();
            if (globeLight == null)
            {
                globeLight = globe.AddComponent<Light>();
            }

            globeLight.type = LightType.Point;
            globeLight.color = new Color(0.13f, 0.78f, 1f, 1f);
            globeLight.intensity = 1.2f;
            globeLight.range = 7f;

            for (int index = 0; index < 16; index++)
            {
                float angle = index * 22.5f;
                Vector3 radial = Quaternion.Euler(0f, angle, 0f) * Vector3.forward;

                GameObject rail = EnsurePrimitive(scene, rig, $"Observation Rail {index + 1:00}", PrimitiveType.Cube);
                rail.transform.localPosition = center + radial * 3.9f + new Vector3(0f, 0.85f, 0f);
                rail.transform.localRotation = Quaternion.Euler(0f, angle, 0f);
                rail.transform.localScale = new Vector3(0.08f, 0.08f, 0.72f);
                SetMaterial(rail, new Color(0.035f, 0.052f, 0.065f, 1f), 0.55f);

                if (index % 2 == 0)
                {
                    GameObject light = EnsurePrimitive(scene, rig, $"Runway Light {index / 2 + 1:00}", PrimitiveType.Sphere);
                    light.transform.localPosition = center + radial * 3.25f + new Vector3(0f, 0.95f, 0f);
                    light.transform.localScale = Vector3.one * 0.12f;
                    SetMaterial(light, new Color(0.05f, 0.9f, 1f, 1f), 0.8f, true);
                }
            }

            EnsureTelemetryPanel(scene, rig, "Telemetry Panel Left", center + new Vector3(-3.9f, 2.0f, 1.2f), 30f);
            EnsureTelemetryPanel(scene, rig, "Telemetry Panel Center", center + new Vector3(0f, 2.25f, 2.5f), 0f);
            EnsureTelemetryPanel(scene, rig, "Telemetry Panel Right", center + new Vector3(3.9f, 2.0f, 1.2f), -30f);
            EnsureRuntimePolisProjection(scene, rig, center);
        }

        private static void EnsureEnvironmentalBackdrop(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);
            GameObject backdrop = FindOrCreateSingleChild(
                scene,
                rig,
                EnvironmentalBackdropName,
                () => new GameObject(EnvironmentalBackdropName)
            );
            Vector3 center = new Vector3(3.8f, 2.95f, -5.2f);

            GameObject dome = EnsurePrimitive(scene, backdrop, "Atmospheric Horizon Dome", PrimitiveType.Sphere);
            dome.transform.localPosition = center + new Vector3(0f, 5.2f, 20.5f);
            dome.transform.localRotation = Quaternion.identity;
            dome.transform.localScale = new Vector3(26f, 8.5f, 7.5f);
            SetMaterial(dome, new Color(0.025f, 0.12f, 0.18f, 1f), 0.72f, true);

            GameObject ridge = EnsurePrimitive(scene, backdrop, "Distant Observatory Ridge", PrimitiveType.Cube);
            ridge.transform.localPosition = center + new Vector3(0f, -1.95f, 9.7f);
            ridge.transform.localRotation = Quaternion.Euler(0f, 0f, 0f);
            ridge.transform.localScale = new Vector3(14.5f, 0.32f, 2.2f);
            SetMaterial(ridge, new Color(0.06f, 0.11f, 0.12f, 1f), 0.2f);

            EnsureSupportMast(
                scene,
                backdrop,
                "Left Structural Support Mast",
                center + new Vector3(-4.25f, -2.55f, -1.65f),
                3.2f
            );
            EnsureSupportMast(
                scene,
                backdrop,
                "Right Structural Support Mast",
                center + new Vector3(4.25f, -2.55f, -1.65f),
                3.2f
            );
            EnsureSupportMast(
                scene,
                backdrop,
                "Central Structural Support Mast",
                center + new Vector3(0f, -2.85f, -0.15f),
                3.6f
            );

            GameObject platform = EnsurePrimitive(scene, backdrop, "Lower Service Platform", PrimitiveType.Cube);
            platform.transform.localPosition = center + new Vector3(0f, -0.62f, -0.65f);
            platform.transform.localRotation = Quaternion.Euler(0f, 45f, 0f);
            platform.transform.localScale = new Vector3(3.2f, 0.08f, 1.45f);
            SetMaterial(platform, new Color(0.05f, 0.1f, 0.112f, 1f), 0.46f);

            GameObject shadow = EnsurePrimitive(scene, backdrop, "Observatory Foundation Shadow", PrimitiveType.Cube);
            shadow.transform.localPosition = center + new Vector3(0f, -3.45f, 0.2f);
            shadow.transform.localRotation = Quaternion.Euler(0f, 45f, 0f);
            shadow.transform.localScale = new Vector3(4.4f, 0.025f, 2.15f);
            SetMaterial(shadow, new Color(0.01f, 0.016f, 0.018f, 1f), 0.06f);
        }

        private static void EnsureSupportMast(
            Scene scene,
            GameObject parent,
            string objectName,
            Vector3 localPosition,
            float height
        )
        {
            GameObject mast = EnsurePrimitive(scene, parent, objectName, PrimitiveType.Cylinder);
            mast.transform.localPosition = localPosition;
            mast.transform.localRotation = Quaternion.identity;
            mast.transform.localScale = new Vector3(0.12f, height, 0.12f);
            SetMaterial(mast, new Color(0.04f, 0.075f, 0.085f, 1f), 0.38f);
        }

        private static void EnsureInvestorArrivalAndDepth(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);
            GameObject layer = FindOrCreateSingleChild(
                scene,
                rig,
                InvestorArrivalLayerName,
                () => new GameObject(InvestorArrivalLayerName)
            );
            Vector3 center = new Vector3(3.8f, 2.95f, -5.2f);

            EnsurePrefabChild(
                scene,
                layer,
                "Arrival Portal Glass Door",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Doors/glass_panel_1_with_door.prefab",
                center + new Vector3(-3.8f, 0.62f, -5.75f),
                Quaternion.Euler(0f, 0f, 0f),
                new Vector3(0.92f, 0.92f, 0.92f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Left Arrival Stair",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Stairs/stairs_big_with_emmision.prefab",
                center + new Vector3(-5.05f, -0.18f, -2.85f),
                Quaternion.Euler(0f, 42f, 0f),
                new Vector3(0.42f, 0.42f, 0.42f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Right Arrival Stair",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Stairs/stairs_big_with_emmision.prefab",
                center + new Vector3(5.05f, -0.18f, -2.85f),
                Quaternion.Euler(0f, -42f, 0f),
                new Vector3(0.42f, 0.42f, 0.42f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Left Glass Catwalk",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Walls/glass_panel_1.prefab",
                center + new Vector3(-3.65f, 0.78f, -2.35f),
                Quaternion.Euler(90f, 28f, 0f),
                new Vector3(0.7f, 1.25f, 0.7f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Right Glass Catwalk",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Walls/glass_panel_1.prefab",
                center + new Vector3(3.65f, 0.78f, -2.35f),
                Quaternion.Euler(90f, -28f, 0f),
                new Vector3(0.7f, 1.25f, 0.7f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Deep Space Star Projector",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/projector stars.prefab",
                center + new Vector3(0f, 1.05f, 3.45f),
                Quaternion.Euler(0f, 180f, 0f),
                new Vector3(0.68f, 0.68f, 0.68f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Left Power Generator",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/generator.prefab",
                center + new Vector3(-4.75f, 0.2f, -2.65f),
                Quaternion.Euler(0f, 72f, 0f),
                new Vector3(0.5f, 0.5f, 0.5f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Right Power Generator",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/generator.prefab",
                center + new Vector3(4.75f, 0.2f, -2.65f),
                Quaternion.Euler(0f, -72f, 0f),
                new Vector3(0.5f, 0.5f, 0.5f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Investor Witness Table",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/Tables/decorative_table_glass.prefab",
                center + new Vector3(-4.65f, 0.16f, -3.8f),
                Quaternion.Euler(0f, 24f, 0f),
                new Vector3(0.52f, 0.52f, 0.52f)
            );

            for (int index = 0; index < 4; index++)
            {
                float z = -4.62f + (index * 0.46f);
                EnsurePrefabChild(
                    scene,
                    layer,
                    $"Investor Witness Chair {index + 1}",
                    "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/decorative_chair.prefab",
                    center + new Vector3(-5.52f, 0.2f, z),
                    Quaternion.Euler(0f, 62f, 0f),
                    new Vector3(0.44f, 0.44f, 0.44f)
                );
            }

            EnsurePrefabChild(
                scene,
                layer,
                "Operator Review Desk",
                "Assets/ScifiOfficeLite/Prefabs/Tables/Metal/Table Metal Variant.prefab",
                center + new Vector3(5.65f, 0.08f, -3.95f),
                Quaternion.Euler(0f, -58f, 0f),
                new Vector3(0.56f, 0.56f, 0.56f)
            );

            EnsurePrefabChild(
                scene,
                layer,
                "Operator Review Chair",
                "Assets/ScifiOfficeLite/Prefabs/Chairs and Stools/Office Chair Variant.prefab",
                center + new Vector3(4.95f, 0.2f, -4.55f),
                Quaternion.Euler(0f, 32f, 0f),
                new Vector3(0.62f, 0.62f, 0.62f)
            );

            EnsurePrimitiveRibbon(
                scene,
                layer,
                "Governance Walkway Spine",
                center + new Vector3(0f, 0.06f, -3.9f),
                Quaternion.Euler(0f, 0f, 0f),
                new Vector3(0.28f, 0.035f, 3.4f),
                new Color(0.03f, 0.75f, 0.9f, 1f)
            );

            EnsurePrimitiveRibbon(
                scene,
                layer,
                "Arrival Threshold Glow",
                center + new Vector3(0f, 0.08f, -7.55f),
                Quaternion.Euler(0f, 0f, 0f),
                new Vector3(3.8f, 0.035f, 0.08f),
                new Color(0.08f, 0.95f, 1f, 1f)
            );

            EnsurePrimitiveRibbon(
                scene,
                layer,
                "Investor Sightline Rails",
                center + new Vector3(-0.2f, 1.05f, -5.72f),
                Quaternion.Euler(0f, 0f, 0f),
                new Vector3(3.2f, 0.045f, 0.06f),
                new Color(0.95f, 0.78f, 0.42f, 1f)
            );

            EnsurePrimitiveRibbon(
                scene,
                layer,
                "Hero Shot Floor Ribbon",
                center + new Vector3(0f, 0.1f, -1.35f),
                Quaternion.Euler(0f, 0f, 0f),
                new Vector3(4.6f, 0.025f, 0.12f),
                new Color(0.1f, 0.86f, 0.7f, 1f)
            );

            EnsureTextPanel(
                scene,
                layer,
                "Executive Proof Caption",
                "evidence | polis | operator review",
                center + new Vector3(-0.15f, 1.18f, -5.45f),
                Quaternion.Euler(8f, 0f, 0f),
                0.022f,
                new Color(0.94f, 1f, 0.84f, 1f)
            );
        }

        private static void EnsurePrimitiveRibbon(
            Scene scene,
            GameObject parent,
            string objectName,
            Vector3 localPosition,
            Quaternion localRotation,
            Vector3 localScale,
            Color color
        )
        {
            GameObject ribbon = EnsurePrimitive(scene, parent, objectName, PrimitiveType.Cube);
            ribbon.transform.localPosition = localPosition;
            ribbon.transform.localRotation = localRotation;
            ribbon.transform.localScale = localScale;
            SetMaterial(ribbon, color, 0.92f, true);
        }

        private static void EnsureRuntimePolisProjection(Scene scene, GameObject rig, Vector3 center)
        {
            RuntimeProjectionSnapshot runtime = LoadRuntimeProjectionSnapshot();
            GameObject projection = FindOrCreateSingleChild(
                scene,
                rig,
                RuntimePolisProjectionName,
                () => new GameObject(RuntimePolisProjectionName)
            );

            EnsureDisplayBackplate(
                scene,
                projection,
                "Investor Runtime Backplate",
                center + new Vector3(-0.25f, 1.42f, 2.12f),
                Quaternion.Euler(9f, 180f, 0f),
                new Vector3(3.35f, 0.86f, 0.04f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Investor Runtime Banner",
                "ADL Observatory",
                center + new Vector3(-0.25f, 1.72f, 1.98f),
                Quaternion.Euler(9f, 180f, 0f),
                0.058f,
                new Color(0.86f, 0.98f, 1f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Investor Runtime Claim",
                "runtime evidence -> governed polis",
                center + new Vector3(-0.25f, 1.44f, 1.99f),
                Quaternion.Euler(9f, 180f, 0f),
                0.027f,
                new Color(0.72f, 1f, 0.9f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Investor Runtime Proof Line",
                "trace-backed | proposal-only | operator-safe",
                center + new Vector3(-0.25f, 1.22f, 2.0f),
                Quaternion.Euler(9f, 180f, 0f),
                0.021f,
                new Color(1f, 0.92f, 0.66f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Runtime Polis Ribbon",
                "observable runtime, governed decisions",
                center + new Vector3(-0.25f, 1.04f, 2.01f),
                Quaternion.Euler(9f, 180f, 0f),
                0.019f,
                new Color(0.76f, 1f, 0.9f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Polis Projection Title",
                runtime.DisplayName,
                center + new Vector3(-2.55f, 2.55f, 2.48f),
                Quaternion.Euler(10f, 180f, 0f),
                0.038f,
                new Color(0.79f, 0.96f, 1f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Polis Projection Health",
                "bounded polis\ntrace-backed",
                center + new Vector3(-2.55f, 2.28f, 2.5f),
                Quaternion.Euler(10f, 180f, 0f),
                0.023f,
                new Color(0.52f, 0.92f, 1f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Polis Projection Governance",
                "proposal only\noperator lens",
                center + new Vector3(2.45f, 2.26f, 2.46f),
                Quaternion.Euler(10f, 180f, 0f),
                0.023f,
                new Color(0.96f, 0.94f, 0.76f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Polis Projection Metrics",
                $"citizens {runtime.CitizenCount}\nepisodes {runtime.EpisodeCount}",
                center + new Vector3(0f, 2.52f, 2.62f),
                Quaternion.Euler(10f, 180f, 0f),
                0.024f,
                new Color(0.82f, 1f, 0.88f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Runtime Contract Ref",
                $"packet: {ShortReference(runtime.SourcePacketRef)}",
                center + new Vector3(-2.72f, 1.78f, 2.52f),
                Quaternion.Euler(10f, 180f, 0f),
                0.02f,
                new Color(0.68f, 0.92f, 1f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Runtime Artifact Root",
                $"artifacts: {ShortReference(runtime.ArtifactRoot)}",
                center + new Vector3(0f, 1.72f, 2.68f),
                Quaternion.Euler(10f, 180f, 0f),
                0.02f,
                new Color(0.72f, 1f, 0.86f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Runtime Operator Report",
                $"report: {ShortReference(runtime.OperatorReportRef)}",
                center + new Vector3(2.72f, 1.76f, 2.52f),
                Quaternion.Euler(10f, 180f, 0f),
                0.02f,
                new Color(1f, 0.9f, 0.62f, 1f)
            );

            EnsureDisplayBackplate(
                scene,
                projection,
                "Polis Evidence Wall",
                center + new Vector3(4.25f, 1.55f, -0.65f),
                Quaternion.Euler(4f, -68f, 0f),
                new Vector3(1.85f, 0.86f, 0.05f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Polis Evidence Header",
                "POLIS",
                center + new Vector3(4.16f, 1.82f, -0.74f),
                Quaternion.Euler(4f, -68f, 0f),
                0.054f,
                new Color(0.86f, 1f, 0.96f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Polis Evidence Flow",
                "runtime evidence | governed review",
                center + new Vector3(4.07f, 1.55f, -0.82f),
                Quaternion.Euler(4f, -68f, 0f),
                0.022f,
                new Color(0.58f, 1f, 0.84f, 1f)
            );

            EnsureTextPanel(
                scene,
                projection,
                "Polis Operator Guardrail",
                "operator-safe lens active",
                center + new Vector3(4.0f, 1.32f, -0.9f),
                Quaternion.Euler(4f, -68f, 0f),
                0.022f,
                new Color(1f, 0.92f, 0.62f, 1f)
            );
        }

        private static RuntimeProjectionSnapshot LoadRuntimeProjectionSnapshot()
        {
            string displayName = "Prototype CSM 02";
            string sourcePacketRef =
                "demos/fixtures/csm_observatory/proto-csm-02-governed-observatory-packet.json";
            string artifactRoot = "runtime_v2";
            string operatorReportRef = "runtime_v2/observatory/operator_report.md";
            int citizenCount = 3;
            int episodeCount = 2;

            TextAsset contractAsset = Resources.Load<TextAsset>(ContractResourcePath);
            if (contractAsset == null || string.IsNullOrWhiteSpace(contractAsset.text))
            {
                return new RuntimeProjectionSnapshot(
                    displayName,
                    sourcePacketRef,
                    artifactRoot,
                    operatorReportRef,
                    citizenCount,
                    episodeCount
                );
            }

            try
            {
                ObservatoryContractDocument contract =
                    JsonUtility.FromJson<ObservatoryContractDocument>(contractAsset.text);
                if (contract != null)
                {
                    displayName = DefaultIfBlank(contract.manifold?.display_name, displayName);
                    sourcePacketRef = DefaultIfBlank(contract.source_packet_ref, sourcePacketRef);
                    artifactRoot = DefaultIfBlank(contract.runtime_artifact_root, artifactRoot);
                    operatorReportRef = DefaultIfBlank(
                        contract.review?.operator_report_ref,
                        operatorReportRef
                    );
                    citizenCount = contract.summary != null && contract.summary.citizen_count > 0
                        ? contract.summary.citizen_count
                        : citizenCount;
                    episodeCount = contract.summary != null && contract.summary.episode_count > 0
                        ? contract.summary.episode_count
                        : episodeCount;
                }
            }
            catch (ArgumentException error)
            {
                Debug.LogWarning(
                    $"Flagship stage could not parse {ContractResourcePath}; using deterministic projection defaults. {error.Message}"
                );
            }

            return new RuntimeProjectionSnapshot(
                displayName,
                sourcePacketRef,
                artifactRoot,
                operatorReportRef,
                citizenCount,
                episodeCount
            );
        }

        private static string ShortReference(string reference)
        {
            if (string.IsNullOrWhiteSpace(reference))
            {
                return "unavailable";
            }

            string normalized = reference.Replace('\\', '/').TrimEnd('/');
            int lastSlash = normalized.LastIndexOf('/');
            return lastSlash >= 0 && lastSlash + 1 < normalized.Length
                ? normalized[(lastSlash + 1)..]
                : normalized;
        }

        private static void EnsureDisplayBackplate(
            Scene scene,
            GameObject parent,
            string objectName,
            Vector3 localPosition,
            Quaternion localRotation,
            Vector3 localScale
        )
        {
            GameObject backplate = EnsurePrimitive(scene, parent, objectName, PrimitiveType.Cube);
            backplate.transform.localPosition = localPosition;
            backplate.transform.localRotation = localRotation;
            backplate.transform.localScale = localScale;
            SetMaterial(backplate, new Color(0.015f, 0.05f, 0.065f, 1f), 0.85f, true);
        }

        private static void EnsureTextPanel(
            Scene scene,
            GameObject parent,
            string objectName,
            string text,
            Vector3 localPosition,
            Quaternion localRotation,
            float characterSize,
            Color color
        )
        {
            GameObject textObject = FindOrCreateSingleChild(
                scene,
                parent,
                objectName,
                () => new GameObject(objectName)
            );
            textObject.transform.SetParent(parent.transform, false);
            textObject.transform.localPosition = localPosition;
            textObject.transform.localRotation = localRotation;
            textObject.transform.localScale = Vector3.one;

            TextMesh textMesh = textObject.GetComponent<TextMesh>();
            if (textMesh == null)
            {
                textMesh = textObject.AddComponent<TextMesh>();
            }

            textMesh.text = text;
            textMesh.characterSize = characterSize;
            textMesh.anchor = TextAnchor.MiddleCenter;
            textMesh.alignment = TextAlignment.Center;
            textMesh.color = color;
            textMesh.fontSize = 48;

            Renderer renderer = textObject.GetComponent<Renderer>();
            if (renderer != null)
            {
                Material material = renderer.sharedMaterial;
                if (material == null || !material.name.StartsWith("ADL Polis Text ", StringComparison.Ordinal))
                {
                    material = new Material(Shader.Find("GUI/Text Shader"))
                    {
                        name = $"ADL Polis Text {objectName} Material",
                    };
                    renderer.sharedMaterial = material;
                }

                material.color = color;
            }
        }

        private static void EnsureAssetBackedArchitecture(Scene scene)
        {
            GameObject rig = FindRoot(scene, ProofRigObjectName);
            GameObject architecture = FindOrCreateSingleChild(
                scene,
                rig,
                AssetBackedArchitectureName,
                () => new GameObject(AssetBackedArchitectureName)
            );
            Vector3 center = new Vector3(3.8f, 2.95f, -5.2f);

            EnsurePrefabChild(
                scene,
                architecture,
                "Presentation Floor Plate",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Floors/floor_5.prefab",
                center + new Vector3(0f, -0.12f, 0f),
                Quaternion.Euler(0f, 45f, 0f),
                new Vector3(1.8f, 1f, 1.8f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Rear Observatory Window",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Windows/Complete Windows/window_big.prefab",
                center + new Vector3(0f, 1.65f, 4.8f),
                Quaternion.Euler(0f, 180f, 0f),
                new Vector3(0.92f, 0.88f, 0.92f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Left Observatory Window Corner",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Windows/Complete Windows/window_big_corner.prefab",
                center + new Vector3(-4.05f, 1.58f, 3.95f),
                Quaternion.Euler(0f, 134f, 0f),
                new Vector3(0.82f, 0.82f, 0.82f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Right Observatory Window Corner",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Windows/Complete Windows/window_big_corner.prefab",
                center + new Vector3(4.05f, 1.58f, 3.95f),
                Quaternion.Euler(0f, -134f, 0f),
                new Vector3(0.82f, 0.82f, 0.82f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Left Presentation Wall",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Walls/Half walls/decorative_half_wall_5.prefab",
                center + new Vector3(-4.25f, 0.72f, -1.15f),
                Quaternion.Euler(0f, 76f, 0f),
                new Vector3(0.46f, 0.46f, 0.46f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Right Presentation Wall",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Walls/Half walls/decorative_half_wall_5.prefab",
                center + new Vector3(4.25f, 0.72f, -1.15f),
                Quaternion.Euler(0f, -76f, 0f),
                new Vector3(0.46f, 0.46f, 0.46f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Rear Focal Column Left",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/Column/column_middle.prefab",
                center + new Vector3(-2.85f, 1.38f, 4.02f),
                Quaternion.Euler(0f, 0f, 0f),
                new Vector3(0.86f, 1.18f, 0.86f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Rear Focal Column Right",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/Column/column_middle.prefab",
                center + new Vector3(2.85f, 1.38f, 4.02f),
                Quaternion.Euler(0f, 0f, 0f),
                new Vector3(0.86f, 1.18f, 0.86f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Ceiling Light Truss Left",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Lights/light_celing_2.prefab",
                center + new Vector3(-1.8f, 3.95f, -0.4f),
                Quaternion.Euler(180f, 28f, 0f),
                new Vector3(0.78f, 0.78f, 0.78f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Ceiling Light Truss Right",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Lights/light_celing_2.prefab",
                center + new Vector3(1.8f, 3.95f, -0.4f),
                Quaternion.Euler(180f, -28f, 0f),
                new Vector3(0.78f, 0.78f, 0.78f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Mission Big Screen",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/big_screen.prefab",
                center + new Vector3(3.1f, 1.75f, 1.7f),
                Quaternion.Euler(0f, -34f, 0f),
                new Vector3(0.5f, 0.5f, 0.5f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Left Console Station",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/console_screen.prefab",
                center + new Vector3(-2.5f, 0.34f, 1.3f),
                Quaternion.Euler(0f, 24f, 0f),
                new Vector3(0.68f, 0.68f, 0.68f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Right Console Station",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Decorative elements/console_screen.prefab",
                center + new Vector3(2.5f, 0.34f, 1.3f),
                Quaternion.Euler(0f, -24f, 0f),
                new Vector3(0.68f, 0.68f, 0.68f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Shield Core Hologram Base",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/Shield Core.prefab",
                center + new Vector3(0f, 0.2f, -0.05f),
                Quaternion.identity,
                new Vector3(0.48f, 0.48f, 0.48f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Orbital Projector",
                "Assets/Sci-Fi Styled Modular Pack/Prefabs/Machines/projector.prefab",
                center + new Vector3(0f, 0.22f, -1.45f),
                Quaternion.Euler(0f, 180f, 0f),
                new Vector3(0.55f, 0.55f, 0.55f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Left Server Rack",
                "Assets/ScifiOfficeLite/Prefabs/Tech Accessories/Server Rack.prefab",
                center + new Vector3(-4.1f, 0.55f, -0.25f),
                Quaternion.Euler(0f, 68f, 0f),
                new Vector3(0.8f, 0.8f, 0.8f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Right Server Rack",
                "Assets/ScifiOfficeLite/Prefabs/Tech Accessories/Server Rack.prefab",
                center + new Vector3(4.1f, 0.55f, -0.25f),
                Quaternion.Euler(0f, -68f, 0f),
                new Vector3(0.8f, 0.8f, 0.8f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Left Mechanical Telescope Arm",
                "Assets/ScifiOfficeLite/Prefabs/Tech Accessories/Mechanical arm 1.prefab",
                center + new Vector3(-2.75f, 0.64f, -1.8f),
                Quaternion.Euler(0f, 38f, 0f),
                new Vector3(0.72f, 0.72f, 0.72f)
            );

            EnsurePrefabChild(
                scene,
                architecture,
                "Right Mechanical Telescope Arm",
                "Assets/ScifiOfficeLite/Prefabs/Tech Accessories/Mechanical arm 1.prefab",
                center + new Vector3(2.75f, 0.64f, -1.8f),
                Quaternion.Euler(0f, -38f, 0f),
                new Vector3(0.72f, 0.72f, 0.72f)
            );

            for (int index = 0; index < 4; index++)
            {
                float angle = 45f + (index * 90f);
                Vector3 offset = Quaternion.Euler(0f, angle, 0f) * new Vector3(3.45f, 0f, 1.35f);
                EnsurePrefabChild(
                    scene,
                    architecture,
                    $"Blue Perimeter Wall Light {index + 1}",
                    "Assets/Sci-Fi Styled Modular Pack/Prefabs/Lights/light_wall_2_blue.prefab",
                    center + offset + new Vector3(0f, 1.15f, 0f),
                    Quaternion.Euler(0f, angle + 180f, 0f),
                    new Vector3(0.55f, 0.55f, 0.55f)
                );
            }
        }

        private static void EnsureTelemetryPanel(
            Scene scene,
            GameObject rig,
            string name,
            Vector3 localPosition,
            float yaw
        )
        {
            GameObject panel = EnsurePrimitive(scene, rig, name, PrimitiveType.Cube);
            panel.transform.localPosition = localPosition;
            panel.transform.localRotation = Quaternion.Euler(0f, yaw, 0f);
            panel.transform.localScale = new Vector3(1.15f, 0.58f, 0.04f);
            SetMaterial(panel, new Color(0.02f, 0.43f, 0.58f, 1f), 0.85f, true);
        }

        private static GameObject EnsurePrimitive(Scene scene, GameObject parent, string objectName, PrimitiveType primitiveType)
        {
            GameObject primitive = FindOrCreateSingleChild(
                scene,
                parent,
                objectName,
                () => GameObject.CreatePrimitive(primitiveType)
            );

            primitive.transform.SetParent(parent.transform, false);
            Collider collider = primitive.GetComponent<Collider>();
            if (collider != null)
            {
                UnityEngine.Object.DestroyImmediate(collider);
            }

            return primitive;
        }

        private static GameObject EnsurePrefabChild(
            Scene scene,
            GameObject parent,
            string objectName,
            string prefabPath,
            Vector3 localPosition,
            Quaternion localRotation,
            Vector3 localScale
        )
        {
            GameObject child = FindOrCreateSingleChild(
                scene,
                parent,
                objectName,
                () =>
                {
                    GameObject prefab = AssetDatabase.LoadAssetAtPath<GameObject>(prefabPath);
                    return prefab == null ? null : (GameObject)PrefabUtility.InstantiatePrefab(prefab, scene);
                }
            );

            if (child == null)
            {
                throw new InvalidOperationException($"Flagship stage could not instantiate prefab: {prefabPath}");
            }

            child.transform.SetParent(parent.transform, false);
            child.transform.localPosition = localPosition;
            child.transform.localRotation = localRotation;
            child.transform.localScale = localScale;
            child.name = objectName;
            return child;
        }

        private static GameObject FindOrCreateSingleChild(
            Scene scene,
            GameObject parent,
            string objectName,
            Func<GameObject> factory
        )
        {
            GameObject kept = null;
            foreach (GameObject found in FindAllInScene(scene, objectName))
            {
                if (kept == null)
                {
                    kept = found;
                    continue;
                }

                UnityEngine.Object.DestroyImmediate(found);
            }

            if (kept == null)
            {
                kept = factory();
                if (kept == null)
                {
                    return null;
                }

                kept.name = objectName;
                SceneManager.MoveGameObjectToScene(kept, scene);
            }

            kept.name = objectName;
            kept.transform.SetParent(parent.transform, false);
            return kept;
        }

        private static void SetMaterial(GameObject gameObject, Color color, float smoothness)
        {
            SetMaterial(gameObject, color, smoothness, false);
        }

        private static void SetMaterial(GameObject gameObject, Color color, float smoothness, bool emissive)
        {
            Renderer renderer = gameObject.GetComponent<Renderer>();
            if (renderer == null)
            {
                return;
            }

            Material material = renderer.sharedMaterial;
            if (material == null || !material.name.StartsWith("ADL Observatory ", StringComparison.Ordinal))
            {
                material = new Material(Shader.Find("Standard"))
                {
                    name = $"ADL Observatory {gameObject.name} Material",
                };
                renderer.sharedMaterial = material;
            }

            material.color = color;
            material.SetFloat("_Glossiness", smoothness);
            if (emissive)
            {
                material.EnableKeyword("_EMISSION");
                material.SetColor("_EmissionColor", color * 1.8f);
            }
            else
            {
                material.DisableKeyword("_EMISSION");
            }
        }

        private static void EnsureAmbientLighting()
        {
            RenderSettings.ambientMode = UnityEngine.Rendering.AmbientMode.Flat;
            RenderSettings.ambientLight = new Color(0.14f, 0.19f, 0.24f, 1f);
            RenderSettings.fog = true;
            RenderSettings.fogColor = new Color(0.08f, 0.16f, 0.2f, 1f);
            RenderSettings.fogDensity = 0.003f;
        }

        private static string AnchorName(int index)
        {
            return $"Flagship Imported Anchor {index + 1}";
        }

        private static string DefaultIfBlank(string value, string fallback)
        {
            return string.IsNullOrWhiteSpace(value) ? fallback : value;
        }

        private static GameObject RequireObject(Scene scene, string objectName)
        {
            GameObject found = FindInScene(scene, objectName);
            if (found == null)
            {
                throw new InvalidOperationException(
                    $"Flagship stage missing required object: {objectName}"
                );
            }

            return found;
        }

        private static GameObject RequireChild(GameObject parent, string objectName)
        {
            Transform found = parent.transform.Find(objectName);
            if (found == null)
            {
                throw new InvalidOperationException(
                    $"Flagship stage missing required child '{objectName}' under '{parent.name}'."
                );
            }

            return found.gameObject;
        }

        private static GameObject FindRoot(Scene scene, string objectName)
        {
            foreach (GameObject rootObject in scene.GetRootGameObjects())
            {
                if (string.Equals(rootObject.name, objectName, StringComparison.Ordinal))
                {
                    return rootObject;
                }
            }

            return null;
        }

        private static GameObject FindInScene(Scene scene, string objectName)
        {
            foreach (GameObject rootObject in scene.GetRootGameObjects())
            {
                GameObject found = FindInTransform(rootObject.transform, objectName);
                if (found != null)
                {
                    return found;
                }
            }

            return null;
        }

        private static List<GameObject> FindAllInScene(Scene scene, string objectName)
        {
            List<GameObject> found = new();
            foreach (GameObject rootObject in scene.GetRootGameObjects())
            {
                FindAllInTransform(rootObject.transform, objectName, found);
            }

            return found;
        }

        private static GameObject FindInTransform(Transform transform, string objectName)
        {
            if (string.Equals(transform.name, objectName, StringComparison.Ordinal))
            {
                return transform.gameObject;
            }

            for (int index = 0; index < transform.childCount; index++)
            {
                GameObject found = FindInTransform(transform.GetChild(index), objectName);
                if (found != null)
                {
                    return found;
                }
            }

            return null;
        }

        private static void FindAllInTransform(Transform transform, string objectName, List<GameObject> found)
        {
            if (string.Equals(transform.name, objectName, StringComparison.Ordinal))
            {
                found.Add(transform.gameObject);
            }

            for (int index = 0; index < transform.childCount; index++)
            {
                FindAllInTransform(transform.GetChild(index), objectName, found);
            }
        }

        private static int CountSceneObjects(string path, string objectHeader)
        {
            int count = 0;
            foreach (string line in File.ReadLines(path))
            {
                if (line.StartsWith(objectHeader, StringComparison.Ordinal))
                {
                    count++;
                }
            }

            return count;
        }
    }
}
