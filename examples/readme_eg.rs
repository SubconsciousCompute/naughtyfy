use naughtyfy::api::*;
use naughtyfy::flags::*;

fn main() {
    unsafe {
        // Use carefully in multi thread.
        FAN_EVENT_BUFFER_LEN = 230.into();
    }
    // Initialise fanotify
    let fd = &init(FAN_CLASS_NOTIF, 0).unwrap();

    // Mark file descriptor for events
    mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "/").unwrap();

    // Try extracting events from the buffer and print it
    let mut iter = 1;
    loop {
        let res = read(fd).unwrap();
        println!("{res:#?}");
        res.iter().for_each(|e| close(e.fd).unwrap());
        iter += 1;
        if iter > 100 {
            break;
        }
    }
}
