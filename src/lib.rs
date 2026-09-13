//! [![GitHub]](https://github.com/0DoubleDare/persona-exporter-types)&ensp;[![crates-io]](https://crates.io/crates/persona-exporter-types)&ensp;[![docs-rs]](crate)
//!
//! [GitHub]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! **Important**: *The crate is constantly updated and can undergo
//! significant changes.*
//!
//! A separate, lightweight crate containing all the structures
//! to be populated with the corresponding information.
//! This crate is used in a custom [exporter](https://github.com/0DoubleDare/persona-exporter).
//!
//! **Also important**:
//! All types and fields are primarily based on the [sysinfo](https://docs.rs/sysinfo/0.39.5/sysinfo/)
//! metrics collection library, but you are free to use them however
//! and in whatever context you wish. The documentation simply describes
//! the purpose of the fields from the perspective of this crate's developer
//! and is relevant only if you are using [sysinfo](https://docs.rs/sysinfo/0.39.5/sysinfo/)
//! (That does, however, absolve me of the responsibility of finishing the documentation :D )
//!
//! Now, here is a brief summary of the information, making full use of
//! all the structures like [`metrics::SystemInfo`], [`metrics::MemoryInfo`], [`metrics::DiskInfo`], [`metrics::NetworkInfo`],
//! [`metrics::CpuListInfo`],
//! [`metrics::ComponentListInfo`] with [`metrics::ComponentInfo`],
//! [`metrics::ProcessListInfo`] with [`metrics::ProcessInfo`] :
//!
//! ```json
//!{
//!   "system": {
//!     "name": "NixOS",
//!     "kernel_version": "7.0.10-zen1",
//!     "kernel_long_version": "Linux 7.0.10-zen1",
//!     "distribution_id": "nixos",
//!     "distribution_id_like": [],
//!     "boot_time": 1786947931,
//!     "uptime": 85991,
//!     "cpu_arch": "x86_64",
//!     "os_version": "26.05",
//!     "host_name": "nikita",
//!     "load_average": {
//!       "one": 1.1,
//!       "five": 1.31,
//!       "fifteen": 1.35
//!     }
//!   },
//!   "process_list": {
//!     "exporter_metrics": {
//!       "name": "persona-exporte",
//!       "status": "Run",
//!       "disk_usage": {
//!         "read_bytes": 0,
//!         "written_bytes": 139284480,
//!         "total_read_bytes": 0,
//!         "total_written_bytes": 139284480
//!       },
//!       "program_id": "353498",
//!       "cpu_usage": 0,
//!       "memory_usage": 67641344,
//!       "virtual_memory": 1651945472,
//!       "run_time": 1,
//!       "start_time": 1787033921,
//!       "user_id": "1000",
//!       "group_id": "100"
//!     },
//!     "process_list": [
//!       {
//!         "name": "WrGlyph~terizer",
//!         "status": "Sleep",
//!         "disk_usage": {
//!           "read_bytes": 1531904,
//!           "written_bytes": 0,
//!           "total_read_bytes": 1531904,
//!           "total_written_bytes": 0
//!         },
//!         "program_id": "6726",
//!         "cpu_usage": 0,
//!         "memory_usage": 805584896,
//!         "virtual_memory": 13410684928,
//!         "run_time": 85932,
//!         "start_time": 1786947990,
//!         "user_id": "1000",
//!         "group_id": "100"
//!       },
//!       {
//!         "name": "WRWorkerLP#2",
//!         "status": "Sleep",
//!         "disk_usage": {
//!           "read_bytes": 0,
//!           "written_bytes": 0,
//!           "total_read_bytes": 0,
//!           "total_written_bytes": 0
//!         },
//!         "program_id": "6720",
//!         "cpu_usage": 0,
//!         "memory_usage": 805584896,
//!         "virtual_memory": 13410684928,
//!         "run_time": 85932,
//!         "start_time": 1786947990,
//!         "user_id": "1000",
//!         "group_id": "100"
//!       },
//!       {
//!         "name": "gdbus",
//!         "status": "Sleep",
//!         "disk_usage": {
//!           "read_bytes": 0,
//!           "written_bytes": 0,
//!           "total_read_bytes": 0,
//!           "total_written_bytes": 0
//!         },
//!         "program_id": "1529",
//!         "cpu_usage": 0,
//!         "memory_usage": 10133504,
//!         "virtual_memory": 398196736,
//!         "run_time": 85985,
//!         "start_time": 1786947937,
//!         "user_id": "0",
//!         "group_id": "0"
//!       },
//!       {
//!         "name": "Worker Launcher",
//!         "status": "Sleep",
//!         "disk_usage": {
//!           "read_bytes": 8192,
//!           "written_bytes": 0,
//!           "total_read_bytes": 8192,
//!           "total_written_bytes": 0
//!         },
//!         "program_id": "6942",
//!         "cpu_usage": 0,
//!         "memory_usage": 111058944,
//!         "virtual_memory": 2768277504,
//!         "run_time": 85930,
//!         "start_time": 1786947992,
//!         "user_id": "1000",
//!         "group_id": "100"
//!       },
//!       {
//!         "name": "Isolated Web Co",
//!         "status": "Sleep",
//!         "disk_usage": {
//!           "read_bytes": 11702272,
//!           "written_bytes": 0,
//!           "total_read_bytes": 11702272,
//!           "total_written_bytes": 0
//!         },
//!         "program_id": "180689",
//!         "cpu_usage": 0,
//!         "memory_usage": 858603520,
//!         "virtual_memory": 3765645312,
//!         "run_time": 59960,
//!         "start_time": 1786973962,
//!         "user_id": "1000",
//!         "group_id": "100"
//!       }
//!     ]
//!   },
//!   "memory": {
//!     "total_memory": 16543600640,
//!     "used_memory": 9833271296,
//!     "free_memory": 503083008,
//!     "available_memory": 6710329344,
//!     "total_swap": 25451552768,
//!     "used_swap": 917835776,
//!     "free_swap": 24533716992
//!   },
//!   "disk": {
//!     "name": "/dev/nvme0n1p2",
//!     "file_system": "ext4",
//!     "kind": "SSD",
//!     "total_space": 501889327104,
//!     "available_space": 86368546816
//!   },
//!   "network": {
//!     "interface_name": "wlp0s20f3",
//!     "total_rx_bytes": 7457014974,
//!     "total_rx_packets": 5190777,
//!     "total_rx_errors": 0,
//!     "total_tx_bytes": 284549525,
//!     "total_tx_packets": 2292098,
//!     "total_tx_errors": 0
//!   },
//!   "cpu": {
//!     "cpu_usage": 14.814815,
//!     "threads": 8,
//!     "physical_core_count": 4
//!   },
//!   "components": {
//!     "count": 8,
//!     "is_empty": false,
//!     "components": [
//!       {
//!         "id": "hwmon4_1",
//!         "name": "coretemp Package id 0",
//!         "temp": 84,
//!         "critical_temp": 100,
//!         "max_temp": 84
//!       },
//!       {
//!         "id": "hwmon4_2",
//!         "name": "coretemp Core 0",
//!         "temp": 84,
//!         "critical_temp": 100,
//!         "max_temp": 84
//!       },
//!       {
//!         "id": "hwmon4_5",
//!         "name": "coretemp Core 3",
//!         "temp": 54,
//!         "critical_temp": 100,
//!         "max_temp": 54
//!       },
//!       {
//!         "id": "hwmon4_4",
//!         "name": "coretemp Core 2",
//!         "temp": 64,
//!         "critical_temp": 100,
//!         "max_temp": 64
//!       },
//!       {
//!         "id": "hwmon4_3",
//!         "name": "coretemp Core 1",
//!         "temp": 60,
//!         "critical_temp": 100,
//!         "max_temp": 60
//!       },
//!       {
//!         "id": "hwmon0_1",
//!         "name": "nvme Composite 511BS0512HB",
//!         "temp": 27.85,
//!         "critical_temp": 79.85,
//!         "max_temp": 27.85
//!       },
//!       {
//!         "id": "hwmon5_1",
//!         "name": "iwlwifi_1 temp1",
//!         "temp": 44,
//!         "critical_temp": 0,
//!         "max_temp": 44
//!       },
//!       {
//!         "id": "hwmon3_1",
//!         "name": "acpitz temp1",
//!         "temp": 74,
//!         "critical_temp": 0,
//!         "max_temp": 74
//!       }
//!     ]
//!   },
//!   "time": 1787033923350927711
//! }
//!
//! In addition to the structures themselves, the crate provides
//! functions for converting them: see [`ConvertTo`]

pub const DEFAULT_UNKNOWN_MESSAGE: &str = "unknown";
pub mod metrics;
pub mod traits;

pub use traits::data_unit as types;
