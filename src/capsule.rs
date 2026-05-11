use crate::errors::Result;
use crate::namespaces::{
    current_user_namespace,
    enter_user_namespace,
    setup_user_mapping,
    read_uid_map,
    read_gid_map,
    become_root_in_namespace
};

pub fn run() -> Result<()> {
    // retrieve current uid/gid for future use
    let host_uid = nix::unistd::getuid().as_raw();
    let host_gid = nix::unistd::getgid().as_raw();

    println!("host_uid: {}", host_uid);
    println!("host_gid: {}", host_gid);

    println!("PID: {}", std::process::id());


    // now we go through the logic from namespace so
    // get current namespace
    let current_ns = current_user_namespace()?;
    println!("current_user_namespace: {}", current_ns);

    // try to create a new namespace for the process
    enter_user_namespace()?;

    // check if we really changed namespaces
    let new_ns = current_user_namespace()?;
    println!("after_user_namespace: {}", new_ns);

    // first we write our uid/gid mapping through procfs
    setup_user_mapping(host_uid, host_gid)?;
    println!("setup user mapping");

    // then we read them for debug
    let uid_map = read_uid_map()?;
    println!("uid map: {:?}", uid_map);

    let gid_map = read_gid_map()?;
    println!("gid map: {:?}", gid_map);

    println!("before setuid/gid:");
    println!("uid: {}", nix::unistd::getuid().as_raw());
    println!("gid: {}", nix::unistd::getgid().as_raw());

    //  after the mapping is in place we can switch to uid/gid 0 inside this namespace
    become_root_in_namespace()?;
    println!("become root in namespace");

    println!("after setuid/gid:");
    println!("uid: {}", nix::unistd::getuid().as_raw());
    println!("gid: {}", nix::unistd::getgid().as_raw());

    Ok(())
}