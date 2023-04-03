//! Low level function mapping for fanotify

use crate::{errors::*, types::*};
use libc::c_void;
use std::{
    ffi::CString,
    io::Error,
    mem,
    os::{fd::RawFd, unix::ffi::OsStrExt},
};

// Used for docs test
#[allow(unused_imports)]
use crate::flags::*;

/// Get current platform sizeof of [`fanotify_event_metadata`].
const FAN_EVENT_METADATA_LEN: usize = mem::size_of::<fanotify_event_metadata>();

/// Get current platform sizeof of [`fanotify_event_with_fid`].
const FAN_EVENT_METADATA_FID_LEN: usize = mem::size_of::<fanotify_event_with_fid>();

/// Get current platform size of [`fanotify_response`]
const FAN_WRITE_RESPONSE_LEN: usize = mem::size_of::<fanotify_response>();

/// Length of memory to be allocated for read buffer
pub static mut FAN_EVENT_BUFFER_LEN: std::sync::Mutex<usize> = std::sync::Mutex::new(250);

/// Initializes a new fanotify group and returns a
/// file descriptor [`Fd`] for the event queue associated
/// with the group.
///
/// The file descriptor is used in calls to [`mark()`] to
/// specify the files, directories, mounts, or filesystems for which
/// fanotify events shall be created. These events are received by
/// reading from the file descriptor. Some events are only
/// informative, indicating that a file has been accessed. Other
/// events can be used to determine whether another application is
/// permitted to access a file or directory.  Permission to access
/// filesystem objects is granted by writing to the file descriptor.
///
/// Multiple programs may be using the fanotify interface at the same
/// time to monitor the same files.
///
/// The number of fanotify groups per user is limited to 128.
/// This limit cannot be overridden.
///
/// Calling [`init()`] requires the `CAP_SYS_ADMIN` capability.
/// This constraint might be relaxed in future versions of the fanotify kernel API.
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
/// let fd = init(FAN_CLASS_NOTIF | FAN_NONBLOCK, O_RDONLY);
/// match fd {
///     Ok(fd) => {
///         assert!(fd.is_valid());
///     }
///     Err(e) => {
///         eprintln!("Cannot get fd due to {e}");
///         
///     }
/// }
/// ```
///
pub fn init(flags: u32, event_f_flags: u32) -> Result<Fd, FanotifyError> {
    unsafe {
        match libc::fanotify_init(flags, event_f_flags) {
            -1 => Err(FanotifyError::Init(
                Error::last_os_error().raw_os_error().unwrap_or_default(),
            )),
            fd => Ok(fd.into()),
        }
    }
}

/// Adds, removes, or modifies an fanotify mark on a
/// filesystem object. The caller must have read permission on the
/// filesystem object that is to be marked.
///
/// # Arguments
/// * `fd` - Refrence to [`Fd`] returned by [`init()`]
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
/// This example may throw error due to absence of `CAP_SYS_ADMIN` [capabilitity](https://man7.org/linux/man-pages/man7/capabilities.7.html)
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// let fd = &init(FAN_CLASS_NOTIF, 0);
/// match fd {
///     Ok(fd) => {
///         let m = mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
///         assert!(m.is_ok());
///         assert!(fd.is_valid());
///     }
///     Err(e) => {
///         // This can fail for multiple reason, most common being privileges.
///         eprintln!("Cannot get fd due to {e}");
///         
///     }
/// }
/// ```
pub fn mark<P: ?Sized + Path>(
    fd: &Fd,
    flags: u32,
    mask: u64,
    dirfd: i32,
    path: &P,
) -> Result<(), FanotifyError> {
    let path = CString::new(path.as_os_str().as_bytes()).unwrap_or_default();
    unsafe {
        match libc::fanotify_mark(fd.into(), flags, mask, dirfd, path.as_ptr()) {
            0 => Ok(()),
            _ => Err(FanotifyError::Mark(
                Error::last_os_error().raw_os_error().unwrap_or_default(),
            )),
        }
    }
}

