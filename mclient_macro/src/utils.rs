use syn::{ItemFn, FnArg, NestedMeta::Meta, NestedMeta::Lit};
use crate::symbol::{PATH, HEADER, PARAM, Symbol, BODY};
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::Meta::NameValue;
use syn::Meta::List;


pub(crate) fn parse_fn_args(item: &mut ItemFn) -> Vec<ReqArgAttr> {
    let mut attrs = Vec::new();

    for arg in &mut item.sig.inputs {
        if let Some(arg_attr) = parse_one_arg(arg) {
            attrs.push(arg_attr);
        }
    }

    attrs
}

#[derive(Debug, Clone)]
pub(crate) struct ReqArgAttr {
    pub path1: Symbol,
    pub path2: Option<Symbol>,
    pub value: String,
    pub var: syn::Ident,
}

#[derive(Debug, Clone)]
pub(crate) struct GenParam {
    pub url: String,
    pub method: String,
    pub item_fn: ItemFn,
}

pub(crate) fn to_symbol(path: &syn::Path) -> Option<Symbol> {
    if path == HEADER {
        Some(HEADER)
    } else if path == PATH {
        Some(PATH)
    } else if path == PARAM {
        Some(PARAM)
    } else if path == BODY {
        Some(BODY)
    } else {
        None
    }
}

pub(crate) fn parse_one_arg(arg: &mut FnArg) -> Option<ReqArgAttr> {
    let mut container = Vec::new();

    if let FnArg::Typed(pt) = arg {
        let attrs = pt.attrs.clone();
        pt.attrs.clear();

        let var_name = format!("{}", pt.pat.to_token_stream());
        let ident = Ident::new(&var_name, Span::call_site());

        for att in attrs.iter() {

            let mut req_arg = ReqArgAttr {
                path1: to_symbol(&att.path)?,
                path2: None,
                value: var_name.clone(),
                var: ident.clone(),
            };

            for meta_item in get_meta_items(att).unwrap() {
                match &meta_item {
                    Meta(NameValue(m)) => {
                        if let syn::Lit::Str(lit) = &m.lit {
                            req_arg.value = lit.value();
                            req_arg.path2 = to_symbol(&m.path);
                            break;
                        }
                    }

                    Lit(lit) => {
                        if let syn::Lit::Str(lit) = &lit {
                            req_arg.value = lit.value();
                            req_arg.path2 = None;
                            break;
                        }
                    }

                    _ => {
                        println!("unknown type");
                    }
                }
            }

            container.push(req_arg);
        }

    };

    if let Some(res) = container.first() {
        return Some(res.clone());
    }

    None
}

pub(crate) fn get_meta_items(attr: &syn::Attribute) -> Result<Vec<syn::NestedMeta>, ()> {
    match attr.parse_meta() {
        Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
        Ok(_other) => {
            //todo log
            Ok(Vec::new())
        }
        Err(_err) => {
            //todo log
            Ok(Vec::new())
        }
    }
}