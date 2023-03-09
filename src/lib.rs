//! Linux file system monitoring library that uses
//! [fanotify](https://man7.org/linux/man-pages/man7/fanotify.7.html)
//! underneath.
//!
//! # Installation
//! Run the command in project root directory
//! ```bash
//! cargo add naughtyfy
//! ```
//! Or manually add it to `Cargo.toml`
//! ```toml
//! [dependencies]
//! naughtyfy = "*"
//! ```
//! # Example
//! ```rust
//! # use naughtyfy::flags::*;
//! # use naughtyfy::types::*;
//! # use naughtyfy::api::*;
//! let fd = init(FAN_CLOEXEC | FAN_CLASS_CONTENT ,
//!                         O_RDONLY | O_LARGEFILE);
//! match fd {
//!     Ok(fd) => {
//!         let m = mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
//!         let events = read(fd).unwrap();
//!         for event in events {
//!             println!("{:#?}",event);
//!             close(event.fd);
//!         }
//!         let status = close(fd);
//!         assert!(status.is_ok());
//!     }
//!     Err(e) => {
//!         // This can fail for multiple reason, most common being privileges.
//!         eprintln!("Cannot get fd due to {e}");
//!     }
//! }
//! ```

pub mod api;
pub mod errors;
pub mod flags;
pub mod types;
