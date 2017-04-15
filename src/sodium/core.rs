// sodium/core.h

extern "C" {
    pub fn sodium_init() -> c_int;
}

/// `init()` initializes libsodium.
pub fn init() -> bool {
    unsafe { sodium_init() != -1 }
}

#[test]
fn test_init() {
    assert!(init());
}