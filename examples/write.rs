use naughtyfy::api::*;
use naughtyfy::flags::*;
fn main() {
    let fd = fanotify_init(
        FAN_CLOEXEC | FAN_CLASS_CONTENT | FAN_NONBLOCK,
        O_RDONLY | O_LARGEFILE,
    );
    match fd {
        Ok(fd) => {
            let m = fanotify_mark(
                fd,
                FAN_MARK_ADD | FAN_MARK_MOUNT,
                FAN_OPEN_PERM | FAN_CLOSE_WRITE,
                libc::AT_FDCWD,
                "./testfd",
            );
            assert!(m.is_ok());
            assert!(fd >= 0);

            loop {
                println!("In loop");
                let events = fanotify_read(fd).unwrap();
                if events.len() > 1 {
                    for event in events {
                        println!("{event:#?}");
                        if event.mask & FAN_OPEN_PERM != 0 {
                            let res = fanotify_write(event.fd, FAN_ALLOW);
                            if res.is_ok() {
                                println!("Writte {}bytes.", res.unwrap());
                            }
                        }
                    }
                }
            }
            // fanotify_close(fd).unwrap();
        }
        Err(e) => {
            // This can fail for multiple reason, most common being privileges.
            eprintln!("Cannot get fd due to {e}");
            assert!(e.code != 0);
        }
    }
}
