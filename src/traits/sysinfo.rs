use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::additional_structs::*;
use crate::metrics::{LoadAverage, ProcessInfo};
use sysinfo::{LoadAvg, Process, ProcessStatus as SysProcessStatus};

impl From<sysinfo::DiskUsage> for DiskUsage {
    fn from(value: sysinfo::DiskUsage) -> Self {
        DiskUsage {
            read_bytes: value.read_bytes,
            written_bytes: value.written_bytes,
            total_read_bytes: value.total_read_bytes,
            total_written_bytes: value.total_written_bytes,
        }
    }
}

impl From<SysProcessStatus> for ProcessStatus {
    fn from(value: SysProcessStatus) -> Self {
        match value {
            SysProcessStatus::Idle => ProcessStatus::Idle,
            SysProcessStatus::Run => ProcessStatus::Run,
            SysProcessStatus::Sleep => ProcessStatus::Sleep,
            SysProcessStatus::Stop => ProcessStatus::Stop,
            SysProcessStatus::Zombie => ProcessStatus::Zombie,
            SysProcessStatus::Tracing => ProcessStatus::Tracing,
            SysProcessStatus::Dead => ProcessStatus::Dead,
            SysProcessStatus::Wakekill => ProcessStatus::Wakekill,
            SysProcessStatus::Waking => ProcessStatus::Waking,
            SysProcessStatus::Parked => ProcessStatus::Parked,
            SysProcessStatus::LockBlocked => ProcessStatus::LockBlocked,
            SysProcessStatus::UninterruptibleDiskSleep => ProcessStatus::UninterruptibleDiskSleep,
            SysProcessStatus::Suspended => ProcessStatus::Suspended,
            _ => ProcessStatus::Unknown,
        }
    }
}

impl ProcessInfo {
    pub fn from_process(value: &mut Process) -> Self {
        // let raw_cpu_usage = value.cpu_usage();
        //
        // let calculate_cpu_usage = if raw_cpu_usage != 0.0 || cpu_cores != 0 {
        //   raw_cpu_usage / cpu_cores as f32
        // } else { 0.0 };

        let uid = value
            .user_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_string());
        let gid = value
            .group_id()
            .map(|id| id.to_string())
            .unwrap_or_else(|| DEFAULT_UNKNOWN_MESSAGE.to_string());
        ProcessInfo {
            name: value.name().to_str().unwrap().to_string(),
            status: ProcessStatus::from(value.status()),
            disk_usage: DiskUsage::from(value.disk_usage()),
            program_id: value.pid().to_string(),
            cpu_usage: value.cpu_usage(),
            memory_usage: value.memory(),
            virtual_memory: value.virtual_memory(),
            run_time: value.run_time(),
            start_time: value.start_time(),
            user_id: uid,
            group_id: gid,
        }
    }
}

impl From<&Process> for ProcessInfo {
    fn from(value: &Process) -> Self {
        let uid = value
            .user_id()
            .map(|id| id.to_string())
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string());
        let gid = value
            .group_id()
            .map(|id| id.to_string())
            .unwrap_or(DEFAULT_UNKNOWN_MESSAGE.to_string());

        ProcessInfo {
            name: value.name().to_str().unwrap().to_string(),
            status: ProcessStatus::from(value.status()),
            disk_usage: DiskUsage::from(value.disk_usage()),
            program_id: value.pid().to_string(),
            cpu_usage: value.cpu_usage(),
            memory_usage: value.memory(),
            virtual_memory: value.virtual_memory(),
            run_time: value.run_time(),
            start_time: value.start_time(),
            user_id: uid,
            group_id: gid,
        }
    }
}

impl From<LoadAvg> for LoadAverage {
    fn from(value: LoadAvg) -> Self {
        LoadAverage {
            one: value.one,
            five: value.five,
            fifteen: value.fifteen,
        }
    }
}
