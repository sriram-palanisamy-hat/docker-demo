use nix::sched::{clone, CloneFlags};
use nix::unistd::{chroot, chdir, sethostname};
use nix::mount::{mount, MsFlags};
use nix::sys::wait::waitpid;
use nix::sys::signal::Signal;
use std::process::Command;

fn main() {
    let mut stack = vec![0; 1024 * 1024];

    let pid = unsafe {
        clone(
            Box::new(|| {
                println!("setting hostname");
                sethostname("mini-container").unwrap();

                println!(" chroot to rootfs");
                chroot("rootfs").unwrap();
                chdir("/").unwrap();

                println!(" mounting /proc");
                mount(
                    Some("proc"),
                    "/proc",
                    Some("proc"),
                    MsFlags::empty(),
                    None::<&str>,
                )
                .unwrap();

                println!("starting node");
                Command::new("/usr/local/bin/node")
                    .arg("/app/server.js")
                    .status()
                    .unwrap();

                0
            }),
            &mut stack,
            CloneFlags::CLONE_NEWPID
                | CloneFlags::CLONE_NEWNS
                | CloneFlags::CLONE_NEWUTS,
                Some(Signal::SIGCHLD as i32),
        )
        .unwrap()
    };

    waitpid(pid, None).unwrap();
}
