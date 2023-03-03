use naughtyfy::api::*;
use naughtyfy::flags::*;

fn main() {
    let fd = fanotify_init(
        FAN_CLOEXEC
            // | FAN_NONBLOCK
            | FAN_UNLIMITED_QUEUE
            | FAN_UNLIMITED_MARKS
            | FAN_CLASS_NOTIF
            | FAN_CLASS_CONTENT,
        O_RDONLY,
    );
    if fd.is_err() {
        eprintln!("Encountered err due to {fd:?}");
    }
    let fd = fd.unwrap();
    let status = fanotify_mark(
        fd,
        FAN_MARK_ADD | FAN_MARK_MOUNT,
        FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN | FAN_EVENT_ON_CHILD,
        AT_FDCWD,
        "/",
    );
    if status.is_err() {
        eprintln!("Encountered err due to {status:#?}");
    }
    let _status = status.unwrap();

    loop {
        fanotify_read_do(fd, |md| {
            let _path = std::fs::read_link(format!("/proc/self/fd/{}", md.fd)).unwrap_or_default();
            // println!("{:?} at {:?}", md.mask, path);
        })
        .unwrap();
    }
}
