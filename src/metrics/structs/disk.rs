use compact_str::CompactString;
use influxdb_line_protocol::builder::AfterField;
use influxdb_line_protocol::LineProtocolBuilder;
use crate::traits::line_protocol::FromWithMeasurement;
use crate::metrics::structs::unknown_or_value;
use crate::metrics::traits::Clear;

/// System disk space information for the root directory "/".
/// There is no breakdown by physical storage devices.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct DiskInfo {
    /// Disk name
    pub name: CompactString,
    /// File system, line "ext4", "btrfs"
    pub file_system: CompactString,
    /// Disk kind: HDD / SDD etc.
    pub kind: CompactString,
    /// Total space
    pub total_space: u64,
    /// Available space
    pub available_space: u64,
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&DiskInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &DiskInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("name", unknown_or_value(&value.name))
            .tag("file_system", unknown_or_value(&value.file_system))
            .tag("kind", unknown_or_value(&value.kind))
            .field("total_space", value.total_space)
            .field("available_space", value.available_space)
    }
}

impl Clear for DiskInfo {
    fn clear_dynamic(&mut self) {
        self.name.clear();
        self.file_system.clear();
        self.kind.clear();

        // self.total_space = 0;
        // self.available_space = 0;
    }
}
