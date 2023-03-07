use naughtyfy::api::*;
use naughtyfy::flags::*;
use naughtyfy::types::*;

fn procedure(md: &fanotify_event_with_fid) {
    println!("{md:#?}");
}

// Run this example with sudo privilages and create
// a directory in root of this project to see results
fn main() {
    let fd = init(FAN_CLASS_NOTIF | FAN_REPORT_DFID_NAME, 0);
    match fd {
        Ok(fd) => {
            mark(
                fd,
                FAN_MARK_ADD | FAN_MARK_ONLYDIR,
                FAN_CREATE | FAN_ONDIR,
                AT_FDCWD,
                "./",
            )
            .unwrap();
            let res = read_with_fid_do(fd, procedure);
            assert!(res.is_ok());
        }
        Err(e) => {
            // This can fail for multiple reason, most common being privileges.
            eprintln!("Cannot get fd due to {e}");
            assert!(e.code != 0);
        }
    }
}
