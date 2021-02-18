#![allow(unused)]

extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType, Item, AttributeArgs, Meta, ImplItem, Path, Type};
use crate::utils::{parse_fn_args, GenParam};
use crate::symbol::{HEADER, PARAM, BODY, PATH};
use syn::NestedMeta::Lit;
use syn::Lit::Str;
//use std::borrow::BorrowMut;
use proc_macro2::TokenStream as TokenStream2;
use crate::macro_impl2::*;
use std::ops::Deref;

pub(crate) fn mc_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let url = args.get(0).unwrap().to_token_stream().to_string();
    let url = &url[1..url.len()-1];

    let mut item_impl = syn::parse_macro_input!(item as syn::ItemImpl);
    //println!("====== {:?}", item_impl.self_ty);

    let mut ty = None;
    if let Type::Path(ty_path) = item_impl.self_ty.deref() {
        ty = ty_path.path.get_ident();
    }
    let ty = ty.unwrap();

    let mut fn_tokens = vec![];
    for item in &item_impl.items {
        if let ImplItem::Method(impl_method) = item {
            let attr = impl_method.attrs.first().unwrap().parse_meta().unwrap();

            //println!("asdf-------: {:?}", attr);
            let mut http_method = None;
            let path = match attr {
                Meta::Path(path) => None,

                Meta::List(meta) => {

                    http_method = Some(meta.path.get_ident().unwrap().to_string());

                    match meta.nested.first().unwrap() {
                        Lit(Str(token)) => Some(token.value()),
                        _ => None,
                    }
                },

                Meta::NameValue(meta) => None,
            };

            let url = format!("{}{}", url, path.unwrap());

            let item_fn = ItemFn {
                attrs: impl_method.attrs.clone(),
                vis: impl_method.vis.clone(),
                sig: impl_method.sig.clone(),
                block: Box::from(impl_method.block.clone()),
            };

            //println!("+++++++++{:?}", http_method);

            let method = http_method.unwrap().to_uppercase();

            let mut param = GenParam {
                url: url.to_string(),
                method: method.to_string(),
                item_fn,
            };

            let func: TokenStream2 = request_gen(&mut param).into();
            fn_tokens.push(func);
        }
    }

    let res = quote! {
        impl #ty {
            #(#fn_tokens)*
        }
    };

    //println!("mc gen:\n {}", res);

    res.into()
}