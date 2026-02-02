use nix::sched::{clone, CloneFlags};
use nix::unistd::{chroot, chdir, sethostname};
use nix::mount::{mount, MsFlags};
use nix::sys::wait::waitpid;
use nix::sys::signal::Signal;
use std::process::Command;

use crate::network;

pub fn run_container(net: &str, ip: &str) -> anyhow::Result<()> {
    let mut stack = vec![0; 1024 * 1024];

    let pid = unsafe {
        clone(
            Box::new(|| {
                sethostname("mini-container").unwrap();

                chroot("rootfs").unwrap();
                chdir("/").unwrap();

                mount(
                    Some("proc"),
                    "/proc",
                    Some("proc"),
                    MsFlags::empty(),
                    None::<&str>,
                )
                .unwrap();

                Command::new("/usr/local/bin/node")
                    .arg("/app/server.js")
                    .status()
                    .unwrap();

                0
            }),
            &mut stack,
            CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWUTS
                | CloneFlags::CLONE_NEWNET,
            Some(Signal::SIGCHLD as i32),
        )?
    };

    // 🔥 wire networking AFTER clone
    network::attach_container(net, pid.as_raw() as u32, ip)?;

    waitpid(pid, None)?;
    Ok(())
}
