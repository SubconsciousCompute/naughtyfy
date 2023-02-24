window.SIDEBAR_ITEMS = {"constant":[["FANOTIFY_METADATA_VERSION","Compare [`fanotify_event_metadata.vers`] to verify that the structures returned at run time match the structures defined at compile time.  In case of a mismatch, the application should abandon trying to use the fanotify file descriptor."],["FAN_ACCESS","Create an event when a file or directory (but see BUGS) is accessed (read)."],["FAN_ACCESS_PERM","An application wants to read a file or directory, for example using read(2) or readdir(2).  The reader must write a response (as described below) that determines whether the permission to access the filesystem object shall be granted."],["FAN_ALLOW","Allow the file operation."],["FAN_ALL_CLASS_BITS",""],["FAN_ALL_EVENTS",""],["FAN_ALL_INIT_FLAGS",""],["FAN_ALL_MARK_FLAGS",""],["FAN_ALL_OUTGOING_EVENTS",""],["FAN_ALL_PERM_EVENTS",""],["FAN_ATTRIB","Create an event when the metadata for a file or directory has changed.  An fanotify group that identifies filesystem objects by file handles is required."],["FAN_AUDIT","Bit mask to create audit record for result"],["FAN_CLASS_CONTENT","This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.  It is intended for event listeners that need to access files when they already contain their final content.  This notification class might be used by malware detection programs, for example."],["FAN_CLASS_NOTIF","This is the default value.  It does not need to be specified.  This value only allows the receipt of events notifying that a file has been accessed.  Permission decisions before the file is accessed are not possible."],["FAN_CLASS_PRE_CONTENT","This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.  It is intended for event listeners that need to access files before they contain their final data.  This notification class might be used by hierarchical storage managers, for example."],["FAN_CLOEXEC","Set the close-on-exec flag (`FD_CLOEXEC`) on the new file descriptor.  See the description of the `O_CLOEXEC` flag in open(2)."],["FAN_CLOSE","Convenience macro - A file is closed ([`FAN_CLOSE_WRITE`]|[`FAN_CLOSE_NOWRITE`])."],["FAN_CLOSE_NOWRITE","Create an event when a read-only file or directory is closed."],["FAN_CLOSE_WRITE","Create an event when a writable file is closed."],["FAN_CREATE","A child file or directory was created in a watched parent."],["FAN_DELETE","A child file or directory was deleted in a watched parent."],["FAN_DELETE_SELF","A watched file or directory was deleted."],["FAN_DENY","Deny the file operation."],["FAN_ENABLE_AUDIT","Enable generation of audit log records about access mediation performed by permission events.  The permission event response has to be marked with the [`FAN_AUDIT`] flag for an audit log record to be generated."],["FAN_EVENT_ON_CHILD","Events for the immediate children of marked directories shall be created.  The flag has no effect when marking mounts and filesystems.  Note that events are not generated for children of the subdirectories of marked directories.  More specifically, the directory entry modification events [`FAN_CREATE`], [`FAN_DELETE`], [`FAN_MOVED_FROM`], and [`FAN_MOVED_TO`] are not generated for any entry modifications performed inside subdirectories of marked directories.  Note that the events [`FAN_DELETE_SELF`] and [`FAN_MOVE_SELF`] are not generated for children of marked directories.  To monitor complete directory trees it is necessary to mark the relevant mount or filesystem."],["FAN_FS_ERROR","Represents filesystem error"],["FAN_MARK_ADD","The events in mask will be added to the mark mask (or to the ignore mask).  mask must be nonempty or the error [`EINVAL`] will occur."],["FAN_MARK_DONT_FOLLOW","If pathname is a symbolic link, mark the link itself, rather than the file to which it refers.  (By default, `fanotify_mark()` dereferences pathname if it is a symbolic link.)"],["FAN_MARK_EVICTABLE",""],["FAN_MARK_FILESYSTEM","Mark the filesystem specified by pathname.  The filesystem containing pathname will be marked.  All the contained files and directories of the filesystem from any mount point will be monitored."],["FAN_MARK_FLUSH","Remove either all marks for filesystems, all marks for mounts, or all marks for directories and files from the fanotify group.  If flags contains [`FAN_MARK_MOUNT`], all marks for mounts are removed from the group.  If flags contains [`FAN_MARK_FILESYSTEM`], all marks for filesystems are removed from the group.  Otherwise, all marks for directories and files are removed.  No flag other than, and at most one of, the flags [`FAN_MARK_MOUNT`] or [`FAN_MARK_FILESYSTEM`] can be used in conjunction with [`FAN_MARK_FLUSH`].  mask is ignored."],["FAN_MARK_IGNORE","This bit is mutually exclusive with [`FAN_MARK_IGNORED_MASK`] bit. When using FAN_MARK_IGNORE for the first time, mark starts using independent event flags in ignore mask.  After that, trying to update the ignore mask with the old [`FAN_MARK_IGNORED_MASK`] API will result in [`EEXIST`] error."],["FAN_MARK_IGNORED_MASK","The events in mask shall be added to or removed from the ignore mask."],["FAN_MARK_IGNORED_SURV_MODIFY","The ignore mask shall survive modify events.  If this flag is not set, the ignore mask is cleared when a modify event occurs for the ignored file or directory."],["FAN_MARK_IGNORE_SURV","Convenience macro - [`FAN_MARK_IGNORE`] requires [`FAN_MARK_IGNORED_SURV_MODIFY`] for non-inode mark types."],["FAN_MARK_INODE",""],["FAN_MARK_MOUNT","Mark the mount specified by pathname.  If pathname is not itself a mount point, the mount containing pathname will be marked.  All directories, subdirectories, and the contained files of the mount will be monitored.  The events which require that filesystem objects are identified by file handles, such as [`FAN_CREATE`], [`FAN_ATTRIB`], [`FAN_MOVE`], and [`FAN_DELETE_SELF`], cannot be provided as a mask when flags contains [`FAN_MARK_MOUNT`]. Attempting to do so will result in the error EINVAL being returned."],["FAN_MARK_ONLYDIR","If the filesystem object to be marked is not a directory, the error [`ENOTDIR`] shall be raised."],["FAN_MARK_REMOVE","The events in argument mask will be removed from the mark mask (or from the ignore mask).  mask must be nonempty or the error [`EINVAL`] will occur."],["FAN_MODIFY","Create an event when a file is modified (write)."],["FAN_MOVE","Convenience macro - A file or directory has been moved ([`FAN_MOVED_FROM`]|[`FAN_MOVED_TO`])."],["FAN_MOVED_FROM","Create an event when a file or directory has been moved from a marked parent directory.  An fanotify group that identifies filesystem objects by file handles is required."],["FAN_MOVED_TO","A file or directory has been moved to a watched parent directory."],["FAN_MOVE_SELF","A watched file or directory was moved."],["FAN_NOFD","Indicates a queue overflow."],["FAN_NONBLOCK","Enable the nonblocking flag (`O_NONBLOCK`) for the file descriptor.  Reading from the file descriptor will not block.  Instead, if no data is available, read(2) fails with the error [`EAGAIN`]."],["FAN_ONDIR","Create events for directories—for example, when opendir(3), readdir(3) (but see BUGS), and closedir(3) are called.  Without this flag, events are created only for files.  In the context of directory entry events, such as [`FAN_CREATE`], [`FAN_DELETE`], [`FAN_MOVED_FROM`], and [`FAN_MOVED_TO`], specifying the flag [`FAN_ONDIR`] is required in order to create events when subdirectory entries are modified (i.e., mkdir(2)/ rmdir(2))."],["FAN_OPEN","Create an event when a file or directory is opened."],["FAN_OPEN_EXEC","A file was opened with the intent to be executed.  See NOTES in fanotify_mark(2) for additional details."],["FAN_OPEN_EXEC_PERM","An application wants to open a file for execution.  The reader must write a response that determines whether the permission to open the filesystem object for execution shall be granted.  See NOTES in fanotify_mark(2) for additional details."],["FAN_OPEN_PERM","Create an event when a permission to open a file or directory is requested.  An fanotify file descriptor created with [`FAN_CLASS_PRE_CONTENT`] or [`FAN_CLASS_CONTENT`] is required."],["FAN_Q_OVERFLOW","The event queue exceeded the limit of 16384 entries.  This limit can be overridden by specifying the [`FAN_UNLIMITED_QUEUE`] flag when calling `fanotify_init()`."],["FAN_RENAME","Create an event when a file is renamed."],["FAN_REPORT_DFID_NAME","Convenience macro - [`FAN_REPORT_NAME`] requires [`FAN_REPORT_DIR_FID`]"],["FAN_REPORT_DFID_NAME_TARGET","Convenience macro - [`FAN_REPORT_TARGET_FID`] requires all other FID flags ([`FAN_REPORT_DFID_NAME`], [`FAN_REPORT_FID`] , [`FAN_REPORT_TARGET_FID`])"],["FAN_REPORT_DIR_FID",""],["FAN_REPORT_FID",""],["FAN_REPORT_NAME",""],["FAN_REPORT_PIDFD",""],["FAN_REPORT_TARGET_FID",""],["FAN_REPORT_TID",""],["FAN_UNLIMITED_MARKS","Remove the limit of 8192 marks.  Use of this flag requires the `CAP_SYS_ADMIN` capability."],["FAN_UNLIMITED_QUEUE","Remove the limit of 16384 events for the event queue.  Use of this flag requires the `CAP_SYS_ADMIN` capability."]]};