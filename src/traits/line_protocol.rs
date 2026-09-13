use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::*;
use influxdb_line_protocol::LineProtocolBuilder;
use influxdb_line_protocol::builder::AfterField;

const DEFAULT_EMPTY_MESSAGE: &str = "empty";
pub trait FromWithMeasurement<T> {
    fn from_with_name(value: T, measurement: &str) -> Self;
}

pub trait IntoWithMeasurement<T> {
    // type Target;
    fn into_with_name(self, measurement: &str) -> T;
}

pub trait FinishLineProtocol {
    fn finish(self, timestamp: i64) -> Vec<u8>;
}

impl FinishLineProtocol for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn finish(self, timestamp: i64) -> Vec<u8> {
        self.timestamp(timestamp).close_line().build()
    }
}

impl<T, U> IntoWithMeasurement<U> for T
where
    U: FromWithMeasurement<T>,
{
    fn into_with_name(self, measurement: &str) -> U {
        U::from_with_name(self, measurement)
    }
}

impl FromWithMeasurement<&SystemInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &SystemInfo, measurement: &str) -> Self {
        let mut distro_like = value.distribution_id_like.join(",");
        if distro_like.is_empty() {
            distro_like = DEFAULT_EMPTY_MESSAGE.to_string();
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

impl FromWithMeasurement<&DiskInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &DiskInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("name", unknown_or_value(value.name.as_str()))
            .tag("file_system", unknown_or_value(&value.file_system))
            .tag("kind", unknown_or_value(&value.kind))
            .field("total_space", value.total_space)
            .field("available_space", value.available_space)
    }
}

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

impl FromWithMeasurement<&CpuCoreInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &CpuCoreInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .tag("os_name", &value.os_name)
            .field("frequency", value.frequency)
            .field("usage", value.cpu_usage as f64)
    }
}
impl FromWithMeasurement<&CpuListInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &CpuListInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .field("usage", value.cpu_usage as f64)
            .field("threads", value.threads as f64)
            .field("physical_core_count", value.physical_core_count as i64)
    }
}

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
// impl From<&ProcessInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
//     fn from(value: &ProcessInfo) -> Self {
//
//     }
// }

#[allow(unused)]
impl FromWithMeasurement<&SendInfo> for LineProtocolBuilder<Vec<u8>, AfterField> {
    fn from_with_name(value: &SendInfo, measurement: &str) -> Self {
        LineProtocolBuilder::new()
            .measurement(measurement)
            .field("url", &*value.url)
    }
}

fn unknown_or_value(v: &str) -> &str {
    if v.is_empty() {
        DEFAULT_UNKNOWN_MESSAGE
    } else {
        v
    }
}
