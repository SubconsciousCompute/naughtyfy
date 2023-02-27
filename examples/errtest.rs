use libc::AT_FDCWD;
use naughtyfy::api::*;
use naughtyfy::flags::*;

fn main() {
    unsafe {
        FAN_EVENT_BUFFER_LEN = 230;
    }
    let fd = fanotify_init(FAN_CLASS_NOTIF, 0).unwrap();
    fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "/").unwrap();
    let mut iter = 0;
    loop {
        let res = fanotify_read(fd).unwrap();
        println!("{res:#?}");
        iter += 1;
        if iter > 10 {
            break;
        }
    }
}
