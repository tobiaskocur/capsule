use crate::errors::{Result};
pub fn read_uid_map() -> Result<String> {
    Ok(std::fs::read_to_string("/proc/self/uid_map")?)
}

pub fn read_gid_map() -> Result<String> {
    Ok(std::fs::read_to_string("/proc/self/gid_map")?)
}