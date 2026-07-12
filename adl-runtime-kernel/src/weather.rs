use serde::{Deserialize, Serialize};
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
}

impl Default for SysinfoWeatherObserver {
    fn default() -> Self {
        Self {
            system: System::new_all(),
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
            components: Components::new_with_refreshed_list(),
        }
    }
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
