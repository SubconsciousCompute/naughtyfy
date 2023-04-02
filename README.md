# 📁 Naughtyfy

A modern fanotify wrapper.

~~Note: This is still under development.~~ <br>
I guess it's in usable state, Open issue
for any feature/bug.

## Example

```rust
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
```

## Docs

- [Docs.rs](https://docs.rs/naughtyfy/latest/naughtyfy/)

## Goals

- Safe
- Less overhead
- Documented
- Desciptive errors
- Only 1 dependency (libc)

Even though it's not designed to be blazzingly fast but is comparable. Will get better with further updates.

## TODO
- [ ] Add more example
- [ ] Higher level API maybe?

## Thanks

- [fanotify and it's manpage](https://man7.org/linux/man-pages/man7/fanotify.7.html)
- [fanotify-rs](https://github.com/Percivalll/fanotify-rs)
