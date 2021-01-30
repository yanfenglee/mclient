// mod route;
extern crate proc_macro;

use proc_macro::{TokenStream};
use proc_macro2::{Ident, Span};

use quote::{quote, ToTokens};
use syn::{ItemFn, FnArg, ReturnType};
use crate::utils::parse_fn_args;


pub(crate) fn get_fn_args(target_fn: &mut ItemFn) -> Vec<Ident> {
    let aa = parse_fn_args(target_fn);

    for a in aa {
        println!("test: {:?}", a);
    }

    let mut fn_arg_ident_vec = vec![];
    for arg in &target_fn.sig.inputs {
        match arg {
            FnArg::Typed(t) => {
                let arg_name = format!("{}", t.pat.to_token_stream());

                if let [att,..] = t.attrs.as_slice() {
                    println!("arg attrs: {:?}", att);
                }

                let ident = Ident::new(&arg_name, Span::call_site());
                fn_arg_ident_vec.push(ident);
            }
            _ => {}
        }
    }

    fn_arg_ident_vec
}

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


pub(crate) fn get_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);


    let fn_args = parse_fn_args(&mut input);

    let return_ty = find_return_type(&input);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;

    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let path = args.get(0).unwrap().to_token_stream();

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig.fn_token, "only async fn is supported")
            .to_compile_error()
            .into();
    }

    //let header = fn_args.
    let stream = quote! {

        #(#attrs)*
        #vis #sig {
            let path = format!("{}", #path);
            let client = reqwest::Client::new();

            let mut reqb = client.get(&path);

            // #(
            //     reqb = reqb.query(&[(stringify!(#fn_args.var), #fn_args.var),]);
            // )*


            let resp: #return_ty = reqb.send().await?.json().await;

            resp
        }
    };

    println!("............gen macro get :\n {}", stream);

    stream.into()
}

pub(crate) fn post_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);


    let fn_arg = get_fn_args(&mut input).get(0).expect("must have body").clone();

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;

    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let path = args.get(0).unwrap().to_token_stream();

    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig.fn_token, "only async fn is supported")
            .to_compile_error()
            .into();
    }

    let return_ty = find_return_type(&input);
    let type_s = format!("{}", return_ty);

    let stream = if type_s.starts_with("Result < String,") {
        quote! {
            #(#attrs)*
            #vis #sig {
                let path = format!("{}", #path);
                let client = reqwest::Client::new();

                let mut reqb = client.post(&path);
                reqb = reqb.json(#fn_arg);

                let resp: #return_ty = reqb.send().await?.text().await;

                resp
            }
        }
    } else {
        quote! {
            #(#attrs)*
            #vis #sig {
                let path = format!("{}", #path);
                let client = reqwest::Client::new();

                let mut reqb = client.post(&path);
                reqb = reqb.json(#fn_arg);

                let resp: #return_ty = reqb.send().await?.json().await;

                resp
            }
        }
    };

    println!("............gen macro post2:\n {}", stream);

    stream.into()
}
