//! 1-1 mapping of all flags that [fanotify.h](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/linux/fanotify.h) has <br>

extern crate libc;
// For documentaton linking
#[allow(unused_imports)]
use libc::{EAGAIN, EEXIST, EINVAL, ENOTDIR};
/* the following events that user-space can register for */

/// Create an event when a file or directory (but see [BUGS](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html#BUGS)) is
/// accessed (read).
pub const FAN_ACCESS: u64 = 0x00000001; /* File was accessed */

/// Create an event when a file is modified (write).
pub const FAN_MODIFY: u64 = 0x00000002; /* File was modified */

/// Create an event when the metadata for a file or directory
/// has changed.  An fanotify group that identifies filesystem
/// objects by file handles is required.
pub const FAN_ATTRIB: u64 = 0x00000004; /* Metadata changed */

/// Create an event when a writable file is closed.
pub const FAN_CLOSE_WRITE: u64 = 0x00000008; /* Writtable file closed */

/// Create an event when a read-only file or directory is
/// closed.
pub const FAN_CLOSE_NOWRITE: u64 = 0x00000010; /* Unwrittable file closed */

/// Create an event when a file or directory is opened.
pub const FAN_OPEN: u64 = 0x00000020; /* File was opened */

/// Create an event when a file or directory has been moved
/// from a marked parent directory.  An fanotify group that
/// identifies filesystem objects by file handles is required.
pub const FAN_MOVED_FROM: u64 = 0x00000040; /* File was moved from X */

/// A file or directory has been moved to a watched parent
/// directory.
pub const FAN_MOVED_TO: u64 = 0x00000080; /* File was moved to Y */

/// A child file or directory was created in a watched parent.
pub const FAN_CREATE: u64 = 0x00000100; /* Subfile was created */

/// A child file or directory was deleted in a watched parent.
pub const FAN_DELETE: u64 = 0x00000200; /* Subfile was deleted */

/// A watched file or directory was deleted.
pub const FAN_DELETE_SELF: u64 = 0x00000400; /* Self was deleted */

/// A watched file or directory was moved.
pub const FAN_MOVE_SELF: u64 = 0x00000800; /* Self was moved */

/// A file was opened with the intent to be executed.  See
/// NOTES in [fanotify_mark(2)](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html#NOTES) for additional details.
pub const FAN_OPEN_EXEC: u64 = 0x00001000; /* File was opened for exec */

/// The event queue exceeded the limit of 16384 entries.  This
/// limit can be overridden by specifying the
/// [`FAN_UNLIMITED_QUEUE`] flag when calling `fanotify_init()`.
pub const FAN_Q_OVERFLOW: u64 = 0x00004000; /* Event queued overflowed */

/// Represents filesystem error
pub const FAN_FS_ERROR: u32 = 0x00008000; /* Filesystem error */

/// Create an event when a permission to open a file or
/// directory is requested.  An fanotify file descriptor
/// created with [`FAN_CLASS_PRE_CONTENT`] or [`FAN_CLASS_CONTENT`] is
/// required.
pub const FAN_OPEN_PERM: u64 = 0x00010000; /* File open in perm check */

/// An application wants to read a file or directory, for
/// example using [read(2)](https://man7.org/linux/man-pages/man2/read.2.html)
/// or [readdir(2)](https://man7.org/linux/man-pages/man2/readdir.2.html).  The reader must
/// write a response (as described below) that determines
/// whether the permission to access the filesystem object
/// shall be granted.
pub const FAN_ACCESS_PERM: u64 = 0x00020000; /* File accessed in perm check */

/// An application wants to open a file for execution.  The
/// reader must write a response that determines whether the
/// permission to open the filesystem object for execution
/// shall be granted.  See NOTES in
/// [fanotify_mark(2)](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html#NOTES) for
/// additional details.
pub const FAN_OPEN_EXEC_PERM: u64 = 0x00040000; /* File open/exec in perm check */

/// Events for the immediate children of marked directories
/// shall be created.  The flag has no effect when marking
/// mounts and filesystems.  Note that events are not
/// generated for children of the subdirectories of marked
/// directories.  More specifically, the directory entry
/// modification events [`FAN_CREATE`], [`FAN_DELETE`],
/// [`FAN_MOVED_FROM`], and [`FAN_MOVED_TO`] are not generated for any
/// entry modifications performed inside subdirectories of
/// marked directories.  Note that the events [`FAN_DELETE_SELF`]
/// and [`FAN_MOVE_SELF`] are not generated for children of marked
/// directories.  To monitor complete directory trees it is
/// necessary to mark the relevant mount or filesystem.
pub const FAN_EVENT_ON_CHILD: u64 = 0x08000000; /* Interested in child events */

