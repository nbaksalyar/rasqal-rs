#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
    html_favicon_url = "https://maidsafe.net/img/favicon.ico",
    test(attr(forbid(warnings))),
)]
// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(
    bad_style,
    exceeding_bitshifts,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types,
    warnings
)]
#![deny(
    deprecated,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    overflowing_literals,
    plugin_as_library,
    private_no_mangle_fns,
    private_no_mangle_statics,
    stable_features,
    unconditional_recursion,
    unknown_lints,
    unused,
    unused_allocation,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_parens,
    while_true
)]
#![warn(
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
// Allow `trivial_casts` to cast `u8` to `c_char`, which is `u8` or `i8`, depending on the
// architecture.
#![allow(
    bad_style,
    box_pointers,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    unsafe_code,
    variant_size_differences
)]

extern crate libc;

// Bindgen generated file. Generated using the following commands:
// ```
// bindgen rasqal.h -o bindgen.rs --ctypes-prefix=libc --distrust-clang-mangling \
// --raw-line="use libc;"
//
// sed -ie 's/&'\''static \[u8; [0-9]*usize\] = \(b".*\\0"\)/*const libc::c_char = (\1 as *const \
// libc::c_uchar) as *const libc::c_char/g' bindgen.rs
// ```
mod bindgen;

pub use bindgen::*;
