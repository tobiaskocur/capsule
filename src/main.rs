mod sandbox;
mod namespaces;
mod errors;
mod utils;

use namespaces::{
    current_user_namespace,
    enter_user_namespace,
    setup_user_mapping,
    read_uid_map,
    read_gid_map,
    become_root_in_namespace
};

fn main() {
    let host_uid = nix::unistd::getuid().as_raw();
    let host_gid = nix::unistd::getgid().as_raw();

    println!("host_uid: {}", host_uid);
    println!("host_gid: {}", host_gid);

    println!("PID: {}", std::process::id());

    match current_user_namespace() {
        Ok(ns) => println!("current user namespace: {:?}", ns),
        Err(e) => {
            println!("failed to get current user namespace: {}", e);
            return;
        },
    }

    match enter_user_namespace() {
        Ok(()) => println!("entered user namespace"),
        Err(e) => {
            println!("failed to enter user namespace: {}", e);
            return;
        }
    }


    match current_user_namespace() {
        Ok(ns) => println!("after user namespace: {:?}", ns),
        Err(e) => {
            println!("failed to get current user namespace after unshare: {}", e);
            return;
        }
    }

    if let Err(e) = setup_user_mapping(host_uid, host_gid) {
        eprintln!("failed to setup user mapping: {}", e);
        return;
    }

    println!("setup user mapping");

    match read_uid_map() {
        Ok(uid_map) => {
            println!("uid map: {:?}", uid_map);
        },
        Err(e) => {
            eprintln!("failed to read uid map: {}", e);
        }
    }

    match read_gid_map() {
        Ok(gid_map) => {
            println!("gid map: {:?}", gid_map);
        }
        Err(e) => {
            eprintln!("failed to read gid map: {}", e);
        }
    }

    println!("before setuid/gid:");
    println!("uid: {}", nix::unistd::getuid().as_raw());
    println!("gid: {}", nix::unistd::getgid().as_raw());

    if let Err(e) = become_root_in_namespace() {
        eprintln!("failed to become root in namespace: {}", e);
        return;
    }

    println!("become root in namespace");
    println!("after setuid/gid:");
    println!("uid: {}", nix::unistd::getuid().as_raw());
    println!("gid: {}", nix::unistd::getgid().as_raw());
}

