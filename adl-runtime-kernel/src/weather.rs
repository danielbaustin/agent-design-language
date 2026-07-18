use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;
use sysinfo::{Components, Disks, Networks, System};

use crate::WeatherConfig;

pub const WEATHER_HEALTH_SCHEMA: &str = "adl.runtime.weather_health.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Observation<T> {
    pub value: Option<T>,
    pub source: String,
}

impl<T> Observation<T> {
    fn available(value: T, source: &str) -> Self {
        Self {
            value: Some(value),
            source: source.to_owned(),
        }
    }

    fn unavailable(source: &str) -> Self {
        Self {
            value: None,
            source: source.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiskWeather {
    pub mount: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GpuWeather {
    pub name: String,
    pub utilization_basis_points: u16,
    pub temperature_millicelsius: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WeatherSample {
    pub platform: String,
    pub cpu_basis_points: Observation<u16>,
    pub per_core_basis_points: Observation<Vec<u16>>,
    pub memory_total_bytes: Observation<u64>,
    pub memory_available_bytes: Observation<u64>,
    pub disks: Observation<Vec<DiskWeather>>,
    pub network_received_bytes: Observation<u64>,
    pub network_transmitted_bytes: Observation<u64>,
    pub max_temperature_millicelsius: Observation<i64>,
    pub gpus: Observation<Vec<GpuWeather>>,
}

impl WeatherSample {
    pub fn memory_used_basis_points(&self) -> Option<u16> {
        let total = self.memory_total_bytes.value?;
        let available = self.memory_available_bytes.value?;
        (total > 0).then(|| {
            (((total.saturating_sub(available) as u128) * 10_000) / total as u128).min(10_000)
                as u16
        })
    }

    pub fn minimum_disk_available_bytes(&self) -> Option<u64> {
        self.disks
            .value
            .as_ref()?
            .iter()
            .map(|disk| disk.available_bytes)
            .min()
    }
}

pub trait WeatherObserver: Send {
    fn sample(&mut self) -> WeatherSample;
}

pub struct SysinfoWeatherObserver {
    system: System,
    disks: Disks,
    networks: Networks,
    components: Components,
    disk_scope: Option<PathBuf>,
}

impl Default for SysinfoWeatherObserver {
    fn default() -> Self {
        Self {
            system: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            components: Components::new_with_refreshed_list(),
            disk_scope: None,
        }
    }
}

impl SysinfoWeatherObserver {
    pub fn for_path(path: impl AsRef<Path>) -> Self {
        Self {
            disk_scope: Some(resolve_scope_path(path.as_ref())),
            ..Self::default()
        }
    }
}

fn resolve_scope_path(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    };
    let mut ancestor = absolute.as_path();
    while !ancestor.exists() {
        let Some(parent) = ancestor.parent() else {
            return absolute;
        };
        ancestor = parent;
    }
    std::fs::canonicalize(ancestor)
        .map(|resolved| resolved.join(absolute.strip_prefix(ancestor).unwrap_or(Path::new(""))))
        .unwrap_or(absolute)
}

impl WeatherObserver for SysinfoWeatherObserver {
    fn sample(&mut self) -> WeatherSample {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.disks.refresh(true);
        self.networks.refresh(true);
        self.components.refresh(true);

        let cpu = to_basis_points(self.system.global_cpu_usage());
        let per_core = self
            .system
            .cpus()
            .iter()
            .map(|core| to_basis_points(core.cpu_usage()))
            .collect();
        let disks = self
            .disks
            .list()
            .iter()
            .map(|disk| DiskWeather {
                mount: disk.mount_point().to_string_lossy().into_owned(),
                total_bytes: disk.total_space(),
                available_bytes: disk.available_space(),
            })
            .collect::<Vec<_>>();
        let disks = scoped_disks(disks, self.disk_scope.as_deref());
        let (received, transmitted) =
            self.networks
                .iter()
                .fold((0_u64, 0_u64), |(received, transmitted), (_, network)| {
                    (
                        received.saturating_add(network.total_received()),
                        transmitted.saturating_add(network.total_transmitted()),
                    )
                });
        let max_temperature = self
            .components
            .iter()
            .filter_map(|component| component.temperature())
            .map(|temperature| (temperature * 1_000.0).round() as i64)
            .max();

        WeatherSample {
            platform: std::env::consts::OS.to_owned(),
            cpu_basis_points: Observation::available(cpu, "sysinfo"),
            per_core_basis_points: Observation::available(per_core, "sysinfo"),
            memory_total_bytes: Observation::available(self.system.total_memory(), "sysinfo"),
            memory_available_bytes: Observation::available(
                self.system.available_memory(),
                "sysinfo",
            ),
            disks: Observation::available(disks, "sysinfo"),
            network_received_bytes: Observation::available(received, "sysinfo"),
            network_transmitted_bytes: Observation::available(transmitted, "sysinfo"),
            max_temperature_millicelsius: Observation {
                value: max_temperature,
                source: "sysinfo".to_owned(),
            },
            gpus: Observation::unavailable("optional_platform_adapter"),
        }
    }
}

fn scoped_disks(mut disks: Vec<DiskWeather>, scope: Option<&Path>) -> Vec<DiskWeather> {
    let Some(scope) = scope else {
        return disks;
    };
    let selected = disks
        .iter()
        .filter(|disk| scope.starts_with(Path::new(&disk.mount)))
        .max_by_key(|disk| Path::new(&disk.mount).components().count())
        .map(|disk| disk.mount.clone());
    disks.retain(|disk| Some(&disk.mount) == selected.as_ref());
    disks
}

fn to_basis_points(percent: f32) -> u16 {
    (percent.clamp(0.0, 100.0) * 100.0).round() as u16
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceState {
    Healthy,
    Warning,
    StopRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShutdownDecision {
    Continue,
    SerializeStateThenStop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GpuProofState {
    Observed,
    UnavailableNotPass,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WeatherHealthReport {
    pub schema: String,
    pub resource_state: ResourceState,
    pub shutdown_decision: ShutdownDecision,
    pub gpu_proof_state: GpuProofState,
    pub cloudwatch_route: String,
    pub sample: WeatherSample,
}

impl WeatherHealthReport {
    pub fn from_sample(
        config: &WeatherConfig,
        sample: WeatherSample,
        previous: ResourceState,
    ) -> Self {
        let resource_state = resource_state(config, &sample, previous);
        let shutdown_decision = if resource_state == ResourceState::StopRequired {
            ShutdownDecision::SerializeStateThenStop
        } else {
            ShutdownDecision::Continue
        };
        let gpu_proof_state = match sample.gpus.value.as_ref() {
            Some(gpus) if !gpus.is_empty() => GpuProofState::Observed,
            _ => GpuProofState::UnavailableNotPass,
        };

        Self {
            schema: WEATHER_HEALTH_SCHEMA.to_owned(),
            resource_state,
            shutdown_decision,
            gpu_proof_state,
            cloudwatch_route: "vector.runtime_v3_cloudwatch_emf".to_owned(),
            sample,
        }
    }
}

pub async fn monitor_until_stop<O, F>(
    config: WeatherConfig,
    mut observer: O,
    mut publish: F,
) -> WeatherHealthReport
where
    O: WeatherObserver,
    F: FnMut(WeatherHealthReport),
{
    let mut previous = ResourceState::Healthy;
    let mut interval = tokio::time::interval(Duration::from_millis(config.sample_millis));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        let report = WeatherHealthReport::from_sample(&config, observer.sample(), previous);
        previous = report.resource_state;
        publish(report.clone());
        if report.shutdown_decision == ShutdownDecision::SerializeStateThenStop {
            return report;
        }
    }
}

pub fn resource_state(
    config: &WeatherConfig,
    sample: &WeatherSample,
    previous: ResourceState,
) -> ResourceState {
    let disk = sample.minimum_disk_available_bytes();
    let memory = sample.memory_used_basis_points();
    let cpu = sample.cpu_basis_points.value;
    let missing_core_evidence = disk.is_none() || memory.is_none() || cpu.is_none();

    if disk.is_some_and(|value| value <= config.disk_stop_free_bytes)
        || memory.is_some_and(|value| value >= config.memory_stop_used_basis_points)
        || cpu.is_some_and(|value| value >= config.cpu_stop_basis_points)
    {
        return ResourceState::StopRequired;
    }

    let recovered = disk.is_some_and(|value| value >= config.disk_recover_free_bytes)
        && memory.is_some_and(|value| value <= config.memory_recover_used_basis_points)
        && cpu.is_some_and(|value| value <= config.cpu_recover_basis_points);
    if previous != ResourceState::Healthy && !recovered {
        return previous;
    }

    if missing_core_evidence {
        return ResourceState::Warning;
    }

    if disk.is_some_and(|value| value <= config.disk_warning_free_bytes)
        || memory.is_some_and(|value| value >= config.memory_warning_used_basis_points)
        || cpu.is_some_and(|value| value >= config.cpu_warning_basis_points)
    {
        ResourceState::Warning
    } else {
        ResourceState::Healthy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_pressure_uses_the_filesystem_containing_continuity_state() {
        let disks = vec![
            DiskWeather {
                mount: "/".to_owned(),
                total_bytes: 100,
                available_bytes: 1,
            },
            DiskWeather {
                mount: "/Volumes/FastWork".to_owned(),
                total_bytes: 1_000,
                available_bytes: 900,
            },
            DiskWeather {
                mount: "/Volumes/unrelated-small-disk".to_owned(),
                total_bytes: 10,
                available_bytes: 0,
            },
        ];

        let selected = scoped_disks(
            disks,
            Some(Path::new("/Volumes/FastWork/runtime-v3/continuity")),
        );

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].mount, "/Volumes/FastWork");
        assert_eq!(selected[0].available_bytes, 900);
    }

    #[cfg(unix)]
    #[test]
    fn disk_scope_resolves_a_symlinked_continuity_ancestor() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let target = root.path().join("target");
        std::fs::create_dir(&target).unwrap();
        let link = root.path().join("continuity-link");
        symlink(&target, &link).unwrap();

        assert_eq!(
            resolve_scope_path(&link.join("future-generation")),
            target.canonicalize().unwrap().join("future-generation")
        );
    }
}
