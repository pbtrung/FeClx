// sodium/utils.h

extern "C" {
    pub fn sodium_memzero(pnt: *mut c_void, len: size_t);
    pub fn sodium_memcmp(b1_: *const c_void, b2_: *const c_void, len: size_t) -> c_int;
    pub fn sodium_compare(b1_: *const u8, b2_: *const u8, len: size_t) -> c_int;
    pub fn sodium_is_zero(n: *const u8, nlen: size_t) -> c_int;

    pub fn sodium_increment(n: *mut u8, len: size_t);
    pub fn sodium_add(a: *mut u8, b: *const u8, len: size_t);
    pub fn sodium_bin2hex(hex: *mut c_char, hex_maxlen: size_t, bin: *const u8, bin_len: size_t) -> *mut c_char;
    pub fn sodium_hex2bin(bin: *const u8, bin_maxlen: size_t, hex: *const c_char, hex_len: size_t,
                          ignore: *const c_char, bin_len: *mut size_t, hex_end: *const *const c_char) -> c_int;

    pub fn sodium_mlock(addr: *mut c_void, len: size_t) -> c_int;
    pub fn sodium_munlock(addr: *mut c_void, len: size_t) -> c_int;

    pub fn sodium_malloc(len: size_t) -> *mut c_void;
    pub fn sodium_allocarray(count: size_t, size: size_t) -> *mut c_void;
    pub fn sodium_free(ptr: *mut c_void);

    pub fn sodium_mprotect_noaccess(ptr: *mut c_void) -> c_int;
    pub fn sodium_mprotect_readonly(ptr: *mut c_void) -> c_int;
    pub fn sodium_mprotect_readwrite(ptr: *mut c_void) -> c_int;
}