//! Contains all the necessary structs
//! needed for fanotify to work

use libc::{__s32, __u16, __u32, __u64, __u8, c_int};
use std::ffi::OsStr;

// For documentaton linking
#[allow(unused_imports)]
use crate::api::*;
#[allow(unused_imports)]
use crate::flags::*;

/// After a successful read(2), the read buffer contains the following structure
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_metadata {
    /// This is the length of the data for the current event and
    /// the offset to the next event in the buffer.  Unless the
    /// group identifies filesystem objects by file handles, the
    /// value of event_len is always `FAN_EVENT_METADATA_LEN`.  For
    /// a group that identifies filesystem objects by file
    /// handles, event_len also includes the variable length file
    /// identifier records.
    pub event_len: __u32,
    /// This field holds a version number for the structure.  It
    /// must be compared to [`FANOTIFY_METADATA_VERSION`] to verify
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
    /// If flag [`FAN_REPORT_TID`] was set in fanotify_init(2), this is
    /// the TID of the thread that caused the event.  Otherwise, this
    /// the PID of the process that caused the event.
    pub pid: __s32,
}

/// To be used within [`fanotify_event_info_fid`]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
struct __kernel_fsid_t {
    #[allow(dead_code)]
    val: [c_int; 2],
}

/// This is the header part of [`fanotify_event_info_fid`]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

/// In case of an fanotify group that identifies filesystem objects
/// by file handles (i.e. [`fanotify_init()`] initilised with
/// [`FAN_REPORT_FID`] or [`FAN_REPORT_DIR_FID`]),
/// you should also expect to receive one or more
/// additional information records of the structure detailed below
/// following the generic [`fanotify_event_metadata`] structure within
/// the read buffer:
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_info_fid {
    /// It is a generic header that contains information used to
    /// describe an additional information record attached to the
    /// event.  For example, when an fanotify file descriptor is
    /// created using [`FAN_REPORT_FID`], a single information record
    /// is expected to be attached to the event with info_type
    /// field value of [`FAN_EVENT_INFO_TYPE_FID`].  When an fanotify
    /// file descriptor is created using the combination of
    /// [`FAN_REPORT_FID`] and [`FAN_REPORT_DIR_FID`], there may be two
    /// information records attached to the event: one with
    /// info_type field value of [`FAN_EVENT_INFO_TYPE_DFID`],
    /// identifying a parent directory object, and one with
    /// info_type field value of [`FAN_EVENT_INFO_TYPE_FID`],
    /// identifying a non-directory object.  The
    /// fanotify_event_info_header contains a len field.  The
    /// value of len is the size of the additional information
    /// record including the fanotify_event_info_header itself.
    /// The total size of all additional information records is
    /// not expected to be bigger than ( event_len - metadata_len ).
    hdr: fanotify_event_info_header,

    /// This is a unique identifier of the filesystem containing
    /// the object associated with the event.  It is a structure
    /// of type __kernel_fsid_t and contains the same value as
    /// f_fsid when calling statfs(2).
    fsid: __kernel_fsid_t,

    /// This is a variable length structure of type struct
    /// file_handle.  It is an opaque handle that corresponds to a
    /// specified object on a filesystem as returned by
    /// name_to_handle_at(2).  It can be used to uniquely identify
    /// a file on a filesystem and can be passed as an argument to
    /// open_by_handle_at(2).  Note that for the directory entry
    /// modification events [`FAN_CREATE`], [`FAN_DELETE`], and [`FAN_MOVE`],
    /// the file_handle identifies the modified directory and not
    /// the created/deleted/moved child object.  If the value of
    /// info_type field is `FAN_EVENT_INFO_TYPE_DFID_NAME`, the file
    /// handle is followed by a null terminated string that
    /// identifies the created/deleted/moved directory entry name.
    /// For other events such as [`FAN_OPEN`], [`FAN_ATTRIB`],
    /// [`FAN_DELETE_SELF`], and [`FAN_MOVE_SELF`], if the value of
    /// info_type field is `FAN_EVENT_INFO_TYPE_FID`, the
    /// file_handle identifies the object correlated to the event.
    /// If the value of info_type field is
    /// `FAN_EVENT_INFO_TYPE_DFID`, the file_handle identifies the
    /// directory object correlated to the event or the parent
    /// directory of a non-directory object correlated to the
    /// event.  If the value of info_type field is
    /// `FAN_EVENT_INFO_TYPE_DFID_NAME`, the file_handle identifies
    /// the same directory object that would be reported with
    /// `FAN_EVENT_INFO_TYPE_DFID` and the file handle is followed
    /// by a null terminated string that identifies the name of a
    /// directory entry in that directory, or '.' to identify the
    /// directory object itself.
    file_handle: __u8,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct fanotify_event_with_fid {
    /// This is a structure of type [`fanotify_event_metadata`].
    /// It contains the event metadata
    pub metadata: fanotify_event_metadata,

    /// This is a structure of type [`fanotify_event_info_fid`].
    /// It contains the file system id and file handle.
    pub fid: fanotify_event_info_fid,
}

#[derive(Debug)]
#[repr(C)]
/// It is used to control file access.
pub struct fanotify_response {
    /// This is the file descriptor from the structure
    /// fanotify_event_metadata.
    pub fd: __s32,
    /// This field indicates whether or not the permission is to
    /// be granted.  Its value must be either [`FAN_ALLOW`] to allow
    /// the file operation or [`FAN_DENY`] to deny the file operation.
    pub response: __u32,
}

/// Converts the implemented types to [`OsStr`] using `as_os_str()` method. <br>
/// This is *NOT* [`std::path::Path`]
///
/// # Example
/// ```
/// # use std::ffi::OsStr;
/// # pub trait Path {
/// # fn as_os_str(&self) -> &OsStr;
/// # }
/// #
/// # impl Path for str {
/// #     fn as_os_str(&self) -> &OsStr {
/// #         OsStr::new(self)
/// #     }
/// # }
/// let path = std::path::Path::new("/usr/bin");
/// let ostr = path.as_os_str();
/// assert_eq!(ostr,"/usr/bin");
/// ```
pub trait Path {
    fn as_os_str(&self) -> &OsStr;
}

impl Path for std::path::Path {
    fn as_os_str(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl Path for str {
    fn as_os_str(&self) -> &OsStr {
        OsStr::new(self)
    }
}

impl Path for String {
    fn as_os_str(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}
