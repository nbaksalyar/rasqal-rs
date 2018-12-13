#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
    html_favicon_url = "https://maidsafe.net/img/favicon.ico",
    test(attr(forbid(warnings)))
)]
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

use ffi_utils::{from_c_str, vec_clone_from_raw_parts};
use rasqal_rs::*;
use std::ffi::CString;
use std::{mem, ptr};

macro_rules! map {
    { } => {
        ::std::collections::HashMap::new()
    };
    { $( $key:expr => $value:expr ),+ , } => {
        // Rule with trailing comma.
        map!{ $( $key => $value),+ }
    };
    { $( $key:expr => $value:expr ),* } => {
        {
            let mut _map = ::std::collections::HashMap::new();

            $(
                let _ = _map.insert($key, $value);
            )*

            _map
        }
    }
}

// Convert a raptor_uri to a String.
unsafe fn raptor_uri_to_string(uri: *mut raptor_uri) -> String {
    let mut len: usize = mem::zeroed();
    let uri_string = raptor_uri_as_counted_string(uri, &mut len);
    assert!(!uri_string.is_null());
    // raptor_uri strings are not null-terminated
    let uri_string = vec_clone_from_raw_parts(uri_string, len);

    unwrap!(String::from_utf8(uri_string))
}

// Init and return world objects. Do NOT free the returned raptor_world.
unsafe fn init_world() -> (*mut rasqal_world, *mut raptor_world) {
    let rasqal_world = rasqal_new_world();
    assert!(!rasqal_world.is_null());
    assert_eq!(rasqal_world_open(rasqal_world), 0);
    let raptor_world = rasqal_world_get_raptor(rasqal_world);
    assert!(!raptor_world.is_null());

    (rasqal_world, raptor_world)
}

unsafe fn init_data(
    world: *mut rasqal_world,
    raptor_world: *mut raptor_world,
    file: &str,
    mime: &str,
    parser: &str,
) -> (*mut rasqal_data_graph, *mut raptor_uri) {
    // Construct raptor_uri object.
    let data_uri = {
        let file_path = unwrap!(CString::new(file)).into_bytes_with_nul();
        // Construct a URI from a filepath.
        let data_uri = raptor_new_uri_from_uri_or_file_string(
            raptor_world,
            ptr::null_mut(),    // existing base URI
            file_path.as_ptr(), // filename
        );
        // Use this function instead when we already have the URI.
        // let data_uri = raptor_new_uri(raptor_world, file_path.as_ptr());
        assert!(!data_uri.is_null());

        data_uri
    };

    println!(
        "Building data graph from {}...",
        raptor_uri_to_string(data_uri)
    );

    // Build the data graph
    let data_graph = {
        let mime = unwrap!(CString::new(mime));
        let parser = unwrap!(CString::new(parser));
        let data_graph = rasqal_new_data_graph_from_uri(
            world,
            data_uri,                                             // source URI
            ptr::null_mut(),                                      // name of graph
            rasqal_data_graph_flags_RASQAL_DATA_GRAPH_BACKGROUND, // type of graph
            mime.as_ptr(),                                        // data format mime type
            parser.as_ptr(),                                      // data format parser name
            ptr::null_mut(),                                      // data format URI
        );
        assert!(!data_graph.is_null());

        data_graph
    };

    // Print the data graph in debug-mode.
    // {
    //     println!("\nData graph (debug):");
    //     println!();
    //     assert_eq!(rasqal_data_graph_print(data_graph, __stdoutp), 0);
    //     println!();
    // }

    (data_graph, data_uri)
}

unsafe fn prepare_query(
    world: *mut rasqal_world,
    data_graph: *mut rasqal_data_graph,
    query_string: &str,
) -> *mut rasqal_query {
    // Construct the query.
    let query = {
        let language = unwrap!(CString::new("sparql"));
        let query = rasqal_new_query(
            world,
            language.as_ptr(), // query language name
            ptr::null(),       // language URI
        );
        assert!(!query.is_null());

        query
    };

    // Prepare the query.
    {
        let query_string = unwrap!(CString::new(query_string)).into_bytes();
        assert_eq!(
            rasqal_query_prepare(
                query,
                query_string.as_ptr(), // query string
                ptr::null_mut()        // base URI of query string
            ),
            0
        );
        assert_eq!(rasqal_query_add_data_graph(query, data_graph), 0);
    }

    // Print the query in debug-mode.
    // {
    //     println!("\nQuery (debug):");
    //     println!();
    //     assert_eq!(rasqal_query_print(query, __stdoutp), 0);
    //     println!();
    // }

    query
}

