use naughtyfy::low_layer::*;
use naughtyfy::flags::*;
use libc::AT_FDCWD;

fn main() {
    let fd = fanotify_init(FAN_CLASS_NOTIF, 0).unwrap();
    fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "/");
    let res = fanotify_read(fd);
    println!("{res:#?}");

}