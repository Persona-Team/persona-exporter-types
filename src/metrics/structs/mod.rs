
#[cfg(feature = "disk")]
pub mod disk;
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "system")]
pub mod system;
#[cfg(feature = "components")]
pub mod components;
#[cfg(feature = "processes")]
pub mod processes;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "cpu")]
pub mod cpu;
pub mod server;

#[cfg(feature = "line-protocol")]
pub fn unknown_or_value(v: &str) -> &str {
    if v.is_empty() {
        crate::DEFAULT_UNKNOWN_MESSAGE
    } else {
        v
    }
}


