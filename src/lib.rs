mod macro_impl;
mod utils;
mod symbol;

use proc_macro::{TokenStream};


#[proc_macro_attribute]
pub fn get(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl::get_impl(args, item)
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl::post_impl(args, item)
}
//
// #[proc_macro_attribute]
// pub fn header(_args: TokenStream, item: TokenStream) -> TokenStream {
//     item
// }