/* automatically generated by rust-bindgen 0.59.2 */

use super::luajit::lua_State;

pub const __GNUC_VA_LIST: u32 = 1;
pub const WINVER: u32 = 1281;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 0;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 0;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_64_BIT_INO_T: &[u8; 9usize] = b"$INODE64\0";
pub const __DARWIN_SUF_1050: &[u8; 6usize] = b"$1050\0";
pub const __DARWIN_SUF_EXTSN: &[u8; 14usize] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const __DARWIN_CLK_TCK: u32 = 100;
pub const CHAR_BIT: u32 = 8;
pub const MB_LEN_MAX: u32 = 6;
pub const CLK_TCK: u32 = 100;
pub const SCHAR_MAX: u32 = 127;
pub const SCHAR_MIN: i32 = -128;
pub const UCHAR_MAX: u32 = 255;
pub const CHAR_MAX: u32 = 127;
pub const CHAR_MIN: i32 = -128;
pub const USHRT_MAX: u32 = 65535;
pub const SHRT_MAX: u32 = 32767;
pub const SHRT_MIN: i32 = -32768;
pub const UINT_MAX: u32 = 4294967295;
pub const INT_MAX: u32 = 2147483647;
pub const INT_MIN: i32 = -2147483648;
pub const ULONG_MAX: i32 = -1;
pub const LONG_MAX: u64 = 9223372036854775807;
pub const LONG_MIN: i64 = -9223372036854775808;
pub const ULLONG_MAX: i32 = -1;
pub const LLONG_MAX: u64 = 9223372036854775807;
pub const LLONG_MIN: i64 = -9223372036854775808;
pub const LONG_BIT: u32 = 64;
pub const SSIZE_MAX: u64 = 9223372036854775807;
pub const WORD_BIT: u32 = 32;
pub const SIZE_T_MAX: i32 = -1;
pub const UQUAD_MAX: i32 = -1;
pub const QUAD_MAX: u64 = 9223372036854775807;
pub const QUAD_MIN: i64 = -9223372036854775808;
pub const ARG_MAX: u32 = 1048576;
pub const CHILD_MAX: u32 = 266;
pub const GID_MAX: u32 = 2147483647;
pub const LINK_MAX: u32 = 32767;
pub const MAX_CANON: u32 = 1024;
pub const MAX_INPUT: u32 = 1024;
pub const NAME_MAX: u32 = 255;
pub const NGROUPS_MAX: u32 = 16;
pub const UID_MAX: u32 = 2147483647;
pub const OPEN_MAX: u32 = 10240;
pub const PATH_MAX: u32 = 1024;
pub const PIPE_BUF: u32 = 512;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const CHARCLASS_NAME_MAX: u32 = 14;
pub const COLL_WEIGHTS_MAX: u32 = 2;
pub const EQUIV_CLASS_MAX: u32 = 2;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 255;
pub const NZERO: u32 = 20;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_EQUIV_CLASS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_KEYS_MAX: u32 = 512;
pub const PTHREAD_STACK_MIN: u32 = 8192;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_SS_REPL_MAX: u32 = 4;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TRACE_EVENT_NAME_MAX: u32 = 30;
pub const _POSIX_TRACE_NAME_MAX: u32 = 8;
pub const _POSIX_TRACE_SYS_MAX: u32 = 8;
pub const _POSIX_TRACE_USER_EVENT_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const OFF_MIN: i64 = -9223372036854775808;
pub const OFF_MAX: u64 = 9223372036854775807;
pub const PASS_MAX: u32 = 128;
pub const NL_ARGMAX: u32 = 9;
pub const NL_LANGMAX: u32 = 14;
pub const NL_MSGMAX: u32 = 32767;
pub const NL_NMAX: u32 = 1;
pub const NL_SETMAX: u32 = 255;
pub const NL_TEXTMAX: u32 = 2048;
pub const _XOPEN_IOV_MAX: u32 = 16;
pub const IOV_MAX: u32 = 1024;
pub const _XOPEN_NAME_MAX: u32 = 255;
pub const _XOPEN_PATH_MAX: u32 = 1024;
pub const LUA_MULTILIB: &[u8; 4usize] = b"lib\0";
pub const LUA_LMULTILIB: &[u8; 4usize] = b"lib\0";
pub const LUA_LROOT: &[u8; 11usize] = b"/usr/local\0";
pub const LUA_LUADIR: &[u8; 10usize] = b"/lua/5.1/\0";
pub const LUA_LJDIR: &[u8; 21usize] = b"/luajit-2.1.0-beta2/\0";
pub const LUA_JROOT: &[u8; 11usize] = b"/usr/local\0";
pub const LUA_JPATH: &[u8; 43usize] = b";/usr/local/share/luajit-2.1.0-beta2/?.lua\0";
pub const LUA_LLDIR: &[u8; 26usize] = b"/usr/local/share/lua/5.1/\0";
pub const LUA_LCDIR: &[u8; 24usize] = b"/usr/local/lib/lua/5.1/\0";
pub const LUA_LLPATH: &[u8; 68usize] =
    b";/usr/local/share/lua/5.1/?.lua;/usr/local/share/lua/5.1/?/init.lua\0";