/// This function attempts to read from a file descriptor `fanotify_fd`
/// into a `Vec<fanotify_event_metadata>` and return a Result.
///
/// # Note
/// [`close()`] is called on metadata's fd when [`fanotify_event_metadata`]
/// is dropped. No need to explicitly call [`close()`] on every fd.
///
/// # Argument
/// * `fd` - Refrence to [`Fd`] returned by [`init()`]
///
/// # Example
/// This example may throw error due to absence of `CAP_SYS_ADMIN` [capabilitity](https://man7.org/linux/man-pages/man7/capabilities.7.html)
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// let fd = &init(FAN_CLASS_NOTIF, 0);
/// match fd {
///     Ok(fd) => {
///         let m = mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
///         let res = read(fd);
///         assert!(res.is_ok());
///         for meta in res.unwrap() {
///             close(meta.fd).unwrap();
///         }
///     }
///     Err(e) => {
///         // This can fail for multiple reason, most common being privileges.
///         eprintln!("Cannot get fd due to {e}");
///         
///     }
/// }
/// ```
pub fn read(fd: &Fd) -> Result<Vec<fanotify_event_metadata>, FanotifyError> {
    let len;
    unsafe {
        match FAN_EVENT_BUFFER_LEN.lock() {
            Ok(value) => {
                len = *value;
            }
            Err(e) => {
                eprintln!("{e}");
                return Err(FanotifyError::Read(libc::ENOMEM));
            }
        }
    }
    let mut buff: Vec<fanotify_event_metadata> = Vec::with_capacity(len);
    let sizeof;
    unsafe {
        // `libc::read()` is unsafe
        sizeof = libc::read(
            fd.into(),
            buff.as_mut_ptr() as *mut c_void,
            FAN_EVENT_METADATA_LEN * len,
        );
    }

    if sizeof == -1 {
        return Err(FanotifyError::Read(
            Error::last_os_error().raw_os_error().unwrap_or_default(),
        ));
    }

    unsafe {
        // Vec.set_len() is unsafe operation. len must be less than capacity, which is gur
        buff.set_len(sizeof as usize / FAN_EVENT_METADATA_LEN);
    }
    Ok(buff)
}

/// This function attempts to read from a file descriptor `fanotify_fd`
/// and performs `process_metadata` on [`fanotify_event_metadata`] recieved after read.
/// returns `Result<(),FanotifyError>`.
///
/// This function closes `metadata.fd` after calling `process_metadata`
///
/// # Argument
/// * `fd` - Refrence to [`Fd`] returned by [`init()`]
/// * `process_metadata` - Function / Closure for processing [`fanotify_event_metadata`].
///
/// # Example
/// This example may throw error due to absence of `CAP_SYS_ADMIN` [capabilitity](https://man7.org/linux/man-pages/man7/capabilities.7.html)
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// fn procedure(md: &fanotify_event_metadata) {
///     println!("{md:#?}");
/// }
///
/// fn main() {
///     let fd = &init(FAN_CLASS_NOTIF, 0);
///      match fd {
///          Ok(fd) => {
///              let m = mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, AT_FDCWD, "./");
///              let res = read(fd);
///              assert!(res.is_ok());
///              read_do(fd,procedure);
///          }
///          Err(e) => {
///              // This can fail for multiple reason, most common being privileges.
///              eprintln!("Cannot get fd due to {e}");
///              
///          }
///      }
/// }
/// ```
pub fn read_do(
    fd: &Fd,
    process_metadata: fn(&fanotify_event_metadata),
) -> Result<(), FanotifyError> {
    let len;
    unsafe {
        match FAN_EVENT_BUFFER_LEN.lock() {
            Ok(value) => {
                len = *value;
            }
            Err(e) => {
                eprintln!("{e}");
                return Err(FanotifyError::Read(libc::ENOMEM));
            }
        }
    }
    let mut buff: Vec<fanotify_event_metadata> = Vec::with_capacity(len);
    let sizeof;
    unsafe {
        // `libc::read()` is unsafe
        sizeof = libc::read(
            fd.into(),
            buff.as_mut_ptr() as *mut c_void,
            FAN_EVENT_METADATA_LEN * len,
        );
    }

    if sizeof == -1 {
        return Err(FanotifyError::Read(
            Error::last_os_error().raw_os_error().unwrap_or_default(),
        ));
    }

    unsafe {
        // Vec.set_len() is unsafe operation. len must be less than capacity, which is gur
        buff.set_len(sizeof as usize / FAN_EVENT_METADATA_LEN);
    }
    for event in &buff {
        process_metadata(event);
    }
    Ok(())
}

/// This function attempts to read from a file descriptor `fanotify_fd`
/// into a [`Vec`] of [`fanotify_event_with_fid`] which was initilated with
/// [`FAN_REPORT_FID`] or [`FAN_REPORT_DIR_FID`] flag. Returns the vector wrapped in `Result`.
///
/// # Important
/// Use this only when `fd` is initialized with [`FAN_REPORT_FID`] or [`FAN_REPORT_DIR_FID`] flag.
///
/// # Argument
/// * `fd` - Refrence to [`Fd`] returned by [`init()`]
pub fn read_with_fid(fd: &Fd) -> Result<Vec<fanotify_event_with_fid>, FanotifyError> {
    let len;
    unsafe {
        match FAN_EVENT_BUFFER_LEN.lock() {
            Ok(value) => {
                len = *value;
            }
            Err(e) => {
                eprintln!("{e}");
                return Err(FanotifyError::Read(libc::ENOMEM));
            }
        }
    }
    let mut buff: Vec<fanotify_event_with_fid> = Vec::with_capacity(len);

    let sizeof;
    unsafe {
        sizeof = libc::read(
            fd.into(),
            buff.as_mut_ptr() as *mut c_void,
            FAN_EVENT_METADATA_FID_LEN * len,
        );
    }

    if sizeof == -1 {
        return Err(FanotifyError::Read(
            Error::last_os_error().raw_os_error().unwrap_or_default(),
        ));
    }

    unsafe {
        buff.set_len(sizeof as usize / FAN_EVENT_METADATA_FID_LEN);
    }
    Ok(buff)
}

