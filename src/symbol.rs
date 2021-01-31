use std::fmt::{self, Display};
use syn::{Ident, Path};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Symbol(&'static str);

/// function argument tags
pub const HEADER: Symbol = Symbol("header");
pub const PARAM: Symbol = Symbol("param");
pub const PATH: Symbol = Symbol("path");
pub const BODY: Symbol = Symbol("body");

/// support http method tags
pub const GET: Symbol = Symbol("get");
pub const POST: Symbol = Symbol("post");
pub const PUT: Symbol = Symbol("put");
pub const DELETE: Symbol = Symbol("delete");


impl PartialEq<Symbol> for Ident {
    fn eq(&self, word: &Symbol) -> bool {
        self == word.0
    }
}

impl<'a> PartialEq<Symbol> for &'a Ident {
    fn eq(&self, word: &Symbol) -> bool {
        *self == word.0
    }
}

impl PartialEq<Symbol> for Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl<'a> PartialEq<Symbol> for &'a Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
