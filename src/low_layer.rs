use lazy_static::lazy_static;
use libc::{__s32, __u16, __u32, __u64, __u8, c_int};
use std::io::Error;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::slice;


lazy_static! {
    /// Get current platform sizeof of fanotify_event_metadata.
    pub static ref FAN_EVENT_METADATA_LEN: usize = mem::size_of::<fanotify_event_metadata>();
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_metadata {
    pub event_len: __u32,
    pub vers: __u8,
    pub reserved: __u8,
    pub metadata_len: __u16,
    pub mask: __u64,
    pub fd: __s32,
    pub pid: __s32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
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

pub trait FanotifyPath {
    fn as_os_str(&self) -> &std::ffi::OsStr;
}

impl FanotifyPath for std::path::Path {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        self.as_os_str()
    }
}

impl FanotifyPath for str {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self)
    }
}

impl FanotifyPath for String {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self.as_str())
    }
}

pub fn fanotify_mark<P: ?Sized + FanotifyPath>(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    dirfd: i32,
    path: &P,
) -> Result<(), Error> {
    unsafe {
        match libc::fanotify_mark(
            fanotify_fd,
            flags,
            mask,
            dirfd,
            path.as_os_str()
                .as_bytes()
                .iter()
                .map(|p| *p as i8)
                .collect::<Vec<i8>>()
                .as_ptr(),
        ) {
            0 => {
                return Ok(());
            }
            _ => {
                return Err(Error::last_os_error());
            }
        }
    }
}

#[derive(Debug)]
#[repr(C)]
/// It is used to control file access.
pub struct fanotify_response {
    pub fd: __s32,
    pub response: __u32,
}

pub fn fanotify_read(fanotify_fd: i32) -> Vec<fanotify_event_metadata> {
    let mut vec = Vec::new();
    unsafe {
        let buffer = libc::malloc(*FAN_EVENT_METADATA_LEN * 200);
        let sizeof = libc::read(fanotify_fd, buffer, *FAN_EVENT_METADATA_LEN * 200);
        if sizeof != libc::EAGAIN as isize && sizeof > 0 {
            let src = slice::from_raw_parts(
                buffer as *mut fanotify_event_metadata,
                sizeof as usize / *FAN_EVENT_METADATA_LEN,
            );
            vec.extend_from_slice(src);
        }
        libc::free(buffer);
    }
    vec
}
pub fn close_fd(fd: i32) {
    unsafe {
        libc::close(fd);
    }
}