/// This function attempts to read from a file descriptor `fanotify_fd`
/// which was initilated with [`FAN_REPORT_FID`] or [`FAN_REPORT_DIR_FID`] flag
/// and performs `process_metadata_fid` on [`fanotify_event_with_fid`]
/// recieved after read. Returns `Result<(),FanotifyError>`.
///
/// # Argument
/// * `fd` - Refrence to [`Fd`] returned by [`init()`].
/// * `process_metadata` - Function / Closure for processing [`fanotify_event_with_fid`].
pub fn read_with_fid_do(
    fd: &Fd,
    process_metadata_fid: fn(&fanotify_event_with_fid),
) -> Result<(), FanotifyError> {
    let len;
    unsafe {
        match FAN_EVENT_BUFFER_LEN.lock() {
            Ok(value) => {
                len = *value;
            }
            Err(e) => {
                eprintln!("{e}");
                return Err(FanotifyError::Read(libc::ENOMEM));
            }
        }
    }
    let mut buff: Vec<fanotify_event_with_fid> = Vec::with_capacity(len);
    let sizeof;
    unsafe {
        // `libc::read()` is unsafe
        sizeof = libc::read(
            fd.into(),
            buff.as_mut_ptr() as *mut c_void,
            FAN_EVENT_METADATA_FID_LEN * len,
        );
    }
    if sizeof == -1 {
        return Err(FanotifyError::Read(
            Error::last_os_error().raw_os_error().unwrap_or_default(),
        ));
    }

    // Vec.set_len() is unsafe operation. len must be less than capacity.
    unsafe {
        buff.set_len(sizeof as usize / FAN_EVENT_METADATA_FID_LEN);
    }
    for event in &buff {
        process_metadata_fid(event);
    }
    Ok(())
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
/// * `fd` - Refrence to [`Fd`] returned by [`init()`].
/// * `response` - This is a struct of type [`fanotify_response`]
///                 that specifies how to deal with the request.
///
/// # Example
/// ```rust
/// # use naughtyfy::flags::*;
/// # use naughtyfy::types::*;
/// # use naughtyfy::api::*;
/// let fd = &init(FAN_CLOEXEC | FAN_CLASS_CONTENT, O_RDONLY | O_LARGEFILE);
/// match fd {
///     Ok(fd) => {
///         let m = mark(
///             fd,
///             FAN_MARK_ADD | FAN_MARK_MOUNT,
///             FAN_OPEN_PERM | FAN_CLOSE_WRITE,
///             AT_FDCWD,
///             "/tmp",
///         );
///         assert!(m.is_ok());
///         assert!(fd.is_valid());
///         
///         let events = read(fd).unwrap();
///         if events.len() > 1 {
///             for event in events {
///                 println!("{event:#?}");
///                 write(
///                     fd,
///                     &fanotify_response {
///                         fd: event.fd,
///                         // Allowig all events
///                         response: FAN_ALLOW,
///                     },
///                 )
///                 .unwrap();
///             }
///         }
///     }
///     Err(e) => {
///         // This can fail for multiple reason, most common being privileges.
///         eprintln!("Cannot get fd due to {e}");
///         
///     }
/// }
/// ```
pub fn write(fd: &Fd, response: &fanotify_response) -> Result<isize, FanotifyError> {
    unsafe {
        match libc::write(
            fd.into(),
            response as *const fanotify_response as *const libc::c_void,
            FAN_WRITE_RESPONSE_LEN,
        ) {
            -1 => Err(FanotifyError::Write(
                Error::last_os_error().raw_os_error().unwrap_or_default(),
            )),
            bytes => Ok(bytes),
        }
    }
}

/// Closes the file descriptor returned by [`init()`] or [`read()`]
///
/// # Argument
/// * `fd` - file descriptor in raw form ([`RawFd`])`
pub fn close(fd: RawFd) -> Result<(), FanotifyError> {
    unsafe {
        match libc::close(fd) {
            0 => Ok(()),
            _ => Err(FanotifyError::Close(
                Error::last_os_error().raw_os_error().unwrap_or_default(),
            )),
        }
    }
}
