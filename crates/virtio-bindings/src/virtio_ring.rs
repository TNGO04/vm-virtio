/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
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
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
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
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 31;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __LONG_DOUBLE_USES_FLOAT128: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const __TIMESIZE: u32 = 64;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const __BITS_PER_LONG: u32 = 64;
pub const VRING_DESC_F_NEXT: u32 = 1;
pub const VRING_DESC_F_WRITE: u32 = 2;
pub const VRING_DESC_F_INDIRECT: u32 = 4;
pub const VRING_PACKED_DESC_F_AVAIL: u32 = 7;
pub const VRING_PACKED_DESC_F_USED: u32 = 15;
pub const VRING_USED_F_NO_NOTIFY: u32 = 1;
pub const VRING_AVAIL_F_NO_INTERRUPT: u32 = 1;
pub const VRING_PACKED_EVENT_FLAG_ENABLE: u32 = 0;
pub const VRING_PACKED_EVENT_FLAG_DISABLE: u32 = 1;
pub const VRING_PACKED_EVENT_FLAG_DESC: u32 = 2;
pub const VRING_PACKED_EVENT_F_WRAP_CTR: u32 = 15;
pub const VIRTIO_RING_F_INDIRECT_DESC: u32 = 28;
pub const VIRTIO_RING_F_EVENT_IDX: u32 = 29;
pub const VRING_AVAIL_ALIGN_SIZE: u32 = 2;
pub const VRING_USED_ALIGN_SIZE: u32 = 4;
pub const VRING_DESC_ALIGN_SIZE: u32 = 16;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__fsid_t>())).__val as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
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
#[derive(Debug, Default, Copy, Clone, PartialEq)]
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
pub type __kernel_time64_t = ::std::os::raw::c_longlong;
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
pub type __virtio16 = __u16;
pub type __virtio32 = __u32;
pub type __virtio64 = __u64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_desc {
    pub addr: __virtio64,
    pub len: __virtio32,
    pub flags: __virtio16,
    pub next: __virtio16,
}
#[test]
fn bindgen_test_layout_vring_desc() {
    assert_eq!(
        ::std::mem::size_of::<vring_desc>(),
        16usize,
        concat!("Size of: ", stringify!(vring_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_desc>(),
        8usize,
        concat!("Alignment of ", stringify!(vring_desc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_desc>())).addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_desc>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_desc>())).flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_desc>())).next as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(next)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct vring_avail {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: __IncompleteArrayField<__virtio16>,
}
#[test]
fn bindgen_test_layout_vring_avail() {
    assert_eq!(
        ::std::mem::size_of::<vring_avail>(),
        4usize,
        concat!("Size of: ", stringify!(vring_avail))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_avail>(),
        2usize,
        concat!("Alignment of ", stringify!(vring_avail))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_avail>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_avail),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_avail>())).idx as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_avail),
            "::",
            stringify!(idx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_avail>())).ring as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_avail),
            "::",
            stringify!(ring)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_used_elem {
    pub id: __virtio32,
    pub len: __virtio32,
}
#[test]
fn bindgen_test_layout_vring_used_elem() {
    assert_eq!(
        ::std::mem::size_of::<vring_used_elem>(),
        8usize,
        concat!("Size of: ", stringify!(vring_used_elem))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_used_elem>(),
        4usize,
        concat!("Alignment of ", stringify!(vring_used_elem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_used_elem>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used_elem),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_used_elem>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used_elem),
            "::",
            stringify!(len)
        )
    );
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default)]
pub struct vring_used {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: __IncompleteArrayField<vring_used_elem>,
}
#[test]
fn bindgen_test_layout_vring_used() {
    assert_eq!(
        ::std::mem::size_of::<vring_used>(),
        4usize,
        concat!("Size of: ", stringify!(vring_used))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_used>(),
        4usize,
        concat!("Alignment of ", stringify!(vring_used))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_used>())).flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_used>())).idx as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used),
            "::",
            stringify!(idx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_used>())).ring as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used),
            "::",
            stringify!(ring)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct vring {
    pub num: ::std::os::raw::c_uint,
    pub desc: *mut vring_desc,
    pub avail: *mut vring_avail,
    pub used: *mut vring_used,
}
#[test]
fn bindgen_test_layout_vring() {
    assert_eq!(
        ::std::mem::size_of::<vring>(),
        32usize,
        concat!("Size of: ", stringify!(vring))
    );
    assert_eq!(
        ::std::mem::align_of::<vring>(),
        8usize,
        concat!("Alignment of ", stringify!(vring))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring>())).num as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring>())).desc as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(desc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring>())).avail as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(avail)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring>())).used as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(used)
        )
    );
}
impl Default for vring {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_packed_desc_event {
    pub off_wrap: __le16,
    pub flags: __le16,
}
#[test]
fn bindgen_test_layout_vring_packed_desc_event() {
    assert_eq!(
        ::std::mem::size_of::<vring_packed_desc_event>(),
        4usize,
        concat!("Size of: ", stringify!(vring_packed_desc_event))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_packed_desc_event>(),
        2usize,
        concat!("Alignment of ", stringify!(vring_packed_desc_event))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<vring_packed_desc_event>())).off_wrap as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc_event),
            "::",
            stringify!(off_wrap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_packed_desc_event>())).flags as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc_event),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_packed_desc {
    pub addr: __le64,
    pub len: __le32,
    pub id: __le16,
    pub flags: __le16,
}
#[test]
fn bindgen_test_layout_vring_packed_desc() {
    assert_eq!(
        ::std::mem::size_of::<vring_packed_desc>(),
        16usize,
        concat!("Size of: ", stringify!(vring_packed_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_packed_desc>(),
        8usize,
        concat!("Alignment of ", stringify!(vring_packed_desc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_packed_desc>())).addr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_packed_desc>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_packed_desc>())).id as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vring_packed_desc>())).flags as *const _ as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(flags)
        )
    );
}
