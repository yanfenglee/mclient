extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType, Item, AttributeArgs, Meta, ImplItem};
use crate::utils::parse_fn_args;
use crate::symbol::{HEADER, PARAM, BODY, PATH};
//use syn::NestedMeta::{Lit};
use syn::NestedMeta::Lit;
use syn::Lit::Str;
use std::borrow::BorrowMut;
use proc_macro2::TokenStream as TokenStream2;
use crate::macro_impl2::*;

pub(crate) fn mc_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let url = args.get(0).unwrap().to_token_stream().to_string();
    let url = &url[1..url.len()-1];
    println!("url: {}", url);

    let mut item_impl = syn::parse_macro_input!(item as syn::ItemImpl);
    println!("mod: {:?}", item_impl);
    //let ident = &item_impl.ident;

    let mut path = None;
    let mut item_fn1 = None;
    if let ImplItem::Method(impl_method) = &item_impl.items[0] {
        println!("mod fn attrs2: {:?}", impl_method.attrs.first().unwrap().parse_meta());
        let attr = impl_method.attrs.first().unwrap().parse_meta().unwrap();
        println!("path: {}", attr.path().get_ident().unwrap());
        println!("tokenstream: {}", attr.to_token_stream());

        item_fn1 = Some(ItemFn {
            attrs: impl_method.attrs.clone(),
            vis: impl_method.vis.clone(),
            sig: impl_method.sig.clone(),
            block: Box::from(impl_method.block.clone()),
        });

        let res = match attr {
            Meta::Path(path) => None,

            Meta::List(meta) => {
                match meta.nested.first().unwrap() {
                    Lit(Str(token)) => Some(token.value()),
                    _ => None,
                }
            },

            Meta::NameValue(meta) => None,
        };

        path = res;
        //item_fn1 = Some(item_fn.clone());
    }

    println!("match mata: {:?}", path);
    let url = format!("{}{}", url, path.unwrap());

    let func: TokenStream2 = request_gen("GET", url.as_str(), item_fn1.as_mut().unwrap()).into();

    let res = quote! {
        impl Car {
            #func
        }
    };

    println!("mc gen:\n {}", res);

    res.into()
}