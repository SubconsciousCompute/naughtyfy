use naughtyfy::api::*;
use naughtyfy::flags::*;
use naughtyfy::types::*;

/// Using naughtyfy to allow all but `/tmp/tmp.txt`
/// file open event on system.
fn main() {
    let fd = &init(FAN_CLOEXEC | FAN_CLASS_CONTENT, O_RDONLY | O_LARGEFILE);
    if fd.is_err() {
        eprintln!("Encountered err due to {fd:?}");
    }
    let fd = fd.as_ref().unwrap();
    let status = mark(
        &fd,
        FAN_MARK_ADD | FAN_MARK_MOUNT,
        FAN_OPEN_PERM | FAN_CLOSE_WRITE,
        AT_FDCWD,
        "/tmp",
    );
    if status.is_err() {
        eprintln!("Encountered err due to {fd:?}");
    }
    let _status = status.unwrap();

    loop {
        // read_do(fd, print_meta).unwrap();
        let data = read(&fd).unwrap();
        data.iter().for_each(|e| {
            if e.fd >= 0 {
                let path =
                    std::fs::read_link(format!("/proc/self/fd/{}", e.fd)).unwrap_or_default();
                if e.mask & FAN_OPEN_PERM != 0 {
                    if path.to_str().unwrap() == "/tmp/tmp.txt" {
                        println!("Denied: {path:?}");
                        write(
                            &fd,
                            &fanotify_response {
                                fd: e.fd,
                                response: FAN_DENY,
                            },
                        )
                        .unwrap();
                    } else {
                        println!("Allowed: {path:?}");
                        write(
                            &fd,
                            &fanotify_response {
                                fd: e.fd,
                                response: FAN_ALLOW,
                            },
                        )
                        .unwrap();
                    }
                }
                close(e.fd).unwrap();
            }
        });
    }
}
