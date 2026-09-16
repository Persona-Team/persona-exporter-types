use influxdb_line_protocol::builder::AfterField;
use influxdb_line_protocol::LineProtocolBuilder;
use crate::traits::line_protocol::FromWithMeasurement;

/// Details system memory information
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct MemoryInfo {
    /// Total amount RAM memory in your system
    pub total_memory: u64,
    /// Used amount RAM memory in your system
    pub used_memory: u64,
    /// Free and physically accessible memory
    pub free_memory: u64,
    /// Available memory that the system can allocate to a
    /// program without compromising the OS.
    pub available_memory: u64,
    /// Total paging file size, see `total_memory`
    pub total_swap: u64,
    /// Used page file size, see `used_memory`
    pub used_swap: u64,
    /// Free and physically accessible swap, see `free_memory`
    pub free_swap: u64,
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&MemoryInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &MemoryInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .field("total_memory", value.total_memory)
            .field("used_memory", value.used_memory)
            .field("free_memory", value.free_memory)
            .field("available_memory", value.available_memory)
            .field("total_swap", value.total_swap)
            .field("used_swap", value.used_swap)
            .field("free_swap", value.free_swap)
    }
}