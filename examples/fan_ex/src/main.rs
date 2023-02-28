use nix::poll::{poll, PollFd, PollFlags};
use std::env;

use naughtyfy::api::*;
use naughtyfy::flags::*;

fn handle_events(fd: i32) {

}

fn main() {
    if env::args().len() != 2 {
        eprintln!("Uasge: ./{} MOUNT", env::args().into_iter().next().unwrap());
        std::process::exit(libc::EXIT_FAILURE);
    }

    println!("Press enter key to exit");

    let fd = fanotify_init(
        FAN_CLOEXEC | FAN_CLASS_CONTENT | FAN_NONBLOCK,
        O_RDONLY | O_LARGEFILE,
    );

    if fd.is_err() {
        eprintln!("fanotify_init");
        std::process::exit(libc::EXIT_SUCCESS);
    }
    let fd = fd.unwrap();
    let path = env::args().into_iter().next().unwrap();
    if fanotify_mark(
        fd,
        FAN_MARK_ADD | FAN_MARK_MOUNT,
        FAN_OPEN_PERM | FAN_CLOSE_WRITE,
        libc::AT_FDCWD,
        path.as_str(),
    )
    .is_err()
    {
        eprintln!("fanotify_mark");
        std::process::exit(libc::EXIT_SUCCESS);
    }

    let mut fds = [PollFd::new(fd, PollFlags::POLLIN)];
    loop {
        let poll_num = poll(&mut fds, -1);
        if poll_num.is_err() {
            eprintln!("poll");
            std::process::exit(libc::EXIT_SUCCESS);
        }
        let poll_num = poll_num.unwrap();
        let mut events: Vec<_>;
        if poll_num > 0 {
            for event in fanotify_read(fd) {
                if event.is_err() {
                    eprintln!("read");
                    std::process::exit(libc::EXIT_SUCCESS);
                }
                itertools::concat(vec![events,event]);
            }
        } else {
            eprintln!("poll_num <= 0!");
            break;
        }
    }
}
