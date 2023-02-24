/// Error codes associated to fanotify, fanotify_init(), fanotify_mark() and read()
/// EINVAL An invalid value was passed in flags or event_f_flags.
///        FAN_ALL_INIT_FLAGS (deprecated since Linux kernel version
///        4.20) defines all allowable bits for flags.
///
/// EMFILE The number of fanotify groups for this user exceeds 128.
///
/// EMFILE The per-process limit on the number of open file
///        descriptors has been reached.
///
/// ENOMEM The allocation of memory for the notification group
///        failed.
///
/// ENOSYS This kernel does not implement fanotify_init().  The
///        fanotify API is available only if the kernel was
///        configured with CONFIG_FANOTIFY.
///
/// EPERM  The operation is not permitted because the caller lacks
///        the CAP_SYS_ADMIN capability.
///
/// Error codes associated to fanotify_mark()
///
/// EBADF  An invalid file descriptor was passed in fanotify_fd.
///
/// EBADF  pathname is relative but dirfd is neither AT_FDCWD nor a
///        valid file descriptor.
///
/// EINVAL An invalid value was passed in flags or mask, or
///        fanotify_fd was not an fanotify file descriptor.
///
/// EINVAL The fanotify file descriptor was opened with
///        FAN_CLASS_NOTIF or the fanotify group identifies
///        filesystem objects by file handles and mask contains a
///        flag for permission events (FAN_OPEN_PERM or
///        FAN_ACCESS_PERM).
///
/// ENODEV The filesystem object indicated by pathname is not
///        associated with a filesystem that supports fsid (e.g.,
///        tmpfs(5)).  This error can be returned only with an
///        fanotify group that identifies filesystem objects by file
///        handles.
///
/// ENOENT The filesystem object indicated by dirfd and pathname does
///        not exist.  This error also occurs when trying to remove a
///        mark from an object which is not marked.
///
/// ENOMEM The necessary memory could not be allocated.
///
/// ENOSPC The number of marks exceeds the limit of 8192 and the
///        FAN_UNLIMITED_MARKS flag was not specified when the
///        fanotify file descriptor was created with
///        fanotify_init(2).
///
/// ENOSYS This kernel does not implement fanotify_mark().  The
///        fanotify API is available only if the kernel was
///        configured with CONFIG_FANOTIFY.
///
/// ENOTDIR
///        flags contains FAN_MARK_ONLYDIR, and dirfd and pathname do
///        not specify a directory.
///
/// EOPNOTSUPP
///        The object indicated by pathname is associated with a
///        filesystem that does not support the encoding of file
///        handles.  This error can be returned only with an fanotify
///        group that identifies filesystem objects by file handles.
///
/// EXDEV  The filesystem object indicated by pathname resides within
///        a filesystem subvolume (e.g., btrfs(5)) which uses a
///        different fsid than its root superblock.  This error can
///        be returned only with an fanotify group that identifies
///        filesystem objects by file handles.
///
use std::fmt;
pub struct FanotifyInitError {
    code: i32,
}
impl fmt::Display for FanotifyInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.code {
            libc::EINVAL => "An invalid value was passed in flags or event_f_flags.
            FAN_ALL_INIT_FLAGS (deprecated since Linux kernel version
            4.20) defines all allowable bits for flags.",
            libc::EMFILE => "The number of fanotify groups for this user exceeds 128 or the per-process limit on the number of open file
            descriptors has been reached.",
            libc::ENOMEM => "The allocation of memory for the notification group
            failed.",
            libc::ENOSYS => "This kernel does not implement fanotify_init().  The
            fanotify API is available only if the kernel was
            configured with CONFIG_FANOTIFY.",
            libc::EPERM => "The operation is not permitted because the caller lacks
            the CAP_SYS_ADMIN capability.",
            _ => "Unknown error occured."
        };

        write!(f, "{err_msg}")
    }
}
