use syn::{Attribute, ItemFn, FnArg, NestedMeta::Meta, NestedMeta::Lit};
use crate::symbol::{PATH, HEADER, PARAM, Symbol};
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::Meta::NameValue;
use syn::Meta::List;

/// arg attrs: Attribute { pound_token: Pound, style: Outer, bracket_token: Bracket, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "path", span: #0 bytes(802..806) }, arguments: None }] }, tokens: TokenStream [Group { delimiter: Parenthesis, stream: TokenStream [Literal { kind: Str, symbol: "pathtest", suffix: None, span: #0 bytes(807..817) }], span: #0 bytes(806..818) }] }
/// arg attrs: Attribute { pound_token: Pound, style: Outer, bracket_token: Bracket, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "header", span: #0 bytes(834..840) }, arguments: None }] }, tokens: TokenStream [Group { delimiter: Parenthesis, stream: TokenStream [Literal { kind: Str, symbol: "x-token", suffix: None, span: #0 bytes(841..850) }], span: #0 bytes(840..851) }] }


pub(crate) fn parse_fn_args(item: &ItemFn) -> Vec<ReqArgAttr> {

    let mut attrs = Vec::new();

    for arg in &item.sig.inputs {
        if let Some(arg_attr) = parse_one_arg(arg) {
            attrs.push(arg_attr);
        }
    }

    attrs
}

#[derive(Debug)]
pub(crate) struct ReqArgAttr {
    path1: Option<syn::Path>,
    path2: Option<syn::Path>,
    value: String,
    var: syn::Ident,
}

pub(crate) fn parse_one_arg(arg: &FnArg) -> Option<ReqArgAttr>{
    if let FnArg::Typed(pat) = arg {
        let var_name = format!("{}", pat.to_token_stream());
        let ident = Ident::new("aaa", Span::call_site());

        for meta_item in pat.attrs.iter()
            .flat_map(|attr| {
                println!("attr path: {:?}", attr.path);
                if attr.path == HEADER {
                    println!("++++++++++header")
                }
                get_meta_items(attr)
            })
            .flatten() {

            match &meta_item {
                Meta(NameValue(m)) => {
                    if let syn::Lit::Str(lit) = &m.lit {
                        return Some(ReqArgAttr {
                            path1: None,
                            path2: Some(m.path.clone()),
                            value: lit.value(),
                            var: ident,
                        });
                    }
                },
                Lit(lit) => {
                    if let syn::Lit::Str(lit) = &lit {
                        return Some(ReqArgAttr {
                            path1: None,
                            path2: None,
                            value: lit.value(),
                            var: ident
                        });
                    }
                },

                _ => {
                    println!("unknown: {:#?}", meta_item);
                }
            }
        }
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