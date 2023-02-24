use libc::AT_FDCWD;
use naughtyfy::flags::*;
use naughtyfy::low_api::*;

fn main() {
    let fd = fanotify_init(FAN_CLASS_NOTIF, 0).unwrap();
    fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "/");
    loop {
        let res = fanotify_read(fd);
        println!("{res:#?}");
    }
}