/// Create an event when a file is renamed.
pub const FAN_RENAME: u64 = 0x10000000; /* File was renamed */

/// Create events for directories—for example, when
/// [opendir(3)](https://man7.org/linux/man-pages/man3/opendir.3.html),
/// [readdir(3)](https://man7.org/linux/man-pages/man3/readdir.3.html)
/// (but see [BUGS](https://man7.org/linux/man-pages/man2/fanotify_mark.2.html#BUGS)), and
/// [closedir(3)](https://man7.org/linux/man-pages/man3/closedir.3.html) are
/// called.  Without this flag, events are created only for
/// files.  In the context of directory entry events, such as
/// [`FAN_CREATE`], [`FAN_DELETE`], [`FAN_MOVED_FROM`], and [`FAN_MOVED_TO`],
/// specifying the flag [`FAN_ONDIR`] is required in order to
/// create events when subdirectory entries are modified
/// (i.e., [mkdir(2)](https://man7.org/linux/man-pages/man2/mkdir.2.html)/
/// [rmdir(2)](https://man7.org/linux/man-pages/man2/rmdir.2.html)).
pub const FAN_ONDIR: u64 = 0x40000000; /* Event occurred against dir */

/* helper events */
/// Convenience macro - A file is closed ([`FAN_CLOSE_WRITE`]|[`FAN_CLOSE_NOWRITE`]).
pub const FAN_CLOSE: u64 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE; /* close */

/// Convenience macro - A file or directory has been moved
/// ([`FAN_MOVED_FROM`]|[`FAN_MOVED_TO`]).
pub const FAN_MOVE: u64 = FAN_MOVED_FROM | FAN_MOVED_TO; /* moves */

/* flags used for fanotify_init() */
/// Set the close-on-exec flag (`FD_CLOEXEC`) on the new file
/// descriptor.  See the description of the `O_CLOEXEC` flag in
/// [open(2)](https://man7.org/linux/man-pages/man2/open.2.html).
pub const FAN_CLOEXEC: u32 = 0x00000001;

/// Enable the nonblocking flag (`O_NONBLOCK`) for the file
/// descriptor.  Reading from the file descriptor will not
/// block.  Instead, if no data is available,
/// [read(2)](https://man7.org/linux/man-pages/man2/read.2.html)
/// fails with the error [`EAGAIN`].
pub const FAN_NONBLOCK: u32 = 0x00000002;

/* These are NOT bitwise flags.  Both bits are used together.  */

/// This is the default value.  It does not need to be
/// specified.  This value only allows the receipt of events
/// notifying that a file has been accessed.  Permission
/// decisions before the file is accessed are not possible.
pub const FAN_CLASS_NOTIF: u32 = 0x00000000;

/// This value allows the receipt of events notifying that a
/// file has been accessed and events for permission decisions
/// if a file may be accessed.  It is intended for event
/// listeners that need to access files when they already
/// contain their final content.  This notification class
/// might be used by malware detection programs, for example.
pub const FAN_CLASS_CONTENT: u32 = 0x00000004;

/// This value allows the receipt of events notifying that a
/// file has been accessed and events for permission decisions
/// if a file may be accessed.  It is intended for event
/// listeners that need to access files before they contain
/// their final data.  This notification class might be used
/// by hierarchical storage managers, for example.
pub const FAN_CLASS_PRE_CONTENT: u32 = 0x00000008;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
pub const FAN_ALL_CLASS_BITS: u32 = FAN_CLASS_NOTIF | FAN_CLASS_CONTENT | FAN_CLASS_PRE_CONTENT;

/// Remove the limit of 16384 events for the event queue.  Use
/// of this flag requires the `CAP_SYS_ADMIN` capability.
pub const FAN_UNLIMITED_QUEUE: u32 = 0x00000010;

/// Remove the limit of 8192 marks.  Use of this flag requires
/// the `CAP_SYS_ADMIN` capability.
pub const FAN_UNLIMITED_MARKS: u32 = 0x00000020;

