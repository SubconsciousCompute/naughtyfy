# 📁 Naughtyfy
A modern fanotify wrapper.

Safe bindings are completed and can be used.

Note: This is still under development.

# Example
```rust
use libc::AT_FDCWD;
use naughtyfy::api::*;
use naughtyfy::flags::*;

fn main() {
    unsafe {
        FAN_EVENT_BUFFER_LEN = 230;
    }
    let fd = fanotify_init(FAN_CLASS_NOTIF, 0).unwrap();
    fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "/").unwrap();
    let mut iter = 10;
    loop {
        let res = fanotify_read(fd).unwrap();
        println!("{res:#?}");
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
- [Github pages](https://github.com/SubconsciousCompute/naughtyfy)

# Build 
Build example using 

`cargo build --release --example lowkey` <br> 

Run `lowkey` using 

`sudo ./target/release/examples/lowkey`
