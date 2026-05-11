use std::fs;
use nix::sched::{unshare, CloneFlags};

use crate::errors::{CapsuleError, Result};

pub fn current_user_namespace() -> Result<String> {
    let path = fs::read_link("/proc/self/ns/user")?;
    Ok(path.display().to_string())
}
pub fn enter_user_namespace() -> Result<()> {
    unshare(CloneFlags::CLONE_NEWUSER)
        .map_err(|e| CapsuleError::Namespace(e.to_string()))?;

    Ok(())
}

// we get our uid/gid, then deny setgroups, and say to the kernel how our namespace-local root should map
// to the host identity, uid/gid_map are files specifically used for the translation of namespace IDs and parent IDs
pub fn setup_user_mapping(host_uid: u32, host_gid: u32) -> Result<()> {
    // we need to add "deny" to setgroups because kernel will deny our rules if not
    // this was added as a security rule due to the abuse of setgroups() in user namespace
    fs::write("/proc/self/setgroups", "deny")?;

    // formatting for uid like 1000 is 0 1000 1
    fs::write("/proc/self/uid_map", format!("0 {} 1\n", host_uid))?;
    fs::write("/proc/self/gid_map", format!("0 {} 1\n", host_gid))?;

    Ok(())
}

pub fn read_uid_map() -> Result<String> {
    Ok(std::fs::read_to_string("/proc/self/uid_map")?)
}

pub fn read_gid_map() -> Result<String> {
    Ok(std::fs::read_to_string("/proc/self/gid_map")?)
}
pub fn become_root_in_namespace() -> Result<()> {
    nix::unistd::setgid(nix::unistd::Gid::from_raw(0))
        .map_err(|e| CapsuleError::Namespace(e.to_string()))?;

    nix::unistd::setuid(nix::unistd::Uid::from_raw(0))
        .map_err(|e| CapsuleError::Namespace(e.to_string()))?;

    Ok(())
}