/// Enable generation of audit log records about access
/// mediation performed by permission events.  The permission
/// event response has to be marked with the [`FAN_AUDIT`] flag
/// for an audit log record to be generated.
pub const FAN_ENABLE_AUDIT: u32 = 0x00000040;

/// Allow the file operation.
pub const FAN_ALLOW: u32 = 0x01;

/// Deny the file operation.
pub const FAN_DENY: u32 = 0x02;

/// Bit mask to create audit record for result
pub const FAN_AUDIT: u32 = 0x10;

/// Indicates a queue overflow.
pub const FAN_NOFD: i32 = -1;

/// This value allows only read access.
pub const O_RDONLY: u32 = 00000000;

/// This value allows only write access.
pub const O_WRONLY: u32 = 1;

/// This value allows read and write access.
pub const O_RDWR: u32 = 2;

// The file is opened in append mode.
pub const O_APPEND: u32 = 2000;

/// When possible, the file is opened in nonblocking mode.
pub const O_NONBLOCK: u32 = 4000;

/// Write operations on the file will complete according to
/// the requirements of synchronized I/O data integrity
/// completion.
pub const O_DSYNC: u32 = 10000; /* direct disk access hint */

/// Enable support for files exceeding 2 GB.  Failing to set
/// this flag will result in an EOVERFLOW error when trying to
/// open a large file which is monitored by an fanotify group
/// on a 32-bit system.
pub const O_LARGEFILE: u32 = 0x40000;

/// Do not update the file last access time (st_atime in the
/// inode) when the file is [read(2)](https://man7.org/linux/man-pages/man2/read.2.html).
pub const O_NOATIME: u32 = 1000000;

/// Enable the close-on-exec flag for the new file descriptor.
pub const O_CLOEXEC: u32 = 2000000; /* set close_on_exec */

/// Special value used to indicate openat should use the current working directory
pub const AT_FDCWD: i32 = -100;

/* Flags to determine fanotify event format */
pub const FAN_REPORT_PIDFD: u32 = 0x00000080; /* Report pidfd for event->pid */
pub const FAN_REPORT_TID: u32 = 0x00000100; /* event->pid is thread id */
pub const FAN_REPORT_FID: u32 = 0x00000200; /* Report unique file id */
pub const FAN_REPORT_DIR_FID: u32 = 0x00000400; /* Report unique directory id */
pub const FAN_REPORT_NAME: u32 = 0x00000800; /* Report events with name */
pub const FAN_REPORT_TARGET_FID: u32 = 0x00001000; /* Report dirent target id  */

/// Convenience macro - [`FAN_REPORT_NAME`] requires [`FAN_REPORT_DIR_FID`]
pub const FAN_REPORT_DFID_NAME: u32 = FAN_REPORT_DIR_FID | FAN_REPORT_NAME;

/// Convenience macro - [`FAN_REPORT_TARGET_FID`] requires all other FID flags
/// ([`FAN_REPORT_DFID_NAME`], [`FAN_REPORT_FID`] , [`FAN_REPORT_TARGET_FID`])
pub const FAN_REPORT_DFID_NAME_TARGET: u32 =
    FAN_REPORT_DFID_NAME | FAN_REPORT_FID | FAN_REPORT_TARGET_FID;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
#[allow(deprecated)] // only allowing it because of 1-1 mapping
pub const FAN_ALL_INIT_FLAGS: u32 =
    FAN_CLOEXEC | FAN_NONBLOCK | FAN_ALL_CLASS_BITS | FAN_UNLIMITED_QUEUE | FAN_UNLIMITED_MARKS;

/* flags used for fanotify_modify_mark() */

/// The events in mask will be added to the mark mask (or to
/// the ignore mask).  mask must be nonempty or the error
/// [`EINVAL`] will occur.
pub const FAN_MARK_ADD: u32 = 0x00000001;

/// The events in argument mask will be removed from the mark
/// mask (or from the ignore mask).  mask must be nonempty or
/// the error [`EINVAL`] will occur.
pub const FAN_MARK_REMOVE: u32 = 0x00000002;

/// If pathname is a symbolic link, mark the link itself,
/// rather than the file to which it refers.  (By default,
/// `fanotify_mark()` dereferences pathname if it is a symbolic
/// link.)
pub const FAN_MARK_DONT_FOLLOW: u32 = 0x00000004;

/// Marks a directory filesystem object for events.
/// If the filesystem object to be marked is not a directory,
/// the error [`ENOTDIR`] shall be raised.
pub const FAN_MARK_ONLYDIR: u32 = 0x00000008;

