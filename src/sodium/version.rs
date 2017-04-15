// sodium/version.h

extern "C" {
    pub fn sodium_version_string() -> *const c_char;
    pub fn sodium_library_version_major() -> c_int;
    pub fn sodium_library_version_minor() -> c_int;
}

/// `version_string()` returns the version string from libsodium.
pub fn version_string() -> &'static str {
    let version = unsafe { CStr::from_ptr(sodium_version_string()) };
    unwrap!(version.to_str())
}

/// `version_major()` returns the major version from libsodium.
pub fn version_major() -> usize {
    unsafe { sodium_library_version_major() as usize }
}

/// `version_minor()` returns the minor version from libsodium.
pub fn version_minor() -> usize {
    unsafe { sodium_library_version_minor() as usize }
}

#[test]
fn test_sodium_library_version_major() {
    assert!(unsafe { sodium_library_version_major() } > 0)
}
#[test]
fn test_sodium_library_version_minor() {
    assert!(unsafe { sodium_library_version_minor() } >= 0)
}
#[test]
fn test_version_string() {
    assert!(!version_string().is_empty());
}