//! Rust language bindings for [libsodium](https://github.com/jedisct1/libsodium)
//!
//! Sodium is a modern, easy-to-use software library for encryption, decryption,
//! signatures, password hashing and more.

// For explanation of lint checks, run `rustc -W help`
#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unused, unused_allocation, unused_attributes, unused_comparisons,
        unused_features, unused_parens, while_true, trivial_numeric_casts, unstable_features,
        unused_import_braces)]
#![warn(unused_extern_crates, unused_qualifications, unused_results)]
// Allow `trivial_casts` to cast `u8` to `c_char`, which is `u8` or `i8`, depending on the
// architecture.
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations, trivial_casts, unsafe_code, variant_size_differences)]

#[macro_use]
extern crate unwrap;

/// This module contains all libsodium bindings.
pub mod sodium;