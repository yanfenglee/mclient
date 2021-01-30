mod macro_impl;
mod utils;
mod symbol;
mod macro_impl2;

use proc_macro::{TokenStream};


#[proc_macro_attribute]
pub fn get(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl::get_impl(args, item)
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl::post_impl(args, item)
}

#[proc_macro_attribute]
pub fn get2(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl2::get_impl(args, item)
}
//
// #[proc_macro_attribute]
// pub fn post2(args: TokenStream, item: TokenStream) -> TokenStream {
//     macro_impl2::post_impl(args, item)
// }