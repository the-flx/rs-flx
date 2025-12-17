// These crate-wide attrs are useful because `bindgen`:

// generates types that don't and can't conform to the Rust naming conventions.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(feature = "bindgen"))]
include!("./flx.rs");

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "./flx.rs"));
