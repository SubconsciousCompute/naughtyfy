//! Calling read(2) for the file descriptor returned by
//! fanotify_init(2) blocks (if the flag FAN_NONBLOCK is not
//! specified in the call to fanotify_init(2)) until either a file
//! event occurs or the call is interrupted by a signal (see
//! signal(7)).
//!
//! The use of one of the flags FAN_REPORT_FID, FAN_REPORT_DIR_FID in
//! fanotify_init(2) influences what data structures are returned to
//! the event listener for each event.  Events reported to a group
//! initialized with one of these flags will use file handles to
//! identify filesystem objects instead of file descriptors.
//!
//! After a successful read(2), the read buffer contains one or more
//! of the following structures:

use libc::{__s32, __u16, __u32, __u64, __u8, c_int};

/// After a successful read(2), the read buffer contains the following structure
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_metadata {
    /// This is the length of the data for the current event and
    /// the offset to the next event in the buffer.  Unless the
    /// group identifies filesystem objects by file handles, the
    /// value of event_len is always [`struct@crate::low_api::FAN_EVENT_METADATA_LEN`].  For
    /// a group that identifies filesystem objects by file
    /// handles, event_len also includes the variable length file
    /// identifier records.
    pub event_len: __u32,
    /// This field holds a version number for the structure.  It
    /// must be compared to [`crate::flags::FANOTIFY_METADATA_VERSION`] to verify
    /// that the structures returned at run time match the
    /// structures defined at compile time.  In case of a
    /// mismatch, the application should abandon trying to use the
    /// fanotify file descriptor.
    pub vers: __u8,
    /// This field is not used.
    pub reserved: __u8,
    /// This is the length of the structure.  The field was introduced
    /// to facilitate the implementation of optional headers per event
    /// type.  No such optional headers exist in the current implementation.
    pub metadata_len: __u16,
    /// This is a bit mask describing the event (see below).
    pub mask: __u64,
    /// This is an open file descriptor for the object being accessed,or FAN_NOFD if a queue overflow occurred.  
    /// If the fanotify file descriptor has been initialized using FAN_REPORT_FID,
    /// applications should expect this value to be set to FAN_NOFD 
    /// for each event that is received.  The file descriptor can be
    /// used to access the contents of the monitored file or directory. 
    /// The reading application is responsible for closing this file descriptor.
    /// When calling fanotify_init(2), the caller may specify (via the event_f_flags argument)
    /// various file status flags that are to be set on the open file description that 
    /// corresponds to this file descriptor.  In addition, the (kernel-internal) 
    /// FMODE_NONOTIFY file status flag is set on the open file description.  
    /// This flag suppresses fanotify event generation. Hence, 
    /// when the receiver of the fanotify event accesses the notified file or directory using this file descriptor,
    /// noadditional events will be created.
    pub fd: __s32,
    /// If flag [`crate::flags::FAN_REPORT_TID`] was set in fanotify_init(2), this is
    /// the TID of the thread that caused the event.  Otherwise, this
    /// the PID of the process that caused the event.
    pub pid: __s32,
}

/// To be used within [`fanotify_event_info_fid`]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
struct __kernel_fsid_t {
    val: [c_int; 2],
}


/// In case of an fanotify group that identifies filesystem objects
/// by file handles, you should also expect to receive one or more
/// additional information records of the structure detailed below
/// following the generic fanotify_event_metadata structure within
/// the read buffer:
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct fanotify_event_info_fid {
    hdr: fanotify_event_info_header,
    fsid: __kernel_fsid_t,
    file_handle: __u8,
}

#[derive(Debug)]
#[repr(C)]
/// It is used to control file access.
pub struct fanotify_response {
    /// This is the file descriptor from the structure
    /// fanotify_event_metadata.
    pub fd: __s32,
    /// This field indicates whether or not the permission is to
    /// be granted.  Its value must be either [`crate::flags::FAN_ALLOW`] to allow
    /// the file operation or [`crate::flags::FAN_DENY`] to deny the file operation.
    pub response: __u32,
}
