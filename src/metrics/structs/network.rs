use compact_str::CompactString;
use influxdb_line_protocol::builder::AfterField;
use influxdb_line_protocol::LineProtocolBuilder;
use crate::metrics::traits::Clear;
use crate::metrics::type_aliases::{ExporterString};
use crate::traits::line_protocol::FromWithMeasurement;

/// Network information
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct NetworkInfo {
    /// The name of your card's network interface
    pub interface_name: CompactString,

    /// Total bytes received since the network card was turned on
    pub total_rx_bytes: u64,
    /// Total data packets received
    pub total_rx_packets: u64,
    /// Total errors when accepting data
    pub total_rx_errors: u64,

    /// Total bytes transferred since the network card was turned on
    pub total_tx_bytes: u64,
    /// Total data packets transferred
    pub total_tx_packets: u64,
    /// Total errors when sending data
    pub total_tx_errors: u64,
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&NetworkInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &NetworkInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("interface_name", &value.interface_name)
            .field("total_rx_bytes", value.total_rx_bytes)
            .field("total_rx_packets", value.total_tx_packets)
            .field("total_rx_errors", value.total_rx_errors)
            .field("total_tx_bytes", value.total_tx_bytes)
            .field("total_tx_packets", value.total_tx_packets)
            .field("total_tx_errors", value.total_tx_errors)
    }
}

impl Clear for NetworkInfo {
    fn clear_dynamic(&mut self) {
        self.interface_name.clear();
        // self.total_rx_bytes = 0;
        // self.total_rx_packets = 0;
        // self.total_rx_errors = 0;
        // self.total_tx_bytes = 0;
        // self.total_tx_packets = 0;
        // self.total_rx_errors = 0;
    }
}