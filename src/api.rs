//! Low level function mapping for fanotify

use crate::errors::*;
use crate::types::*;
use std::ffi::CString;
use std::io::Error;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::slice;

#[allow(unused_imports)]
use crate::flags::*;

/// Get current platform sizeof of [`fanotify_event_metadata`].
const FAN_EVENT_METADATA_LEN: usize = mem::size_of::<fanotify_event_metadata>();

/// Get current platform size of [`fanotify_response`]
const FAN_WRITE_RESPONSE_LEN: usize = mem::size_of::<fanotify_response>();

/// Length of memory to be allocated for read buffer
pub static mut FAN_EVENT_BUFFER_LEN: usize = 250;

/// Initializes a new fanotify group and returns a
/// file descriptor for the event queue associated with the group.
///
/// The file descriptor is used in calls to [`fanotify_mark()`] to
/// specify the files, directories, mounts, or filesystems for which
/// fanotify events shall be created.  These events are received by
/// reading from the file descriptor.  Some events are only
/// informative, indicating that a file has been accessed.  Other
/// events can be used to determine whether another application is
/// permitted to access a file or directory.  Permission to access
/// filesystem objects is granted by writing to the file descriptor.
///
/// Multiple programs may be using the fanotify interface at the same
/// time to monitor the same files.
///
/// In the current implementation, the number of fanotify groups per
/// user is limited to 128.  This limit cannot be overridden.
///
/// Calling [`fanotify_init()`] requires the `CAP_SYS_ADMIN` capability.
/// This constraint might be relaxed in future versions of the API.
/// Therefore, certain additional capability checks have been
/// implemented as indicated below.
///
/// The flags argument contains a multi-bit field defining the
/// notification class of the listening application and further
/// single bit fields specifying the behavior of the file descriptor.
///
/// If multiple listeners for permission events exist, the
/// notification class is used to establish the sequence in which the
/// listeners receive the events.
///
/// # Arguments
/// * `flags` - Sets the notification group, can be mask of <br>
///     * [`FAN_CLASS_PRE_CONTENT`]
///     * [`FAN_CLASS_CONTENT`]
///     * [`FAN_CLASS_NOTIF`]
///
///     The following bits can additionally be set in `flags`: <br>
///     * [`FAN_CLOEXEC`]
///     * [`FAN_NONBLOCK`]
///     * [`FAN_UNLIMITED_QUEUE`]
///     * [`FAN_UNLIMITED_MARKS`]
///     * [`FAN_ENABLE_AUDIT`]
///     * [`FAN_REPORT_FID`]
///     * [`FAN_REPORT_DIR_FID`]
///     * [`FAN_REPORT_NAME`]
///     * [`FAN_REPORT_DFID_NAME`]
/// * `event_f_flags` - Defines the file status flags that
///                     will be set on the open file descriptions that are created for
///                     fanotify events.  For details of these flags, see the description
///                     of the flags values in open(2).  `event_f_flags` includes a multi-
///                     bit field for the access mode. This field can take the following
///                     values:
///     * [`O_RDONLY`]
///     * [`O_WRONLY`]
///     * [`O_RDWR`]
///
///     Additional bits can be set in `event_f_flags`.
///     * [`O_LARGEFILE`]
///     * [`O_CLOEXEC`]
///     * [`O_APPEND`]
///     * [`O_DSYNC`]
///     * [`O_NOATIME`]
///     * [`O_NONBLOCK`]
///
/// # Example
/// This example may thorw error due to absence of `CAP_SYS_ADMIN` [capabilitity](https://man7.org/linux/man-pages/man7/capabilities.7.html)
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::api::*;
/// let fd = fanotify_init(FAN_CLASS_NOTIF | FAN_NONBLOCK, O_RDONLY);
/// match fd {
///     Ok(fd) => {
///         assert!(fd >= 0);
///     }
///     Err(e) => {
///         eprintln!("Cannot get fd due to {e}");
///         assert!(e.code != 0);
///     }
/// }
/// ```
///
pub fn fanotify_init(flags: u32, event_f_flags: u32) -> Result<i32, FanotifyError<Init>> {
    unsafe {
        match libc::fanotify_init(flags, event_f_flags) {
            -1 => Err(Error::last_os_error().into()),
            fd => Ok(fd),
        }
    }
}

