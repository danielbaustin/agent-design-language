//! Small Runtime v3 host-resource weather report.
//!
//! The weather service is intentionally a thin `sysinfo` wrapper plus ADL
//! pressure policy. Vector/CloudWatch delivery stays in the observability
//! component; this module only emits a bounded event shape for that pipeline.

use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sysinfo::{Disks, System, MINIMUM_CPU_UPDATE_INTERVAL};

pub const WEATHER_SCHEMA: &str = "adl.runtime_v3.weather.v1";
pub const CLOUDWATCH_EVENT_SCHEMA: &str = "adl.runtime_v3.weather.cloudwatch_event.v1";
pub const DEFAULT_CPU_STOP_PERCENT: f32 = 99.0;
pub const DEFAULT_MEMORY_STOP_PERCENT: f32 = 95.0;
pub const DEFAULT_DISK_STOP_PERCENT: f32 = 97.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WeatherHealth {
    Nominal,
    Degraded,
    GracefulStopRequired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PressureKind {
    Cpu,
    Memory,
    Disk,
    Gpu,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GpuTelemetryStatus {
    Observed,
    Unavailable,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherThresholds {
    pub cpu_stop_percent: f32,
    pub memory_stop_percent: f32,
    pub disk_stop_percent: f32,
}

impl Default for WeatherThresholds {
    fn default() -> Self {
        Self {
            cpu_stop_percent: DEFAULT_CPU_STOP_PERCENT,
            memory_stop_percent: DEFAULT_MEMORY_STOP_PERCENT,
            disk_stop_percent: DEFAULT_DISK_STOP_PERCENT,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CpuWeather {
    pub average_usage_percent: f32,
    pub logical_cpus: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryWeather {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub used_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiskWeather {
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpuWeather {
    pub status: GpuTelemetryStatus,
    pub provider: String,
    pub used_percent: Option<f32>,
    pub reason_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherPressure {
    pub kind: PressureKind,
    pub observed_percent: f32,
    pub threshold_percent: f32,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SerializationBudget {
    pub payload_bytes: u64,
    pub elapsed_micros: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherReport {
    pub schema: String,
    pub sampled_at_epoch_ms: u128,
    pub health: WeatherHealth,
    pub shutdown_decision: String,
    pub cpu: CpuWeather,
    pub memory: MemoryWeather,
    pub disk: DiskWeather,
    pub gpu: GpuWeather,
    pub pressure: Vec<WeatherPressure>,
    pub thresholds: WeatherThresholds,
    pub serialization_budget: SerializationBudget,
    pub cloudwatch_delivery: String,
}

impl WeatherReport {
    pub fn cloudwatch_event(&self) -> Value {
        json!({
            "schema": CLOUDWATCH_EVENT_SCHEMA,
            "_aws": {
                "Timestamp": self.sampled_at_epoch_ms,
                "CloudWatchMetrics": [{
                    "Namespace": "ADL/RuntimeV3",
                    "Dimensions": [["component"]],
                    "Metrics": [
                        {"Name": "cpu_used_percent", "Unit": "Percent"},
                        {"Name": "memory_used_percent", "Unit": "Percent"},
                        {"Name": "disk_used_percent", "Unit": "Percent"},
                        {"Name": "serialization_elapsed_micros", "Unit": "Microseconds"}
                    ]
                }]
            },
            "component": "runtime_v3_weather",
            "health": self.health,
            "shutdown_decision": self.shutdown_decision,
            "cpu_used_percent": self.cpu.average_usage_percent,
            "memory_used_percent": self.memory.used_percent,
            "disk_used_percent": self.disk.used_percent,
            "serialization_elapsed_micros": self.serialization_budget.elapsed_micros,
            "gpu_status": self.gpu.status,
            "gpu_reason_code": self.gpu.reason_code,
            "pressure_count": self.pressure.len()
        })
    }

    pub fn should_serialize_and_stop(&self) -> bool {
        self.health == WeatherHealth::GracefulStopRequired
    }
}

pub fn collect_weather(
    state_payload: &[u8],
    state_root: impl AsRef<Path>,
    thresholds: WeatherThresholds,
) -> WeatherReport {
    let mut system = System::new_all();
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_cpu_usage();
    let disks = Disks::new_with_refreshed_list();

    let cpu = cpu_weather(&system);
    let memory = memory_weather(&system);
    let disk = disk_weather(&disks, state_root.as_ref());
    let gpu = deferred_gpu_weather();
    weather_report_from_parts(cpu, memory, disk, gpu, thresholds, state_payload)
}

pub fn weather_report_from_parts(
    cpu: CpuWeather,
    memory: MemoryWeather,
    disk: DiskWeather,
    gpu: GpuWeather,
    thresholds: WeatherThresholds,
    state_payload: &[u8],
) -> WeatherReport {
    let serialization_budget = measure_serialization_budget(state_payload);
    let mut pressure = Vec::new();
    if cpu.average_usage_percent >= thresholds.cpu_stop_percent {
        pressure.push(pressure_event(
            PressureKind::Cpu,
            cpu.average_usage_percent,
            thresholds.cpu_stop_percent,
        ));
    }
    if memory.used_percent >= thresholds.memory_stop_percent {
        pressure.push(pressure_event(
            PressureKind::Memory,
            memory.used_percent,
            thresholds.memory_stop_percent,
        ));
    }
    if disk.used_percent >= thresholds.disk_stop_percent {
        pressure.push(pressure_event(
            PressureKind::Disk,
            disk.used_percent,
            thresholds.disk_stop_percent,
        ));
    }

    let health = if pressure.is_empty() {
        WeatherHealth::Nominal
    } else {
        WeatherHealth::GracefulStopRequired
    };
    let shutdown_decision = if health == WeatherHealth::GracefulStopRequired {
        "serialize_state_then_stop".to_string()
    } else {
        "continue".to_string()
    };

    WeatherReport {
        schema: WEATHER_SCHEMA.to_string(),
        sampled_at_epoch_ms: epoch_millis_now(),
        health,
        shutdown_decision,
        cpu,
        memory,
        disk,
        gpu,
        pressure,
        thresholds,
        serialization_budget,
        cloudwatch_delivery: "vector_cloudwatch_emf_event".to_string(),
    }
}

fn cpu_weather(system: &System) -> CpuWeather {
    let logical_cpus = system.cpus().len();
    let average_usage_percent = if logical_cpus == 0 {
        0.0
    } else {
        system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / logical_cpus as f32
    };
    CpuWeather {
        average_usage_percent,
        logical_cpus,
    }
}

fn memory_weather(system: &System) -> MemoryWeather {
    let total_bytes = system.total_memory();
    let available_bytes = system.available_memory();
    let used_bytes = total_bytes.saturating_sub(available_bytes);
    let used_percent = percent(used_bytes, total_bytes);
    MemoryWeather {
        total_bytes,
        used_bytes,
        used_percent,
    }
}

fn disk_weather(disks: &Disks, state_root: &Path) -> DiskWeather {
    let selected = disks
        .iter()
        .filter(|disk| path_starts_with(state_root, disk.mount_point()))
        .max_by_key(|disk| disk.mount_point().as_os_str().len())
        .or_else(|| disks.iter().next());
    if let Some(disk) = selected {
        let total_bytes = disk.total_space();
        let available_bytes = disk.available_space();
        let used_bytes = total_bytes.saturating_sub(available_bytes);
        DiskWeather {
            mount_point: disk.mount_point().display().to_string(),
            total_bytes,
            available_bytes,
            used_percent: percent(used_bytes, total_bytes),
        }
    } else {
        DiskWeather {
            mount_point: state_root.display().to_string(),
            total_bytes: 0,
            available_bytes: 0,
            used_percent: 0.0,
        }
    }
}

fn deferred_gpu_weather() -> GpuWeather {
    GpuWeather {
        status: GpuTelemetryStatus::Deferred,
        provider: "external_gpu_probe".to_string(),
        used_percent: None,
        reason_code: "gpu_host_not_available_in_local_proof".to_string(),
    }
}

fn pressure_event(
    kind: PressureKind,
    observed_percent: f32,
    threshold_percent: f32,
) -> WeatherPressure {
    WeatherPressure {
        kind,
        observed_percent,
        threshold_percent,
        action: "serialize_state_then_stop".to_string(),
    }
}

fn measure_serialization_budget(payload: &[u8]) -> SerializationBudget {
    let started = Instant::now();
    let encoded = serde_json::to_vec(&json!({
        "schema": "adl.runtime_v3.weather.serialization_probe.v1",
        "payload_bytes": payload.len(),
        "payload": payload,
    }))
    .unwrap_or_default();
    SerializationBudget {
        payload_bytes: encoded.len() as u64,
        elapsed_micros: started.elapsed().as_micros(),
    }
}

fn percent(used: u64, total: u64) -> f32 {
    if total == 0 {
        0.0
    } else {
        ((used as f64 / total as f64) * 100.0) as f32
    }
}

fn path_starts_with(path: &Path, prefix: &Path) -> bool {
    let path = normalize_for_prefix(path);
    let prefix = normalize_for_prefix(prefix);
    path.starts_with(prefix)
}

fn normalize_for_prefix(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn epoch_millis_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_report(thresholds: WeatherThresholds) -> WeatherReport {
        weather_report_from_parts(
            CpuWeather {
                average_usage_percent: 12.5,
                logical_cpus: 8,
            },
            MemoryWeather {
                total_bytes: 100,
                used_bytes: 60,
                used_percent: 60.0,
            },
            DiskWeather {
                mount_point: "/".to_string(),
                total_bytes: 100,
                available_bytes: 50,
                used_percent: 50.0,
            },
            deferred_gpu_weather(),
            thresholds,
            b"runtime-state",
        )
    }

    #[test]
    fn weather_report_stays_nominal_below_stop_thresholds() {
        let report = fixture_report(WeatherThresholds::default());
        assert_eq!(report.schema, WEATHER_SCHEMA);
        assert_eq!(report.health, WeatherHealth::Nominal);
        assert_eq!(report.shutdown_decision, "continue");
        assert!(!report.should_serialize_and_stop());
        assert_eq!(report.gpu.status, GpuTelemetryStatus::Deferred);
    }

    #[test]
    fn disk_pressure_requests_serialize_then_stop() {
        let report = weather_report_from_parts(
            CpuWeather {
                average_usage_percent: 10.0,
                logical_cpus: 4,
            },
            MemoryWeather {
                total_bytes: 100,
                used_bytes: 20,
                used_percent: 20.0,
            },
            DiskWeather {
                mount_point: "/".to_string(),
                total_bytes: 100,
                available_bytes: 1,
                used_percent: 99.0,
            },
            deferred_gpu_weather(),
            WeatherThresholds::default(),
            b"state",
        );
        assert_eq!(report.health, WeatherHealth::GracefulStopRequired);
        assert!(report.should_serialize_and_stop());
        assert_eq!(report.pressure[0].kind, PressureKind::Disk);
        assert_eq!(report.pressure[0].action, "serialize_state_then_stop");
        let event = report.cloudwatch_event();
        assert_eq!(event["health"], "graceful_stop_required");
        assert_eq!(event["shutdown_decision"], "serialize_state_then_stop");
        assert_eq!(event["pressure_count"], 1);
    }

    #[test]
    fn cloudwatch_event_is_vector_deliverable_without_credentials() {
        let event = fixture_report(WeatherThresholds::default()).cloudwatch_event();
        assert_eq!(event["schema"], CLOUDWATCH_EVENT_SCHEMA);
        assert_eq!(event["component"], "runtime_v3_weather");
        assert_eq!(event["health"], "nominal");
        assert_eq!(
            event["_aws"]["CloudWatchMetrics"][0]["Namespace"],
            "ADL/RuntimeV3"
        );
        assert!(event.get("account_id").is_none());
        assert!(event.get("access_key").is_none());
    }

    #[test]
    fn collect_weather_reports_local_host_shape() {
        let report = collect_weather(b"state", ".", WeatherThresholds::default());
        assert_eq!(report.schema, WEATHER_SCHEMA);
        assert!(report.cpu.logical_cpus > 0);
        assert!(report.memory.total_bytes >= report.memory.used_bytes);
        assert!(report.serialization_budget.payload_bytes > 0);
    }
}
