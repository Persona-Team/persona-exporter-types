use compact_str::CompactString;
use influxdb_line_protocol::builder::AfterField;
use influxdb_line_protocol::LineProtocolBuilder;
use smallvec::SmallVec;
use crate::metrics::traits::Clear;
use crate::traits::line_protocol::FromWithMeasurement;

/// Statistics on the machine's processor
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct CpuListInfo {
    /// System CPU usage. Measured as a percentage from 0%-100%
    pub normalize_cpu_usage: f64,
    /// System CPu usage in one thread
    pub cpu_usage_by_thread: f64,
    /// Total number of logical threads
    pub threads: usize,
    /// Number of physical processor cores
    pub physical_core_count: usize,
    pub cpu_cores: SmallVec<[CpuThreadInfo; 8]>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct CpuThreadInfo {
    pub os_name: CompactString,
    pub frequency: u64,
    pub cpu_usage: f32,
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&CpuThreadInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &CpuThreadInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("os_name", &value.os_name)
            .field("frequency", value.frequency)
            .field("usage", value.cpu_usage as f64)
    }
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&CpuListInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &CpuListInfo, measurement: &str) -> Self {
        let normalize_cpu = if value.threads != 0 {
            value.cpu_usage_by_thread / value.threads as f64
        } else { 0.0 };

        LineProtocolBuilder::new()
            .measurement(measurement)
            .field("cpu_usage_by_thread", value.cpu_usage_by_thread)
            .field("normalize_cpu_usage", normalize_cpu)
            .field("threads", value.threads)
            .field("physical_core_count", value.physical_core_count)
    }
}

impl Clear for CpuListInfo {
    fn clear_dynamic(&mut self) {
        self.cpu_cores.clear();

        // self.cpu_usage = 0.0;
        // self.threads = 0;
        // self.physical_core_count = 0;
    }
}

