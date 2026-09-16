use compact_str::CompactString;
use crate::metrics::*;

/// General structure that contains all the metrics of the working machine
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ServerMetrics {
    #[cfg(feature = "system")]
    /// Display system information
    pub system: Option<structs::system::SystemInfo>,
    #[cfg(feature = "processes")]
    /// Display process list
    pub process_list: Option<structs::processes::ProcessListInfo>,
    #[cfg(feature = "memory")]
    /// RAM memory information
    pub memory: Option<structs::memory::MemoryInfo>,
    #[cfg(feature = "disk")]
    /// Disk indicators
    pub disk: Option<structs::disk::DiskInfo>,
    #[cfg(feature = "network")]
    /// Network indicators
    #[cfg(feature = "network")]
    pub network: Option<structs::network::NetworkInfo>,
    #[cfg(feature = "cpu")]
    /// CPU indicators
    pub cpu: Option<structs::cpu::CpuListInfo>,
    #[cfg(feature = "components")]
    /// Components indicators
    pub components: Option<structs::components::ComponentListInfo>,

    /// Metric showing the average load on processor threads
    // pub load_average: Option<LoadAverage>,
    /// Time in UNIX when the metrics were recorded
    pub time: i64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct SendInfo {
    pub url: CompactString,
    pub server_name: CompactString,
}

use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;
use crate::traits::line_protocol::FromWithMeasurement;

impl FromWithMeasurement<&SendInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &SendInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("url", &*value.url)
            .tag("server_name", &*value.server_name)
            .field("", 0.0)
    }
}