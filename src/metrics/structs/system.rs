use compact_str::CompactString;
use influxdb_line_protocol::builder::AfterField;
use influxdb_line_protocol::LineProtocolBuilder;
use smallvec::SmallVec;
use sysinfo::LoadAvg;
use crate::metrics::traits::Clear;
use crate::traits::line_protocol::{FromWithMeasurement, DEFAULT_EMPTY_MESSAGE};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct SystemInfo {
    /// Name of your operating system.
    ///
    /// | PLATFORM | NAME |
    /// | --- | --- |
    /// | laptop with **Linux** | "NixOS" |
    /// | PC with **Windows** | "Windows" |
    pub name: CompactString,
    /// kernel version of your OS
    pub kernel_version: CompactString,
    /// kernel version + system name
    pub kernel_long_version: CompactString,
    /// Unique your distribution ID
    pub distribution_id: CompactString,
    /// The family to which your distribution belongs.
    /// for example, if you are using Ubuntu, the field
    /// value will be `["debian"]`, since Ubuntu is a derivative of Debian.
    pub distribution_id_like: SmallVec<[CompactString; 2]>,
    // pub distribution_id_like: SmallVec::<[CompactString; 2]>,
    /// The UNIX time at which the system booted
    pub boot_time: u64,
    /// System uptime
    pub uptime: u64,
    /// Your processor architecture
    pub cpu_arch: CompactString,
    /// Your OS version
    pub os_version: CompactString,
    /// Your hostname
    pub host_name: CompactString,
    /// Metric showing the average load on processor threads
    pub load_average: LoadAverage,


}
/// Load Average structure for `load_avg` field in [CpuListInfo]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Clone)]
pub struct LoadAverage {
    /// LA at one minute
    pub one: f64,
    /// LA at five minutes
    pub five: f64,
    /// LA at fifteen minutes
    pub fifteen: f64,
}

#[cfg(feature = "line-protocol")]
impl FromWithMeasurement<&SystemInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &SystemInfo, measurement: &str) -> Self {
        let mut distro_like = CompactString::from(value.distribution_id_like.join(","));
        if distro_like.is_empty() {
            distro_like = CompactString::from(DEFAULT_EMPTY_MESSAGE);
        }

        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("name", &value.name)
            .tag("kernel_version", &value.kernel_version)
            .tag("kernel_long_version", &value.kernel_long_version)
            .tag("distribution_id", &value.distribution_id)
            .tag("distribution_id_like", distro_like.as_str())
            .tag("cpu_arch", &value.cpu_arch)
            .tag("os_version", &value.os_version)
            .tag("host_name", &value.host_name)
            .field("boot_time", value.boot_time)
            .field("uptime", value.uptime)
            .field("load_average.one", value.load_average.one)
            .field("load_average.five", value.load_average.five)
            .field("load_average.fifteen", value.load_average.fifteen)
    }
}

#[cfg(feature = "from-trait-sysinfo")]


impl Clear for SystemInfo {
    fn clear_dynamic(&mut self) {
        self.name.clear();
        self.kernel_version.clear();
        self.kernel_long_version.clear();
        self.distribution_id.clear();
        self.distribution_id_like.clear();
        self.cpu_arch.clear();
        self.os_version.clear();
        self.host_name.clear();

        // self.load_average.clear();
        // self.boot_time = 0;
        // self.uptime = 0;
    }
}