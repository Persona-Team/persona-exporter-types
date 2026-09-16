use compact_str::CompactString;
use influxdb_line_protocol::builder::AfterField;
use influxdb_line_protocol::LineProtocolBuilder;
use crate::metrics::traits::Clear;
use crate::traits::line_protocol::FromWithMeasurement;

/// The component information primarily consists of data regarding the temperatures of
/// individual components (circuit boards, processor cores, etc.).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentListInfo {
    /// Components count
    pub count: usize,
    /// Checks whether the component field is empty.
    pub is_empty: bool,
    /// Components info, see [`ComponentInfo`]
    pub components: Vec<ComponentInfo>,
}

/// Processor thread information, used in [`ComponentListInfo`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct ComponentInfo {
    /// Component identifier recognized by the system kernel
    pub id: CompactString,
    /// Component name
    pub name: CompactString,
    /// Component temp
    pub temp: f32,
    /// Critical temp
    pub critical_temp: f32,
    /// Max temp of component
    pub max_temp: f32,
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&ComponentInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &ComponentInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("id", &value.id)
            .tag("name", &value.name)
            .field("temp", value.temp as i64)
            .field("critical_temp", value.critical_temp as i64)
            .field("max_temp", value.max_temp as i64)
    }
}

impl Clear for ComponentListInfo {
    fn clear_dynamic(&mut self) {
        self.components.clear();

        // self.is_empty = true;
        // self.count = 0;
    }
}