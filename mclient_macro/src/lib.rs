
mod utils;
mod symbol;
mod macro_impl2;

use proc_macro::{TokenStream};

#[proc_macro_attribute]
pub fn get(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl2::request_impl("GET", args, item)
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl2::request_impl("POST", args, item)
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl2::request_impl("PUT", args, item)
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl2::request_impl("DELETE", args, item)
}

#[proc_macro_attribute]
pub fn mc(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl2::mc_impl(args, item)
}