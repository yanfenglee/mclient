extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType};
use crate::utils::parse_fn_args;
use crate::symbol::{HEADER, PARAM, BODY, PATH};

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
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);


    let fn_args = parse_fn_args(&mut input);

    let return_ty = find_return_type(&input);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;

    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let url = args.get(0).unwrap().to_token_stream();

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig.fn_token, "only async fn is supported")
            .to_compile_error()
            .into();
    }

    // for header
    let header_name: Vec<String> = fn_args.iter()
        .filter(|x| x.path1 == HEADER )
        .map(|x| x.value.clone())
        .collect();

    let header_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == HEADER )
        .map(|x| x.var.clone())
        .collect();

    // for query string
    let param_name: Vec<String> = fn_args.iter()
        .filter(|x| x.path1 == PARAM )
        .map(|x| x.value.clone())
        .collect();

    let param_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == PARAM )
        .map(|x| x.var.clone())
        .collect();

    // for path variable
    let path_name: Vec<String> = fn_args.iter()
        .filter(|x| x.path1 == PATH )
        .map(|x| x.value.clone())
        .collect();

    let path_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == PATH )
        .map(|x| x.var.clone())
        .collect();

    // for request body TODO constrain only one json body?
    let body_value: Vec<syn::Ident> = fn_args.iter()
        .filter(|x| x.path1 == BODY )
        .map(|x| x.var.clone())
        .collect();

    let return_ty_str = format!("{}", return_ty);
    let is_string = return_ty_str.starts_with("Result < String,");

    let stream = if is_string {
        quote! {
            #(#attrs)*
            #vis #sig {
                use std::str::FromStr;
                use http::Method;
                use url::Url;
                use std::collections::HashMap;
                use mclient;

                let url = format!("{}", #url);

                // process path variable
                let mut path_variables: HashMap<&str,&str> = HashMap::new();
                #(
                    path_variables.insert(#path_name, #path_value);
                )*

                let url = mclient::str_utils::replace_named(url.as_str(), &path_variables);

                // begin build request
                let client = reqwest::Client::new();

                let method = Method::from_str(#method).unwrap();
                let mut reqb = client.request(method, Url::parse(url.as_str()).unwrap());

                #(
                    reqb = reqb.header(#header_name, #header_value);
                )*

                #(
                    reqb = reqb.query(&[(#param_name, #param_value),]);
                )*

                #(
                    reqb = reqb.json(#body_value);
                )*

                let resp: #return_ty  = reqb.send().await?.text().await;

                resp
            }
        }
    } else {
        quote! {
            #(#attrs)*
            #vis #sig {
                use std::str::FromStr;
                use http::Method;
                use url::Url;
                use std::collections::HashMap;
                use mclient;

                let url = format!("{}", #url);

                // process path variable
                let mut path_variables: HashMap<&str,&str> = HashMap::new();
                #(
                    path_variables.insert(#path_name, #path_value);
                )*

                let url = mclient::str_utils::replace_named(url.as_str(), &path_variables);

                // begin build request
                let client = reqwest::Client::new();

                let method = Method::from_str(#method).unwrap();
                let mut reqb = client.request(method, Url::parse(url.as_str()).unwrap());

                #(
                    reqb = reqb.header(#header_name, #header_value);
                )*

                #(
                    reqb = reqb.query(&[(#param_name, #param_value),]);
                )*

                #(
                    reqb = reqb.json(#body_value);
                )*

                let resp: #return_ty  = reqb.send().await?.json().await;

                resp
            }
        }
    };

    //println!("............gen macro get :\n {}", stream);

    stream.into()
}