/* FAN_MARK_MOUNT is		0x00000010 */
/// The events in mask shall be added to or removed from the
/// ignore mask.
pub const FAN_MARK_IGNORED_MASK: u32 = 0x00000020;

/// The ignore mask shall survive modify events.  If this flag
/// is not set, the ignore mask is cleared when a modify event
/// occurs for the ignored file or directory.
pub const FAN_MARK_IGNORED_SURV_MODIFY: u32 = 0x00000040;

/// Remove either all marks for filesystems, all marks for
/// mounts, or all marks for directories and files from the
/// fanotify group.  If flags contains [`FAN_MARK_MOUNT`], all
/// marks for mounts are removed from the group.  If flags
/// contains [`FAN_MARK_FILESYSTEM`], all marks for filesystems
/// are removed from the group.  Otherwise, all marks for
/// directories and files are removed.  No flag other than,
/// and at most one of, the flags [`FAN_MARK_MOUNT`] or
/// [`FAN_MARK_FILESYSTEM`] can be used in conjunction with
/// [`FAN_MARK_FLUSH`].  mask is ignored.
pub const FAN_MARK_FLUSH: u32 = 0x00000080;

/* FAN_MARK_FILESYSTEM is	0x00000100 */

pub const FAN_MARK_EVICTABLE: u32 = 0x00000200;

/// This bit is mutually exclusive with [`FAN_MARK_IGNORED_MASK`] bit.
/// When using FAN_MARK_IGNORE for the first time, mark starts using
/// independent event flags in ignore mask.  After that, trying to
/// update the ignore mask with the old [`FAN_MARK_IGNORED_MASK`] API
/// will result in [`EEXIST`] error.
pub const FAN_MARK_IGNORE: u32 = 0x00000400;

/* These are NOT bitwise flags.  Both bits can be used togther.  */
pub const FAN_MARK_INODE: u32 = 0x00000000;

/// Mark the mount specified by pathname.  If pathname is not
/// itself a mount point, the mount containing pathname will
/// be marked.  All directories, subdirectories, and the
/// contained files of the mount will be monitored.  The
/// events which require that filesystem objects are
/// identified by file handles, such as [`FAN_CREATE`],
/// [`FAN_ATTRIB`], [`FAN_MOVE`], and [`FAN_DELETE_SELF`], cannot be
/// provided as a mask when flags contains [`FAN_MARK_MOUNT`].
/// Attempting to do so will result in the error EINVAL being
/// returned.
pub const FAN_MARK_MOUNT: u32 = 0x00000010;

/// Mark the filesystem specified by pathname.  The filesystem
/// containing pathname will be marked.  All the contained
/// files and directories of the filesystem from any mount
/// point will be monitored.
pub const FAN_MARK_FILESYSTEM: u32 = 0x00000100;

/// Convenience macro - [`FAN_MARK_IGNORE`] requires [`FAN_MARK_IGNORED_SURV_MODIFY`]
/// for non-inode mark types.
pub const FAN_MARK_IGNORE_SURV: u32 = FAN_MARK_IGNORE | FAN_MARK_IGNORED_SURV_MODIFY;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
pub const FAN_ALL_MARK_FLAGS: u32 = FAN_MARK_ADD
    | FAN_MARK_REMOVE
    | FAN_MARK_DONT_FOLLOW
    | FAN_MARK_ONLYDIR
    | FAN_MARK_MOUNT
    | FAN_MARK_IGNORED_MASK
    | FAN_MARK_IGNORED_SURV_MODIFY
    | FAN_MARK_FLUSH;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
pub const FAN_ALL_EVENTS: u64 = FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN;

/*
 * All events which require a permission response from userspace
 */
/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
pub const FAN_ALL_PERM_EVENTS: u64 = FAN_OPEN_PERM | FAN_ACCESS_PERM;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
#[allow(deprecated)] // only allowing it because of 1-1 mapping
pub const FAN_ALL_OUTGOING_EVENTS: u64 = FAN_ALL_EVENTS | FAN_ALL_PERM_EVENTS | FAN_Q_OVERFLOW;

/// Compare [`fanotify_event_metadata.vers`] to verify
/// that the structures returned at run time match the
/// structures defined at compile time.  In case of a
/// mismatch, the application should abandon trying to use the
/// fanotify file descriptor.
pub const FANOTIFY_METADATA_VERSION: u32 = 3;
