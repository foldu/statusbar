/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}
pub const _SYS_SYSINFO_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 28;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const SI_LOAD_SHIFT: u32 = 16;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fd_set>())).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__kernel_fsid_t>())).val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug)]
pub struct sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3usize],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_sysinfo() {
    assert_eq!(
        ::std::mem::size_of::<sysinfo>(),
        112usize,
        concat!("Size of: ", stringify!(sysinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<sysinfo>(),
        8usize,
        concat!("Alignment of ", stringify!(sysinfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).uptime as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(uptime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).loads as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(loads)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).totalram as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(totalram)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).freeram as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(freeram)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).sharedram as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(sharedram)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).bufferram as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(bufferram)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).totalswap as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(totalswap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).freeswap as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(freeswap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).procs as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(procs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).pad as *const _ as usize },
        82usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).totalhigh as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(totalhigh)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).freehigh as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(freehigh)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>())).mem_unit as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(mem_unit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sysinfo>()))._f as *const _ as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(sysinfo),
            "::",
            stringify!(_f)
        )
    );
}
extern "C" {
    pub fn sysinfo(__info: *mut sysinfo) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_nprocs_conf() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_nprocs() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn get_phys_pages() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn get_avphys_pages() -> ::std::os::raw::c_long;
}
