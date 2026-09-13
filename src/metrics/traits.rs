use crate::metrics::{
    ComponentListInfo, CpuListInfo, DiskInfo, NetworkInfo, ProcessInfo,
    ProcessListInfo, SystemInfo,
};

pub trait Clear {
    /// Очищаем только поля с динамическими типами (String, Vec, HashMap и т.д.)
    /// Делать очистику для простых чисел (int, float, bool, enum) не имеет смысла, так как они всё равно буду перезаписаны
    fn clear_dynamic(&mut self);
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

// impl Clear for DiskUsage {
//     fn clear_dynamic(&mut self) {
//         self.written_bytes = 0;
//         self.read_bytes = 0;
//         self.total_written_bytes = 0;
//         self.total_read_bytes = 0;
//     }
// }

impl Clear for CpuListInfo {
    fn clear_dynamic(&mut self) {
        self.cpu_cores.clear();

        // self.cpu_usage = 0.0;
        // self.threads = 0;
        // self.physical_core_count = 0;
    }
}

impl Clear for ComponentListInfo {
    fn clear_dynamic(&mut self) {
        self.components.clear();

        // self.is_empty = true;
        // self.count = 0;
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

impl Clear for DiskInfo {
    fn clear_dynamic(&mut self) {
        self.name.clear();
        self.file_system.clear();
        self.kind.clear();

        // self.total_space = 0;
        // self.available_space = 0;
    }
}

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

// impl Clear for LoadAverage {
//     fn clear_dynamic(&mut self) {
//         *self = LoadAverage::default();
//     }
// }
//
// impl Clear for MemoryInfo {
//     fn clear_dynamic(&mut self) {
//         *self = MemoryInfo::default();
//     }
// }
