mod sandbox;
mod namespaces;
mod errors;
mod utils;

use namespaces::{current_user_namespace, enter_user_namespace, setup_user_mapping};

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
    println!("user uid: {:?}", utils::read_uid_map());
}

