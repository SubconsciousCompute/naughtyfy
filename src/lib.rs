pub mod flags;
pub mod errors;
pub mod low_layer;

#[cfg(test)]
mod tests {
    use super::*;
    use flags::*;

    #[test]
    fn test_consts() {
        assert_eq!(FAN_ACCESS, 0x00000001);
        assert_eq!(FAN_ACCESS, 0x00000001); /* File was accessed */
        assert_eq!(FAN_MODIFY, 0x00000002); /* File was modified */
        assert_eq!(FAN_ATTRIB, 0x00000004); /* Metadata changed */
        assert_eq!(FAN_CLOSE_WRITE, 0x00000008); /* Writtable file closed */
        assert_eq!(FAN_CLOSE_NOWRITE, 0x00000010); /* Unwrittable file closed */
        assert_eq!(FAN_OPEN, 0x00000020); /* File was opened */
        assert_eq!(FAN_MOVED_FROM, 0x00000040); /* File was moved from X */
        assert_eq!(FAN_MOVED_TO, 0x00000080); /* File was moved to Y */
        assert_eq!(FAN_CREATE, 0x00000100); /* Subfile was created */
        assert_eq!(FAN_DELETE, 0x00000200); /* Subfile was deleted */
        assert_eq!(FAN_DELETE_SELF, 0x00000400); /* Self was deleted */
        assert_eq!(FAN_MOVE_SELF, 0x00000800); /* Self was moved */
        assert_eq!(FAN_OPEN_EXEC, 0x00001000); /* File was opened for exec */
        assert_eq!(FAN_Q_OVERFLOW, 0x00004000); /* Event queued overflowed */
        assert_eq!(FAN_FS_ERROR, 0x00008000); /* Filesystem error */
        assert_eq!(FAN_OPEN_PERM, 0x00010000); /* File open in perm check */
        assert_eq!(FAN_ACCESS_PERM, 0x00020000); /* File accessed in perm check */
        assert_eq!(FAN_OPEN_EXEC_PERM, 0x00040000); /* File open/exec in perm check */
        assert_eq!(FAN_EVENT_ON_CHILD, 0x08000000); /* Interested in child events */
        assert_eq!(FAN_RENAME, 0x10000000); /* File was renamed */
        assert_eq!(FAN_ONDIR, 0x40000000); /* Event occurred against dir */
        /* helper events */
        assert_eq!(FAN_CLOSE, (FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE)); /* close */
        assert_eq!(FAN_MOVE, (FAN_MOVED_FROM | FAN_MOVED_TO)); /* moves */
        /* flags used for fanotify_init() */
        assert_eq!(FAN_CLOEXEC, 0x00000001);
        assert_eq!(FAN_NONBLOCK, 0x00000002);
        /* These are NOT bitwise flags.  Both bits are used together.  */
        assert_eq!(FAN_CLASS_NOTIF, 0x00000000);
        assert_eq!(FAN_CLASS_CONTENT, 0x00000004);
        assert_eq!(FAN_CLASS_PRE_CONTENT, 0x00000008);
    }
}
