//! Exports a few trivial procedural macros for testing.

pub static PROC_MACRO_TEST_LOCATION: &str =
    include_str!(concat!(env!("OUT_DIR"), "/proc_macro_test_location.txt"));