unsafe fn execute_query(
    _raptor_world: *mut raptor_world,
    query: *mut rasqal_query,
) -> *mut rasqal_query_results {
    // Store the results in memory to enable rewinding.
    // The second parameter (1) needs to be non-zero. The docs don't explain this parameter.
    assert_eq!(rasqal_query_set_store_results(query, 1), 0);

    // Execute the prepared query.
    let results = rasqal_query_execute(query);
    assert!(!results.is_null());

    // Print the results in debug-mode.
    // {
    //     let iostream = raptor_new_iostream_to_file_handle(raptor_world, __stdoutp);
    //     let name = unwrap!(CString::new("turtle"));
    //     let mime = unwrap!(CString::new("text/turtle"));

    //     println!("\nResults (debug):");
    //     println!();
    //     assert_eq!(
    //         rasqal_query_results_write(
    //             iostream,
    //             results,
    //             name.as_ptr(),   // format name
    //             mime.as_ptr(),   // format MIME type
    //             ptr::null_mut(), // format URI
    //             ptr::null_mut(), // base URI
    //         ),
    //         0
    //     );
    //     assert_eq!(rasqal_query_results_rewind(results), 0);
    //     println!();

    //     raptor_free_iostream(iostream);
    // }

    results
}

// Test basic RDF query operations.
#[test]
fn basic() {
    unsafe {
        let data = "./tests/test-basic-data.ttl";
        let query_string = "PREFIX  dc: <http://purl.org/dc/elements/1.1/> \
                            PREFIX  x: <http://example.org/ns#> \
                            SELECT  ?title ?price \
                            WHERE \
                            { ?book dc:title ?title .  \
                            OPTIONAL \
                            { ?book x:price ?price .  \
                            FILTER (?price < 15) . \
                            } . \
                            }";
        let expected = vec![
            map! {
                "title" => "TITLE 1",
                "price" => "10",
            },
            map! {
                "title" => "TITLE 2"
            },
            map! {
                "title" => "TITLE 3"
            },
        ];

        println!("Initializing libraries...");

        let (world, raptor_world) = init_world();

        println!("Initializing data graph...");

        let mime = "text/turtle";
        let parser = "turtle";
        let (data_graph, data_uri) = init_data(world, raptor_world, data, mime, parser);

        println!("Preparing query...");

        let query = prepare_query(world, data_graph, query_string);

        println!("Executing query...");

        let results = execute_query(raptor_world, query);

        println!("Checking results...");

        {
            // Iterate over solutions.
            println!();
            let mut solution_i = 0;
            while rasqal_query_results_finished(results) == 0 {
                println!("Solution {}:", solution_i);
                let count = rasqal_query_results_get_bindings_count(results);

                for i in 0..count {
                    let name = rasqal_query_results_get_binding_name(results, i);
                    if name.is_null() {
                        break;
                    }
                    let value = rasqal_query_results_get_binding_value(results, i);
                    if value.is_null() {
                        break;
                    }

                    let name = unwrap!(from_c_str(name as *const i8));
                    let value = unwrap!(from_c_str(rasqal_literal_as_string(value) as *const i8));

                    // Compare to actual results.
                    assert_eq!(value, expected[solution_i][name.as_str()]);
                    println!("  {}: {}", name, value);
                }

                solution_i += 1;
                if rasqal_query_results_next(results) != 0 {
                    assert_eq!(solution_i, expected.len());
                }
            }
            println!();
        }

        println!("Freeing objects...");

        raptor_free_uri(data_uri);

        rasqal_free_query_results(results);
        rasqal_free_query(query);
        rasqal_free_data_graph(data_graph);
        rasqal_free_world(world);

        println!("Test finished successfully!");
    }
}