/// Adds, removes, or modifies an fanotify mark on a
/// filesystem object. The caller must have read permission on the
/// filesystem object that is to be marked.
///
/// # Arguments
/// * `fanotify_fd` - File descriptor returned by [`fanotify_init()`].
/// * `flags` - Bit mask describing the modification to perform. <br>
///     It must include **exactly one** of the following values:
///     * [`FAN_MARK_ADD`]
///     * [`FAN_MARK_REMOVE`]
///     * [`FAN_MARK_FLUSH`]
///
///      In addition, zero or more of the following values may be ORed
///      into flags:
///     * [`FAN_MARK_DONT_FOLLOW`]
///     * [`FAN_MARK_ONLYDIR`]
///     * [`FAN_MARK_MOUNT`]
///     * [`FAN_MARK_FILESYSTEM`]
///     * [`FAN_MARK_IGNORED_MASK`]
///     * [`FAN_MARK_IGNORED_SURV_MODIFY`]
/// * `mask` - Which events shall be listened for (or which shall be ignored). <br>
///     It is a bit mask composed of the following values:
///     * [`FAN_ACCESS`]
///     * [`FAN_MODIFY`]
///     * [`FAN_CLOSE_WRITE`]
///     * [`FAN_CLOSE_NOWRITE`]
///     * [`FAN_OPEN`]
///     * [`FAN_OPEN_EXEC`]
///     * [`FAN_ATTRIB`]
///     * [`FAN_CREATE`]
///     * [`FAN_DELETE`]
///     * [`FAN_DELETE_SELF`]
///     * [`FAN_MOVED_FROM`]
///     * [`FAN_MOVED_TO`]
///     * [`FAN_MOVE_SELF`]
///     * [`FAN_OPEN_PERM`]
///     * [`FAN_OPEN_EXEC_PERM`]
///     * [`FAN_ACCESS_PERM`]
///     * [`FAN_ONDIR`]
///     * [`FAN_EVENT_ON_CHILD`]
///     * [`FAN_CLOSE`]
///     * [`FAN_MOVE`]
/// * `dirfd` - Defines the filesystem object to be marked.
/// * `path` - Filesystem path of file or diretory.
///
/// The filesystem object to be marked is determined by the file
/// descriptor dirfd and the pathname specified in path:
///
/// * If pathname is `NULL`, dirfd defines the filesystem object to be
///   marked.
/// * If pathname is `NULL`, and dirfd takes the special value
///   [`AT_FDCWD`], the current working directory is to be marked.

/// * If pathname is absolute, it defines the filesystem object to
///   be marked, and dirfd is ignored.

/// * If pathname is relative, and dirfd does not have the value
///   [`AT_FDCWD`], then the filesystem object to be marked is
///   determined by interpreting pathname relative the directory
///   referred to by dirfd.

/// * If pathname is relative, and dirfd has the value [`AT_FDCWD`],
///   then the filesystem object to be marked is determined by
///   interpreting pathname relative to the current working
///   directory.
///
/// # Example
/// This example will throw error due to absence of `CAP_SYS_ADMIN` [capabilitity](https://man7.org/linux/man-pages/man7/capabilities.7.html)
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// let fd = fanotify_init(FAN_CLASS_NOTIF, 0);
/// match fd {
///     Ok(fd) => {
///         let m = fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
///         assert!(m.is_ok());
///         assert!(fd >= 0);
///     }
///     Err(e) => {
///         // This can fail for multiple reason, most common being privileges.
///         eprintln!("Cannot get fd due to {e}");
///         assert!(e.code != 0);
///     }
/// }
/// ```
pub fn fanotify_mark<P: ?Sized + Path>(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    dirfd: i32,
    path: &P,
) -> Result<(), FanotifyError<Mark>> {
    unsafe {
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();
        match libc::fanotify_mark(fanotify_fd, flags, mask, dirfd, path.as_ptr()) {
            0 => Ok(()),
            _ => Err(Error::last_os_error().into()),
        }
    }
}

/// This function attempts to read from a file descriptor `fanotify_fd`
/// into a `Vec<fanotify_event_metadata>` and return a Result.
///
/// # Argument
/// * `fd` - file descriptor returned by [`fanotify_init()`]  
///
/// # Example
/// This example will throw error due to absence of `CAP_SYS_ADMIN` [capabilitity](https://man7.org/linux/man-pages/man7/capabilities.7.html)
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// let fd = fanotify_init(FAN_CLASS_NOTIF, 0);
/// match fd {
///     Ok(fd) => {
///         let m = fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
///         let res = fanotify_read(fd);
///         assert!(res.is_ok());
///     }
///     Err(e) => {
///         // This can fail for multiple reason, most common being privileges.
///         eprintln!("Cannot get fd due to {e}");
///         assert!(e.code != 0);
///     }
/// }
/// ```
pub fn fanotify_read(
    fanotify_fd: i32,
) -> Result<Vec<fanotify_event_metadata>, FanotifyError<Read>> {
    let mut vec = Vec::new();
    unsafe {
        let buffer = libc::malloc(FAN_EVENT_METADATA_LEN * FAN_EVENT_BUFFER_LEN);

        // allocation may fail due to limited memory.
        if buffer.is_null() {
            return Err(Error::last_os_error().into());
        }
        let sizeof = libc::read(
            fanotify_fd,
            buffer,
            FAN_EVENT_METADATA_LEN * FAN_EVENT_BUFFER_LEN,
        );
        if sizeof != libc::EAGAIN as isize && sizeof > 0 {
            let src = slice::from_raw_parts(
                buffer as *mut fanotify_event_metadata,
                sizeof as usize / FAN_EVENT_METADATA_LEN,
            );
            vec = src.to_vec();
        }
        libc::free(buffer);
    }
    Ok(vec)
}

