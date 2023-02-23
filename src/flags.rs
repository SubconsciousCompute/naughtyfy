// 1-1 mapping of all flags that torvalds/linux/blob/master/include/uapi/linux/fanotify.h has

/* the following events that user-space can register for */
pub const FAN_ACCESS: u64 = 0x00000001; /* File was accessed */
pub const FAN_MODIFY: u64 = 0x00000002; /* File was modified */
pub const FAN_ATTRIB: u64 = 0x00000004; /* Metadata changed */
pub const FAN_CLOSE_WRITE: u64 = 0x00000008; /* Writtable file closed */
pub const FAN_CLOSE_NOWRITE: u64 = 0x00000010; /* Unwrittable file closed */
pub const FAN_OPEN: u64 = 0x00000020; /* File was opened */
pub const FAN_MOVED_FROM: u64 = 0x00000040; /* File was moved from X */
pub const FAN_MOVED_TO: u64 = 0x00000080; /* File was moved to Y */
pub const FAN_CREATE: u64 = 0x00000100; /* Subfile was created */
pub const FAN_DELETE: u64 = 0x00000200; /* Subfile was deleted */
pub const FAN_DELETE_SELF: u64 = 0x00000400; /* Self was deleted */
pub const FAN_MOVE_SELF: u64 = 0x00000800; /* Self was moved */
pub const FAN_OPEN_EXEC: u64 = 0x00001000; /* File was opened for exec */

pub const FAN_Q_OVERFLOW: u64 = 0x00004000; /* Event queued overflowed */
pub const FAN_FS_ERROR: u64 = 0x00008000; /* Filesystem error */

pub const FAN_OPEN_PERM: u64 = 0x00010000; /* File open in perm check */
pub const FAN_ACCESS_PERM: u64 = 0x00020000; /* File accessed in perm check */
pub const FAN_OPEN_EXEC_PERM: u64 = 0x00040000; /* File open/exec in perm check */

pub const FAN_EVENT_ON_CHILD: u64 = 0x08000000; /* Interested in child events */

pub const FAN_RENAME: u64 = 0x10000000; /* File was renamed */

pub const FAN_ONDIR: u64 = 0x40000000; /* Event occurred against dir */

/* helper events */
pub const FAN_CLOSE: u64 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE; /* close */
pub const FAN_MOVE: u64 = FAN_MOVED_FROM | FAN_MOVED_TO; /* moves */

/* flags used for fanotify_init() */
pub const FAN_CLOEXEC: u64 = 0x00000001;
pub const FAN_NONBLOCK: u64 = 0x00000002;

/* These are NOT bitwise flags.  Both bits are used together.  */
pub const FAN_CLASS_NOTIF: u64 = 0x00000000;
pub const FAN_CLASS_CONTENT: u64 = 0x00000004;
pub const FAN_CLASS_PRE_CONTENT: u64 = 0x00000008;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
pub const FAN_ALL_CLASS_BITS: u64 = FAN_CLASS_NOTIF | FAN_CLASS_CONTENT | FAN_CLASS_PRE_CONTENT;

pub const FAN_UNLIMITED_QUEUE: u64 = 0x00000010;
pub const FAN_UNLIMITED_MARKS: u64 = 0x00000020;
pub const FAN_ENABLE_AUDIT: u64 = 0x00000040;

/* Flags to determine fanotify event format */
pub const FAN_REPORT_PIDFD: u64 = 0x00000080; /* Report pidfd for event->pid */
pub const FAN_REPORT_TID: u64 = 0x00000100; /* event->pid is thread id */
pub const FAN_REPORT_FID: u64 = 0x00000200; /* Report unique file id */
pub const FAN_REPORT_DIR_FID: u64 = 0x00000400; /* Report unique directory id */
pub const FAN_REPORT_NAME: u64 = 0x00000800; /* Report events with name */
pub const FAN_REPORT_TARGET_FID: u64 = 0x00001000; /* Report dirent target id  */

/* Convenience macro - FAN_REPORT_NAME requires FAN_REPORT_DIR_FID */
pub const FAN_REPORT_DFID_NAME: u64 = FAN_REPORT_DIR_FID | FAN_REPORT_NAME;
/* Convenience macro - FAN_REPORT_TARGET_FID requires all other FID flags */
pub const FAN_REPORT_DFID_NAME_TARGET: u64 =
    FAN_REPORT_DFID_NAME | FAN_REPORT_FID | FAN_REPORT_TARGET_FID;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
pub const FAN_ALL_INIT_FLAGS: u64 =
    FAN_CLOEXEC | FAN_NONBLOCK | FAN_ALL_CLASS_BITS | FAN_UNLIMITED_QUEUE | FAN_UNLIMITED_MARKS;

/* flags used for fanotify_modify_mark() */
pub const FAN_MARK_ADD: u64 = 0x00000001;
pub const FAN_MARK_REMOVE: u64 = 0x00000002;
pub const FAN_MARK_DONT_FOLLOW: u64 = 0x00000004;
pub const FAN_MARK_ONLYDIR: u64 = 0x00000008;
/* FAN_MARK_MOUNT is		0x00000010 */
pub const FAN_MARK_IGNORED_MASK: u64 = 0x00000020;
pub const FAN_MARK_IGNORED_SURV_MODIFY: u64 = 0x00000040;
pub const FAN_MARK_FLUSH: u64 = 0x00000080;
/* FAN_MARK_FILESYSTEM is	0x00000100 */
pub const FAN_MARK_EVICTABLE: u64 = 0x00000200;
/* This bit is mutually exclusive with FAN_MARK_IGNORED_MASK bit */
pub const FAN_MARK_IGNORE: u64 = 0x00000400;

/* These are NOT bitwise flags.  Both bits can be used togther.  */
pub const FAN_MARK_INODE: u64 = 0x00000000;
pub const FAN_MARK_MOUNT: u64 = 0x00000010;
pub const FAN_MARK_FILESYSTEM: u64 = 0x00000100;

/*
 * Convenience macro - FAN_MARK_IGNORE requires FAN_MARK_IGNORED_SURV_MODIFY
 * for non-inode mark types.
 */
pub const FAN_MARK_IGNORE_SURV: u64 = FAN_MARK_IGNORE | FAN_MARK_IGNORED_SURV_MODIFY;

/* Deprecated - do not use this in programs and do not add new flags here! */
#[deprecated(note = "do not use this in programs!")]
pub const FAN_ALL_MARK_FLAGS: u64 = FAN_MARK_ADD
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
pub const FAN_ALL_OUTGOING_EVENTS: u64 = FAN_ALL_EVENTS | FAN_ALL_PERM_EVENTS | FAN_Q_OVERFLOW;

pub const FANOTIFY_METADATA_VERSION: u64 = 3;
