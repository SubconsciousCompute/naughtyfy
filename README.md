# 📁 Naughtyfy
A modern fanotify wrapper.

Note: This is still under development.

Feel free to open issues on the repo.
# Example
`lowkey.rs` - Example of low level api biding.
```rust
use libc::AT_FDCWD;
use naughtyfy::api::*;
use naughtyfy::flags::*;

fn main() {
    unsafe {
        // Use carefully in multi thread.
        FAN_EVENT_BUFFER_LEN = 230;
    }
    // Initialise fanotify
    let fd = fanotify_init(FAN_CLASS_NOTIF, 0).unwrap();

    // Mark file descriptor for events
    fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "/").unwrap();

    // Try extracting events from the buffer and print it
    // Do not use infinite loop in your code, it will use 100% of CPU core,
    // Instead use Polling method via nix crate.(will share an example soon)
    let mut iter = 10;
    loop {
        let res = fanotify_read(fd).unwrap();
        println!("{res:#?}");
        res.iter().for_each(|e| fanotify_close(e.fd).unwrap());
        iter += 1;
        if iter > 10 {
            break;
        }
    }
    fanotify_close(fd).unwrap();
}
```

# Docs
- [Docs.rs](https://docs.rs/naughtyfy/latest/naughtyfy/)

# Build 
Build example using 

`cargo build --release --example lowkey` <br> 

Run `lowkey` using 

`sudo ./target/release/examples/lowkey`

# Goals 
- Safe
- Less overhead
- Documented
- Desciptive errors
  
Even though it's not designed to be blazzingly fast but is comparable. Will get better with further updates.

# Thanks
- [fanotify and it's manpage](https://man7.org/linux/man-pages/man7/fanotify.7.html)
- [fanotify-rs](https://github.com/Percivalll/fanotify-rs)