/// Writes up to count bytes from the buffer starting at buf
/// to the file referred to by the file descriptor fd.
///
/// The number of bytes written may be less than count if, for
/// example, there is insufficient space on the underlying physical
/// medium, or the `RLIMIT_FSIZE` resource limit is encountered,
/// or the call was interrupted by a signal handler
/// after having written less than count bytes.
///
/// For a seekable file (i.e., one to which lseek(2) may be applied,
/// for example, a regular file) writing takes place at the file
/// offset, and the file offset is incremented by the number of bytes
/// actually written.  If the file was open(2)ed with O_APPEND, the
/// file offset is first set to the end of the file before writing.
/// The adjustment of the file offset and the write operation are
/// performed as an atomic step.
///
/// POSIX requires that a read(2) that can be proved to occur after a
/// write() has returned will return the new data.  Note that not all
/// filesystems are POSIX conforming.
///
/// According to POSIX.1, if count is greater than SSIZE_MAX, the
/// result is implementation-defined; see NOTES for the upper limit
/// on Linux.
///
/// # Argument
/// * `fd` - This is the file descriptor from the structure [`fanotify_event_metadata`].
/// * `response` - This field indicates whether or not the permission is to
///                be granted.  Its value must be either FAN_ALLOW to allow
///                the file operation or FAN_DENY to deny the file operation.
///
/// # Example
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// let fd = fanotify_init(FAN_CLOEXEC | FAN_CLASS_CONTENT | FAN_NONBLOCK,
///                         O_RDONLY | O_LARGEFILE);
/// match fd {
///     Ok(fd) => {
///         let m = fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
///         assert!(m.is_ok());
///         assert!(fd >= 0);
///         
///         let events = fanotify_read(fd).unwrap();
///         if events.len() > 1 {
///             for event in events {
///                 println!("{event:#?}");
///
///                 let res = fanotify_write(event.fd,FAN_ALLOW);
///             }
///         }
///         fanotify_close(fd);
///     }
///     Err(e) => {
///         // This can fail for multiple reason, most common being privileges.
///         eprintln!("Cannot get fd due to {e}");
///         assert!(e.code != 0);
///     }
/// }
/// ```
pub fn fanotify_write(fd: i32, response: u32) -> Result<isize, FanotifyError<Write>> {
    let res = &fanotify_response { fd, response };
    unsafe {
        match libc::write(
            fd,
            res as *const fanotify_response as *const libc::c_void,
            FAN_WRITE_RESPONSE_LEN,
        ) {
            -1 => Err(Error::last_os_error().into()),
            bytes => Ok(bytes),
        }
    }
}

/// Closes the file descriptor returned by [`fanotify_init()`]
///
/// # Argument
/// * `fd` - file descriptor returned by [`fanotify_init()`]
///
/// # Example
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// let fd = fanotify_init(FAN_CLOEXEC | FAN_CLASS_CONTENT | FAN_NONBLOCK,
///                         O_RDONLY | O_LARGEFILE);
/// match fd {
///     Ok(fd) => {
///         let m = fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
///         let events = fanotify_read(fd).unwrap();
///         if events.len() > 1 {
///             for event in events {
///                 let res = fanotify_write(event.fd,FAN_ALLOW);
///             }
///         }
///         let status = fanotify_close(fd);
///         assert!(status.is_ok());
///     }
///     Err(e) => {
///         // This can fail for multiple reason, most common being privileges.
///         eprintln!("Cannot get fd due to {e}");
///         assert!(e.code != 0);
///     }
/// }
/// ```
pub fn fanotify_close(fd: i32) -> Result<(), FanotifyError<Close>> {
    unsafe {
        match libc::close(fd) {
            0 => Ok(()),
            _ => Err(Error::last_os_error().into()),
        }
    }
}
