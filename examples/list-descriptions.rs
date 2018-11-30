//! An example to test the bindings while doing something useful (displaying the available query
//! languages).

// For explanation of lint checks, run `rustc -W help`.
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

extern crate ffi_utils;
extern crate rasqal_rs;
#[macro_use]
extern crate unwrap;

use ffi_utils::from_c_str;
use rasqal_rs::*;
use std::slice;

unsafe fn print_syntax_description(desc: *const raptor_syntax_description) {
    let desc = *desc;

    let names = slice::from_raw_parts(desc.names, desc.names_count as usize);
    let mime_types = slice::from_raw_parts(desc.mime_types, desc.mime_types_count as usize);
    let uri_strings = slice::from_raw_parts(desc.uri_strings, desc.uri_strings_count as usize);

    println!("{}", unwrap!(from_c_str(desc.label)));

    println!("  Names:");
    for name in names {
        println!("    {}", unwrap!(from_c_str(*name)));
    }

    println!("  MIME types:");
    for mime_type in mime_types {
        let mime_type = *mime_type;

        println!("    {}, Q: {}", unwrap!(from_c_str(mime_type.mime_type)), mime_type.q);
    }

    println!("  URI strings:");
    for uri_string in uri_strings {
        println!("    {}", unwrap!(from_c_str(*uri_string)));
    }

    // println!("Bitflags:");
    // println!("  {:b}", desc.flags);
}

fn main() {
    unsafe {
        let world = rasqal_new_world();

        println!("Printing query language descriptions...");
        println!();

        let mut i = 0;
        loop {
            let desc = rasqal_world_get_query_language_description(world, i);
            if desc.is_null() {
                break;
            }

            print_syntax_description(desc);

            println!();
            i += 1;
        }

        println!("Printing query results format descriptions...");
        println!();

        let mut i = 0;
        loop {
            let desc = rasqal_world_get_query_results_format_description(world, i);
            if desc.is_null() {
                break;
            }

            print_syntax_description(desc);

            println!();
            i += 1;
        }
    }
}