pub const LUA_LCPATH1: &[u8; 29usize] = b";/usr/local/lib/lua/5.1/?.so\0";
pub const LUA_LCPATH2: &[u8; 35usize] = b";/usr/local/lib/lua/5.1/loadall.so\0";
pub const LUA_PATH_DEFAULT : & [u8 ; 117usize] = b"./?.lua;/usr/local/share/luajit-2.1.0-beta2/?.lua;/usr/local/share/lua/5.1/?.lua;/usr/local/share/lua/5.1/?/init.lua\0" ;
pub const LUA_PATH: &[u8; 9usize] = b"LUA_PATH\0";
pub const LUA_CPATH: &[u8; 10usize] = b"LUA_CPATH\0";
pub const LUA_INIT: &[u8; 9usize] = b"LUA_INIT\0";
pub const LUA_DIRSEP: &[u8; 2usize] = b"/\0";
pub const LUA_PATHSEP: &[u8; 2usize] = b";\0";
pub const LUA_PATH_MARK: &[u8; 2usize] = b"?\0";
pub const LUA_EXECDIR: &[u8; 2usize] = b"!\0";
pub const LUA_IGMARK: &[u8; 2usize] = b"-\0";
pub const LUA_PATH_CONFIG: &[u8; 11usize] = b"/\n;\n?\n!\n-\n\0";
pub const LUAI_MAXSTACK: u32 = 65500;
pub const LUAI_MAXCSTACK: u32 = 8000;
pub const LUAI_GCPAUSE: u32 = 200;
pub const LUAI_GCMUL: u32 = 200;
pub const LUA_MAXCAPTURES: u32 = 32;
pub const LUA_IDSIZE: u32 = 60;
pub const LUA_NUMBER_SCAN: &[u8; 4usize] = b"%lf\0";
pub const LUA_NUMBER_FMT: &[u8; 6usize] = b"%.14g\0";
pub const LUAI_MAXNUMBER2STR: u32 = 32;
pub const LUA_INTFRMLEN: &[u8; 2usize] = b"l\0";
pub const LUA_VERSION: &[u8; 8usize] = b"Lua 5.1\0";
pub const LUA_RELEASE: &[u8; 10usize] = b"Lua 5.1.4\0";
pub const LUA_VERSION_NUM: u32 = 501;
pub const LUA_COPYRIGHT: &[u8; 41usize] = b"Copyright (C) 1994-2008 Lua.org, PUC-Rio\0";
pub const LUA_AUTHORS: &[u8; 49usize] = b"R. Ierusalimschy, L. H. de Figueiredo & W. Celes\0";
pub const LUA_SIGNATURE: &[u8; 5usize] = b"\x1BLua\0";
pub const LUA_MULTRET: i32 = -1;
pub const LUA_REGISTRYINDEX: i32 = -10000;
pub const LUA_ENVIRONINDEX: i32 = -10001;
pub const LUA_GLOBALSINDEX: i32 = -10002;
pub const LUA_OK: u32 = 0;
pub const LUA_YIELD: u32 = 1;
pub const LUA_ERRRUN: u32 = 2;
pub const LUA_ERRSYNTAX: u32 = 3;
pub const LUA_ERRMEM: u32 = 4;
pub const LUA_ERRERR: u32 = 5;
pub const LUA_TNONE: i32 = -1;
pub const LUA_TNIL: u32 = 0;
pub const LUA_TBOOLEAN: u32 = 1;
pub const LUA_TLIGHTUSERDATA: u32 = 2;
pub const LUA_TNUMBER: u32 = 3;
pub const LUA_TSTRING: u32 = 4;
pub const LUA_TTABLE: u32 = 5;
pub const LUA_TFUNCTION: u32 = 6;
pub const LUA_TUSERDATA: u32 = 7;
pub const LUA_TTHREAD: u32 = 8;
pub const LUA_MINSTACK: u32 = 20;
pub const LUA_GCSTOP: u32 = 0;
pub const LUA_GCRESTART: u32 = 1;
pub const LUA_GCCOLLECT: u32 = 2;
pub const LUA_GCCOUNT: u32 = 3;
pub const LUA_GCCOUNTB: u32 = 4;
pub const LUA_GCSTEP: u32 = 5;
pub const LUA_GCSETPAUSE: u32 = 6;
pub const LUA_GCSETSTEPMUL: u32 = 7;
pub const LUA_GCISRUNNING: u32 = 9;
pub const LUA_HOOKCALL: u32 = 0;
pub const LUA_HOOKRET: u32 = 1;
pub const LUA_HOOKLINE: u32 = 2;
pub const LUA_HOOKCOUNT: u32 = 3;
pub const LUA_HOOKTAILRET: u32 = 4;
pub const LUA_MASKCALL: u32 = 1;
pub const LUA_MASKRET: u32 = 2;
pub const LUA_MASKLINE: u32 = 4;
pub const LUA_MASKCOUNT: u32 = 8;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = u128;
pub type lua_CFunction =
    ::std::option::Option<unsafe extern "C" fn(L: *mut lua_State) -> ::std::os::raw::c_int>;
