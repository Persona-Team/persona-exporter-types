use core::fmt;
use std::fmt::Formatter;
use compact_str::{CompactString, ToCompactString};
use influxdb_line_protocol::builder::AfterField;
use influxdb_line_protocol::LineProtocolBuilder;
use sysinfo::Process;
use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::traits::line_protocol::FromWithMeasurement;
use sysinfo::ProcessStatus as SysProcessStatus;
use crate::metrics::traits::Clear;

/// System process information: top N processes and information from the exporter itself.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct ProcessListInfo {
    /// Exporter information
    pub exporter_metrics: Option<ProcessInfo>,
    /// Information on processes
    pub process_list: Vec<ProcessInfo>,
}

/// Information about the process
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub name: CompactString,

    /// Process status at the time of recording. See also [`ProcessStatus`]
    pub status: ProcessStatus,
    /// Disk space usage information. See also [`ProcessDiskUsage`]
    pub disk_usage: ProcessDiskUsage,

    /// Unique process identifier
    pub program_id: CompactString,

    /// Information about process usage (usually as a percentage) at a given point in time
    pub cpu_usage: f32,
    /// Information about RAM usage (in bytes or another unit of measurement) at this moment in time
    pub memory_usage: u64,
    /// Information on swap file usage
    pub virtual_memory: u64,
    /// Process Lifespan in seconds
    pub run_time: u64,
    /// Displays the time the process was started in UNIX time
    pub start_time: u64,
    /// User ID of the user who started the process
    pub user_id: CompactString,
    /// ID of the group that initiated the process
    pub group_id: CompactString,
}

/// Represents the statistics of disk activity for a specific process.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct ProcessDiskUsage {
    /// Number of bytes read from disk since the last update.
    pub read_bytes: u64,
    /// Number of bytes written to disk since the last update.
    pub written_bytes: u64,
    /// Total number of bytes read from disk since the process started.
    pub total_read_bytes: u64,
    /// Total number of bytes written to disk since the process started.
    pub total_written_bytes: u64,
}

/// Represents the current execution state of a process.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum ProcessStatus {
    /// The state of the process cannot be determined.
    #[default]
    Unknown,
    /// The process is actively running on a CPU core.
    Run,
    /// The process is idle and waiting for work (common on BSD).
    Idle,
    /// The process finished execution but its parent has not read its exit code yet.
    Zombie,
    /// The process is sleeping and waiting for an event or signal.
    Sleep,
    /// The process is being inspected by a debugger or tracing tool.
    Tracing,
    /// The process was stopped by a signal (like SIGSTOP).
    Stop,
    /// The process is dead and completely terminated.
    Dead,
    /// The process is in a deep sleep but will wake up to handle a fatal signal.
    Wakekill,
    /// The process is currently moving from sleep to a running state.
    Waking,
    /// The process thread is parked (usually for internal kernel management).
    Parked,
    /// The process is blocked waiting for a lock.
    LockBlocked,
    /// The process is in a deep sleep waiting for disk Input/Output operations.
    UninterruptibleDiskSleep,
    /// The process is suspended or paused (common on macOS).
    Suspended,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for ProcessInfo {
    fn default() -> Self {
        ProcessInfo {
            name: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
            status: ProcessStatus::default(),
            disk_usage: ProcessDiskUsage::default(),
            program_id: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
            cpu_usage: 0.0,
            memory_usage: 0,
            virtual_memory: 0,
            run_time: 0,
            start_time: 0,
            user_id: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
            group_id: DEFAULT_UNKNOWN_MESSAGE.to_string().parse().unwrap(),
        }
    }
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&ProcessInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &ProcessInfo, measurement: &str) -> Self {
        let status = value.status.to_string();
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("name", &value.name)
            .tag("user_id", &value.user_id)
            .tag("group_id", &value.group_id)
            .field("status", &*status)
            .field("disk_usage.read_bytes", value.disk_usage.read_bytes)
            .field("disk_usage.written_bytes", value.disk_usage.written_bytes)
            .field(
                "disk_usage.total_read_bytes",
                value.disk_usage.total_read_bytes,
            )
            .field(
                "disk_usage.total_written_bytes",
                value.disk_usage.total_written_bytes,
            )
            .field("program_id", &*value.program_id)
            .field("cpu_usage", value.cpu_usage as f64)
            .field("memory_usage", value.memory_usage)
            .field("virtual_memory", value.virtual_memory)
            .field("run_time", value.run_time)
            .field("start_time", value.start_time)
    }
}

#[cfg(feature = "from-trait-sysinfo")]
impl From<sysinfo::DiskUsage> for ProcessDiskUsage {
    fn from(value: sysinfo::DiskUsage) -> Self {
        ProcessDiskUsage {
            read_bytes: value.read_bytes,
            written_bytes: value.written_bytes,
            total_read_bytes: value.total_read_bytes,
            total_written_bytes: value.total_written_bytes,
        }
    }
}



impl ProcessInfo {
    pub fn from_process(value: &mut Process) -> Self {
        // let raw_cpu_usage = value.cpu_usage();
        //
        // let calculate_cpu_usage = if raw_cpu_usage != 0.0 || cpu_cores != 0 {
        //   raw_cpu_usage / cpu_cores as f32
        // } else { 0.0 };

        let uid = value
            .user_id()
            .map(|id| id.to_compact_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());
        let gid = value
            .group_id()
            .map(|id| id.to_compact_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_compact_string());
        ProcessInfo {
            name: value.name().to_str().unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE).to_compact_string(),
            status: ProcessStatus::from(value.status()),
            disk_usage: ProcessDiskUsage::from(value.disk_usage()),
            program_id: value.pid().to_compact_string(),
            cpu_usage: value.cpu_usage(),
            memory_usage: value.memory(),
            virtual_memory: value.virtual_memory(),
            run_time: value.run_time(),
            start_time: value.start_time(),
            user_id: uid,
            group_id: gid,
        }
    }
}

impl Clear for ProcessListInfo {
    fn clear_dynamic(&mut self) {
        self.exporter_metrics = None;
        // self.exporter_metrics.get_or_insert_default().clear_dynamic();
        self.process_list.clear();
    }
}

impl Clear for ProcessInfo {
    fn clear_dynamic(&mut self) {
        self.name.clear();
        self.program_id.clear();
        self.user_id.clear();
        self.group_id.clear();

        // self.status = ProcessStatus::Unknown;
        // self.disk_usage.clear();
    }
}