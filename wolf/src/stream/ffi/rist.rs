/* automatically generated by rust-bindgen 0.61.0 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const W_MAX_PATH: u32 = 260;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type max_align_t = f64;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rist_data_block {
    _unused: [u8; 0],
}
pub type w_rist_data_block = *mut rist_data_block;
extern "C" {
    #[doc = " initialize rist data block"]
    #[doc = " @param p_block the rist block data,"]
    #[doc = " @returns void"]
    pub fn w_rist_init_data_block(p_block: *mut w_rist_data_block);
}
extern "C" {
    #[doc = " set rist data block"]
    #[doc = " @param p_block the rist block data,"]
    #[doc = " @param p_data_len length of rist block data,"]
    #[doc = " @param p_data the data,"]
    #[doc = " @returns void"]
    pub fn w_rist_set_data_block(
        p_block: w_rist_data_block,
        p_data_len: usize,
        p_data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    #[doc = " get rist data block"]
    #[doc = " @param p_block the rist block data,"]
    #[doc = " @returns pointer to data block"]
    pub fn w_rist_get_data_block(p_block: w_rist_data_block) -> *const ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " get the length of rist data block"]
    #[doc = " @returns the length as size_t"]
    pub fn w_rist_get_data_block_len(p_block: w_rist_data_block) -> usize;
}
extern "C" {
    #[doc = " free the rist data block"]
    #[doc = " @returns void"]
    pub fn w_rist_free_data_block(p_block: *mut w_rist_data_block);
}
pub type rist_log_level = ::std::os::raw::c_int;
pub type rist_ctx_mode = ::std::os::raw::c_int;
pub type rist_profile = ::std::os::raw::c_int;
pub type rist_connection_status = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rist_peer {
    _unused: [u8; 0],
}
pub type w_rist_peer = *mut rist_peer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rist_oob_block {
    _unused: [u8; 0],
}
pub type w_rist_oob_block = *const rist_oob_block;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rist_stats {
    _unused: [u8; 0],
}
pub type w_rist_stats = *const rist_stats;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct w_rist_ctx_t {
    _unused: [u8; 0],
}
pub type w_rist_ctx = *mut w_rist_ctx_t;
pub type w_rist_log_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: rist_log_level,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
pub type w_rist_auth_connect_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_char,
        arg3: u16,
        arg4: *const ::std::os::raw::c_char,
        arg5: u16,
        arg6: w_rist_peer,
    ) -> ::std::os::raw::c_int,
>;
pub type w_rist_auth_disconnect_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: w_rist_peer,
    ) -> ::std::os::raw::c_int,
>;
pub type w_rist_connection_status_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: w_rist_peer,
        arg3: rist_connection_status,
    ),
>;
pub type w_rist_out_of_band_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: w_rist_oob_block,
    ) -> ::std::os::raw::c_int,
>;
pub type w_rist_stats_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: w_rist_stats,
    ) -> ::std::os::raw::c_int,
>;
pub type w_receiver_data_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_void,
        arg2: w_rist_data_block,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn w_rist_init(
        p_rist: *mut w_rist_ctx,
        p_mode: rist_ctx_mode,
        p_profile: rist_profile,
        p_loss_percentage: u16,
        p_log_level: rist_log_level,
        p_log_callback: w_rist_log_callback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_connect(
        p_rist: w_rist_ctx,
        p_url: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_set_auth_handler(
        p_rist: w_rist_ctx,
        p_arg: *mut ::std::os::raw::c_void,
        p_on_auth_connect_cb: w_rist_auth_connect_callback,
        p_on_auth_disconnect_cb: w_rist_auth_disconnect_callback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_set_conn_status_callback(
        p_rist: w_rist_ctx,
        p_arg: *mut ::std::os::raw::c_void,
        p_on_connect_status_cb: w_rist_connection_status_callback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_set_out_of_band_callback(
        p_rist: w_rist_ctx,
        p_arg: *mut ::std::os::raw::c_void,
        p_on_out_of_band_cb: w_rist_out_of_band_callback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_set_stats_callback(
        p_rist: w_rist_ctx,
        p_interval: ::std::os::raw::c_int,
        p_arg: *mut ::std::os::raw::c_void,
        p_on_stats_cb: w_rist_stats_callback,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_write(p_rist: w_rist_ctx, p_block: w_rist_data_block) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_read(
        p_rist: w_rist_ctx,
        p_block: *mut w_rist_data_block,
        p_timeout: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_set_receiver_data_callback(
        p_rist: w_rist_ctx,
        p_on_receive_cb: w_receiver_data_callback,
        p_arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn w_rist_fini(p_rist: *mut w_rist_ctx);
}
