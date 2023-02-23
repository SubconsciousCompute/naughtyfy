use libc::{__s32, __u16, __u32, __u64, __u8, c_int};
use std::io::Error;

/// After a successful read(2), the read buffer contains one or more
/// of the following structures:
/// struct fanotify_event_metadata {
///         __u32 event_len;
///         __u8 vers;
///         __u8 reserved;
///         __u16 metadata_len;
///         __aligned_u64 mask;
///         __s32 fd;
///         __s32 pid;
///     };
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_metadata {
    /// This is the length of the data for the current event and the
    /// offset to the next event in the buffer.  Without
    /// FAN_REPORT_FID, the value of event_len is always
    /// FAN_EVENT_METADATA_LEN.  With FAN_REPORT_FID, event_len also
    /// includes the variable length file identifier.
    pub event_len: __u32,
    /// This field holds a version number for the structure.  It must
    /// be compared to FANOTIFY_METADATA_VERSION to verify that the
    /// structures returned at run time match the structures defined
    /// at compile time.  In case of a mismatch, the application
    /// should abandon trying to use the fanotify file descriptor.
    pub vers: __u8,
    /// This field is not used.
    pub reserved: __u8,
    /// This is the length of the structure.  The field was introduced
    /// to facilitate the implementation of optional headers per event
    /// type.  No such optional headers exist in the current implemen‐
    /// tation.
    pub metadata_len: __u16,
    /// This is a bit mask describing the event (see below).
    pub mask: __u64,
    /// This is an open file descriptor for the object being accessed,or FAN_NOFD if a queue overflow occurred.  
    /// If the fanotify file descriptor has been initialized using FAN_REPORT_FID,
    /// applications should expect this value to be set to FAN_NOFDfor each event that is received.  The file descriptor can be
    /// used to access the contents of the monitored file or directory.  The reading application is responsible for closing this file descriptor.
    /// When calling fanotify_init(2), the caller may specify (via the event_f_flags argument) various file status flags that are to
    /// be set on the open file description that corresponds to this file descriptor.  In addition, the (kernel-internal) FMODE_NONOTIFY file status flag is set on the open file description.  
    /// This flag suppresses fanotify event generation. Hence, when the receiver of the fanotify event accesses the notified file or directory using this file descriptor, noadditional events will be created.
    pub fd: __s32,
    /// If flag FAN_REPORT_TID was set in fanotify_init(2), this is
    /// the TID of the thread that caused the event.  Otherwise, this
    /// the PID of the process that caused the event.
    pub pid: __s32,
}

/// In case of an fanotify group that identifies filesystem objects
///        by file handles, you should also expect to receive one or more
///        additional information records of the structure detailed below
///        following the generic fanotify_event_metadata structure within
///        the read buffer:
///
///     struct fanotify_event_info_header {
///         __u8 info_type;
///         __u8 pad;
///         __u16 len;
///     };
///
///     struct fanotify_event_info_fid {
///         struct fanotify_event_info_header hdr;
///         __kernel_fsid_t fsid;
///         unsigned char file_handle[0];
///     };
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

/// fanotify_event_info_fid has __kernel_fsid_t type
#[derive(Debug, Clone, Copy)]
struct __kernel_fsid_t {
    val: [c_int; 2],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct fanotify_event_info_fid {
    hdr: fanotify_event_info_header,
    fsid: __kernel_fsid_t,
    file_handle: __u8,
}

/// Refrence: https://man7.org/linux/man-pages/man2/fanotify_init.2.html
/// fanotify_init()
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
