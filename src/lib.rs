#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![forbid(unsafe_code)]

pub mod str_utils;

#[macro_use]
extern crate mclient_macro;

pub use mclient_macro::{get, post, put, delete};