pub type lua_Reader = ::std::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        ud: *mut ::std::os::raw::c_void,
        sz: *mut size_t,
    ) -> *const ::std::os::raw::c_char,
>;
pub type lua_Writer = ::std::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        p: *const ::std::os::raw::c_void,
        sz: size_t,
        ud: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type lua_Alloc = ::std::option::Option<
    unsafe extern "C" fn(
        ud: *mut ::std::os::raw::c_void,
        ptr: *mut ::std::os::raw::c_void,
        osize: size_t,
        nsize: size_t,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type lua_Number = f64;
pub type lua_Integer = isize;
extern "C" {
    pub fn lua_newstate(f: lua_Alloc, ud: *mut ::std::os::raw::c_void) -> *mut lua_State;
}
extern "C" {
    pub fn lua_close(L: *mut lua_State);
}
extern "C" {
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
}
extern "C" {
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
}
extern "C" {
    pub fn lua_gettop(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_settop(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_pushvalue(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_remove(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_insert(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_replace(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_checkstack(L: *mut lua_State, sz: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_isnumber(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_isstring(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_iscfunction(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_isuserdata(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_type(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_typename(
        L: *mut lua_State,
        tp: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_equal(
        L: *mut lua_State,
        idx1: ::std::os::raw::c_int,
        idx2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_rawequal(
        L: *mut lua_State,
        idx1: ::std::os::raw::c_int,
        idx2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_lessthan(
        L: *mut lua_State,
        idx1: ::std::os::raw::c_int,
        idx2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_tonumber(L: *mut lua_State, idx: ::std::os::raw::c_int) -> lua_Number;
}
extern "C" {
    pub fn lua_tointeger(L: *mut lua_State, idx: ::std::os::raw::c_int) -> lua_Integer;
}
extern "C" {
    pub fn lua_toboolean(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_tolstring(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        len: *mut size_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_objlen(L: *mut lua_State, idx: ::std::os::raw::c_int) -> size_t;
}
extern "C" {
    pub fn lua_tocfunction(L: *mut lua_State, idx: ::std::os::raw::c_int) -> lua_CFunction;
}
extern "C" {
    pub fn lua_touserdata(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_tothread(L: *mut lua_State, idx: ::std::os::raw::c_int) -> *mut lua_State;
}
extern "C" {
    pub fn lua_topointer(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_pushnil(L: *mut lua_State);
}
extern "C" {
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
}
extern "C" {
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
}
extern "C" {
    pub fn lua_pushlstring(L: *mut lua_State, s: *const ::std::os::raw::c_char, l: size_t);
}
extern "C" {
    pub fn lua_pushstring(L: *mut lua_State, s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const ::std::os::raw::c_char,
        argp: *mut __va_list_tag,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_pushcclosure(L: *mut lua_State, fn_: lua_CFunction, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_pushboolean(L: *mut lua_State, b: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn lua_pushthread(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gettable(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_getfield(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn lua_rawget(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_rawgeti(L: *mut lua_State, idx: ::std::os::raw::c_int, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_createtable(
        L: *mut lua_State,
        narr: ::std::os::raw::c_int,
        nrec: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_getmetatable(
        L: *mut lua_State,
        objindex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getfenv(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_settable(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_setfield(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn lua_rawset(L: *mut lua_State, idx: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_rawseti(L: *mut lua_State, idx: ::std::os::raw::c_int, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_setmetatable(
        L: *mut lua_State,
        objindex: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_setfenv(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_call(
        L: *mut lua_State,
        nargs: ::std::os::raw::c_int,
        nresults: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_pcall(
        L: *mut lua_State,
        nargs: ::std::os::raw::c_int,
        nresults: ::std::os::raw::c_int,
        errfunc: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_cpcall(
        L: *mut lua_State,
        func: lua_CFunction,
        ud: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut ::std::os::raw::c_void,
        chunkname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_dump(
        L: *mut lua_State,
        writer: lua_Writer,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_yield(L: *mut lua_State, nresults: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_resume(L: *mut lua_State, narg: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_status(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gc(
        L: *mut lua_State,
        what: ::std::os::raw::c_int,
        data: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_error(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_next(L: *mut lua_State, idx: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_concat(L: *mut lua_State, n: ::std::os::raw::c_int);
}
extern "C" {
    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut ::std::os::raw::c_void) -> lua_Alloc;
}
extern "C" {
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn lua_setlevel(from: *mut lua_State, to: *mut lua_State);
}
pub type lua_Hook =
    ::std::option::Option<unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;
extern "C" {
    pub fn lua_getstack(
        L: *mut lua_State,
        level: ::std::os::raw::c_int,
        ar: *mut lua_Debug,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getinfo(
        L: *mut lua_State,
        what: *const ::std::os::raw::c_char,
        ar: *mut lua_Debug,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_setlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
}
extern "C" {
    pub fn lua_gethookmask(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_gethookcount(L: *mut lua_State) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_upvalueid(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn lua_upvaluejoin(
        L: *mut lua_State,
        idx1: ::std::os::raw::c_int,
        n1: ::std::os::raw::c_int,
        idx2: ::std::os::raw::c_int,
        n2: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_loadx(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut ::std::os::raw::c_void,
        chunkname: *const ::std::os::raw::c_char,
        mode: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
}
extern "C" {
    pub fn lua_copy(
        L: *mut lua_State,
        fromidx: ::std::os::raw::c_int,
        toidx: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn lua_tonumberx(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        isnum: *mut ::std::os::raw::c_int,
    ) -> lua_Number;
}
extern "C" {
    pub fn lua_tointegerx(
        L: *mut lua_State,
        idx: ::std::os::raw::c_int,
        isnum: *mut ::std::os::raw::c_int,
    ) -> lua_Integer;
}
extern "C" {
    pub fn lua_isyieldable(L: *mut lua_State) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lua_Debug {
    pub event: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
    pub namewhat: *const ::std::os::raw::c_char,
    pub what: *const ::std::os::raw::c_char,
    pub source: *const ::std::os::raw::c_char,
    pub currentline: ::std::os::raw::c_int,
    pub nups: ::std::os::raw::c_int,
    pub linedefined: ::std::os::raw::c_int,
    pub lastlinedefined: ::std::os::raw::c_int,
    pub short_src: [::std::os::raw::c_char; 60usize],
    pub i_ci: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_lua_Debug() {
    assert_eq!(
        ::std::mem::size_of::<lua_Debug>(),
        120usize,
        concat!("Size of: ", stringify!(lua_Debug))
    );
    assert_eq!(
        ::std::mem::align_of::<lua_Debug>(),
        8usize,
        concat!("Alignment of ", stringify!(lua_Debug))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).event as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).namewhat as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(namewhat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).what as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(what)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).source as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).currentline as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(currentline)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).nups as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(nups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).linedefined as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(linedefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).lastlinedefined as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(lastlinedefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).short_src as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(short_src)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lua_Debug>())).i_ci as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(i_ci)
        )
    );
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
