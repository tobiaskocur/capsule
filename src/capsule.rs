use nix::unistd::{fork, ForkResult, getpid, getppid};
use nix::sys::wait::{waitpid, WaitStatus};
use crate::errors::{CapsuleError, Result};
use crate::namespaces::{
    current_user_namespace,
    enter_user_namespace,
    setup_user_mapping,
    read_uid_map,
    read_gid_map,
    become_root_in_namespace,
    current_pid_namespace,
    enter_pid_namespace
};

pub fn run() -> Result<()> {
    // retrieve current uid/gid for future use
    let host_uid = nix::unistd::getuid().as_raw();
    let host_gid = nix::unistd::getgid().as_raw();

    println!("host_uid: {}", host_uid);
    println!("host_gid: {}", host_gid);

    println!("PID: {}", std::process::id());


    // get current namespace
    let current_namespace = current_user_namespace()?;
    println!("current_user_namespace: {}", current_namespace);

    // try to create a new namespace for the process
    enter_user_namespace()?;

    // check if we really changed namespaces
    let new_namespace = current_user_namespace()?;
    println!("after_user_namespace: {}", new_namespace);

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


    // retrieve the current pid namespace before changing
    let current_pid_ns = current_pid_namespace()?;
    println!("before entering new namespace: {}", current_pid_ns);

    // try to enter a new pid namespace
    enter_pid_namespace()?;

    // SAFETY: we call fork in a controlled sandbox bootstrap path
    // the child branch will stay minimal and should eventually exec the target process

    // we create a child process because after entering the new pid namespace only the children are inside it not the current process
    // check for child result and we debug our new pid and compare the new pid namespace
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("parent pid: {}", child); // for debug purposes only, never use println! in a fork in production
            let status = waitpid(child, None)
                .map_err(|e| CapsuleError::Namespace(e.to_string()))?;

            match status {
                WaitStatus::Exited(pid, exit_code) => {
                    println!("child {} exited with status {}", pid, exit_code);
                }
                WaitStatus::Signaled(pid, signal, _) => {
                    println!("child {} was killed by signal {:?}", pid, signal);
                }
                other => {
                    println!("child changed state: {:?}", other);
                }
            }
        }

        Ok(ForkResult::Child) => {
            println!("child: inside new pid namespace");
            println!("child pid: {}", getpid());
            println!("child parent pid: {}", getppid());

            let pid_ns = current_pid_namespace()?;
            println!("current pid namespace: {}", pid_ns);
        }

        Err(e) => return Err(CapsuleError::Namespace(e.to_string())),
    }

    Ok(())
}