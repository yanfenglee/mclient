extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType};
use crate::utils::{parse_fn_args, GenParam};
use crate::symbol::{HEADER, PARAM, BODY, PATH};
//use syn::NestedMeta::{Lit};
//use syn::NestedMeta::Lit;
//use syn::Lit::Str;
//use std::borrow::BorrowMut;
//use proc_macro2::TokenStream as TokenStream2;

pub(crate) fn find_return_type(target_fn: &ItemFn) -> proc_macro2::TokenStream {
    let mut return_ty = target_fn.sig.output.to_token_stream();
    match &target_fn.sig.output {
        ReturnType::Type(_, b) => {
            return_ty = b.to_token_stream();
        }
        _ => {}
    }

    return_ty
}

pub(crate) fn request_impl(method: &str, args: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = syn::parse_macro_input!(item as syn::ItemFn);

    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let url = args.get(0).unwrap().to_token_stream();
    let url = &format!("{}", url);
    let url = &url[1..url.len()-1];

    let mut param = GenParam {
        url: url.to_string(),
        method: method.to_string(),
        item_fn: item_fn,
    };

    request_gen(&mut param)
}

pub(crate) fn request_gen(param: &mut GenParam) -> TokenStream {
    let method = param.method.as_str();
    let url = param.url.as_str();
    let item_fn = &mut param.item_fn;

    let fn_args = parse_fn_args(item_fn);

    let return_ty = find_return_type(item_fn);

    //let attrs = &item_fn.attrs;
    let vis = &item_fn.vis;
    let sig = &item_fn.sig;

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig.fn_token, "only async fn is supported")
            .to_compile_error()
            .into();
    }

    // for header
    let header_name: Vec<String> = fn_args.iter()
        .filter(|x| x.path1 == HEADER)
        .map(|x| x.value.clone())
        .collect();

    let header_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == HEADER)
        .map(|x| x.var.clone())
        .collect();

    // for query string
    let param_name: Vec<String> = fn_args.iter()
        .filter(|x| x.path1 == PARAM)
        .map(|x| x.value.clone())
        .collect();

    let param_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == PARAM)
        .map(|x| x.var.clone())
        .collect();

    // for path variable
    let path_name: Vec<String> = fn_args.iter()
        .filter(|x| x.path1 == PATH)
        .map(|x| x.value.clone())
        .collect();

    let path_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == PATH)
        .map(|x| x.var.clone())
        .collect();

    // for request body TODO constrain only one json body?
    let body_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == BODY)
        .map(|x| x.var.clone())
        .collect();

    let return_ty_str = format!("{}", return_ty);
    let is_string = return_ty_str.starts_with("Result < String,");

    let ret_fn = if is_string {
        quote! {text}
    } else {
        quote! {json}
    };

    let stream = quote! {

        //#(#attrs)*
        #vis #sig {
            use std::str::FromStr;
            use std::collections::HashMap;
            use mclient;

            // process path variable
            let mut path_variables: HashMap<&str,String> = HashMap::new();
            #(
                path_variables.insert(#path_name, format!("{}", #path_value));
            )*

            let url = mclient::str_utils::replace_named(#url, &path_variables);

            // begin build request
            let client = mclient::Client::new();

            let method = mclient::Method::from_str(#method).unwrap();
            let mut reqb = client.request(method, mclient::Url::parse(url.as_str()).unwrap());

            #(
                reqb = reqb.header(#header_name, #header_value);
            )*

            #(
                reqb = reqb.query(&[(#param_name, #param_value),]);
            )*

            #(
                reqb = reqb.json(#body_value);
            )*

            let resp: #return_ty  = reqb.send().await?.#ret_fn().await;

            resp
        }
    };

    //println!("............gen macro get :\n {}", stream);

    stream.into()
}
//
// pub(crate) fn mc_impl(args: TokenStream, item: TokenStream) -> TokenStream {
//     let args = syn::parse_macro_input!(args as syn::AttributeArgs);
//     let url = args.get(0).unwrap().to_token_stream().to_string();
//     let url = &url[1..url.len()-1];
//     println!("url: {}", url);
//
//     let mut item_mod = syn::parse_macro_input!(item as syn::ItemMod);
//     println!("mod: {:?}", item_mod);
//     let vis = &item_mod.vis;
//     let ident = &item_mod.ident;
//
//     let mut path = None;
//     let mut item_fn1 = None;
//     if let Item::Fn(item_fn) = &item_mod.content.unwrap().1[0] {
//         println!("mod fn attrs2: {:?}", item_fn.attrs.first().unwrap().parse_meta());
//         let attr = item_fn.attrs.first().unwrap().parse_meta().unwrap();
//         println!("path: {}", attr.path().get_ident().unwrap());
//         println!("tokenstream: {}", attr.to_token_stream());
//
//         let res = match attr {
//             Meta::Path(path) => None,
//
//             Meta::List(meta) => {
//                 match meta.nested.first().unwrap() {
//                     Lit(Str(token)) => Some(token.value()),
//                     _ => None,
//                 }
//             },
//
//             Meta::NameValue(meta) => None,
//         };
//
//         path = res;
//         item_fn1 = Some(item_fn.clone());
//     }
//
//     println!("match mata: {:?}", path);
//     let url = format!("{}{}", url, path.unwrap());
//
//     let func: TokenStream2 = request_gen("GET", url.as_str(), item_fn1.as_mut().unwrap()).into();
//
//     let res = quote! {
//         #vis mod #ident {
//             #func
//         }
//     };
//
//     println!("mc gen:\n {}", res);
//
//     res.into()
// }