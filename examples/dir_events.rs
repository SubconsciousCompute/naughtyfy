use naughtyfy::api::*;
use naughtyfy::flags::*;
use naughtyfy::types::Fd;
use naughtyfy::types::FdToPath;

/// Using naughtyfy to report(print) all
/// file access, modify, close, open events (for files)
/// on a specific dir recursively
fn main() {
    let fd = init(
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
    let fd = fd.as_ref().unwrap();
    let status = mark(
        fd,
        FAN_MARK_ADD | FAN_MARK_MOUNT,
        FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN | FAN_EVENT_ON_CHILD,
        AT_FDCWD,
        "/tmp",
    );
    if status.is_err() {
        eprintln!("Encountered err due to {status:#?}");
    }
    let _status = status.unwrap();

    loop {
        read_do(fd, |md| {
            let path = Fd::path_from_rawfd(md.fd);
            println!("{:?} at {:?}", md.mask, path);
        })
        .unwrap();
    }
}
