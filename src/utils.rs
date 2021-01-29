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
    path1: Option<Symbol>,
    path2: Option<Symbol>,
    value: String,
    var: syn::Ident,
}

pub(crate) fn to_symbol(path: &syn::Path) -> Option<Symbol> {
    if path == HEADER {
        Some(HEADER)
    } else if path == PATH {
        Some(PATH)
    } else if path == PARAM {
        Some(PARAM)
    } else {
        None
    }
}

pub(crate) fn parse_one_arg(arg: &FnArg) -> Option<ReqArgAttr> {
    if let FnArg::Typed(pat) = arg {
        let var_name = format!("{}", pat.to_token_stream());
        let ident = Ident::new("aaa", Span::call_site());

        for att in pat.attrs.iter() {
            println!("attr path: {:?}", att.path);
            if att.path == HEADER {
                println!("++++++++++header")
            }

            for meta_item in get_meta_items(att).unwrap() {
                match &meta_item {
                    Meta(NameValue(m)) => {
                        if let syn::Lit::Str(lit) = &m.lit {
                            return Some(ReqArgAttr {
                                path1: to_symbol(&att.path),
                                path2: to_symbol(&m.path),
                                value: lit.value(),
                                var: ident,
                            });
                        }
                    }
                    Lit(lit) => {
                        if let syn::Lit::Str(lit) = &lit {
                            return Some(ReqArgAttr {
                                path1: to_symbol(&att.path),
                                path2: None,
                                value: lit.value(),
                                var: ident,
                            });
                        }
                    }

                    _ => {
                        println!("unknown: {:#?}", meta_item);
                    }
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