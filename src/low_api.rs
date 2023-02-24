use lazy_static::lazy_static;

use crate::structs::*;
use std::{io::Error, mem, os::unix::ffi::OsStrExt, slice};

lazy_static! {
    /// Get current platform sizeof of fanotify_event_metadata.
    pub static ref FAN_EVENT_METADATA_LEN: usize = mem::size_of::<fanotify_event_metadata>();
}

/// Initializes a new fanotify group and returns a file descriptor for the event queue associated with the group.<br/>
///
/// The file descriptor is used in calls to fanotify_mark(2) to specify the files, directories, mounts or filesystems for which fanotify events shall be created.  
/// These events are received by reading from the file descriptor.  <br/>
/// Some events are only informative, indicating that a file has been accessed.
/// Other events can be used to determine whether another application is permitted to access a file or directory.
/// Permission to access filesystem objects is granted by writing to the file descriptor.
/// Multiple programs may be using the fanotify interface at the same time to monitor the same files.<br/>
/// In the current implementation, the number of fanotify groups per user is limited to 128.  This limit cannot be overridden.
/// Calling fanotify_init() requires the CAP_SYS_ADMIN capability.  
/// This constraint might be relaxed in future versions of the API. <br/>
/// Therefore, certain additional capability checks have been implemented as indicated below.<br/>
/// The `flags` argument contains a multi-bit field defining the notification class of the listening application and further single bit fields specifying the behavior of the file descriptor.<br/>
/// If multiple listeners for permission events exist, the notification class is used to establish the sequence in which the listeners receive the events.<br/>
///
/// Only one of the following notification classes may be specified in `flags`:<br/>
/// * FAN_CLASS_PRE_CONTENT
/// * FAN_CLASS_CONTENT
/// * FAN_CLASS_NOTIF
///
/// Listeners with different notification classes will receive events in the order `FAN_CLASS_PRE_CONTENT`, `FAN_CLASS_CONTENT`, `FAN_CLASS_NOTIF`.
/// The order of notification for listeners in the same notification class is undefined.<br/>
/// The following bits can additionally be set in flags:<br/>
/// * FAN_CLOEXEC
/// * FAN_NONBLOCK
/// * FAN_UNLIMITED_QUEUE
/// * FAN_UNLIMITED_MARKS
/// * FAN_REPORT_TID (since Linux 4.20)
/// * FAN_REPORT_FID (since Linux 5.1)
///
/// The event_f_flags argument defines the file status flags that will be set on the open file descriptions that are created for fanotify events.  <br/>
/// For details of these flags, see the description of the flags values in open(2).  event_f_flags includes a multi-bit field for the access mode.  <br/>
/// This field can take the following values:
/// * O_RDONLY       
/// * O_WRONLY
/// * O_RDWR
///     
/// Additional bits can be set in event_f_flags.  The most useful values are:
/// * O_LARGEFILE
/// * O_CLOEXEC (since Linux 3.18)
///
/// The following are also allowable: `O_APPEND`, `O_DSYNC`, `O_NOATIME`,`O_NONBLOCK`, and `O_SYNC`.  Specifying any other flag in `event_f_flags` yields the error `EINVAL`.
/// # Examples
/// ```
/// use naughtyfy::low_layer::*;
/// let fd = fanotify_init(FAN_CLASS_NOTIF, O_RDONLY).unwrap();
/// assert!(fd > 0)
/// ```
pub fn fanotify_init(flags: u32, event_f_flags: u32) -> Result<i32, Error> {
    unsafe {
        match libc::fanotify_init(flags, event_f_flags) {
            -1 => {
                return Err(Error::last_os_error());
            }
            fd => {
                return Ok(fd);
            }
        };
    }
}

pub trait FanotifyPath {
    fn as_os_str(&self) -> &std::ffi::OsStr;
}

impl FanotifyPath for std::path::Path {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        self.as_os_str()
    }
}

impl FanotifyPath for str {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self)
    }
}

impl FanotifyPath for String {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self.as_str())
    }
}

pub fn fanotify_mark<P: ?Sized + FanotifyPath>(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    dirfd: i32,
    path: &P,
) -> Result<(), Error> {
    unsafe {
        match libc::fanotify_mark(
            fanotify_fd,
            flags,
            mask,
            dirfd,
            path.as_os_str()
                .as_bytes()
                .iter()
                .map(|p| *p as i8)
                .collect::<Vec<i8>>()
                .as_ptr(),
        ) {
            0 => {
                return Ok(());
            }
            _ => {
                return Err(Error::last_os_error());
            }
        }
    }
}

pub fn fanotify_read(fanotify_fd: i32) -> Vec<fanotify_event_metadata> {
    let mut vec = Vec::new();
    unsafe {
        let buffer = libc::malloc(*FAN_EVENT_METADATA_LEN * 200);
        let sizeof = libc::read(fanotify_fd, buffer, *FAN_EVENT_METADATA_LEN * 200);
        if sizeof != libc::EAGAIN as isize && sizeof > 0 {
            let src = slice::from_raw_parts(
                buffer as *mut fanotify_event_metadata,
                sizeof as usize / *FAN_EVENT_METADATA_LEN,
            );
            // vec.extend_from_slice(src);
            vec = src.to_vec();
        }
        libc::free(buffer);
    }
    vec
}
pub fn close_fd(fd: i32) {
    unsafe {
        libc::close(fd);
    }
}
