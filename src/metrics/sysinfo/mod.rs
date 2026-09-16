use sysinfo::{LoadAvg, Process};
use crate::DEFAULT_UNKNOWN_MESSAGE;
use crate::metrics::structs::processes::{ProcessDiskUsage, ProcessInfo, ProcessStatus};
use crate::metrics::structs::system::LoadAverage;
use sysinfo::ProcessStatus as SysProcessStatus;
impl From<LoadAvg> for LoadAverage {
    fn from(value: LoadAvg) -> Self {
        LoadAverage {
            one: value.one,
            five: value.five,
            fifteen: value.fifteen,
        }
    }
}

#[cfg(feature = "from-trait-sysinfo")]
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
            name: value.name().to_str().unwrap().to_string().parse().unwrap(),
            status: ProcessStatus::from(value.status()),
            disk_usage: ProcessDiskUsage::from(value.disk_usage()),
            program_id: value.pid().to_string().parse().unwrap(),
            cpu_usage: value.cpu_usage(),
            memory_usage: value.memory(),
            virtual_memory: value.virtual_memory(),
            run_time: value.run_time(),
            start_time: value.start_time(),
            user_id: uid.parse().unwrap(),
            group_id: gid.parse().unwrap(),
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